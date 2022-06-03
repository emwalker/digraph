use async_graphql::dataloader::*;
use async_graphql::types::ID;
use async_graphql::SimpleObject;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;
use std::sync::Arc;

use super::shared::uuids;
use crate::schema::User;

#[derive(sqlx::FromRow, Clone, Debug, SimpleObject)]
pub struct Row {
    avatar_url: String,
    id: Uuid,
    name: String,
    selected_repository_id: Option<Uuid>,
}

impl Row {
    fn to_user(&self) -> User {
        User::Registered {
            id: ID(self.id.to_string()),
            name: self.name.to_owned(),
            avatar_url: self.avatar_url.to_owned(),
            selected_repository_id: self.selected_repository_id.map(|uuid| ID(uuid.to_string())),
        }
    }
}

pub struct UserLoader(PgPool);

impl UserLoader {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait::async_trait]
impl Loader<String> for UserLoader {
    type Value = User;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, ids: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        log::debug!("load links by batch {:?}", ids);

        let uuids = uuids(ids);
        let rows = sqlx::query_as!(
            Row,
            r#"select
                u.id as "id!",
                u.name as "name!",
                u.avatar_url as "avatar_url!",
                u.selected_repository_id

            from users u
            where u.id = any($1)"#,
            &uuids,
        )
        .fetch_all(&self.0)
        .await;

        Ok(rows?
            .iter()
            .map(|r| (r.id.to_string(), r.to_user()))
            .collect::<HashMap<_, _>>())
    }
}
