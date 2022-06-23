use async_graphql::dataloader::*;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;

use super::{fetch_user, repository};
use crate::prelude::*;
use crate::schema::{Repository, User, Viewer};

const REPOSITORY_FIELDS: &str = r#"
    r.id,
    r.name,
    r.organization_id,
    r.system,
    r.owner_id,
    -- Let's come up with something better than this
    (r.system and r.name = 'system:default') private,
    t.id root_topic_id,
    o.login lookup_prefix
"#;

const REPOSITORY_JOINS: &str = r#"
    from repositories r
    join organization_members om on r.organization_id = om.organization_id
    join organizations o on o.id = r.organization_id
    join topics t on r.id = t.repository_id
"#;

const REPOSITORY_GROUP_BY: &str = r#"
    group by r.id, t.id, o.login
"#;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Row {
    id: Uuid,
    lookup_prefix: String,
    name: String,
    organization_id: Uuid,
    owner_id: Uuid,
    private: bool,
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
            private: self.private,
            root_topic_id: self.root_topic_id.to_string(),
            system: self.system,
        }
    }
}

pub struct FetchRepositoriesForUser {
    user_id: String,
    viewer: Viewer,
}

impl FetchRepositoriesForUser {
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
                join organization_members om2 on om.organization_id = om2.organization_id
                where om.user_id = $1::uuid
                    and om2.user_id = any($2::uuid[])
                    and t.root = true
                {REPOSITORY_GROUP_BY}
            "#,
        );
        let rows = sqlx::query_as::<_, Row>(&query)
            .bind(&self.user_id)
            .bind(&self.viewer.query_ids)
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
                    and t.root = true
                    and om.user_id = any($2::uuid[])
                {REPOSITORY_GROUP_BY}
            "#
        );
        let rows = sqlx::query_as::<_, Row>(&query)
            .bind(&ids)
            .bind(&self.viewer.query_ids)
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
                    and t.root = true
                    and om.user_id = any($2::uuid[])
                {REPOSITORY_GROUP_BY}
           "#
        );
        let rows = sqlx::query_as::<_, Row>(&query)
            .bind(&names)
            .bind(&self.viewer.query_ids)
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
            self.viewer.query_ids
        );

        let query = format!(
            r#"select
                {REPOSITORY_FIELDS}
                {REPOSITORY_JOINS}
                where o.login = any($1::text[])
                    and t.root = true
                    and om.user_id = any($2::uuid[])
                {REPOSITORY_GROUP_BY}
           "#
        );
        let rows = sqlx::query_as::<_, Row>(&query)
            .bind(&prefixes)
            .bind(&self.viewer.query_ids)
            .fetch_all(&self.pool)
            .await?;

        log::debug!("repo query results: {:?}", rows);
        Ok(rows
            .iter()
            .map(|r| (r.lookup_prefix.to_string(), r.to_repository()))
            .collect::<HashMap<_, _>>())
    }
}

pub struct SelectRepository {
    actor: Viewer,
    repository_id: Option<String>,
}

pub struct SelectRepositoryResult {
    pub repository: Option<Repository>,
    pub actor: User,
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
                        and om.user_id = any($2::uuid[])
                        and t.root = true
                    {REPOSITORY_GROUP_BY}
                "#
            );

            let row = sqlx::query_as::<_, repository::Row>(&query)
                .bind(repository_id)
                .bind(&self.actor.mutation_ids)
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

        let row = fetch_user(&self.actor.mutation_ids, &self.actor.user_id, pool).await?;

        Ok(SelectRepositoryResult {
            actor: row.to_user(),
            repository,
        })
    }
}
