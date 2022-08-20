use std::collections::BTreeSet;

use digraph::git::{
    FetchTopicLiveSearch, FetchTopicLiveSearchResult, FindMatches, FindMatchesResult, Kind, Object,
    Search, SearchMatch, SortKey,
};
use digraph::prelude::WIKI_ROOT_TOPIC_PATH;
use digraph::prelude::*;

use super::{actor, Fixtures};

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
            prefixes: vec![RepoPrefix::wiki()],
            search: Search::parse("existing non-root topic").unwrap(),
            viewer: actor(),
        }
        .call(&f.git)
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
        let repo = RepoPrefix::wiki();
        let parent = PathSpec::try_from(WIKI_ROOT_TOPIC_PATH).unwrap();
        let search = Search::parse("clim chan soc").unwrap();

        let FetchTopicLiveSearchResult {
            synonym_matches: matches,
            ..
        } = FetchTopicLiveSearch {
            limit: 10,
            prefixes: vec![repo.to_owned()],
            search: search.clone(),
            viewer: actor(),
        }
        .call(&f.git)
        .unwrap();

        assert!(matches.is_empty());

        f.upsert_topic(
            &parent.repo,
            "Climate change and society",
            &parent,
            OnMatchingSynonym::Ask,
        )
        .unwrap();

        let FetchTopicLiveSearchResult {
            synonym_matches: matches,
            ..
        } = FetchTopicLiveSearch {
            limit: 10,
            prefixes: vec![repo],
            search,
            viewer: actor(),
        }
        .call(&f.git)
        .unwrap();

        assert_eq!(matches.len(), 1);
    }

    // #[test]
    // fn excludes_ancestors() {
    //     todo!()
    // }
}

#[cfg(test)]
mod fetch_matches {
    use digraph::git::Client;
    use digraph::types::{Downset, ReadPath, Timespec};
    use std::collections::HashSet;

    use crate::git::valid_url;

    use super::*;

    struct FetchDownset(Client);

    impl Downset for FetchDownset {
        fn intersection(&self, topic_paths: &[ReadPath]) -> Result<HashSet<String>> {
            if topic_paths.is_empty() {
                return Ok(HashSet::new());
            }

            let (head, tail) = topic_paths.split_at(1);
            match head.get(0) {
                Some(path) => {
                    let mut set = self.downset(path);
                    for other_path in tail {
                        let other = self.downset(other_path);
                        set.retain(|path| other.contains(path));
                    }
                    Ok(set)
                }

                None => Ok(HashSet::new()),
            }
        }

        fn downset(&self, path: &ReadPath) -> HashSet<String> {
            self.0.downset(path).collect::<HashSet<String>>()
        }
    }

    fn root() -> PathSpec {
        PathSpec::try_from(WIKI_ROOT_TOPIC_PATH).unwrap()
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
        topic_path: &PathSpec,
        input: &str,
        recursive: bool,
    ) -> BTreeSet<SearchMatch> {
        let fetcher = FetchDownset(f.git.clone());
        let search = Search::parse(input).unwrap();
        let viewer = actor();

        let FindMatchesResult { matches } = FindMatches {
            limit: 100,
            locale: Locale::EN,
            repos: viewer.read_repos.to_owned(),
            recursive,
            search,
            timespec: Timespec,
            topic_path: topic_path.to_owned(),
            viewer,
        }
        .call(&f.git, &fetcher)
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

        let fetcher = FetchDownset(f.git.clone());
        let root = root();
        let climate_change = f.topic_path("Climate change and weather").unwrap().unwrap();
        let query = format!("in:{}", climate_change);
        let search = Search::parse(&query).unwrap();

        assert!(search.urls.is_empty());

        let FindMatchesResult { matches } = FindMatches {
            limit: 3,
            locale: Locale::EN,
            recursive: true,
            repos: RepoList::from(&vec![root.repo.to_owned()]),
            search,
            timespec: Timespec,
            topic_path: root,
            viewer: actor(),
        }
        .call(&f.git, &fetcher)
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

    #[test]
    fn search_works_across_prefixes() {
        let f = Fixtures::copy("simple");
        let repo = RepoPrefix::try_from("/other/").unwrap();

        let result = f.upsert_link(&repo, &valid_url(), Some("Other repo".to_owned()), None);
        assert_eq!(result.link.unwrap().path().unwrap().repo, repo);

        let matches = search(&f, &root(), "other repo", true);
        assert!(!matches.is_empty());
        let row = matches.iter().next().unwrap();

        let SearchMatch { sort_key, object } = row;

        assert_eq!(
            sort_key,
            &SortKey(Kind::Link, false, "Other repo".to_owned())
        );

        if let Object::Link(link) = &object {
            assert_eq!(link.title(), "Other repo");
        }
    }
}
