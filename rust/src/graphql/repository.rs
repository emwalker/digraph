use async_graphql::connection::*;
use async_graphql::{Context, Object, SimpleObject, ID};

use super::{Organization, Topic, User};
use crate::prelude::*;
use crate::store::Store;

const PRIVATE_REPOSITORY_COLOR: &str = "#dbedff";

#[derive(Clone, Debug)]
pub enum Repository {
    Default,
    Fetched {
        id: String,
        name: String,
        organization_id: String,
        owner_id: String,
        private: bool,
    },
}

// Due to an apparent clash of names, use of the name "RepositoryEdge" will cause the fields on
// this struct to be ignored.
#[derive(SimpleObject)]
pub struct RepositoryEdgeFields {
    pub is_selected: bool,
}

pub type RepositoryConnection = Connection<usize, Repository, EmptyFields, RepositoryEdgeFields>;

#[Object]
impl Repository {
    async fn display_color(&self) -> &str {
        PRIVATE_REPOSITORY_COLOR
    }

    async fn display_name(&self, ctx: &Context<'_>) -> String {
        match self {
            Self::Default => DEFAULT_REPOSITORY_NAME,
            Self::Fetched { name, .. } => {
                if self.is_private(ctx).await.unwrap_or(true) {
                    "Private repository"
                } else {
                    name
                }
            }
        }
        .to_string()
    }

    pub async fn full_name(&self, ctx: &Context<'_>) -> Result<String> {
        match self {
            Self::Default => Ok("wiki/General collection".to_string()),
            Self::Fetched {
                organization_id,
                name,
                ..
            } => {
                let org = ctx
                    .data_unchecked::<Store>()
                    .organization(organization_id.to_string())
                    .await?
                    .ok_or_else(|| {
                        Error::NotFound(format!("no org found: {}", organization_id.as_str()))
                    })?;

                match org {
                    Organization::Wiki => Ok("wiki/General collection".to_string()),
                    Organization::Selected { login, .. } => {
                        let name = if self.is_private(ctx).await? {
                            "private"
                        } else {
                            name
                        };
                        Ok(format!("{}/{}", login, name))
                    }
                }
            }
        }
    }

    pub async fn id(&self) -> ID {
        match self {
            Self::Default => ID(WIKI_REPOSITORY_ID.to_string()),
            Self::Fetched { id, .. } => ID(id.to_owned()),
        }
    }

    pub async fn is_private(&self) -> bool {
        match self {
            Self::Default => false,
            Self::Fetched { private, .. } => *private,
        }
    }

    async fn name(&self) -> &str {
        match self {
            Self::Default => "Default repo",
            Self::Fetched { name, .. } => name.as_str(),
        }
    }

    async fn organization(&self, ctx: &Context<'_>) -> Result<Organization> {
        match self {
            Self::Default => Ok(Organization::Wiki),
            Self::Fetched {
                organization_id, ..
            } => ctx
                .data_unchecked::<Store>()
                .organization(organization_id.to_string())
                .await?
                .ok_or_else(|| {
                    Error::NotFound(format!("no org found: {}", organization_id.as_str()))
                }),
        }
    }

    pub async fn owner(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        match self {
            Repository::Default => Ok(None),
            Repository::Fetched { owner_id, .. } => {
                let user = ctx
                    .data_unchecked::<Store>()
                    .user(owner_id.to_string())
                    .await?;
                Ok(user)
            }
        }
    }

    async fn root_topic(&self, ctx: &Context<'_>) -> Result<Topic> {
        let topic_id = Oid::root_topic();

        ctx.data_unchecked::<Store>()
            // FIXME
            .topic(&RepoId::wiki(), &topic_id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("root topic id: {}", topic_id)))
    }
}
