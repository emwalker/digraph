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
    name: String,
    login: String,
    default_repository_id: Uuid,
}

impl Row {
    fn to_organization(&self) -> Organization {
        Organization::Selected {
            id: ID(self.id.to_string()),
            name: self.name.to_owned(),
            login: self.login.to_owned(),
            default_repository_id: ID(self.default_repository_id.to_string()),
        }
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
                r.id as default_repository_id

            from organizations o
            join repositories r on r.organization_id = o.id and r.system
            where o.id = any($1::uuid[]) and r.prefix = any($2::text[])",
        )
        .bind(&ids)
        .bind(&self.viewer.read_prefixes.to_vec())
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from)?;

        Ok(rows
            .iter()
            .map(|r| (r.id.to_string(), r.to_organization()))
            .collect::<HashMap<_, _>>())
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
                r.id as default_repository_id

            from organizations o
            join repositories r on r.organization_id = o.id and r.system
            where o.login = any($1) and r.prefix = any($2::text[])",
        )
        .bind(&logins)
        .bind(&self.viewer.read_prefixes.to_vec())
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from)?;

        Ok(rows
            .iter()
            .map(|r| (r.id.to_string(), r.to_organization()))
            .collect::<HashMap<_, _>>())
    }
}
