use std::collections::BTreeSet;

use digraph::git::{
    FetchTopicLiveSearch, FetchTopicLiveSearchResult, Kind, Object, Search, SearchMatch,
    SearchWithinTopic, SearchWithinTopicResult, SortKey,
};
use digraph::prelude::WIKI_ROOT_TOPIC_PATH;
use digraph::prelude::*;

use super::{actor, upsert_topic, Fixtures};

#[cfg(test)]
mod fetch_topic_live_search {
    use super::*;
    use digraph::git::{OnMatchingSynonym, Search, SynonymEntry};

    #[test]
    fn returns_matches() {
        let f = Fixtures::copy("simple");

        let FetchTopicLiveSearchResult {
            synonym_matches: matches,
            ..
        } = FetchTopicLiveSearch {
            limit: 10,
            prefixes: vec![RepoPrefix::from("/wiki/")],
            search: Search::parse("existing non-root topic").unwrap(),
            viewer: actor(),
        }
        .call(&f.repo.git)
        .unwrap();

        assert_eq!(
            matches.iter().next().unwrap(),
            &SynonymEntry {
                name: "Existing non-root topic".to_owned(),
                path: "/wiki/00002".to_owned(),
            }
        );
    }

    #[test]
    fn indexing_works() {
        let f = Fixtures::copy("simple");
        let parent = RepoPath::from(WIKI_ROOT_TOPIC_PATH);
        let search = Search::parse("clim chan soc").unwrap();

        let FetchTopicLiveSearchResult {
            synonym_matches: matches,
            ..
        } = FetchTopicLiveSearch {
            limit: 10,
            prefixes: vec![RepoPrefix::from("/wiki/")],
            search: search.clone(),
            viewer: actor(),
        }
        .call(&f.repo.git)
        .unwrap();

        assert!(matches.is_empty());

        upsert_topic(
            &f,
            "Climate change and society",
            &parent,
            &parent.prefix,
            OnMatchingSynonym::Ask,
        )
        .unwrap();

        let FetchTopicLiveSearchResult {
            synonym_matches: matches,
            ..
        } = FetchTopicLiveSearch {
            limit: 10,
            prefixes: vec![RepoPrefix::from("/wiki/")],
            search,
            viewer: actor(),
        }
        .call(&f.repo.git)
        .unwrap();

        assert_eq!(matches.len(), 1);
    }

    // #[test]
    // fn excludes_ancestors() {
    //     todo!()
    // }
}

#[cfg(test)]
mod search_within_topic {
    use digraph::git::Git;
    use digraph::types::DownSet;
    use std::collections::HashSet;

    use super::*;

    struct FetchDownSet(Git);

    impl DownSet for FetchDownSet {
        fn transitive_closure(&self, topic_paths: &[&RepoPath]) -> Result<HashSet<String>> {
            let (head, tail) = topic_paths.split_at(1);
            match head.get(0) {
                Some(path) => {
                    let mut set = self.down_set(path);
                    for other_path in tail {
                        let other = self.down_set(other_path);
                        set.retain(|path| other.contains(path));
                    }
                    Ok(set)
                }

                None => Ok(HashSet::new()),
            }
        }

        fn down_set(&self, path: &RepoPath) -> HashSet<String> {
            self.0
                .topic_down_set(path)
                .map(|topic| topic.metadata.path)
                .collect::<HashSet<String>>()
        }
    }

    fn root() -> RepoPath {
        RepoPath::from(WIKI_ROOT_TOPIC_PATH)
    }

    fn count(kind: Kind, matches: &BTreeSet<SearchMatch>) -> usize {
        matches
            .iter()
            .filter(|m| match kind {
                Kind::Link => matches!(m.object, Object::Link(_)),
                Kind::Topic => matches!(m.object, Object::Topic(_)),
            })
            .count()
    }

    fn search(
        f: &Fixtures,
        topic_path: &RepoPath,
        input: &str,
        recursive: bool,
    ) -> BTreeSet<SearchMatch> {
        let fetcher = FetchDownSet(f.repo.git.clone());
        let search = Search::parse(input).unwrap();

        let SearchWithinTopicResult { matches } = SearchWithinTopic {
            limit: 100,
            locale: Locale::EN,
            prefixes: vec![RepoPrefix::from("/wiki/")],
            recursive,
            search,
            topic_path: topic_path.to_owned(),
            viewer: actor(),
        }
        .call(&f.repo.git, &fetcher)
        .unwrap();

        matches
    }

    #[test]
    fn matching_topics() {
        let f = Fixtures::copy("simple");

        let matches = search(&f, &root(), "exist non root topic", true);
        assert!(!matches.is_empty());
        let row = matches.iter().next().unwrap();

        let SearchMatch { sort_key, object } = row;

        assert_eq!(
            sort_key,
            &SortKey(Kind::Topic, true, "Existing non-root topic".to_owned())
        );

        if let Object::Topic(topic) = &object {
            assert_eq!(topic.name(Locale::EN), "Existing non-root topic");
        }
    }

    // Relevant structure of /wiki repo in the simple fixture:
    //
    //   - Everything
    //     - Climate change
    //       - Climate change and weather
    //         - https://climate.nasa.gov/news/3184/a-force-of-nature-hurricanes-in-a-changing-climate/
    //         - https://royalsociety.org/topics-policy/projects/climate-change-evidence-causes/question-13/
    //       - https://en.wikipedia.org/wiki/Climate_change
    //     - Weather
    //       - Climate change and weather
    //         - https://climate.nasa.gov/news/3184/a-force-of-nature-hurricanes-in-a-changing-climate/
    //         - https://royalsociety.org/topics-policy/projects/climate-change-evidence-causes/question-13/
    //       - https://en.wikipedia.org/wiki/Weather
    //     - https://climate.nasa.gov/resources/global-warming-vs-climate-change/
    //
    #[test]
    fn topic_search() {
        let f = Fixtures::copy("simple");

        let root = root();
        let climate_change = f.topic_path("Climate change").unwrap().unwrap();
        let query = format!("in:{}", climate_change);

        let matches = search(&f, &root, &query, true);
        assert_eq!(count(Kind::Topic, &matches), 2);
        assert_eq!(count(Kind::Link, &matches), 3);
    }

    #[test]
    fn combined_search() {
        let f = Fixtures::copy("simple");

        let root = root();
        let climate_change = f.topic_path("Climate change").unwrap().unwrap();
        let query = format!("in:{} frequency", climate_change);

        // A link should be returned, since it matches the token search and is a child of the topic
        // included in the search.  The topic should not be included, because it doesn't match the
        // token search.
        let matches = search(&f, &root, &query, true);
        assert_eq!(count(Kind::Topic, &matches), 0);
        assert_eq!(count(Kind::Link, &matches), 1);
    }

    #[test]
    fn result_size_and_order() {
        let f = Fixtures::copy("simple");

        let fetcher = FetchDownSet(f.repo.git.clone());
        let root = root();
        let climate_change = f.topic_path("Climate change and weather").unwrap().unwrap();
        let query = format!("in:{}", climate_change);
        let search = Search::parse(&query).unwrap();

        assert!(search.urls.is_empty());

        let SearchWithinTopicResult { matches } = SearchWithinTopic {
            limit: 3,
            locale: Locale::EN,
            prefixes: vec![root.prefix.to_owned()],
            recursive: true,
            search,
            topic_path: root,
            viewer: actor(),
        }
        .call(&f.repo.git, &fetcher)
        .unwrap();

        assert_eq!(count(Kind::Topic, &matches), 1);
        assert_eq!(count(Kind::Link, &matches), 2);
    }

    #[test]
    fn topic_used_in_search_appears_at_top() {
        let f = Fixtures::copy("simple");

        let root = root();
        let weather = f.topic_path("Weather").unwrap().unwrap();
        let query = format!("in:{}", weather);
        let matches = search(&f, &root, &query, true);

        match &matches.iter().next().unwrap().object {
            Object::Topic(topic) => assert_eq!(topic.name(Locale::EN), "Weather"),
            Object::Link(_) => unreachable!(),
        }
    }

    #[test]
    fn url_search() {
        let f = Fixtures::copy("simple");
        let root = root();

        let matches = search(
            &f,
            &root,
            "https://en.wikipedia.org/wiki/Climate_change",
            true,
        );
        assert_eq!(count(Kind::Topic, &matches), 0);
        assert_eq!(count(Kind::Link, &matches), 1);
    }
}
