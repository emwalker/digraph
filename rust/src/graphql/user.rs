use async_graphql::connection::*;
use async_graphql::{Context, Object, SimpleObject, ID};

use super::{Repository, RepositoryConnection, RepositoryEdgeFields};
use crate::prelude::*;
use crate::store::Store;

#[derive(Clone, Debug)]
pub enum User {
    Guest,
    Registered {
        avatar_url: String,
        id: ID,
        name: String,
        selected_repo_id: Option<ID>,
    },
}

impl Default for User {
    fn default() -> Self {
        Self::Guest
    }
}

#[derive(Debug, SimpleObject)]
pub struct UserEdge {
    pub cursor: String,
    pub node: User,
}

#[Object]
impl User {
    pub async fn avatar_url(&self) -> Option<String> {
        match self {
            Self::Guest => None,
            Self::Registered { avatar_url, .. } => Some(avatar_url.to_owned()),
        }
    }

    pub async fn id(&self) -> ID {
        match self {
            Self::Guest => ID(GUEST_ID.to_string()),
            Self::Registered { id, .. } => id.to_owned(),
        }
    }

    pub async fn is_guest(&self) -> bool {
        match self {
            Self::Guest => true,
            Self::Registered { .. } => false,
        }
    }

    pub async fn name(&self) -> String {
        match self {
            Self::Guest => "Guest".to_owned(),
            Self::Registered { name, .. } => name.to_owned(),
        }
    }

    pub async fn repos(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<RepositoryConnection> {
        let (selected_repo_id, results) = match self {
            Self::Guest => (ID("".into()), vec![]),
            Self::Registered {
                id,
                selected_repo_id,
                ..
            } => {
                let results = ctx
                    .data_unchecked::<Store>()
                    .repositories_for_user(id.to_string())
                    .await?;
                (selected_repo_id.clone().unwrap_or_default(), results)
            }
        };

        query(
            after,
            before,
            first,
            last,
            |_after, _before, _first, _last| async move {
                let mut connection = Connection::new(false, false);
                connection.edges.extend(results.into_iter().map(|n| {
                    let repo_id = match &n {
                        Repository::Default => ID("".to_string()),
                        Repository::Fetched { id, .. } => ID(id.clone()),
                    };

                    Edge::with_additional_fields(
                        0_usize,
                        n,
                        RepositoryEdgeFields {
                            is_selected: repo_id == selected_repo_id,
                        },
                    )
                }));
                Ok::<_, Error>(connection)
            },
        )
        .await
        .map_err(Error::from)
    }

    pub async fn selected_repo(&self, ctx: &Context<'_>) -> Result<Option<Repository>> {
        match self {
            Self::Guest => Ok(None),
            Self::Registered {
                selected_repo_id, ..
            } => match selected_repo_id {
                Some(id) => ctx.data_unchecked::<Store>().repo(id.to_string()).await,
                None => Ok(None),
            },
        }
    }

    pub async fn selected_repo_id(&self) -> Result<Option<ID>> {
        match self {
            Self::Guest => Ok(None),

            Self::Registered {
                selected_repo_id, ..
            } => match selected_repo_id {
                Some(repo_id) => Ok(Some(repo_id.to_owned())),
                None => Ok(None),
            },
        }
    }
}
