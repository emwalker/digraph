use super::Repository;
use crate::prelude::*;
use crate::repo::Repo;

pub const DEFAULT_ORGANIZATION_NAME: &str = "system:default";
pub const WIKI_ORGANIZATION_ID: &str = "45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb";

#[derive(Clone)]
pub enum Organization {
    #[allow(dead_code)]
    Wiki,
    Selected {
        default_repository_id: ID,
        id: ID,
        login: String,
        name: String,
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
            Self::Selected {
                default_repository_id,
                ..
            } => ctx
                .data_unchecked::<Repo>()
                .repository(default_repository_id.to_string())
                .await?
                .ok_or_else(|| Error::NotFound(format!("repo id {}", **default_repository_id,))),
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
