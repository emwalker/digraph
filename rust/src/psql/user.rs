use async_graphql::dataloader::*;
use async_graphql::types::ID;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;

use crate::prelude::*;
use crate::schema::{User, Viewer};

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Row {
    pub avatar_url: String,
    pub id: Uuid,
    pub name: String,
    pub selected_repository_id: Option<Uuid>,
}

impl Row {
    pub fn to_user(&self) -> User {
        User::Registered {
            id: ID(self.id.to_string()),
            name: self.name.to_owned(),
            avatar_url: self.avatar_url.to_owned(),
            selected_repository_id: self.selected_repository_id.map(|uuid| ID(uuid.to_string())),
        }
    }
}

pub struct UserLoader {
    pool: PgPool,
    #[allow(dead_code)]
    viewer: Viewer,
}

impl UserLoader {
    pub fn new(viewer: Viewer, pool: PgPool) -> Self {
        Self { viewer, pool }
    }
}

#[async_trait::async_trait]
impl Loader<String> for UserLoader {
    type Value = User;
    type Error = Error;

    async fn load(&self, ids: &[String]) -> Result<HashMap<String, Self::Value>> {
        log::debug!("batch load users: {:?}", ids);

        let rows = sqlx::query_as::<_, Row>(
            r#"select
                u.id,
                u.name,
                u.avatar_url,
                u.selected_repository_id

            from users u
            where u.id = any($1::uuid[])"#,
        )
        .bind(&ids)
        .fetch_all(&self.pool)
        .await;

        Ok(rows?
            .iter()
            .map(|r| (r.id.to_string(), r.to_user()))
            .collect::<HashMap<_, _>>())
    }
}
