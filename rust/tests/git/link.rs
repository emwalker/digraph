use digraph::prelude::*;
use digraph::redis;
use itertools::Itertools;

use super::{actor, path, valid_url, Fixtures};

mod delete_link {
    use super::*;
    use digraph::git::{activity, DeleteLink, Link, UpsertLinkResult};

    fn link(f: &Fixtures, title: &str, parent_topic: &str) -> Link {
        let url = valid_url();

        let UpsertLinkResult { link, .. } = f.upsert_link(
            &RepoName::wiki(),
            &url,
            Some(title.into()),
            Some(parent_topic.to_owned()),
        );

        link.unwrap()
    }

    #[test]
    fn link_deleted() {
        let f = Fixtures::copy("simple");

        let link = link(&f, "Page title", "/wiki/00001");
        let link_id = link.path().unwrap();
        let repo = &link_id.repo;
        assert!(f.git.exists(repo, &link_id).unwrap());

        DeleteLink {
            actor: actor(),
            link_id: link_id.clone(),
        }
        .call(f.update(), &redis::Noop)
        .unwrap();

        assert!(!f.git.exists(repo, &link_id).unwrap());
    }

    #[test]
    fn activity_log_updated() {
        let f = Fixtures::copy("simple");

        let link = link(&f, "Page title", "/wiki/00001");
        let path = link.path().unwrap();

        let activity = f.git.fetch_activity(&path, 1).unwrap();

        assert!(!activity.is_empty());

        DeleteLink {
            actor: actor(),
            link_id: path.clone(),
        }
        .call(f.update(), &redis::Noop)
        .unwrap();

        let activity = f.git.fetch_activity(&path, 100).unwrap();
        let mut found = false;

        for change in activity {
            if let activity::Change::UpsertLink(activity::UpsertLink { upserted_link, .. }) = change
            {
                if upserted_link.path == path.inner {
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
        let parent1 = path(WIKI_ROOT_TOPIC_PATH);
        let parent2 = path("/wiki/00001");
        let url = valid_url();

        let UpsertLinkResult { link, .. } = f.upsert_link(
            &RepoName::wiki(),
            &url,
            Some("Page title".into()),
            Some(parent1.inner.to_owned()),
        );
        let link = link.unwrap();
        assert_eq!(link.parent_topics.len(), 1);

        UpdateLinkParentTopics {
            actor: actor(),
            link_id: link.path().unwrap(),
            parent_topic_ids: BTreeSet::from([parent1, parent2]),
        }
        .call(f.update(), &redis::Noop)
        .unwrap();

        let link_id = link.path().unwrap();
        let link = f.git.fetch_link(&link_id.repo, &link_id).unwrap();
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
        let repo = RepoName::wiki();
        let path = url.path(&repo).unwrap();
        let search = Search::parse("page title https://www.google.com/").unwrap();
        let entry = SearchEntry {
            path: path.inner.to_owned(),
            kind: Kind::Link,
        };

        assert!(!f.git.exists(&repo, &path).unwrap());
        assert!(!f.git.appears_in(&search, &entry).unwrap());

        f.upsert_link(
            &repo,
            &url,
            Some("Page title".into()),
            Some("/wiki/00001".to_owned()),
        );

        assert!(f.git.exists(&repo, &path).unwrap());
        assert!(f.git.appears_in(&search, &entry).unwrap());
    }

    #[test]
    fn no_orphans() {
        let f = Fixtures::copy("simple");
        let url = valid_url();
        let repo = RepoName::wiki();
        let link_id = url.path(&repo).unwrap();
        assert!(!f.git.exists(&repo, &link_id).unwrap());

        f.upsert_link(&repo, &url, None, None);

        f.fetch_link(&link_id, |link| {
            assert_eq!(link.parent_topics.len(), 1);
            let topic = &link.parent_topics.iter().next().unwrap();
            assert_eq!(topic.path, WIKI_ROOT_TOPIC_PATH);
        });
    }

    #[test]
    fn updates_are_idempotent() {
        let f = Fixtures::copy("simple");
        let url = valid_url();
        let link_id = url.path(&RepoName::wiki()).unwrap();
        let repo = &link_id.repo;
        assert!(!f.git.exists(repo, &link_id).unwrap());

        f.upsert_link(
            &RepoName::wiki(),
            &url,
            None,
            Some("/wiki/00001".to_owned()),
        );
        f.upsert_link(
            &RepoName::wiki(),
            &url,
            None,
            Some("/wiki/00001".to_owned()),
        );

        assert!(f.git.exists(repo, &link_id).unwrap());
    }

    #[test]
    fn details_updated() {
        let f = Fixtures::copy("simple");
        let url = RepoUrl::parse("https://www.google.com").unwrap();
        let repo = RepoName::wiki();
        let path = url.path(&repo).unwrap();
        assert!(!f.git.exists(&repo, &path).unwrap());

        f.upsert_link(
            &repo,
            &url,
            Some("A".into()),
            Some("/wiki/00001".to_owned()),
        );

        f.fetch_link(&path, |link| {
            assert_eq!(link.title(), "A");

            let topics = link
                .parent_topics
                .iter()
                .map(|topic| topic.path.to_owned())
                .collect_vec();

            assert_eq!(topics, &["/wiki/00001"]);
        });

        f.upsert_link(
            &repo,
            &url,
            Some("B".into()),
            Some("/wiki/00002".to_owned()),
        );

        f.fetch_link(&path, |link| {
            assert_eq!(link.title(), "B");

            let topics = link
                .parent_topics
                .iter()
                .map(|topic| topic.path.to_owned())
                .collect_vec();

            assert_eq!(topics, &["/wiki/00001", "/wiki/00002"]);
        });
    }

    #[test]
    fn parent_topic_updated() {
        let f = Fixtures::copy("simple");
        let url = RepoUrl::parse("https://www.google.com").unwrap();
        let repo = RepoName::wiki();
        let path = url.path(&repo).unwrap();
        let topic = RepoPath::try_from("/wiki/00001").unwrap();
        assert!(!f.git.exists(&repo, &path).unwrap());

        let result = f.upsert_link(&repo, &url, Some("A".into()), Some(topic.inner.to_owned()));
        let link = result.link.unwrap();

        f.fetch_topic(&topic, |parent| {
            assert_eq!(topic.inner, parent.metadata.path);

            let mut found = false;
            for child in parent.children {
                if child.path == link.metadata.path {
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
        let topic = RepoPath::try_from("/wiki/00001").unwrap();
        let search = Search::parse("a link").unwrap();

        let UpsertLinkResult { link, .. } = f.upsert_link(
            &RepoName::wiki(),
            &url,
            Some("a link title".into()),
            Some(topic.inner.to_owned()),
        );
        assert!(f
            .git
            .appears_in(&search, &link.unwrap().to_search_entry())
            .unwrap());

        let UpsertLinkResult { link, .. } = f.upsert_link(
            &RepoName::wiki(),
            &url,
            Some("a url title".into()),
            Some(topic.inner),
        );
        assert!(!f
            .git
            .appears_in(&search, &link.unwrap().to_search_entry())
            .unwrap());
    }

    #[test]
    fn link_added_to_correct_repo() {
        let f = Fixtures::copy("simple");
        let url = valid_url();
        let repo = RepoName::try_from("/other/").unwrap();
        let link_id = url.path(&repo).unwrap();
        let parent_id = RepoPath::try_from("/wiki/00001").unwrap();

        assert_eq!(link_id.repo, repo);
        assert!(!f.git.exists(&repo, &link_id).unwrap());

        // We're specifying /wiki/00001 as the parent topic, which is under /wiki/. But what will
        // happen is that the link will be added with the path "/other/00001", which makes the
        // topic a reference to /wiki/00001 under the /other/ repo.  No path with /other/ should
        // appear within the /wiki/ repo.
        f.upsert_link(
            &link_id.repo,
            &url,
            Some("Page title".into()),
            Some(parent_id.inner.to_owned()),
        );

        assert!(f.git.exists(&repo, &link_id).unwrap());

        let topic = f
            .git
            .fetch_topic(&parent_id.repo, &parent_id)
            .expect("/wiki/00001");

        for child in &topic.children {
            let path = RepoPath::try_from(&child.path).unwrap();
            assert!(
                path.id != link_id.id,
                "link placed under /wiki/ topic"
            );
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
