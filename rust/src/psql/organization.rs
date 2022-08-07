use async_graphql::dataloader::*;
pub use async_graphql::ID;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;

use crate::graphql::Organization;
use crate::prelude::*;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Row {
    id: Uuid,
    login: String,
    name: String,
    repo_prefix: String,
}

impl TryFrom<Row> for Organization {
    type Error = Error;

    fn try_from(row: Row) -> Result<Self> {
        Ok(Organization::Selected {
            id: ID(row.id.to_string()),
            name: row.name.to_owned(),
            login: row.login.to_owned(),
            repo_prefix: row.repo_prefix.try_into()?,
        })
    }
}

pub struct OrganizationLoader {
    pool: PgPool,
    viewer: Viewer,
}

impl OrganizationLoader {
    pub fn new(viewer: Viewer, pool: PgPool) -> Self {
        Self { viewer, pool }
    }
}

fn try_convert(rows: Vec<Row>) -> Result<HashMap<String, Organization>> {
    let mut map: HashMap<String, Organization> = HashMap::new();
    for row in rows {
        map.insert(row.id.to_string(), row.try_into()?);
    }

    Ok(map)
}

#[async_trait::async_trait]
impl Loader<String> for OrganizationLoader {
    type Value = Organization;
    type Error = Error;

    async fn load(&self, ids: &[String]) -> Result<HashMap<String, Self::Value>> {
        log::debug!("batch load organizations: {:?}", ids);

        let rows = sqlx::query_as::<_, Row>(
            "select
                o.id,
                o.name,
                o.login,
                o.repo_prefix

            from organizations o
            where o.id = any($1::uuid[]) and o.repo_prefix = any($2::text[])",
        )
        .bind(&ids)
        .bind(&self.viewer.read_repos.to_vec())
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from)?;

        try_convert(rows)
    }
}

pub struct OrganizationByLoginLoader {
    pool: PgPool,
    viewer: Viewer,
}

impl OrganizationByLoginLoader {
    pub fn new(viewer: Viewer, pool: PgPool) -> Self {
        Self { viewer, pool }
    }
}

#[async_trait::async_trait]
impl Loader<String> for OrganizationByLoginLoader {
    type Value = Organization;
    type Error = Error;

    async fn load(&self, logins: &[String]) -> Result<HashMap<String, Self::Value>> {
        log::debug!("batch load organizations by login: {:?}", logins);

        let rows = sqlx::query_as::<_, Row>(
            "select
                o.id,
                o.name,
                o.login,
                o.repo_prefix

            from organizations o
            where o.login = any($1) and o.repo_prefix = any($2::text[])",
        )
        .bind(&logins)
        .bind(&self.viewer.read_repos.to_vec())
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from)?;

        try_convert(rows)
    }
}
