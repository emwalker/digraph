use std::collections::BTreeSet;

use super::{Mutation, Synonym, Topic, TopicMetadata};
use crate::git::TopicDetails;
use crate::prelude::*;
use crate::redis;

pub struct DeleteAccount {
    pub actor: Viewer,
    pub user_id: String,
    pub personal_repos: RepoNames,
}

impl DeleteAccount {
    pub fn call(&self, update: &Mutation) -> Result<()> {
        if self.actor.user_id != self.user_id {
            return Err(Error::Repo("not allowed to do that".into()));
        }

        log::warn!("deleting repos for {}", self.user_id);

        let wiki = RepoName::wiki();
        for repo in self.personal_repos.iter() {
            if repo == &wiki {
                return Err(Error::Repo(format!("not allowed to delete {}", wiki)));
            }
        }

        for repo in self.personal_repos.iter() {
            update.delete_repo(repo)?;
        }
        log::warn!("personal repos of {} have been deleted", self.user_id);

        Ok(())
    }
}

pub struct EnsurePersonalRepo {
    pub actor: Viewer,
    pub user_id: String,
    pub personal_repo: RepoName,
}

impl EnsurePersonalRepo {
    pub fn call(&self, mut update: Mutation) -> Result<()> {
        if !self.actor.super_user {
            return Err(Error::Repo("not allowed to do that".into()));
        }

        log::info!("ensuring personal repo for {}", self.user_id);

        let wiki = RepoName::wiki();
        if self.personal_repo == wiki {
            return Err(Error::Repo(format!(
                "not allowed to associate {} with {}",
                wiki, self.user_id
            )));
        }

        update.repo(&self.personal_repo)?;
        let path = self.personal_repo.default_topic_path()?;

        if !update.exists(&path)? {
            log::info!("creating root topic: {}", path);
            let added = chrono::Utc::now();

            let root = Topic {
                api_version: API_VERSION.into(),
                metadata: TopicMetadata {
                    added,
                    path: path.to_string(),
                    details: Some(TopicDetails {
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

            update.save_topic(&path, &root)?;
            update.write(&redis::Noop)?;
        }

        log::info!("personal repo of {} exists", self.user_id);

        Ok(())
    }
}
