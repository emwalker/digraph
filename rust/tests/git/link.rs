use digraph::git::Search;
use digraph::http::repo_url;

use digraph::schema::WIKI_ROOT_TOPIC_PATH;
use itertools::Itertools;

use super::{fetch_link, upsert_link, Fixtures};

fn valid_url() -> repo_url::Url {
    repo_url::Url::parse("https://www.google.com").unwrap()
}

#[actix_web::test]
async fn link_is_added() {
    let f = Fixtures::copy("empty");
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
async fn no_orphan_links() {
    let f = Fixtures::copy("empty");
    let url = valid_url();
    let path = url.path(&f.repo.prefix);
    assert!(!f.repo.exists(&path).unwrap());

    upsert_link(&f, &url, None, &[]).await;

    fetch_link(&f, &path, |link| {
        assert_eq!(link.parent_topics.len(), 1);
        let topic = &link.parent_topics[0];
        assert_eq!(topic.path, WIKI_ROOT_TOPIC_PATH);
    });
}

#[actix_web::test]
async fn updates_are_idempotent() {
    let f = Fixtures::copy("empty");
    let url = valid_url();
    let path = url.path(&f.repo.prefix);
    assert!(!f.repo.exists(&path).unwrap());

    upsert_link(&f, &url, None, &[]).await;
    upsert_link(&f, &url, None, &[]).await;

    assert!(f.repo.exists(&path).unwrap());
}

#[actix_web::test]
async fn details_are_updated() {
    let f = Fixtures::copy("empty");
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
