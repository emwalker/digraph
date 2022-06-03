use async_graphql::connection::*;

use super::organization::Organization;
use super::topic::Topic;
use crate::prelude::*;
use crate::psql::Repo;

const PRIVATE_REPO_COLOR: &str = "#dbedff";
const DEFAULT_REPO_ID: &str = "23862a92-07d7-47b3-8dad-347b5c2d3557";
const DEFAULT_REPO_NAME: &str = "system:default";

#[derive(Clone)]
pub enum Repository {
    Default,
    Fetched {
        id: ID,
        name: String,
        organization_id: String,
        root_topic_id: String,
        system: bool,
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
        PRIVATE_REPO_COLOR
    }

    async fn display_name(&self, ctx: &Context<'_>) -> String {
        match self {
            Self::Default => "missing repo".to_string(),
            Self::Fetched { name, .. } => {
                if self.is_private(ctx).await.unwrap_or(false) {
                    "Private repository".to_string()
                } else {
                    name.to_string()
                }
            }
        }
    }

    pub async fn full_name(&self, ctx: &Context<'_>) -> Result<String> {
        match self {
            Self::Default => Ok("wiki/general".to_string()),
            Self::Fetched {
                organization_id,
                name,
                system,
                ..
            } => {
                let org = ctx
                    .data_unchecked::<Repo>()
                    .organization(organization_id.to_string())
                    .await?;

                match org {
                    Some(Organization::Wiki) => Ok("wiki/wiki".to_string()),
                    Some(Organization::Selected { login, .. }) => {
                        let name = if self.is_private(ctx).await? {
                            "private"
                        } else if *system {
                            "general"
                        } else {
                            name
                        };
                        Ok(format!("{}/{}", login, name))
                    }
                    None => Err(Error::NotFound),
                }
            }
        }
    }

    async fn id(&self) -> ID {
        match self {
            Self::Default => ID(DEFAULT_REPO_ID.to_string()),
            Self::Fetched { id, .. } => id.to_owned(),
        }
    }

    async fn is_private(&self) -> bool {
        match self {
            Self::Default => false,
            Self::Fetched { system, name, .. } => *system && name == DEFAULT_REPO_NAME,
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
                .data_unchecked::<Repo>()
                .organization(organization_id.clone())
                .await?
                .ok_or(Error::NotFound),
        }
    }

    async fn root_topic(&self, ctx: &Context<'_>) -> Result<Option<Topic>> {
        match self {
            Self::Default => Ok(None),
            Self::Fetched { root_topic_id, .. } => ctx
                .data_unchecked::<Repo>()
                .topic(root_topic_id.clone())
                .await
                .map_err(|_e| Error::NotFound),
        }
    }
}
