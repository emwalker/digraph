use digraph::prelude::*;

mod fixtures;
pub use fixtures::*;
mod link;
mod repo;
mod search;
mod topic;

fn actor() -> Viewer {
    Viewer {
        user_id: "2".into(),
        read_repos: RepoList::from(&vec!["2".into()]),
        write_repos: RepoList::from(&vec!["2".into()]),
        super_user: false,
        session_id: Some("2".into()),
    }
}

fn valid_url() -> RepoUrl {
    RepoUrl::parse("https://www.google.com").unwrap()
}
