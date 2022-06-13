use async_graphql::connection::*;

use super::{Repository, RepositoryConnection, RepositoryEdgeFields};
use crate::prelude::*;
use crate::psql::Repo;

pub static GUEST_ID: &str = "11a13e26-ee64-4c31-8af1-d1e953899ee0";

#[derive(Clone, Debug)]
pub enum User {
    Guest,
    Registered {
        avatar_url: String,
        id: ID,
        name: String,
        selected_repository_id: Option<ID>,
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

    pub async fn repositories(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<RepositoryConnection> {
        let (selected_repository_id, results) = match self {
            Self::Guest => (ID("".into()), vec![]),
            Self::Registered {
                id,
                selected_repository_id,
                ..
            } => {
                let results = ctx
                    .data_unchecked::<Repo>()
                    .repositories_for_user(id.to_string())
                    .await?;
                (selected_repository_id.clone().unwrap_or_default(), results)
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
                    let repository_id = match &n {
                        Repository::Default => ID("".to_string()),
                        Repository::Fetched { id, .. } => ID(id.clone()),
                    };

                    Edge::with_additional_fields(
                        0_usize,
                        n,
                        RepositoryEdgeFields {
                            is_selected: repository_id == selected_repository_id,
                        },
                    )
                }));
                Ok::<_, Error>(connection)
            },
        )
        .await
        .map_err(Error::from)
    }

    pub async fn selected_repository(&self, ctx: &Context<'_>) -> Result<Option<Repository>> {
        match self {
            Self::Guest => Ok(None),
            Self::Registered {
                selected_repository_id,
                ..
            } => match selected_repository_id {
                Some(id) => {
                    ctx.data_unchecked::<Repo>()
                        .repository(id.to_string())
                        .await
                }
                None => Ok(None),
            },
        }
    }
}
