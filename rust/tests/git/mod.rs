use fs_extra::dir;
use scraper::Html;
use std::env;
use std::path::PathBuf;
use tempfile::{self, TempDir};

use digraph::git::{DataRoot, Fetch, Git, Link, Object, Repository, UpsertLink};
use digraph::http::{repo_url, Response};
use digraph::prelude::*;

mod link;

struct Fetcher(String);

impl Fetch for Fetcher {
    fn fetch(&self, url: &repo_url::Url) -> Result<Response> {
        Ok(Response {
            url: url.to_owned(),
            body: Html::parse_document(&self.0),
        })
    }
}

struct Fixtures {
    path: PathBuf,
    source: PathBuf,
    repo: Repository,
    _tempdir: TempDir,
}

impl Fixtures {
    fn blank(fixture_dirname: &str) -> Self {
        let root = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
        let mut source = PathBuf::from(root);
        source.push("tests/fixtures");
        source.push(&fixture_dirname);

        let tempdir = tempfile::tempdir().unwrap();
        let path = PathBuf::from(&tempdir.path());
        let root = DataRoot::new(path.clone());
        let git = Git::new(root);
        let repo = Repository::new("/wiki", git);

        Fixtures {
            _tempdir: tempdir,
            path,
            repo,
            source,
        }
    }
}

impl Fixtures {
    fn copy(fixture_dirname: &str) -> Self {
        let fixture = Fixtures::blank(fixture_dirname);
        let options = dir::CopyOptions {
            overwrite: true,
            ..Default::default()
        };
        log::debug!("copying: {:?}", fixture.source);
        dir::copy(&fixture.source, &fixture.path, &options).unwrap_or_else(|_| {
            panic!("problem copying {:?} to {:?}", fixture.source, fixture.path)
        });
        fixture
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
    match f.repo.git.get(&path.inner).unwrap() {
        Object::Link(link) => block(link),
        other => panic!("unexpected object: {:?}", other),
    }
}

async fn upsert_link(
    f: &Fixtures,
    url: &repo_url::Url,
    title: Option<String>,
    parent_topics: &[&str],
) {
    use itertools::Itertools;

    let html = match &title {
        Some(title) => format!("<title>{}</title>", title),
        None => "<title>Some title</title>".into(),
    };

    let parent_topics = parent_topics
        .iter()
        .map(|path| RepoPath::from(*path))
        .collect_vec();

    UpsertLink {
        actor: actor(),
        add_parent_topic_paths: parent_topics,
        fetcher: Box::new(Fetcher(html)),
        prefix: "/wiki".into(),
        url: url.to_string(),
        title,
    }
    .call(&f.repo.git)
    .await
    .unwrap();
}
