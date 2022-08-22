use std::fs;

use super::{actor, viewer, Fixtures};
use digraph::git;

mod topic_references {
    use super::*;

    #[test]
    fn no_leaks() {
        let f = Fixtures::copy("simple");
        let other = f.git.root.path.join("other");
        fs::create_dir_all(&other).expect("unable create other repo");

        assert!(f.no_leaks().unwrap());

        let filename = other.join("file.txt");
        fs::write(&filename, "path: /wiki/some-path").expect("unable to write file");

        assert!(!f.no_leaks().unwrap());
    }
}

mod delete_account {
    use digraph::prelude::*;
    use digraph::types::{RepoName, RepoNames};

    use super::*;

    fn delete(f: &Fixtures, repo: &RepoName, actor: &Viewer, user_id: &str) -> Result<()> {
        git::DeleteAccount {
            actor: actor.to_owned(),
            user_id: user_id.to_owned(),
            personal_repos: RepoNames::try_from(&vec![repo.to_owned()]).unwrap(),
        }
        .call(&f.update())
    }

    #[test]
    fn repo_deleted() {
        let f = Fixtures::copy("simple");
        let actor = actor();

        let repo = "/other/".try_into().unwrap();
        git::core::Repo::ensure(&f.git.root, &repo).unwrap();
        let path = f.git.root.repo_path(&repo);

        assert!(path.exists());
        delete(&f, &repo, &actor, &actor.user_id).unwrap();
        assert!(!path.exists());
    }

    #[test]
    fn invalid_user() {
        let f = Fixtures::copy("simple");
        let actor = actor();

        let repo = "/other/".try_into().unwrap();
        git::core::Repo::ensure(&f.git.root, &repo).unwrap();
        let path = f.git.root.repo_path(&repo);

        assert!(path.exists());
        assert_ne!(actor.user_id, "1");

        let result = delete(&f, &repo, &actor, "1");
        assert!(matches!(result, Err(_)));
        assert!(path.exists());
    }

    #[test]
    fn wiki_repo() {
        let f = Fixtures::copy("simple");
        let actor = actor();

        let repo = RepoName::wiki();
        let path = f.git.root.repo_path(&repo);

        assert!(path.exists());

        let result = delete(&f, &repo, &actor, &actor.user_id);
        assert!(matches!(result, Err(_)));
        assert!(path.exists());
    }

    #[test]
    fn idempotency() {
        let f = Fixtures::copy("simple");
        let actor = actor();

        let repo = "/other/".try_into().unwrap();
        git::core::Repo::ensure(&f.git.root, &repo).unwrap();
        let path = f.git.root.repo_path(&repo);
        assert!(path.exists());

        delete(&f, &repo, &actor, &actor.user_id).unwrap();
        assert!(!path.exists());

        let result = delete(&f, &repo, &actor, &actor.user_id);
        assert!(matches!(result, Ok(_)));
        assert!(!path.exists());
    }
}

mod ensure_personal_repo {
    use digraph::prelude::*;
    use digraph::redis;
    use digraph::types::RepoName;

    use super::*;

    fn ensure(f: &Fixtures, repo: &RepoName, actor: &Viewer, user_id: &str) -> Result<()> {
        git::EnsurePersonalRepo {
            actor: actor.to_owned(),
            user_id: user_id.to_owned(),
            personal_repo: repo.to_owned(),
        }
        .call(f.update())
    }

    #[test]
    fn default_topic_created() {
        let f = Fixtures::copy("simple");
        let actor = Viewer::service_account();

        let repo = "/other/".try_into().unwrap();
        ensure(&f, &repo, &actor, &actor.user_id).unwrap();

        let path = repo.default_topic_path().unwrap();
        let topic = f.topic(&path.inner);
        assert_eq!(topic.name(Locale::EN), "Everything");
    }

    #[test]
    fn view_stats() {
        let f = Fixtures::copy("simple");
        let stats = f.git.view_stats(&RepoName::wiki()).unwrap();
        assert_eq!(stats.topic_count, Some(9));
        assert_eq!(stats.link_count, Some(4));
    }

    #[actix_web::test]
    async fn fetch_stats() {
        let f = Fixtures::copy("simple");
        let repos = RepoNames::try_from(&vec!["/wiki/".to_owned(), "/other/".to_owned()]).unwrap();
        let viewer = viewer(&repos);

        let git::FetchStatsResult { stats } = git::FetchStats { viewer }
            .call(&f.git, redis::Noop)
            .await
            .unwrap();

        assert_eq!(stats.topic_count(), 0);
        assert_eq!(stats.link_count(), 0);
    }
}
