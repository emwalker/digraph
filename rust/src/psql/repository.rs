use async_graphql::dataloader::*;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;
use std::sync::Arc;

use super::{fetch_user, repository, user};
use crate::graphql::Repository;
use crate::prelude::*;

const REPOSITORY_FIELDS: &str = r#"
    r.id,
    r.name,
    r.organization_id,
    r.owner_id,
    r.private
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
    private: bool,
}

impl Row {
    fn to_repository(&self) -> Repository {
        Repository::Fetched {
            id: self.id.to_string(),
            name: self.name.to_owned(),
            organization_id: self.organization_id.to_string(),
            owner_id: self.owner_id.to_string(),
            private: self.private,
        }
    }
}

pub struct FetchWriteableRepositoriesForUser {
    user_id: String,
    viewer: Arc<Viewer>,
}

impl FetchWriteableRepositoriesForUser {
    pub fn new(viewer: Arc<Viewer>, user_id: String) -> Self {
        Self { viewer, user_id }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<Vec<Repository>> {
        log::debug!("fetching repositories for user {:?}", self.user_id);

        let query = format!(
            r#"
            select
                {REPOSITORY_FIELDS}
                {REPOSITORY_JOINS}
                join users_repositories ur on r.id = ur.repository_id
                where ur.user_id = $1::uuid
                    and r.id = any($2::uuid[])
                    and ur.can_write
                group by r.id, ur.user_id
            "#,
        );
        let rows = sqlx::query_as::<_, Row>(&query)
            .bind(&self.user_id)
            .bind(&self.viewer.write_repo_ids.to_vec())
            .fetch_all(pool)
            .await?;

        Ok(rows.iter().map(Row::to_repository).collect())
    }
}

pub struct RepositoryLoader {
    pool: PgPool,
    viewer: Arc<Viewer>,
}

impl RepositoryLoader {
    pub fn new(viewer: Arc<Viewer>, pool: PgPool) -> Self {
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
                    and r.id = any($2::uuid[])
                {REPOSITORY_GROUP_BY}
            "#
        );
        let rows = sqlx::query_as::<_, Row>(&query)
            .bind(ids)
            .bind(&self.viewer.read_repo_ids.to_vec())
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .iter()
            .map(|r| (r.id.to_string(), r.to_repository()))
            .collect::<HashMap<_, _>>())
    }
}

pub struct SelectRepository {
    actor: Arc<Viewer>,
    repo_id: Option<RepoId>,
}

pub struct SelectRepositoryResult {
    pub repo: Option<Repository>,
    pub actor: user::Row,
}

impl SelectRepository {
    pub fn new(actor: Arc<Viewer>, repo_id: Option<RepoId>) -> Self {
        Self { actor, repo_id }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<SelectRepositoryResult> {
        log::info!(
            "selecting repository {:?} for viewer {:?}",
            self.repo_id,
            self.actor
        );

        let repo_id = self.repo_id.as_ref().map(|repo_id| repo_id.to_string());

        let repo = if let Some(repo_id) = &repo_id {
            sqlx::query("update users set selected_repository_id = $1::uuid where id = $2::uuid")
                .bind(repo_id)
                .bind(&self.actor.user_id)
                .execute(pool)
                .await?;

            let query = format!(
                r#"
                select
                    {REPOSITORY_FIELDS}
                    {REPOSITORY_JOINS}
                    where r.id = $1::uuid
                        and r.id = any($2::uuid[])
                    {REPOSITORY_GROUP_BY}
                "#
            );

            let row = sqlx::query_as::<_, repository::Row>(&query)
                .bind(repo_id)
                .bind(&self.actor.write_repo_ids.to_vec())
                .fetch_one(pool)
                .await?;

            Some(row.to_repository())
        } else {
            sqlx::query("update users set selected_repository_id = null where id = $1::uuid")
                .bind(&self.actor.user_id)
                .execute(pool)
                .await?;

            None
        };

        let row = fetch_user(&self.actor.write_repo_ids, &self.actor.user_id, pool).await?;

        Ok(SelectRepositoryResult { actor: row, repo })
    }
}
