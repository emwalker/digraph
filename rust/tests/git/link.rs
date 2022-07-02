use super::{fetch_link, fetch_topic, upsert_link, Fixtures};

mod upsert_link {
    use super::*;
    use digraph::git::{Search, UpsertLinkResult};
    use digraph::http::repo_url;
    use digraph::prelude::*;
    use itertools::Itertools;

    fn valid_url() -> repo_url::Url {
        repo_url::Url::parse("https://www.google.com").unwrap()
    }

    #[actix_web::test]
    async fn link_added() {
        let f = Fixtures::copy("simple");
        let url = valid_url();
        let path = url.path(&f.repo.prefix);
        let search = Search::parse("page title https://www.google.com/").unwrap();

        assert!(!f.repo.exists(&path).unwrap());
        assert!(!f.repo.appears_in(&search, &path).unwrap());

        upsert_link(&f, &url, Some("Page title".into()), &[]).await;

        assert!(f.repo.exists(&path).unwrap());
        assert!(f.repo.appears_in(&search, &path).unwrap());
    }

    #[actix_web::test]
    async fn no_orphans() {
        let f = Fixtures::copy("simple");
        let url = valid_url();
        let path = url.path(&f.repo.prefix);
        assert!(!f.repo.exists(&path).unwrap());

        upsert_link(&f, &url, None, &[]).await;

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

        upsert_link(&f, &url, None, &[]).await;
        upsert_link(&f, &url, None, &[]).await;

        assert!(f.repo.exists(&path).unwrap());
    }

    #[actix_web::test]
    async fn details_updated() {
        let f = Fixtures::copy("simple");
        let url = repo_url::Url::parse("https://www.google.com").unwrap();
        let path = url.path(&f.repo.prefix);
        assert!(!f.repo.exists(&path).unwrap());

        upsert_link(&f, &url, Some("A".into()), &["/wiki/00001"]).await;

        fetch_link(&f, &path, |link| {
            assert_eq!(link.metadata.title, "A");

            let topics = link
                .parent_topics
                .iter()
                .map(|topic| topic.path.to_owned())
                .collect_vec();

            assert_eq!(topics, &["/wiki/00001"]);
        });

        upsert_link(&f, &url, Some("B".into()), &["/wiki/00002"]).await;

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
        let url = repo_url::Url::parse("https://www.google.com").unwrap();
        let path = url.path(&f.repo.prefix);
        let topic = RepoPath::from("/wiki/00001");
        assert!(!f.repo.exists(&path).unwrap());

        let result = upsert_link(&f, &url, Some("A".into()), &[&topic.inner]).await;
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
        let url = repo_url::Url::parse("https://www.google.com").unwrap();
        let topic = RepoPath::from("/wiki/00001");
        let search = Search::parse("a link").unwrap();

        let UpsertLinkResult { link, .. } =
            upsert_link(&f, &url, Some("a link title".into()), &[&topic.inner]).await;
        assert!(f.repo.appears_in(&search, &link.unwrap().path()).unwrap());

        let UpsertLinkResult { link, .. } =
            upsert_link(&f, &url, Some("a url title".into()), &[&topic.inner]).await;
        assert!(!f.repo.appears_in(&search, &link.unwrap().path()).unwrap());
    }
}
