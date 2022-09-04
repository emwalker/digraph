use geotime::Geotime;

use super::{actor, parse_id, Fixtures};
use digraph::prelude::*;
use digraph::redis;
use std::collections::BTreeSet;

#[cfg(test)]
mod visibility {
    use super::*;
    use digraph::{git::Client, types::Timespec};

    fn viewer(repo_ids: &Vec<RepoId>) -> Viewer {
        let repo_ids = RepoIds::try_from(repo_ids).unwrap();
        Viewer {
            read_repo_ids: repo_ids.to_owned(),
            write_repo_ids: repo_ids,
            session_id: Some("1".to_owned()),
            super_user: false,
            user_id: "2".to_owned(),
        }
    }

    #[test]
    fn viewer_can_read() {
        let f = Fixtures::copy("simple");
        let repo = RepoId::wiki();
        let topic = f.topic(&repo, ROOT_TOPIC_ID);
        let topic_id = topic.id();

        let git = Client::new(&viewer(&vec![RepoId::wiki()]), &f.git.root, Timespec);
        assert!(git.fetch(&repo, topic_id).is_some());

        let git = Client::new(&viewer(&vec![RepoId::other()]), &f.git.root, Timespec);
        assert!(!git.exists(&repo, topic_id).unwrap());
        assert!(git.fetch(&repo, topic_id).is_none());
    }
}

#[cfg(test)]
mod delete_topic {
    use super::*;
    use digraph::git::{
        activity, DeleteTopic, DeleteTopicResult, OnMatchingSynonym, Topic, UpsertTopic,
        UpsertTopicResult,
    };

    #[test]
    fn topic_deleted() {
        let f = Fixtures::copy("simple");
        let repo = RepoId::wiki();
        let topic_id = parse_id("00001");
        assert!(f.git.exists(&repo, &topic_id).unwrap());

        let DeleteTopicResult {
            deleted_topic_id, ..
        } = DeleteTopic {
            actor: actor(),
            repo: repo.to_owned(),
            topic_id: topic_id.to_owned(),
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        assert_eq!(topic_id, deleted_topic_id);
        assert!(!f.git.exists(&repo, &topic_id).unwrap());
    }

    #[test]
    fn parent_topics_updated() {
        let f = Fixtures::copy("simple");
        let repo = RepoId::wiki();
        let path = parse_id("00001");
        let root = Oid::root_topic();
        let parent = f.git.fetch_topic(&repo, &root).unwrap();
        assert!(parent.has_child(&path));

        DeleteTopic {
            actor: actor(),
            repo: repo.to_owned(),
            topic_id: path.clone(),
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        let parent = f.git.fetch_topic(&repo, &root).unwrap();
        assert!(!parent.has_child(&path));
    }

    #[test]
    fn children_added_to_parents() {
        let f = Fixtures::copy("simple");
        let repo = RepoId::wiki();

        let root = f.git.fetch_topic(&repo, &Oid::root_topic()).unwrap();
        let topic_id = f.find_topic("Climate change").unwrap();
        let child_id = f.find_topic("Climate change and weather").unwrap();

        assert!(root.has_child(&topic_id));
        assert!(!root.has_child(&child_id));

        DeleteTopic {
            actor: actor(),
            repo: repo.to_owned(),
            topic_id: topic_id.clone(),
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        let root = f.git.fetch_topic(&repo, &Oid::root_topic()).unwrap();

        assert!(!root.has_child(&topic_id));
        assert!(root.has_child(&child_id));
    }

    #[test]
    fn cannot_delete_root_topic() {
        let f = Fixtures::copy("simple");
        let repo = RepoId::wiki();
        let root = Oid::root_topic();
        let topic = f.git.fetch_topic(&repo, &root).unwrap();
        assert!(topic.root());

        let result = DeleteTopic {
            actor: actor(),
            repo: repo.to_owned(),
            topic_id: topic.id().to_owned(),
        }
        .call(f.mutation(), &redis::Noop);

        assert!(matches!(result, Err(Error::Repo(_))));
        let topic = f.git.fetch_topic(&repo, &root).unwrap();
        assert!(topic.root());
    }

    fn make_topic(f: &Fixtures, parent: &Oid, name: &str) -> Topic {
        let topic_id = parse_id("dPqrU4sZaPkNZEDyr9T68G4RJYV8bncmIXumedBNls9F994v8poSbxTo7dKK3Vhi");

        let UpsertTopicResult { topic, .. } = UpsertTopic {
            actor: actor(),
            locale: Locale::EN,
            name: name.to_owned(),
            repo: RepoId::wiki(),
            on_matching_synonym: OnMatchingSynonym::Update(topic_id),
            parent_topic: parent.to_owned(),
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        topic.unwrap()
    }

    #[test]
    fn change_entries_updated() {
        let f = Fixtures::copy("simple");
        let repo = RepoId::wiki();
        let root = Oid::root_topic();

        let climate_change = make_topic(&f, &root, "Climate change");
        let activity = f.git.fetch_activity(&repo, climate_change.id(), 1).unwrap();
        assert!(!activity.is_empty());

        DeleteTopic {
            actor: actor(),
            repo: repo.to_owned(),
            topic_id: climate_change.id().to_owned(),
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        let activity = f
            .git
            .fetch_activity(&repo, climate_change.id(), 100)
            .unwrap();

        let mut found = false;

        for change in activity {
            if let activity::Change::UpsertTopic(activity::UpsertTopic { upserted_topic, .. }) =
                change
            {
                if &upserted_topic.id == climate_change.id() {
                    assert!(upserted_topic.deleted);
                    found = true;
                }
            }
        }

        assert!(found);
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
        let repo = RepoId::wiki();
        let topic_id = parse_id("00001");

        UpsertTopicTimerange {
            actor: actor(),
            repo_id: repo.to_owned(),
            timerange: Timerange {
                prefix_format: TimerangePrefixFormat::StartYearMonth,
                starts: Geotime::now().into(),
            },
            topic_id: topic_id.to_owned(),
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        let topic = f.git.fetch_topic(&repo, &topic_id).unwrap();
        assert!(topic.timerange().is_some());

        RemoveTopicTimerange {
            actor: actor(),
            repo: repo.to_owned(),
            topic_id: topic_id.to_owned(),
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        let topic = f.git.fetch_topic(&repo, &topic_id).unwrap();
        assert!(topic.timerange().is_none());
    }
}

#[cfg(test)]
mod update_topic_parent_topics {
    use super::*;
    use digraph::git::UpdateTopicParentTopics;

    #[test]
    fn parent_topic_added() {
        let f = Fixtures::copy("simple");
        let repo = RepoId::wiki();
        let parent = f.topic(&repo, "00001");
        assert_eq!(parent.children, BTreeSet::new());

        let child = f.topic(&repo, "00002");
        assert!(!parent.has_child(child.id()));

        let result = UpdateTopicParentTopics {
            actor: actor(),
            repo: repo.to_owned(),
            topic_id: child.id().to_owned(),
            parent_topic_ids: BTreeSet::from([parent.id().to_owned()]),
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        assert_eq!(result.topic, child);

        let parent = f.topic(&repo, "00001");
        let child = f.topic(&repo, "00002");
        assert!(parent.has_child(child.id()));
    }

    #[test]
    fn parent_topic_removed() {
        let f = Fixtures::copy("simple");
        let repo = RepoId::wiki();
        let parent = f.topic(&repo, "00001");
        assert_eq!(parent.children, BTreeSet::new());

        let child = f.topic(&repo, "00002");
        assert!(!parent.has_child(child.id()));

        let result = UpdateTopicParentTopics {
            actor: actor(),
            repo: repo.to_owned(),
            topic_id: child.id().to_owned(),
            parent_topic_ids: BTreeSet::from([parent.id().to_owned()]),
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        assert_eq!(result.topic, child);

        let parent = f.topic(&repo, "00001");
        let child = f.topic(&repo, "00002");
        assert!(parent.has_child(child.id()));
    }

    #[test]
    fn no_orphans() {
        let f = Fixtures::copy("simple");
        let repo = RepoId::wiki();
        let child = f.topic(&repo, "00002");

        let result = UpdateTopicParentTopics {
            actor: actor(),
            repo,
            topic_id: child.id().to_owned(),
            parent_topic_ids: BTreeSet::new(),
        }
        .call(f.mutation(), &redis::Noop);

        assert!(matches!(result, Err(Error::Repo(_))));
    }

    #[test]
    fn no_cycles() {
        let f = Fixtures::copy("simple");
        let repo = RepoId::wiki();
        let parent = f.topic(&repo, ROOT_TOPIC_ID);
        let child = f.topic(&repo, "00001");
        assert!(parent.has_child(child.id()));

        let result = UpdateTopicParentTopics {
            actor: actor(),
            repo,
            topic_id: parent.id().to_owned(),
            parent_topic_ids: BTreeSet::from([child.id().to_owned()]),
        }
        .call(f.mutation(), &redis::Noop);

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
        f.git
            .synonym_phrase_matches(&[&RepoId::wiki()], name)
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
        let repo = RepoId::wiki();
        let topic_id = parse_id("00001");
        let topic = f.git.fetch_topic(&repo, &topic_id).unwrap();

        assert_eq!(topic.name(Locale::EN), "A topic");
        assert_eq!(topic.synonyms().len(), 1);

        assert_eq!(count(&f, "A topic"), 1);
        assert_eq!(count(&f, "B topic"), 0);
        assert_eq!(count(&f, "C topic"), 0);

        let UpdateTopicSynonymsResult { topic, .. } = UpdateTopicSynonyms {
            actor: actor(),
            repo_id: repo,
            topic_id,
            synonyms: vec![synonym("A topic"), synonym("B topic"), synonym("C topic")],
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        assert_eq!(topic.synonyms().len(), 3);

        assert_eq!(count(&f, "A topic"), 1);
        assert_eq!(count(&f, "B topic"), 1);
        assert_eq!(count(&f, "C topic"), 1);
    }

    #[test]
    fn synonyms_deduped() {
        let f = Fixtures::copy("simple");
        let repo = RepoId::wiki();
        let topic_id = parse_id("00001");
        let topic = f.git.fetch_topic(&repo, &topic_id).unwrap();

        assert_eq!(topic.name(Locale::EN), "A topic");
        assert_eq!(topic.synonyms().len(), 1);

        assert_eq!(count(&f, "A topic"), 1);

        let UpdateTopicSynonymsResult { topic, .. } = UpdateTopicSynonyms {
            actor: actor(),
            repo_id: repo,
            topic_id,
            synonyms: vec![synonym("A topic"), synonym("A topic")],
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        assert_eq!(topic.synonyms().len(), 1);

        assert_eq!(count(&f, "A topic"), 1);
    }

    #[test]
    fn synonyms_removed() {
        let f = Fixtures::copy("simple");
        let repo = RepoId::wiki();
        let topic_id = parse_id("00001");

        let UpdateTopicSynonymsResult { topic, .. } = UpdateTopicSynonyms {
            actor: actor(),
            repo_id: repo.to_owned(),
            topic_id: topic_id.clone(),
            synonyms: vec![synonym("A topic"), synonym("B topic"), synonym("C topic")],
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        assert_eq!(topic.synonyms().len(), 3);
        assert_eq!(count(&f, "A topic"), 1);
        assert_eq!(count(&f, "B topic"), 1);
        assert_eq!(count(&f, "C topic"), 1);

        let UpdateTopicSynonymsResult { topic, .. } = UpdateTopicSynonyms {
            actor: actor(),
            repo_id: repo,
            topic_id,
            synonyms: vec![synonym("C topic")],
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        assert_eq!(topic.synonyms().len(), 1);
        assert_eq!(count(&f, "A topic"), 0);
        assert_eq!(count(&f, "B topic"), 0);
        assert_eq!(count(&f, "C topic"), 1);
    }

    #[test]
    fn synonym_added_date() {
        let f = Fixtures::copy("simple");
        let repo = RepoId::wiki();
        let topic_id = parse_id("00001");

        let topic = f.git.fetch_topic(&repo, &topic_id).unwrap();
        let syn = topic.synonyms().first().unwrap();
        let added = syn.added;

        UpdateTopicSynonyms {
            actor: actor(),
            repo_id: repo.to_owned(),
            topic_id: topic_id.clone(),
            synonyms: vec![synonym(&syn.name)],
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        let topic = f.git.fetch_topic(&repo, &topic_id).unwrap();
        let syn = topic.synonyms().first().unwrap();
        assert_eq!(syn.added, added);
    }

    #[test]
    fn lookup_indexes_updated() {
        let f = Fixtures::copy("simple");
        let repo = RepoId::wiki();
        let topic_id = parse_id("00001");
        let search = Search::parse("topicA").unwrap();
        let entry = SearchEntry {
            id: topic_id.to_owned(),
            kind: Kind::Topic,
        };
        assert!(!f.git.appears_in(&repo, &search, &entry).unwrap());

        UpdateTopicSynonyms {
            actor: actor(),
            repo_id: repo.to_owned(),
            topic_id: topic_id.clone(),
            synonyms: vec![synonym("topicA")],
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        assert!(f.git.appears_in(&repo, &search, &entry).unwrap());

        UpdateTopicSynonyms {
            actor: actor(),
            repo_id: repo.to_owned(),
            topic_id,
            synonyms: vec![synonym("topicB")],
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        assert!(!f.git.appears_in(&repo, &search, &entry).unwrap());
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
        let repo = RepoId::wiki();
        let path = parse_id("00001");

        let result = f
            .upsert_topic(&repo, "Topic name", &path, OnMatchingSynonym::Ask)
            .unwrap();

        assert!(result.saved);
        assert_eq!(result.matching_synonyms, BTreeSet::new());

        let topic = &result.topic;
        assert!(topic.is_some());

        let topic = (*topic).clone().unwrap();
        assert!(f
            .git
            .appears_in(&repo, &search, &topic.to_search_entry())
            .unwrap());
    }

    #[test]
    fn action_requested() {
        let f = Fixtures::copy("simple");
        let repo = RepoId::wiki();
        let path = parse_id("00001");

        let result = f
            .upsert_topic(&repo, "Topic name", &path, OnMatchingSynonym::Ask)
            .unwrap();
        assert!(result.saved);

        let result = f
            .upsert_topic(&repo, "Topic Name", &path, OnMatchingSynonym::Ask)
            .unwrap();

        assert!(result.topic.is_none());
        assert!(!result.saved);
        assert_ne!(result.matching_synonyms, BTreeSet::new());
    }

    #[test]
    fn update_topic() {
        let f = Fixtures::copy("simple");
        let repo = RepoId::wiki();
        let topic_id = parse_id("00001");

        let result = f
            .upsert_topic(&repo, "Topic name", &topic_id, OnMatchingSynonym::Ask)
            .unwrap();
        assert!(result.saved);
        let parent_topic = result.topic.unwrap();
        let parent_id = parent_topic.id();

        let topic_path = parse_id("00002");
        let result = f
            .upsert_topic(
                &repo,
                "Topic Name",
                &topic_path,
                OnMatchingSynonym::Update(parent_id.to_owned()),
            )
            .unwrap();

        assert!(result.topic.is_some());
        assert!(result.saved);

        let parent_topics = result
            .topic
            .unwrap()
            .parent_topics
            .iter()
            .map(|topic| topic.id.to_string())
            .collect::<Vec<String>>();

        assert_eq!(parent_topics, &["00001", "00002"]);
    }

    #[test]
    fn create_distinct() {
        let f = Fixtures::copy("simple");
        let repo = RepoId::wiki();
        let topic_id = parse_id("00001");

        let result = f
            .upsert_topic(&repo, "Topic name", &topic_id, OnMatchingSynonym::Ask)
            .unwrap();
        assert!(result.saved);
        let path1 = &result.topic.unwrap().metadata.id;

        let topic_path = parse_id("00002");
        let result = f
            .upsert_topic(
                &repo,
                "Topic Name",
                &topic_path,
                OnMatchingSynonym::CreateDistinct,
            )
            .unwrap();

        assert!(result.topic.is_some());
        assert!(result.saved);
        let path2 = &result.topic.unwrap().metadata.id;

        assert_ne!(path1, path2);

        let matches = f
            .git
            .synonym_phrase_matches(&[&RepoId::wiki()], "Topic name")
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
        let repo = RepoId::wiki();
        let path = parse_id("00001");
        let parent = f.topic(&repo, "00001");
        assert_eq!(parent.children, BTreeSet::new());

        let result = f
            .upsert_topic(&repo, "Topic name", &path, OnMatchingSynonym::Ask)
            .unwrap();
        assert!(result.saved);
        let child = result.topic.unwrap();
        let child_id = &child.id();

        let parent = f.topic(&repo, "00001");
        let children = parent
            .children
            .iter()
            .map(|child| child.id.to_string())
            .collect::<Vec<String>>();

        assert_eq!(children, vec![child_id.to_string()]);
    }

    #[test]
    fn no_cycles() {
        let f = Fixtures::copy("simple");
        let repo = RepoId::wiki();
        let parent = f.topic(&repo, ROOT_TOPIC_ID);
        let topic_id = parse_id("00001");
        let child = f.git.fetch_topic(&repo, &topic_id).unwrap();
        assert!(parent.has_child(&topic_id));

        let result = f
            .upsert_topic(
                &repo,
                "Everything",
                child.id(),
                OnMatchingSynonym::Update(parse_id(ROOT_TOPIC_ID)),
            )
            .unwrap();
        assert!(!result.saved);

        let alert = result.alerts.first().unwrap();
        assert!(matches!(alert, Alert::Warning(_)));

        let synonym_match = result.matching_synonyms.iter().next().unwrap();
        assert_eq!(&synonym_match.topic, &parent);
        assert!(synonym_match.cycle);
    }

    // #[test]
    #[allow(dead_code)]
    fn another_repo() {
        let f = Fixtures::copy("simple");
        let other_repo = RepoId::try_from("/other/").unwrap();
        let parent_path = parse_id("00001");

        let result = f
            .upsert_topic(
                &other_repo,
                "Topic name",
                &parent_path,
                OnMatchingSynonym::Ask,
            )
            .unwrap();
        let topic = result.topic.unwrap();
        let topic_id = topic.id();

        assert!(f.git.exists(&other_repo, topic_id).unwrap());
    }
}

#[cfg(test)]
mod upsert_topic_timerange {
    use super::*;

    use digraph::git::UpsertTopicTimerange;
    use digraph::types::{Timerange, TimerangePrefixFormat};

    fn count(f: &Fixtures, name: &str) -> usize {
        f.git
            .synonym_phrase_matches(&[&RepoId::wiki()], name)
            .unwrap()
            .len()
    }

    #[test]
    fn timerange_added() {
        let f = Fixtures::copy("simple");
        let repo = RepoId::wiki();
        let topic_id = parse_id("00001");

        let topic = f.git.fetch_topic(&repo, &topic_id).unwrap();
        assert!(topic.timerange().is_none());

        UpsertTopicTimerange {
            actor: actor(),
            repo_id: repo.to_owned(),
            timerange: Timerange {
                prefix_format: TimerangePrefixFormat::StartYearMonth,
                starts: Geotime::now().into(),
            },
            topic_id: topic_id.clone(),
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        let topic = f.git.fetch_topic(&repo, &topic_id).unwrap();
        assert!(topic.timerange().is_some());
    }

    #[test]
    fn synonym_indexes() {
        let f = Fixtures::copy("simple");
        let repo = RepoId::wiki();
        let path = parse_id("00001");
        let date = Geotime::from(0);

        let topic = f.git.fetch_topic(&repo, &path).unwrap();
        assert!(topic.timerange().is_none());

        assert_eq!(count(&f, "A topic"), 1);
        assert_eq!(count(&f, "1970 A topic"), 0);

        UpsertTopicTimerange {
            actor: actor(),
            repo_id: repo,
            timerange: Timerange {
                prefix_format: TimerangePrefixFormat::StartYear,
                starts: date.into(),
            },
            topic_id: path,
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        assert_eq!(count(&f, "1970 A topic"), 1);
    }
}