use std::collections::BTreeSet;
use std::sync::Arc;

use super::{Mutation, RepoTopic, RepoTopicMetadata, Synonym};
use crate::git::RepoTopicDetails;
use crate::prelude::*;
use crate::redis;

pub struct DeleteAccount {
    pub actor: Arc<Viewer>,
    pub user_id: String,
    pub personal_repos: RepoIds,
}

impl DeleteAccount {
    pub fn call(&self, update: &Mutation) -> Result<()> {
        if self.actor.user_id != self.user_id {
            return Err(Error::Repo("not allowed to do that".into()));
        }

        log::warn!("deleting repos for {}", self.user_id);

        let wiki = RepoId::wiki();
        for repo in self.personal_repos.iter() {
            if repo == &wiki {
                return Err(Error::Repo(format!("not allowed to delete {wiki}")));
            }
        }

        for &repo_id in self.personal_repos.iter() {
            update.delete_repo(repo_id)?;
        }
        log::warn!("personal repos of {} have been deleted", self.user_id);

        Ok(())
    }
}

pub struct EnsurePersonalRepo {
    pub actor: Arc<Viewer>,
    pub user_id: String,
    pub personal_repo_ids: Vec<RepoId>,
}

pub struct EnsurePersonalRepoResult {
    pub created_repo_id: Option<RepoId>,
}

impl EnsurePersonalRepo {
    pub fn call(&self, mut mutation: Mutation) -> Result<EnsurePersonalRepoResult> {
        if !self.personal_repo_ids.is_empty() {
            log::info!(
                "user {} already has one or more personal repos",
                self.user_id
            );
            return Ok(EnsurePersonalRepoResult {
                created_repo_id: None,
            });
        }

        if !self.actor.super_user {
            return Err(Error::Repo("not allowed to do that".into()));
        }

        log::info!("ensuring personal repo for {}", self.user_id);
        let repo_id = RepoId::make();

        mutation.repo(repo_id)?;
        let topic_id = repo_id.root_topic_id();

        if !mutation.exists(repo_id, &topic_id)? {
            log::info!("creating root topic: {}", topic_id);
            let added = chrono::Utc::now();

            let root = RepoTopic {
                api_version: API_VERSION.into(),
                metadata: RepoTopicMetadata {
                    added,
                    id: topic_id,
                    details: Some(RepoTopicDetails {
                        root: false,
                        synonyms: vec![Synonym {
                            added,
                            locale: Locale::EN,
                            name: DEFAULT_ROOT_TOPIC_NAME.to_owned(),
                        }],
                        timerange: None,
                    }),
                },
                parent_topics: BTreeSet::new(),
                children: BTreeSet::new(),
            };

            mutation.save_topic(repo_id, &root)?;
            mutation.write(&redis::Noop)?;
        }

        log::info!("personal repo {} created for {}", repo_id, self.user_id);

        Ok(EnsurePersonalRepoResult {
            created_repo_id: Some(repo_id),
        })
    }
}
