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
            join organization_members om1 on u.id = om1.organization_id
            join organization_members om2 on om1.organization_id = om2.organization_id 
            where u.id = any($1::uuid[]) and om2.user_id = $2::uuid"#,
        )
        .bind(&ids)
        .bind(&self.viewer.user_id)
        .fetch_all(&self.pool)
        .await;

        Ok(rows?
            .iter()
            .map(|r| (r.id.to_string(), r.to_user()))
            .collect::<HashMap<_, _>>())
    }
}
