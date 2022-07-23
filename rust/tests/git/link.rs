use digraph::prelude::*;
use digraph::redis;

use super::{actor, fetch_link, fetch_topic, upsert_link, valid_url, Fixtures};

mod delete_link {
    use super::*;
    use digraph::git::{activity, DeleteLink, Link, UpsertLinkResult};

    async fn link(f: &Fixtures, title: &str, parent_topic: &str) -> Link {
        let url = valid_url();

        let UpsertLinkResult { link, .. } = upsert_link(
            f,
            &url,
            Some(title.into()),
            Some(parent_topic.to_owned()),
            &RepoPrefix::from(WIKI_REPO_PREFIX),
        )
        .await;

        link.unwrap()
    }

    #[actix_web::test]
    async fn link_deleted() {
        let f = Fixtures::copy("simple");

        let link = link(&f, "Page title", "/wiki/00001").await;
        let path = link.path();
        assert!(f.repo.exists(&path).unwrap());

        DeleteLink {
            actor: actor(),
            link_path: path.clone(),
        }
        .call(&f.repo.git, &redis::Noop)
        .unwrap();
        assert!(!f.repo.exists(&path).unwrap());
    }

    #[actix_web::test]
    async fn activity_log_updated() {
        let f = Fixtures::copy("simple");

        let link = link(&f, "Page title", "/wiki/00001").await;
        let path = link.path();

        let activity = f.repo.git.fetch_activity(&path, 1).unwrap();
        assert!(!activity.is_empty());

        DeleteLink {
            actor: actor(),
            link_path: path.clone(),
        }
        .call(&f.repo.git, &redis::Noop)
        .unwrap();

        let activity = f.repo.git.fetch_activity(&path, 100).unwrap();
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

    #[actix_web::test]
    async fn topics_updated() {
        let f = Fixtures::copy("simple");
        let parent1 = RepoPath::from(WIKI_ROOT_TOPIC_PATH);
        let parent2 = RepoPath::from("/wiki/00001");
        let url = valid_url();

        let UpsertLinkResult { link, .. } = upsert_link(
            &f,
            &url,
            Some("Page title".into()),
            Some(parent1.inner.to_owned()),
            &RepoPrefix::from(WIKI_REPO_PREFIX),
        )
        .await;
        let link = link.unwrap();
        assert_eq!(link.parent_topics.len(), 1);

        UpdateLinkParentTopics {
            actor: actor(),
            link_path: link.path(),
            parent_topic_paths: BTreeSet::from([parent1, parent2]),
        }
        .call(&f.repo.git, &redis::Noop)
        .unwrap();

        let link = f.repo.git.fetch_link(&link.path()).unwrap();
        assert_eq!(link.parent_topics.len(), 2);
    }
}

mod upsert_link {
    use super::*;
    use digraph::git::{Kind, Search, SearchEntry, UpsertLinkResult};
    use itertools::Itertools;

    #[actix_web::test]
    async fn link_added() {
        let f = Fixtures::copy("simple");
        let url = valid_url();
        let path = url.path(&f.repo.prefix);
        let search = Search::parse("page title https://www.google.com/").unwrap();
        let entry = SearchEntry {
            path: path.inner.to_owned(),
            kind: Kind::Link,
        };

        assert!(!f.repo.exists(&path).unwrap());
        assert!(!f.repo.appears_in(&search, &entry).unwrap());

        upsert_link(
            &f,
            &url,
            Some("Page title".into()),
            Some("/wiki/00001".to_owned()),
            &RepoPrefix::from(WIKI_REPO_PREFIX),
        )
        .await;

        assert!(f.repo.exists(&path).unwrap());
        assert!(f.repo.appears_in(&search, &entry).unwrap());
    }

    #[actix_web::test]
    async fn no_orphans() {
        let f = Fixtures::copy("simple");
        let url = valid_url();
        let path = url.path(&f.repo.prefix);
        assert!(!f.repo.exists(&path).unwrap());

        upsert_link(&f, &url, None, None, &RepoPrefix::from(WIKI_REPO_PREFIX)).await;

        fetch_link(&f, &path, |link| {
            assert_eq!(link.parent_topics.len(), 1);
            let topic = &link.parent_topics.iter().next().unwrap();
            assert_eq!(topic.path, WIKI_ROOT_TOPIC_PATH);
        });
    }

    #[actix_web::test]
    async fn updates_are_idempotent() {
        let f = Fixtures::copy("simple");
        let url = valid_url();
        let path = url.path(&f.repo.prefix);
        assert!(!f.repo.exists(&path).unwrap());

        upsert_link(
            &f,
            &url,
            None,
            Some("/wiki/00001".to_owned()),
            &RepoPrefix::from(WIKI_REPO_PREFIX),
        )
        .await;
        upsert_link(
            &f,
            &url,
            None,
            Some("/wiki/00001".to_owned()),
            &RepoPrefix::from(WIKI_REPO_PREFIX),
        )
        .await;

        assert!(f.repo.exists(&path).unwrap());
    }

    #[actix_web::test]
    async fn details_updated() {
        let f = Fixtures::copy("simple");
        let url = RepoUrl::parse("https://www.google.com").unwrap();
        let path = url.path(&f.repo.prefix);
        assert!(!f.repo.exists(&path).unwrap());

        upsert_link(
            &f,
            &url,
            Some("A".into()),
            Some("/wiki/00001".to_owned()),
            &RepoPrefix::from(WIKI_REPO_PREFIX),
        )
        .await;

        fetch_link(&f, &path, |link| {
            assert_eq!(link.metadata.title, "A");

            let topics = link
                .parent_topics
                .iter()
                .map(|topic| topic.path.to_owned())
                .collect_vec();

            assert_eq!(topics, &["/wiki/00001"]);
        });

        upsert_link(
            &f,
            &url,
            Some("B".into()),
            Some("/wiki/00002".to_owned()),
            &RepoPrefix::from(WIKI_REPO_PREFIX),
        )
        .await;

        fetch_link(&f, &path, |link| {
            assert_eq!(link.metadata.title, "B");

            let topics = link
                .parent_topics
                .iter()
                .map(|topic| topic.path.to_owned())
                .collect_vec();

            assert_eq!(topics, &["/wiki/00001", "/wiki/00002"]);
        });
    }

    #[actix_web::test]
    async fn parent_topic_updated() {
        let f = Fixtures::copy("simple");
        let url = RepoUrl::parse("https://www.google.com").unwrap();
        let path = url.path(&f.repo.prefix);
        let topic = RepoPath::from("/wiki/00001");
        assert!(!f.repo.exists(&path).unwrap());

        let result = upsert_link(
            &f,
            &url,
            Some("A".into()),
            Some(topic.inner.to_owned()),
            &RepoPrefix::from(WIKI_REPO_PREFIX),
        )
        .await;
        let link = result.link.unwrap();

        fetch_topic(&f, &topic, |parent| {
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

    #[actix_web::test]
    async fn lookup_indexes_updated() {
        let f = Fixtures::copy("simple");
        let url = RepoUrl::parse("https://www.google.com").unwrap();
        let topic = RepoPath::from("/wiki/00001");
        let search = Search::parse("a link").unwrap();

        let UpsertLinkResult { link, .. } = upsert_link(
            &f,
            &url,
            Some("a link title".into()),
            Some(topic.inner.to_owned()),
            &RepoPrefix::from(WIKI_REPO_PREFIX),
        )
        .await;
        assert!(f
            .repo
            .appears_in(&search, &link.unwrap().to_search_entry())
            .unwrap());

        let UpsertLinkResult { link, .. } = upsert_link(
            &f,
            &url,
            Some("a url title".into()),
            Some(topic.inner.to_owned()),
            &RepoPrefix::from(WIKI_REPO_PREFIX),
        )
        .await;
        assert!(!f
            .repo
            .appears_in(&search, &link.unwrap().to_search_entry())
            .unwrap());
    }

    #[actix_web::test]
    async fn another_prefix() {
        let f = Fixtures::copy("simple");
        let url = valid_url();
        let prefix = RepoPrefix::from("/other/");
        let link_path = url.path(&prefix);

        assert_eq!(link_path.prefix, prefix);
        assert!(!f.repo.exists(&link_path).unwrap());

        upsert_link(
            &f,
            &url,
            Some("Page title".into()),
            Some("/wiki/00001".to_owned()),
            &link_path.prefix,
        )
        .await;

        assert!(f.repo.exists(&link_path).unwrap());
    }
}
