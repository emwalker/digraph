use async_graphql::{Context, Object, ID};

use super::Repository;
use crate::prelude::*;
use crate::store::Store;

#[derive(Clone)]
pub enum Organization {
    #[allow(dead_code)]
    Wiki,
    Selected {
        id: ID,
        login: String,
        name: String,
        repo_prefix: RepoPrefix,
    },
}

impl Default for Organization {
    fn default() -> Self {
        Self::Wiki
    }
}

#[Object]
impl Organization {
    pub async fn default_repository(&self, ctx: &Context<'_>) -> Result<Repository> {
        match self {
            Self::Wiki => Ok(Repository::Default),
            Self::Selected { repo_prefix, .. } => ctx
                .data_unchecked::<Store>()
                .repository_by_prefix(repo_prefix.to_string())
                .await?
                .ok_or_else(|| Error::NotFound(format!("repo {}", repo_prefix))),
        }
    }

    async fn login(&self) -> &str {
        match self {
            Self::Wiki => "wiki",
            Self::Selected { login, .. } => login,
        }
    }

    async fn id(&self) -> ID {
        match self {
            Self::Wiki => ID(WIKI_ORGANIZATION_ID.to_owned()),
            Self::Selected { id, .. } => id.to_owned(),
        }
    }

    async fn name(&self) -> &str {
        match self {
            Self::Wiki => "General",
            Self::Selected { name, .. } => name,
        }
    }

    async fn path(&self) -> String {
        match self {
            Self::Wiki => "wiki".to_string(),
            Self::Selected { login, .. } => login.to_owned(),
        }
    }
}
