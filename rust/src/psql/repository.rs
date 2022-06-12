use async_graphql::dataloader::*;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;
use std::sync::Arc;

use super::shared::uuids;
use crate::schema::Repository;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Row {
    id: Uuid,
    name: String,
    organization_id: Uuid,
    owner_id: Uuid,
    root_topic_id: Uuid,
    system: bool,
}

impl Row {
    fn to_repository(&self) -> Repository {
        Repository::Fetched {
            id: self.id.to_string(),
            name: self.name.to_owned(),
            organization_id: self.organization_id.to_string(),
            owner_id: self.owner_id.to_string(),
            root_topic_id: self.root_topic_id.to_string(),
            system: self.system,
        }
    }
}

pub struct RepositoryLoader(PgPool);

impl RepositoryLoader {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait::async_trait]
impl Loader<String> for RepositoryLoader {
    type Value = Repository;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, ids: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        log::debug!("load links by batch {:?}", ids);

        let uuids = uuids(ids);
        let rows = sqlx::query_as!(
            Row,
            r#"select
                r.id as "id!: Uuid",
                r.name as "name!",
                r.organization_id as "organization_id!",
                r.system as "system!",
                r.owner_id as "owner_id!",
                t.id as "root_topic_id!"

            from repositories r
            join topics t on r.id = t.repository_id
            where r.id = any($1)
              and t.root = true
            group by r.id, t.id"#,
            &uuids,
        )
        .fetch_all(&self.0)
        .await;

        Ok(rows?
            .iter()
            .map(|r| (r.id.to_string(), r.to_repository()))
            .collect::<HashMap<_, _>>())
    }
}

pub struct RepositoryByNameLoader(PgPool);

impl RepositoryByNameLoader {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait::async_trait]
impl Loader<String> for RepositoryByNameLoader {
    type Value = Repository;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, names: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        log::debug!("batch load repos by name: {:?}", names);

        let rows = sqlx::query_as!(
            Row,
            r#"select
                r.id as "id!: Uuid",
                r.name as "name!",
                r.organization_id as "organization_id!",
                r.system as "system!",
                r.owner_id as "owner_id!",
                t.id as "root_topic_id!"

            from repositories r
            join topics t on r.id = t.repository_id
            where r.name = any($1)
              and t.root = true
            group by r.id, t.id"#,
            &names,
        )
        .fetch_all(&self.0)
        .await;

        Ok(rows?
            .iter()
            .map(|r| (r.id.to_string(), r.to_repository()))
            .collect::<HashMap<_, _>>())
    }
}
