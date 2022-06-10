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
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<RepositoryConnection> {
        let results: Vec<Repository> = vec![];
        query(
            after,
            before,
            first,
            last,
            |_after, _before, _first, _last| async move {
                let mut connection = Connection::new(false, false);
                connection.append(results.into_iter().map(|n| {
                    Edge::with_additional_fields(
                        0_usize,
                        n,
                        RepositoryEdgeFields { is_selected: false },
                    )
                }));
                Ok::<_, Error>(connection)
            },
        )
        .await
        .map_err(Error::Resolver)
    }

    pub async fn selected_repository(&self, ctx: &Context<'_>) -> Result<Option<Repository>> {
        match self {
            Self::Guest => Ok(None),
            Self::Registered {
                selected_repository_id,
                ..
            } => match selected_repository_id {
                Some(id) => ctx
                    .data_unchecked::<Repo>()
                    .repository(id.to_string())
                    .await
                    .map_err(|_e| Error::NotFound(format!("repo id {}", **id))),
                None => Ok(None),
            },
        }
    }
}
