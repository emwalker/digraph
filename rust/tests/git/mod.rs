use digraph::prelude::*;

mod fixtures;
pub use fixtures::*;
mod link;
mod repo;
mod search;
mod topic;

fn viewer(repos: &RepoNames) -> Viewer {
    Viewer {
        user_id: "2".into(),
        read_repos: repos.to_owned(),
        write_repos: repos.to_owned(),
        super_user: false,
        session_id: Some("2".into()),
    }
}

fn actor() -> Viewer {
    let repos = RepoNames::try_from(&vec!["/wiki/".to_owned(), "/other/".to_owned()]).unwrap();
    viewer(&repos)
}

fn valid_url() -> RepoUrl {
    RepoUrl::parse("https://www.google.com").unwrap()
}

fn path(path: &str) -> RepoPath {
    RepoPath::try_from(path).unwrap()
}
