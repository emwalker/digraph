use digraph::prelude::*;
use digraph::redis;

use super::{actor, parse_id, valid_url, Fixtures};

mod delete_link {
    use super::*;
    use digraph::git::{activity, DeleteLink, RepoLink, UpsertLinkResult};

    fn link(f: &Fixtures, title: &str, parent_topic: &str) -> RepoLink {
        let url = valid_url();
        let topic_id = parse_id(parent_topic);

        let UpsertLinkResult { link, .. } =
            f.upsert_link(&RepoId::wiki(), &url, Some(title.into()), Some(topic_id));

        link.unwrap()
    }

    #[test]
    fn link_deleted() {
        let f = Fixtures::copy("simple");

        let link = link(&f, "Page title", "00001");
        let link_id = link.id();
        let repo = RepoId::wiki();
        assert!(f.git.exists(&repo, link_id).unwrap());

        DeleteLink {
            actor: actor(),
            repo: repo.to_owned(),
            link_id: link_id.clone(),
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        assert!(!f.git.exists(&repo, link_id).unwrap());
    }

    #[test]
    fn activity_log_updated() {
        let f = Fixtures::copy("simple");

        let repo = RepoId::wiki();
        let link = link(&f, "Page title", "00001");
        let link_id = link.id();

        let activity = f.git.fetch_activity(&repo, link_id, 1).unwrap();

        assert!(!activity.is_empty());

        DeleteLink {
            actor: actor(),
            repo: repo.to_owned(),
            link_id: link_id.clone(),
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        let activity = f.git.fetch_activity(&repo, link_id, 100).unwrap();
        let mut found = false;

        for change in activity {
            if let activity::Change::UpsertLink(activity::UpsertLink { upserted_link, .. }) = change
            {
                if &upserted_link.id == link_id {
                    assert!(upserted_link.deleted);
                    found = true;
                }
            }
        }

        assert!(found);
    }
}

mod update_link_parent_topics {
    use super::*;
    use digraph::git::{UpdateLinkParentTopics, UpsertLinkResult};
    use std::collections::BTreeSet;

    #[test]
    fn topics_updated() {
        let f = Fixtures::copy("simple");
        let repo = RepoId::wiki();
        let parent1 = Oid::root_topic();
        let parent2 = parse_id("00001");
        let url = valid_url();

        let UpsertLinkResult { link, .. } = f.upsert_link(
            &repo,
            &url,
            Some("Page title".into()),
            Some(parent1.to_owned()),
        );
        let link = link.unwrap();
        assert_eq!(link.parent_topics.len(), 1);

        UpdateLinkParentTopics {
            actor: actor(),
            repo_id: repo.to_owned(),
            link_id: link.id().to_owned(),
            parent_topic_ids: BTreeSet::from([parent1, parent2]),
        }
        .call(f.mutation(), &redis::Noop)
        .unwrap();

        let link_id = link.id();
        let link = f.git.fetch_link(&repo, link_id).unwrap();
        assert_eq!(link.parent_topics.len(), 2);
    }
}

mod upsert_link {
    use super::*;
    use digraph::git::{Kind, Search, SearchEntry, UpsertLinkResult};

    #[test]
    fn link_added() {
        let f = Fixtures::copy("simple");
        let url = valid_url();
        let repo = RepoId::wiki();
        let link_id = url.id().unwrap();
        let parent_topic = parse_id("00001");
        let search = Search::parse("page title https://www.google.com/").unwrap();
        let entry = SearchEntry {
            id: link_id.to_owned(),
            kind: Kind::Link,
        };

        assert!(!f.git.exists(&repo, &link_id).unwrap());
        assert!(!f.git.appears_in(&repo, &search, &entry).unwrap());

        f.upsert_link(
            &repo,
            &url,
            Some("Page title".to_owned()),
            Some(parent_topic),
        );

        assert!(f.git.exists(&repo, &link_id).unwrap());
        assert!(f.git.appears_in(&repo, &search, &entry).unwrap());
    }

    #[test]
    fn whitespace_removed() {
        let f = Fixtures::copy("simple");
        let url = valid_url();
        let repo_id = RepoId::wiki();
        let link_id = url.id().unwrap();
        let parent_topic = parse_id("00001");

        f.upsert_link(
            &repo_id,
            &url,
            Some("   Page title   ".to_owned()),
            Some(parent_topic),
        );

        f.fetch_link(&repo_id, &link_id, |link| {
            assert_eq!(link.title(), "Page title");
        });
    }

    #[test]
    fn no_orphans() {
        let f = Fixtures::copy("simple");
        let url = valid_url();
        let repo = RepoId::wiki();
        let link_id = url.id().unwrap();
        assert!(!f.git.exists(&repo, &link_id).unwrap());

        f.upsert_link(&repo, &url, None, None);

        f.fetch_link(&repo, &link_id, |link| {
            assert_eq!(link.parent_topics.len(), 1);
            let topic = link.parent_topics.iter().next().unwrap();
            assert_eq!(topic.id, Oid::root_topic());
        });
    }

    #[test]
    fn updates_are_idempotent() {
        let f = Fixtures::copy("simple");
        let url = valid_url();
        let repo = RepoId::wiki();
        let link_id = url.id().unwrap();
        let parent_topic = parse_id("00001");
        assert!(!f.git.exists(&repo, &link_id).unwrap());

        f.upsert_link(&repo, &url, None, Some(parent_topic.to_owned()));
        let UpsertLinkResult { alerts, .. } = f.upsert_link(&repo, &url, None, Some(parent_topic));

        assert!(f.git.exists(&repo, &link_id).unwrap());

        // An alert says that the link was already in the selected repo
        assert_eq!(alerts.len(), 1);
    }

    #[test]
    fn details_updated() {
        let f = Fixtures::copy("simple");
        let url = RepoUrl::parse("https://www.google.com").unwrap();
        let repo = RepoId::wiki();
        let path = url.id().unwrap();

        assert!(!f.git.exists(&repo, &path).unwrap());

        f.upsert_link(&repo, &url, Some("A".into()), Some(parse_id("00001")));

        f.fetch_link(&repo, &path, |link| {
            assert_eq!(link.title(), "A");

            let topics = link
                .parent_topics
                .iter()
                .map(|topic| topic.id.to_string())
                .collect::<Vec<String>>();

            assert_eq!(topics, &["00001"]);
        });

        f.upsert_link(&repo, &url, Some("B".into()), Some(parse_id("00002")));

        f.fetch_link(&repo, &path, |link| {
            assert_eq!(link.title(), "B");

            let topics = link
                .parent_topics
                .iter()
                .map(|topic| topic.id.to_string())
                .collect::<Vec<String>>();

            assert_eq!(topics, &["00001", "00002"]);
        });
    }

    #[test]
    fn parent_topic_updated() {
        let f = Fixtures::copy("simple");
        let url = RepoUrl::parse("https://www.google.com").unwrap();
        let repo = RepoId::wiki();
        let path = url.id().unwrap();
        let topic = parse_id("00001");
        assert!(!f.git.exists(&repo, &path).unwrap());

        let result = f.upsert_link(&repo, &url, Some("A".into()), Some(topic.to_owned()));
        let link = result.link.unwrap();

        f.fetch_topic(&repo, &topic, |parent| {
            assert_eq!(&topic, parent.topic_id());

            let mut found = false;
            for child in parent.children {
                if child.id == link.metadata.id {
                    found = true;
                    break;
                }
            }
            assert!(found, "link not found in parent topic children");
        });
    }

    #[test]
    fn lookup_indexes_updated() {
        let f = Fixtures::copy("simple");
        let url = RepoUrl::parse("https://www.google.com").unwrap();
        let repo = RepoId::wiki();
        let topic = parse_id("00001");
        let search = Search::parse("a link").unwrap();

        let UpsertLinkResult { link, .. } = f.upsert_link(
            &repo,
            &url,
            Some("a link title".into()),
            Some(topic.to_owned()),
        );
        assert!(f
            .git
            .appears_in(&repo, &search, &link.unwrap().to_search_entry())
            .unwrap());

        let UpsertLinkResult { link, .. } =
            f.upsert_link(&repo, &url, Some("a url title".into()), Some(topic));
        assert!(!f
            .git
            .appears_in(&repo, &search, &link.unwrap().to_search_entry())
            .unwrap());
    }

    #[test]
    fn link_added_to_correct_repo() {
        let f = Fixtures::copy("simple");
        let url = valid_url();
        let repo = RepoId::wiki();
        let other_repo = RepoId::other();
        let link_id = url.id().unwrap();
        let parent_id = parse_id("00001");

        assert!(!f.git.exists(&other_repo, &link_id).unwrap());

        // Update description:
        // We're specifying /wiki/00001 as the parent topic, which is under /wiki/. But what will
        // happen is that the link will be added with the path "/other/00001", which makes the
        // topic a reference to /wiki/00001 under the /other/ repo.  No path with /other/ should
        // appear within the /wiki/ repo.
        f.upsert_link(
            &other_repo,
            &url,
            Some("Page title".into()),
            Some(parent_id.to_owned()),
        );

        assert!(f.git.exists(&other_repo, &link_id).unwrap());

        let topic = f.git.fetch_topic(&repo, &parent_id).expect("00001");

        for child in &topic.children {
            assert!(child.id != link_id, "link placed under /wiki/ topic");
        }

        // let topic_path = repo.path(&parent_path.short_id).unwrap();
        // let topic = f.git.fetch_topic(&topic_path).expect("/other/00001");
        // let mut found = false;

        // for child in &topic.children {
        //     let path = PathSpec::try_from(&child.path).unwrap();
        //     if path.short_id == link_path.short_id {
        //         found = true;
        //         break;
        //     }
        // }

        // assert!(found, "link not found in topic children");
    }
}
