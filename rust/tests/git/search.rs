use super::{actor, upsert_topic, Fixtures};

#[cfg(test)]
mod fetch_topic_live_search {
    use super::*;
    use digraph::git::{
        FetchTopicLiveSearch, FetchTopicLiveSearchResult, OnMatchingSynonym, Search, SynonymEntry,
    };
    use digraph::prelude::*;

    #[test]
    fn returns_matches() {
        let f = Fixtures::copy("simple");

        let FetchTopicLiveSearchResult {
            synonym_matches: matches,
            ..
        } = FetchTopicLiveSearch {
            prefixes: vec!["/wiki".to_owned()],
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
            prefixes: vec!["/wiki".to_owned()],
            search: search.clone(),
            viewer: actor(),
        }
        .call(&f.repo.git)
        .unwrap();

        assert!(matches.is_empty());

        upsert_topic(
            &f,
            "Climate change and society",
            parent,
            OnMatchingSynonym::Ask,
        )
        .unwrap();

        let FetchTopicLiveSearchResult {
            synonym_matches: matches,
            ..
        } = FetchTopicLiveSearch {
            prefixes: vec!["/wiki".to_owned()],
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
    use super::*;
    use digraph::git::{
        Kind, Object, Search, SearchMatch, SearchWithinTopic, SearchWithinTopicResult,
    };
    use digraph::prelude::WIKI_ROOT_TOPIC_PATH;
    use digraph::prelude::*;
    use digraph::Locale;

    #[test]
    fn matching_topics() {
        let f = Fixtures::copy("simple");
        let search = Search::parse("exist non root topic").unwrap();

        let SearchWithinTopicResult { matches } = SearchWithinTopic {
            locale: Locale::EN,
            prefixes: vec!["/wiki".to_owned()],
            search,
            topic_path: RepoPath::from(WIKI_ROOT_TOPIC_PATH),
            viewer: actor(),
        }
        .call(&f.repo.git)
        .unwrap();

        assert!(!matches.is_empty());
        let row = matches.iter().next().unwrap();

        let SearchMatch { sort_key, object } = row;

        assert_eq!(
            sort_key,
            &(true, Kind::Topic, "Existing non-root topic".to_owned())
        );

        if let Object::Topic(topic) = &object {
            assert_eq!(topic.name(Locale::EN), "Existing non-root topic");
        }
    }

    // #[test]
    // fn topic_search() {
    //     todo!()
    // }

    // #[test]
    // fn result_order() {
    //     todo!()
    // }

    // #[test]
    // fn url_search() {
    //     todo!()
    // }
}
