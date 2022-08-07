use async_graphql::dataloader::*;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;

use super::{fetch_user, repository, user};
use crate::graphql::Repository;
use crate::prelude::*;

const REPOSITORY_FIELDS: &str = r#"
    r.id,
    r.name,
    r.organization_id,
    r.owner_id,
    r.prefix,
    r.private,
    r.root_topic_path
"#;

const REPOSITORY_JOINS: &str = r#"
    from repositories r
"#;

const REPOSITORY_GROUP_BY: &str = r#"
    group by r.id
"#;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Row {
    id: Uuid,
    name: String,
    organization_id: Uuid,
    owner_id: Uuid,
    prefix: String,
    private: bool,
    root_topic_path: String,
}

impl Row {
    fn to_repository(&self) -> Repository {
        Repository::Fetched {
            id: self.id.to_string(),
            name: self.name.to_owned(),
            organization_id: self.organization_id.to_string(),
            owner_id: self.owner_id.to_string(),
            private: self.private,
            prefix: self.prefix.to_owned(),
            root_topic_path: Box::new(PathSpec::try_from(&self.root_topic_path).unwrap()),
        }
    }
}

pub struct FetchWriteableRepositoriesForUser {
    user_id: String,
    viewer: Viewer,
}

impl FetchWriteableRepositoriesForUser {
    pub fn new(viewer: Viewer, user_id: String) -> Self {
        Self { viewer, user_id }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<Vec<Repository>> {
        log::debug!("fetching repositories for user {:?}", self.user_id);

        let query = format!(
            r#"
            select
                {REPOSITORY_FIELDS}
                {REPOSITORY_JOINS}
                join users u on r.prefix = any(u.write_prefixes)
                where u.id = $1::uuid
                    and r.prefix = any($2::text[])
                group by r.id, u.id
            "#,
        );
        let rows = sqlx::query_as::<_, Row>(&query)
            .bind(&self.user_id)
            .bind(&self.viewer.write_repos.to_vec())
            .fetch_all(pool)
            .await?;

        Ok(rows.iter().map(Row::to_repository).collect())
    }
}

pub struct RepositoryLoader {
    pool: PgPool,
    viewer: Viewer,
}

impl RepositoryLoader {
    pub fn new(viewer: Viewer, pool: PgPool) -> Self {
        Self { viewer, pool }
    }
}

#[async_trait::async_trait]
impl Loader<String> for RepositoryLoader {
    type Value = Repository;
    type Error = Error;

    async fn load(&self, ids: &[String]) -> Result<HashMap<String, Self::Value>> {
        log::debug!("batch load repositories {:?}", ids);

        let query = format!(
            r#"
            select
                {REPOSITORY_FIELDS}
                {REPOSITORY_JOINS}
                where r.id = any($1::uuid[])
                    and r.prefix = any($2::text[])
                {REPOSITORY_GROUP_BY}
            "#
        );
        let rows = sqlx::query_as::<_, Row>(&query)
            .bind(&ids)
            .bind(&self.viewer.read_repos.to_vec())
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .iter()
            .map(|r| (r.id.to_string(), r.to_repository()))
            .collect::<HashMap<_, _>>())
    }
}

pub struct RepositoryByNameLoader {
    pool: PgPool,
    viewer: Viewer,
}

impl RepositoryByNameLoader {
    pub fn new(viewer: Viewer, pool: PgPool) -> Self {
        Self { viewer, pool }
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
                    and r.prefix = any($2::text[])
                {REPOSITORY_GROUP_BY}"#
        );
        let rows = sqlx::query_as::<_, Row>(&query)
            .bind(&names)
            .bind(&self.viewer.read_repos.to_vec())
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .iter()
            .map(|r| (r.id.to_string(), r.to_repository()))
            .collect::<HashMap<_, _>>())
    }
}

pub struct RepositoryByPrefixLoader {
    pool: PgPool,
    viewer: Viewer,
}

impl RepositoryByPrefixLoader {
    pub fn new(viewer: Viewer, pool: PgPool) -> Self {
        Self { viewer, pool }
    }
}

#[async_trait::async_trait]
impl Loader<String> for RepositoryByPrefixLoader {
    type Value = Repository;
    type Error = Error;

    async fn load(&self, prefixes: &[String]) -> Result<HashMap<String, Self::Value>> {
        log::debug!(
            "batch load repos by id prefixes {:?} for users {:?}",
            prefixes,
            self.viewer.read_repos
        );

        let query = format!(
            r#"select
                {REPOSITORY_FIELDS}
                {REPOSITORY_JOINS}
                where r.prefix = any($1::text[])
                    and r.prefix = any($2::text[])
                {REPOSITORY_GROUP_BY}
           "#
        );
        let rows = sqlx::query_as::<_, Row>(&query)
            .bind(&prefixes)
            .bind(&self.viewer.read_repos.to_vec())
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .iter()
            .map(|r| (r.prefix.to_string(), r.to_repository()))
            .collect::<HashMap<_, _>>())
    }
}

pub struct SelectRepository {
    actor: Viewer,
    repository_id: Option<String>,
}

pub struct SelectRepositoryResult {
    pub repository: Option<Repository>,
    pub actor: user::Row,
}

impl SelectRepository {
    pub fn new(actor: Viewer, repository_id: Option<String>) -> Self {
        Self {
            actor,
            repository_id,
        }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<SelectRepositoryResult> {
        log::info!(
            "selecting repository {:?} for viewer {:?}",
            self.repository_id,
            self.actor
        );
        let repository_id = &self.repository_id;

        let repository = if let Some(repository_id) = repository_id {
            sqlx::query("update users set selected_repository_id = $1::uuid where id = $2::uuid")
                .bind(repository_id)
                .bind(&self.actor.user_id)
                .execute(pool)
                .await?;

            let query = format!(
                r#"
                select
                    {REPOSITORY_FIELDS}
                    {REPOSITORY_JOINS}
                    where r.id = $1::uuid
                        and r.prefix = any($2::text[])
                    {REPOSITORY_GROUP_BY}
                "#
            );

            let row = sqlx::query_as::<_, repository::Row>(&query)
                .bind(repository_id)
                .bind(&self.actor.write_repos.to_vec())
                .fetch_one(pool)
                .await?;

            Some(row.to_repository())
        } else {
            sqlx::query("update users set selected_repository_id = null where id = $::uuid")
                .bind(&self.actor.user_id)
                .execute(pool)
                .await?;

            None
        };

        let row = fetch_user(&self.actor.write_repos, &self.actor.user_id, pool).await?;

        Ok(SelectRepositoryResult {
            actor: row,
            repository,
        })
    }
}
