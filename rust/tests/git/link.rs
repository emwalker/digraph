use digraph::git::{Fetch, Search, UpsertLink};
use digraph::http::{repo_url, Response};
use digraph::prelude::*;
use scraper::Html;

use super::{actor, Fixtures};

struct Fetcher(String);

impl Fetch for Fetcher {
    fn fetch(&self, url: &repo_url::Url) -> Result<Response> {
        Ok(Response {
            url: url.to_owned(),
            body: Html::parse_document(&self.0),
        })
    }
}

#[actix_web::test]
async fn create_link() {
    let f = Fixtures::copy("empty");

    let url = repo_url::Url::parse("https://www.google.com").unwrap();
    let path = url.path(&f.repo.prefix);
    let search = Search::parse("page title https://www.google.com/").unwrap();

    assert!(!f.repo.exists(&path).unwrap());
    assert!(!f.repo.appears_in(&search, &path).unwrap());

    UpsertLink {
        actor: actor(),
        add_parent_topic_paths: vec![],
        fetcher: Box::new(Fetcher("<title>Page title</title>".into())),
        prefix: "/wiki".into(),
        url: url.to_string(),
        title: None,
    }
    .call(&f.repo.git)
    .await
    .unwrap();

    assert!(f.repo.exists(&path).unwrap());
    assert!(f.repo.appears_in(&search, &path).unwrap());
}
