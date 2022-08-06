use digraph::prelude::*;

mod fixtures;
pub use fixtures::*;
mod link;
mod repo;
mod search;
mod topic;

fn actor() -> Viewer {
    let repos = RepoList::from(&vec!["/wiki/".to_owned(), "/other/".to_owned()]);
    Viewer {
        user_id: "2".into(),
        read_repos: repos.to_owned(),
        write_repos: repos,
        super_user: false,
        session_id: Some("2".into()),
    }
}

fn valid_url() -> RepoUrl {
    RepoUrl::parse("https://www.google.com").unwrap()
}

fn path(path: &str) -> PathSpec {
    PathSpec::try_from(path).unwrap()
}
