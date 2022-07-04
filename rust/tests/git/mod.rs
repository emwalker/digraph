use async_trait::async_trait;
use scraper::html::Html;

use digraph::git::{
    Link, OnMatchingSynonym, Topic, UpsertLink, UpsertLinkResult, UpsertTopic, UpsertTopicResult,
};
use digraph::http::{Fetch, Response};
use digraph::prelude::*;
use digraph::Locale;

mod fixtures;
pub use fixtures::*;
mod link;
mod search;
mod topic;

struct Fetcher(String);

#[async_trait]
impl Fetch for Fetcher {
    async fn fetch(&self, url: &RepoUrl) -> Result<Response> {
        Ok(Response {
            url: url.to_owned(),
            body: Html::parse_document(&self.0),
        })
    }
}

fn actor() -> Viewer {
    Viewer {
        user_id: "2".into(),
        query_ids: vec!["2".into()],
        mutation_ids: vec!["2".into()],
        session_id: Some("2".into()),
    }
}

fn fetch_link<F>(f: &Fixtures, path: &RepoPath, block: F)
where
    F: Fn(Link),
{
    let link = f.repo.git.fetch_link(&path.inner).unwrap();
    block(link);
}

fn fetch_topic<F>(f: &Fixtures, path: &RepoPath, block: F)
where
    F: Fn(Topic),
{
    let topic = f.repo.git.fetch_topic(&path.inner).unwrap();
    block(topic);
}

async fn upsert_link(
    f: &Fixtures,
    url: &RepoUrl,
    title: Option<String>,
    parent_topics: &[&str],
) -> UpsertLinkResult {
    use itertools::Itertools;

    let html = match &title {
        Some(title) => format!("<title>{}</title>", title),
        None => "<title>Some title</title>".into(),
    };

    let parents = parent_topics
        .iter()
        .map(|path| RepoPath::from(*path))
        .collect_vec();

    UpsertLink {
        actor: actor(),
        add_parent_topic_paths: parents,
        fetcher: Box::new(Fetcher(html)),
        prefix: "/wiki".into(),
        url: url.normalized.to_owned(),
        title,
    }
    .call(&f.repo.git)
    .await
    .unwrap()
}

fn upsert_topic(
    f: &Fixtures,
    name: &str,
    parent_topic: RepoPath,
    on_matching_synonym: OnMatchingSynonym,
) -> Result<UpsertTopicResult> {
    UpsertTopic {
        actor: actor(),
        parent_topic,
        locale: Locale::EN,
        name: name.into(),
        on_matching_synonym,
        prefix: "/wiki".into(),
    }
    .call(&f.repo.git)
}

fn valid_url() -> RepoUrl {
    RepoUrl::parse("https://www.google.com").unwrap()
}
