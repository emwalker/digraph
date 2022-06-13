use async_graphql::dataloader::*;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;

use super::{repository, user};
use crate::prelude::*;
use crate::schema::{Repository, User, Viewer};

const REPOSITORY_FIELDS: &str = r#"
    r.id,
    r.name,
    r.organization_id,
    r.system,
    r.owner_id,
    t.id root_topic_id
"#;

const REPOSITORY_JOINS: &str = r#"
    from repositories r
    join topics t on r.id = t.repository_id
"#;

const REPOSITORY_GROUP_BY: &str = r#"
    group by r.id, t.id
"#;

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

pub struct FetchRepositoriesForUser {
    user_id: String,
}

impl FetchRepositoriesForUser {
    pub fn new(user_id: String) -> Self {
        Self { user_id }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<Vec<Repository>> {
        log::debug!("fetching repositories for user {:?}", self.user_id);

        let query = format!(
            r#"
            select
                {REPOSITORY_FIELDS}
                {REPOSITORY_JOINS}
                join organization_members om on r.organization_id = om.organization_id
                where om.user_id = $1::uuid
                    and t.root = true
                {REPOSITORY_GROUP_BY}
            "#,
        );
        let rows = sqlx::query_as::<_, Row>(&query)
            .bind(&self.user_id)
            .fetch_all(pool)
            .await?;

        Ok(rows.iter().map(Row::to_repository).collect())
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
    type Error = Error;

    async fn load(&self, ids: &[String]) -> Result<HashMap<String, Self::Value>> {
        log::debug!("load links by batch {:?}", ids);

        let query = format!(
            r#"
            select
                {REPOSITORY_FIELDS}
                {REPOSITORY_JOINS}
                where r.id = any($1::uuid[])
                    and t.root = true
                {REPOSITORY_GROUP_BY}
            "#
        );
        let rows = sqlx::query_as::<_, Row>(&query)
            .bind(&ids)
            .fetch_all(&self.0)
            .await?;

        Ok(rows
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
    type Error = Error;

    async fn load(&self, names: &[String]) -> Result<HashMap<String, Self::Value>> {
        log::debug!("batch load repos by name: {:?}", names);

        let query = format!(
            r#"
            "select
                {REPOSITORY_FIELDS}
                {REPOSITORY_JOINS}
                where r.name = any($1)
                    and t.root = true
                {REPOSITORY_GROUP_BY}
           "#
        );
        let rows = sqlx::query_as::<_, Row>(&query)
            .bind(&names)
            .fetch_all(&self.0)
            .await?;

        Ok(rows
            .iter()
            .map(|r| (r.id.to_string(), r.to_repository()))
            .collect::<HashMap<_, _>>())
    }
}

pub struct SelectRepository {
    viewer: Viewer,
    repository_id: Option<String>,
}

pub struct SelectRepositoryResult {
    pub repository: Option<Repository>,
    pub viewer: User,
}

impl SelectRepository {
    pub fn new(viewer: Viewer, repository_id: Option<String>) -> Self {
        Self {
            viewer,
            repository_id,
        }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<SelectRepositoryResult> {
        log::info!(
            "selecting repository {:?} for viewer {:?}",
            self.repository_id,
            self.viewer
        );
        let repository_id = &self.repository_id;

        let repository = if let Some(repository_id) = repository_id {
            sqlx::query("update users set selected_repository_id = $1::uuid where id = $2::uuid")
                .bind(repository_id)
                .bind(&self.viewer.user_id)
                .execute(pool)
                .await?;

            let query = format!(
                r#"
                select
                    {REPOSITORY_FIELDS}
                    {REPOSITORY_JOINS}
                    where r.id = $1::uuid
                        and t.root = true
                    {REPOSITORY_GROUP_BY}
                "#
            );

            let row = sqlx::query_as::<_, repository::Row>(&query)
                .bind(repository_id)
                .fetch_one(pool)
                .await?;

            Some(row.to_repository())
        } else {
            sqlx::query("update users set selected_repository_id = null where id = $::uuid")
                .bind(&self.viewer.user_id)
                .execute(pool)
                .await?;

            None
        };

        let row = sqlx::query_as::<_, user::Row>("select * from users where id = $1::uuid")
            .bind(&self.viewer.user_id)
            .fetch_one(pool)
            .await?;

        Ok(SelectRepositoryResult {
            viewer: row.to_user(),
            repository,
        })
    }
}
