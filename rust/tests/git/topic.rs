use super::{actor, upsert_topic, Fixtures};
use digraph::prelude::*;
use digraph::redis;
use std::collections::BTreeSet;

#[cfg(test)]
mod delete_topic {
    use super::*;
    use digraph::git::{DeleteTopic, DeleteTopicResult};

    #[test]
    fn topic_deleted() {
        let f = Fixtures::copy("simple");
        let path = RepoPath::from("/wiki/00001");
        assert!(f.repo.git.exists(&path).unwrap());

        let DeleteTopicResult {
            deleted_topic_path, ..
        } = DeleteTopic {
            actor: actor(),
            topic_path: path.clone(),
        }
        .call(&f.repo.git, &redis::Noop)
        .unwrap();

        assert_eq!(path, deleted_topic_path);
        assert!(!f.repo.git.exists(&path).unwrap());
    }

    #[test]
    fn parent_topics_updated() {
        let f = Fixtures::copy("simple");
        let path = RepoPath::from("/wiki/00001");
        let parent = f.repo.git.fetch_topic(WIKI_ROOT_TOPIC_PATH).unwrap();
        assert!(parent.has_child(&path));

        DeleteTopic {
            actor: actor(),
            topic_path: path.clone(),
        }
        .call(&f.repo.git, &redis::Noop)
        .unwrap();

        let parent = f.repo.git.fetch_topic(WIKI_ROOT_TOPIC_PATH).unwrap();
        assert!(!parent.has_child(&path));
    }

    #[test]
    fn cannot_delete_root_topic() {
        let f = Fixtures::copy("simple");
        let topic = f.repo.git.fetch_topic(WIKI_ROOT_TOPIC_PATH).unwrap();
        assert!(topic.metadata.root);

        let result = DeleteTopic {
            actor: actor(),
            topic_path: topic.path(),
        }
        .call(&f.repo.git, &redis::Noop);

        assert!(matches!(result, Err(Error::Repo(_))));
        let topic = f.repo.git.fetch_topic(WIKI_ROOT_TOPIC_PATH).unwrap();
        assert!(topic.metadata.root);
    }
}

#[cfg(test)]
mod delete_topic_timerange {
    use super::*;
    use digraph::git::{RemoveTopicTimerange, UpsertTopicTimerange};
    use digraph::types::{Timerange, TimerangePrefixFormat};

    #[test]
    fn timerange_deleted() {
        let f = Fixtures::copy("simple");
        let path = RepoPath::from("/wiki/00001");

        UpsertTopicTimerange {
            actor: actor(),
            timerange: Timerange {
                prefix_format: TimerangePrefixFormat::StartYearMonth,
                starts: chrono::Utc::now(),
            },
            topic_path: path.clone(),
        }
        .call(&f.repo.git, &redis::Noop)
        .unwrap();

        let topic = f.repo.git.fetch_topic(&path.inner).unwrap();
        assert!(topic.metadata.timerange.is_some());

        RemoveTopicTimerange {
            actor: actor(),
            topic_path: path.clone(),
        }
        .call(&f.repo.git, &redis::Noop)
        .unwrap();

        let topic = f.repo.git.fetch_topic(&path.inner).unwrap();
        assert!(topic.metadata.timerange.is_none());
    }
}

#[cfg(test)]
mod upsert_topic {
    use super::*;
    use digraph::git::{OnMatchingSynonym, Search};

    #[test]
    fn topic_added() {
        let f = Fixtures::copy("simple");
        let search = Search::parse("topic name").unwrap();

        let result = upsert_topic(
            &f,
            "Topic name",
            RepoPath::from("/wiki/00001"),
            OnMatchingSynonym::Ask,
        )
        .unwrap();

        assert!(result.saved);
        assert_eq!(result.matching_synonyms, BTreeSet::new());

        let topic = &result.topic;
        assert!(topic.is_some());

        let topic = (*topic).clone().unwrap();
        assert!(f
            .repo
            .appears_in(&search, &topic.to_search_entry())
            .unwrap());
    }

    #[test]
    fn action_requested() {
        let f = Fixtures::copy("simple");

        let result = upsert_topic(
            &f,
            "Topic name",
            RepoPath::from("/wiki/00001"),
            OnMatchingSynonym::Ask,
        )
        .unwrap();
        assert!(result.saved);

        let result = upsert_topic(
            &f,
            "Topic Name",
            RepoPath::from("/wiki/00001"),
            OnMatchingSynonym::Ask,
        )
        .unwrap();

        assert!(result.topic.is_none());
        assert!(!result.saved);
        assert_ne!(result.matching_synonyms, BTreeSet::new());
    }

    #[test]
    fn update_topic() {
        let f = Fixtures::copy("simple");

        let result = upsert_topic(
            &f,
            "Topic name",
            RepoPath::from("/wiki/00001"),
            OnMatchingSynonym::Ask,
        )
        .unwrap();
        assert!(result.saved);
        let path = RepoPath::from(&result.topic.unwrap().metadata.path);

        let result = upsert_topic(
            &f,
            "Topic Name",
            RepoPath::from("/wiki/00002"),
            OnMatchingSynonym::Update(path),
        )
        .unwrap();

        assert!(result.topic.is_some());
        assert!(result.saved);

        let parent_topics = result
            .topic
            .unwrap()
            .parent_topics
            .iter()
            .map(|topic| topic.path.to_owned())
            .collect::<Vec<String>>();

        assert_eq!(parent_topics, &["/wiki/00001", "/wiki/00002"]);
    }

    #[test]
    fn create_distinct() {
        let f = Fixtures::copy("simple");

        let result = upsert_topic(
            &f,
            "Topic name",
            RepoPath::from("/wiki/00001"),
            OnMatchingSynonym::Ask,
        )
        .unwrap();
        assert!(result.saved);
        let path1 = &result.topic.unwrap().metadata.path;

        let result = upsert_topic(
            &f,
            "Topic Name",
            RepoPath::from("/wiki/00002"),
            OnMatchingSynonym::CreateDistinct,
        )
        .unwrap();

        assert!(result.topic.is_some());
        assert!(result.saved);
        let path2 = &result.topic.unwrap().metadata.path;

        assert_ne!(path1, path2);

        let matches = f
            .repo
            .git
            .synonym_phrase_matches(&["/wiki"], "Topic name")
            .unwrap();
        let mut names = matches
            .iter()
            .map(|m| m.entry.name.to_owned())
            .collect::<Vec<String>>();
        names.sort();
        assert_eq!(names, vec!["Topic Name", "Topic name"]);
    }

    #[test]
    fn parent_topic_updated() {
        let f = Fixtures::copy("simple");
        let parent = f.repo.git.fetch_topic("/wiki/00001").unwrap();
        assert_eq!(parent.children, BTreeSet::new());

        let result = upsert_topic(
            &f,
            "Topic name",
            RepoPath::from("/wiki/00001"),
            OnMatchingSynonym::Ask,
        )
        .unwrap();
        assert!(result.saved);
        let child_path = &result.topic.unwrap().metadata.path;

        let parent = f.repo.git.fetch_topic("/wiki/00001").unwrap();
        let children = parent
            .children
            .iter()
            .map(|child| child.path.to_owned())
            .collect::<Vec<String>>();

        assert_eq!(children, vec![child_path.to_string()]);
    }

    #[test]
    fn no_cycles() {
        let f = Fixtures::copy("simple");
        let parent = f.repo.git.fetch_topic(WIKI_ROOT_TOPIC_PATH).unwrap();
        let path = RepoPath::from("/wiki/00001");
        let child = f.repo.git.fetch_topic(&path.inner).unwrap();
        assert!(parent.has_child(&path));

        let result = upsert_topic(
            &f,
            "Everything",
            child.path(),
            OnMatchingSynonym::Update(RepoPath::from(WIKI_ROOT_TOPIC_PATH)),
        )
        .unwrap();
        assert!(!result.saved);

        let alert = result.alerts.first().unwrap();
        assert!(matches!(alert, Alert::Warning(_)));

        let synonym_match = result.matching_synonyms.iter().next().unwrap();
        assert_eq!(&synonym_match.topic, &parent);
        assert!(synonym_match.cycle);
    }
}

#[cfg(test)]
mod update_topic_parent_topics {
    use super::*;
    use digraph::git::UpdateTopicParentTopics;

    #[test]
    fn parent_topic_added() {
        let f = Fixtures::copy("simple");
        let parent = f.repo.git.fetch_topic("/wiki/00001").unwrap();
        assert_eq!(parent.children, BTreeSet::new());

        let child = f.repo.git.fetch_topic("/wiki/00002").unwrap();
        assert!(!parent.has_child(&child.path()));

        let result = UpdateTopicParentTopics {
            actor: actor(),
            topic_path: child.path(),
            parent_topic_paths: BTreeSet::from([parent.path()]),
        }
        .call(&f.repo.git, &redis::Noop)
        .unwrap();

        assert_eq!(result.topic, child);

        let parent = f.repo.git.fetch_topic("/wiki/00001").unwrap();
        let child = f.repo.git.fetch_topic("/wiki/00002").unwrap();
        assert!(parent.has_child(&child.path()));
    }

    #[test]
    fn parent_topic_removed() {
        let f = Fixtures::copy("simple");
        let parent = f.repo.git.fetch_topic("/wiki/00001").unwrap();
        assert_eq!(parent.children, BTreeSet::new());

        let child = f.repo.git.fetch_topic("/wiki/00002").unwrap();
        assert!(!parent.has_child(&child.path()));

        let result = UpdateTopicParentTopics {
            actor: actor(),
            topic_path: child.path(),
            parent_topic_paths: BTreeSet::from([parent.path()]),
        }
        .call(&f.repo.git, &redis::Noop)
        .unwrap();

        assert_eq!(result.topic, child);

        let parent = f.repo.git.fetch_topic("/wiki/00001").unwrap();
        let child = f.repo.git.fetch_topic("/wiki/00002").unwrap();
        assert!(parent.has_child(&child.path()));
    }

    #[test]
    fn no_orphans() {
        let f = Fixtures::copy("simple");
        let child = f.repo.git.fetch_topic("/wiki/00002").unwrap();

        let result = UpdateTopicParentTopics {
            actor: actor(),
            topic_path: child.path(),
            parent_topic_paths: BTreeSet::new(),
        }
        .call(&f.repo.git, &redis::Noop);

        assert!(matches!(result, Err(Error::Repo(_))));
    }

    #[test]
    fn no_cycles() {
        let f = Fixtures::copy("simple");
        let parent = f.repo.git.fetch_topic(WIKI_ROOT_TOPIC_PATH).unwrap();
        let child = f.repo.git.fetch_topic("/wiki/00001").unwrap();
        assert!(parent.has_child(&child.path()));

        let result = UpdateTopicParentTopics {
            actor: actor(),
            topic_path: parent.path(),
            parent_topic_paths: BTreeSet::from([child.path()]),
        }
        .call(&f.repo.git, &redis::Noop);

        assert!(matches!(result, Err(Error::Repo(_))));
    }
}

#[cfg(test)]
mod update_topic_synonyms {
    use super::*;
    use digraph::git::{
        Kind, Search, SearchEntry, Synonym, UpdateTopicSynonyms, UpdateTopicSynonymsResult,
    };

    fn count(f: &Fixtures, name: &str) -> usize {
        f.repo
            .git
            .synonym_phrase_matches(&["/wiki"], name)
            .unwrap()
            .len()
    }

    fn synonym(name: &str) -> Synonym {
        Synonym {
            added: chrono::Utc::now(),
            locale: Locale::EN,
            name: name.to_owned(),
        }
    }

    #[test]
    fn synonyms_added() {
        let f = Fixtures::copy("simple");
        let path = RepoPath::from("/wiki/00001");
        let topic = f.repo.git.fetch_topic(&path.inner).unwrap();

        assert_eq!(topic.name(Locale::EN), "A topic");
        assert_eq!(topic.metadata.synonyms.len(), 1);

        assert_eq!(count(&f, "A topic"), 1);
        assert_eq!(count(&f, "B topic"), 0);
        assert_eq!(count(&f, "C topic"), 0);

        let UpdateTopicSynonymsResult { topic, .. } = UpdateTopicSynonyms {
            actor: actor(),
            topic_path: path,
            synonyms: vec![synonym("A topic"), synonym("B topic"), synonym("C topic")],
        }
        .call(&f.repo.git, &redis::Noop)
        .unwrap();

        assert_eq!(topic.metadata.synonyms.len(), 3);

        assert_eq!(count(&f, "A topic"), 1);
        assert_eq!(count(&f, "B topic"), 1);
        assert_eq!(count(&f, "C topic"), 1);
    }

    #[test]
    fn synonyms_removed() {
        let f = Fixtures::copy("simple");
        let path = RepoPath::from("/wiki/00001");

        let UpdateTopicSynonymsResult { topic, .. } = UpdateTopicSynonyms {
            actor: actor(),
            topic_path: path.clone(),
            synonyms: vec![synonym("A topic"), synonym("B topic"), synonym("C topic")],
        }
        .call(&f.repo.git, &redis::Noop)
        .unwrap();

        assert_eq!(topic.metadata.synonyms.len(), 3);
        assert_eq!(count(&f, "A topic"), 1);
        assert_eq!(count(&f, "B topic"), 1);
        assert_eq!(count(&f, "C topic"), 1);

        let UpdateTopicSynonymsResult { topic, .. } = UpdateTopicSynonyms {
            actor: actor(),
            topic_path: path,
            synonyms: vec![synonym("C topic")],
        }
        .call(&f.repo.git, &redis::Noop)
        .unwrap();

        assert_eq!(topic.metadata.synonyms.len(), 1);
        assert_eq!(count(&f, "A topic"), 0);
        assert_eq!(count(&f, "B topic"), 0);
        assert_eq!(count(&f, "C topic"), 1);
    }

    #[test]
    fn synonym_added_date() {
        let f = Fixtures::copy("simple");
        let path = RepoPath::from("/wiki/00001");

        let topic = f.repo.git.fetch_topic(&path.inner).unwrap();
        let syn = topic.metadata.synonyms.first().unwrap();
        let added = syn.added;

        UpdateTopicSynonyms {
            actor: actor(),
            topic_path: path.clone(),
            synonyms: vec![synonym(&syn.name)],
        }
        .call(&f.repo.git, &redis::Noop)
        .unwrap();

        let topic = f.repo.git.fetch_topic(&path.inner).unwrap();
        let syn = topic.metadata.synonyms.first().unwrap();
        assert_eq!(syn.added, added);
    }

    #[test]
    fn lookup_indexes_updated() {
        let f = Fixtures::copy("simple");
        let path = RepoPath::from("/wiki/00001");
        let search = Search::parse("topicA").unwrap();
        let entry = SearchEntry {
            path: path.inner.to_owned(),
            kind: Kind::Topic,
        };
        assert!(!f.repo.appears_in(&search, &entry).unwrap());

        UpdateTopicSynonyms {
            actor: actor(),
            topic_path: path.clone(),
            synonyms: vec![synonym("topicA")],
        }
        .call(&f.repo.git, &redis::Noop)
        .unwrap();

        assert!(f.repo.appears_in(&search, &entry).unwrap());

        UpdateTopicSynonyms {
            actor: actor(),
            topic_path: path,
            synonyms: vec![synonym("topicB")],
        }
        .call(&f.repo.git, &redis::Noop)
        .unwrap();

        assert!(!f.repo.appears_in(&search, &entry).unwrap());
    }
}

#[cfg(test)]
mod upsert_topic_timerange {
    use super::*;

    use digraph::git::UpsertTopicTimerange;
    use digraph::types::{Timerange, TimerangePrefixFormat};


    fn count(f: &Fixtures, name: &str) -> usize {
        f.repo
            .git
            .synonym_phrase_matches(&["/wiki"], name)
            .unwrap()
            .len()
    }

    #[test]
    fn timerange_added() {
        let f = Fixtures::copy("simple");
        let path = RepoPath::from("/wiki/00001");

        let topic = f.repo.git.fetch_topic(&path.inner).unwrap();
        assert!(topic.metadata.timerange.is_none());

        UpsertTopicTimerange {
            actor: actor(),
            timerange: Timerange {
                prefix_format: TimerangePrefixFormat::StartYearMonth,
                starts: chrono::Utc::now(),
            },
            topic_path: path.clone(),
        }
        .call(&f.repo.git, &redis::Noop)
        .unwrap();

        let topic = f.repo.git.fetch_topic(&path.inner).unwrap();
        assert!(topic.metadata.timerange.is_some());
    }

    #[test]
    fn synonym_indexes() {
        use chrono::TimeZone;
        let f = Fixtures::copy("simple");
        let path = RepoPath::from("/wiki/00001");
        let date = chrono::Utc.ymd(1970, 1, 1).and_hms(0, 0, 0);

        let topic = f.repo.git.fetch_topic(&path.inner).unwrap();
        assert!(topic.metadata.timerange.is_none());

        assert_eq!(count(&f, "A topic"), 1);
        assert_eq!(count(&f, "1970 A topic"), 0);

        UpsertTopicTimerange {
            actor: actor(),
            timerange: Timerange {
                prefix_format: TimerangePrefixFormat::StartYear,
                starts: date,
            },
            topic_path: path.to_owned(),
        }
        .call(&f.repo.git, &redis::Noop)
        .unwrap();

        assert_eq!(count(&f, "1970 A topic"), 1);
    }
}