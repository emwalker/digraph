use std::collections::BTreeSet;

use digraph::git::{
    FetchTopicLiveSearch, FetchTopicLiveSearchResult, FindMatches, FindMatchesResult, Kind, Search,
    SearchMatch, SortKey,
};
use digraph::prelude::*;

use super::{actor, parse_id, Fixtures};

#[cfg(test)]
mod fetch_topic_live_search {
    use super::*;
    use digraph::git::{OnMatchingSynonym, Search, SynonymEntry};

    #[test]
    fn returns_matches() {
        let f = Fixtures::copy("simple");

        let FetchTopicLiveSearchResult {
            synonyms: matches, ..
        } = FetchTopicLiveSearch {
            limit: 10,
            repos: RepoIds::from(vec![RepoId::wiki()]),
            search: Search::parse("existing non-root topic").unwrap(),
            viewer: actor(),
        }
        .call(&f.git)
        .unwrap();

        assert_eq!(
            matches.iter().next().unwrap(),
            &SynonymEntry {
                name: "Existing non-root topic".to_owned(),
                id: parse_id("00002"),
            }
        );
    }

    #[test]
    fn indexing_works() {
        let f = Fixtures::copy("simple");
        let repo_id = RepoId::wiki();
        let parent = ExternalId::root_topic();
        let search = Search::parse("clim chan soc").unwrap();

        let FetchTopicLiveSearchResult {
            synonyms: matches, ..
        } = FetchTopicLiveSearch {
            limit: 10,
            repos: RepoIds::from(vec![repo_id]),
            search: search.clone(),
            viewer: actor(),
        }
        .call(&f.git)
        .unwrap();

        assert!(matches.is_empty());

        f.upsert_topic(
            repo_id,
            "Climate change and society",
            &parent,
            OnMatchingSynonym::Ask,
        )
        .unwrap();

        let FetchTopicLiveSearchResult {
            synonyms: matches, ..
        } = FetchTopicLiveSearch {
            limit: 10,
            repos: RepoIds::from(vec![repo_id]),
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
    use digraph::types::{Downset, Timespec, TopicPath};
    use std::collections::HashSet;

    use crate::git::valid_url;

    use super::*;

    struct FetchDownset(Client);

    impl Downset for FetchDownset {
        fn intersection(&self, topic_paths: &[TopicPath]) -> Result<HashSet<ExternalId>> {
            if topic_paths.is_empty() {
                return Ok(HashSet::new());
            }

            let (head, tail) = topic_paths.split_at(1);
            match head.first() {
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

        fn downset(&self, path: &TopicPath) -> HashSet<ExternalId> {
            self.0.downset(path).collect::<HashSet<ExternalId>>()
        }
    }

    fn count(kind: Kind, matches: &BTreeSet<SearchMatch>) -> usize {
        matches.iter().filter(|m| m.kind == kind).count()
    }

    fn search(
        f: &Fixtures,
        topic_id: &ExternalId,
        input: &str,
        recursive: bool,
    ) -> BTreeSet<SearchMatch> {
        let fetcher = FetchDownset(f.git.clone());
        let search = Search::parse(input).unwrap();
        let viewer = actor();

        let FindMatchesResult { matches } = FindMatches {
            context_repo_id: RepoId::wiki(),
            limit: 100,
            locale: Locale::EN,
            recursive,
            search,
            timespec: Timespec,
            topic_id: topic_id.to_owned(),
            viewer,
        }
        .call(&f.git, &fetcher)
        .unwrap();

        matches
    }

    #[test]
    fn matching_topics() {
        let f = Fixtures::copy("simple");

        let matches = search(&f, &ExternalId::root_topic(), "exist non root topic", true);
        assert!(!matches.is_empty());
        let row = matches.iter().next().unwrap();

        let SearchMatch {
            sort_key, object, ..
        } = row;

        assert_eq!(
            sort_key,
            &SortKey(
                Kind::Topic,
                true,
                "Existing non-root topic".to_owned(),
                object.id().to_owned()
            )
        );

        assert_eq!(object.display_string(Locale::EN), "Existing non-root topic");
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

        let root = ExternalId::root_topic();
        let climate_change = f.find_topic("Climate change").unwrap();
        let query = format!("in:{climate_change}");

        let matches = search(&f, &root, &query, true);
        assert_eq!(count(Kind::Topic, &matches), 2);
        assert_eq!(count(Kind::Link, &matches), 3);
    }

    #[test]
    fn combined_search() {
        let f = Fixtures::copy("simple");

        let root = ExternalId::root_topic();
        let climate_change = f.find_topic("Climate change").unwrap();
        let query = format!("in:{climate_change} frequency");

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
        let root = ExternalId::root_topic();
        let climate_change = f.find_topic("Climate change and weather").unwrap();
        let query = format!("in:{climate_change}");
        let search = Search::parse(&query).unwrap();

        assert!(search.urls.is_empty());

        let FindMatchesResult { matches } = FindMatches {
            context_repo_id: RepoId::wiki(),
            limit: 3,
            locale: Locale::EN,
            recursive: true,
            search,
            timespec: Timespec,
            topic_id: root,
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

        let root = ExternalId::root_topic();
        let weather = f.find_topic("Weather").unwrap();
        let query = format!("in:{weather}");
        let matches = search(&f, &root, &query, true);

        let object = &matches.iter().next().unwrap().object;
        assert_eq!(object.display_string(Locale::EN), "Weather");
    }

    #[test]
    fn url_search() {
        let f = Fixtures::copy("simple");
        let root = ExternalId::root_topic();

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
        let repo_id = RepoId::other();
        let root_id = ExternalId::root_topic();

        let _ = f.upsert_link(repo_id, &valid_url(), Some("Other repo".to_owned()), None);

        let matches = search(&f, &root_id, "other repo", true);
        assert!(!matches.is_empty());
        let row = matches.iter().next().unwrap();

        let SearchMatch {
            sort_key, object, ..
        } = row;

        assert_eq!(
            sort_key,
            &SortKey(
                Kind::Link,
                false,
                "Other repo".to_owned(),
                object.id().to_owned()
            )
        );

        assert_eq!(object.display_string(Locale::EN), "Other repo");
    }
}
