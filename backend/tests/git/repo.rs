use std::fs;
use std::sync::Arc;

use super::{actor, viewer, Fixtures};
use digraph::git;

mod topic_references {
    use digraph::prelude::{OTHER_REPOSITORY_ID, WIKI_REPOSITORY_ID};

    use super::*;

    #[test]
    fn no_leaks() {
        let f = Fixtures::copy("simple");
        let other = f.git.root.path.join(OTHER_REPOSITORY_ID);
        fs::create_dir_all(&other).expect("unable create other repo");

        assert!(f.no_leaks().unwrap());

        let filename = other.join("file.txt");
        fs::write(filename, WIKI_REPOSITORY_ID).expect("unable to write file");

        assert!(!f.no_leaks().unwrap());
    }
}

mod delete_account {
    use digraph::prelude::*;
    use digraph::types::{RepoId, RepoIds};

    use super::*;

    fn delete(f: &Fixtures, repo_id: RepoId, actor: &Arc<Viewer>, user_id: &str) -> Result<()> {
        git::DeleteAccount {
            actor: Arc::clone(actor),
            user_id: user_id.to_owned(),
            personal_repos: RepoIds::from(&vec![repo_id]),
        }
        .call(&f.mutation())
    }

    #[test]
    fn repo_deleted() {
        let f = Fixtures::copy("simple");
        let actor = actor();

        let repo_id = RepoId::other();
        git::core::Repo::ensure(&f.git.root, repo_id).unwrap();
        let path = f.git.root.repo_path(repo_id);

        assert!(path.exists());
        delete(&f, repo_id, &actor, &actor.user_id).unwrap();
        assert!(!path.exists());
    }

    #[test]
    fn invalid_user() {
        let f = Fixtures::copy("simple");
        let actor = actor();

        let repo_id = RepoId::other();
        git::core::Repo::ensure(&f.git.root, repo_id).unwrap();
        let path = f.git.root.repo_path(repo_id);

        assert!(path.exists());
        assert_ne!(actor.user_id, "1");

        let result = delete(&f, repo_id, &actor, "1");
        assert!(result.is_err());
        assert!(path.exists());
    }

    #[test]
    fn wiki_repo() {
        let f = Fixtures::copy("simple");
        let actor = actor();

        let repo_id = RepoId::wiki();
        let path = f.git.root.repo_path(repo_id);

        assert!(path.exists());

        let result = delete(&f, repo_id, &actor, &actor.user_id);
        assert!(result.is_err());
        assert!(path.exists());
    }

    #[test]
    fn idempotency() {
        let f = Fixtures::copy("simple");
        let actor = actor();

        let repo_id = RepoId::other();
        git::core::Repo::ensure(&f.git.root, repo_id).unwrap();
        let path = f.git.root.repo_path(repo_id);
        assert!(path.exists());

        delete(&f, repo_id, &actor, &actor.user_id).unwrap();
        assert!(!path.exists());

        let result = delete(&f, repo_id, &actor, &actor.user_id);
        assert!(result.is_ok());
        assert!(!path.exists());
    }
}

mod ensure_personal_repo {
    use digraph::prelude::*;
    use digraph::redis;
    use digraph::types::RepoId;

    use super::*;

    fn ensure(
        f: &Fixtures,
        repo_ids: &Vec<RepoId>,
        actor: &Arc<Viewer>,
        user_id: &str,
    ) -> Result<git::EnsurePersonalRepoResult> {
        git::EnsurePersonalRepo {
            actor: Arc::clone(actor),
            user_id: user_id.to_owned(),
            personal_repo_ids: repo_ids.to_owned(),
        }
        .call(f.mutation())
    }

    #[test]
    fn root_topic_created() {
        let f = Fixtures::copy("simple");
        let actor = &f.git.viewer;

        let result = ensure(&f, &vec![], actor, &actor.user_id).unwrap();

        let root = ExternalId::root_topic();
        let repo_id = result.created_repo_id.unwrap();
        let topic = f.git.fetch_topic(repo_id, &root).unwrap();
        assert_eq!(topic.name(Locale::EN), "Everything");
    }

    #[test]
    fn view_stats() {
        let f = Fixtures::copy("simple");
        let stats = f.git.view_stats(RepoId::wiki()).unwrap();
        assert_eq!(stats.topic_count, Some(9));
        assert_eq!(stats.link_count, Some(4));
    }

    #[tokio::test]
    async fn fetch_stats() {
        let f = Fixtures::copy("simple");
        let repos = RepoIds::from(&vec![RepoId::wiki(), RepoId::other()]);
        let viewer = viewer(&repos);

        let git::FetchStatsResult { stats } = git::FetchStats { viewer }
            .call(&f.git, &redis::Noop)
            .await
            .unwrap();

        assert_eq!(stats.topic_count(), 0);
        assert_eq!(stats.link_count(), 0);
    }
}
