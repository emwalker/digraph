use digraph::prelude::*;

mod fixtures;
pub use fixtures::*;
mod link;
mod repo;
mod search;
mod topic;

fn viewer(repo_ids: &RepoIds) -> Viewer {
    Viewer {
        context_repo_id: RepoId::wiki(),
        read_repo_ids: repo_ids.to_owned(),
        session_id: Some("2".into()),
        super_user: false,
        user_id: "2".into(),
        write_repo_ids: repo_ids.to_owned(),
    }
}

fn actor() -> Viewer {
    let repos = RepoIds::try_from(&vec![RepoId::wiki(), RepoId::other()]).unwrap();
    viewer(&repos)
}

fn valid_url() -> RepoUrl {
    RepoUrl::parse("https://www.google.com").unwrap()
}

fn parse_id(id: &str) -> Oid {
    Oid::try_from(id).unwrap()
}
