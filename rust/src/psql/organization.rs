use async_graphql::dataloader::*;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;

use super::shared::uuids;
use crate::prelude::*;
use crate::schema::Organization;

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

pub struct OrganizationLoader(PgPool);

impl OrganizationLoader {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait::async_trait]
impl Loader<String> for OrganizationLoader {
    type Value = Organization;
    type Error = Error;

    async fn load(&self, ids: &[String]) -> Result<HashMap<String, Self::Value>> {
        log::debug!("batch load organizations: {:?}", ids);

        let uuids = uuids(ids);
        let rows = sqlx::query_as!(
            Row,
            r#"select
                o.id as "id!: Uuid",
                o.name as "name!",
                o.login as "login!",
                r.id as "default_repository_id!"

            from organizations o
            join repositories r on r.organization_id = o.id and r.system
            where o.id = any($1)"#,
            &uuids,
        )
        .fetch_all(&self.0)
        .await
        .map_err(Error::from)?;

        Ok(rows
            .iter()
            .map(|r| (r.id.to_string(), r.to_organization()))
            .collect::<HashMap<_, _>>())
    }
}

pub struct OrganizationByLoginLoader(PgPool);

impl OrganizationByLoginLoader {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait::async_trait]
impl Loader<String> for OrganizationByLoginLoader {
    type Value = Organization;
    type Error = Error;

    async fn load(&self, logins: &[String]) -> Result<HashMap<String, Self::Value>> {
        log::debug!("batch load organizations by login: {:?}", logins);

        let rows = sqlx::query_as!(
            Row,
            r#"select
                o.id as "id!",
                o.name as "name!",
                o.login as "login!",
                r.id as "default_repository_id!"

            from organizations o
            join repositories r on r.organization_id = o.id and r.system
            where o.login = any($1)"#,
            &logins,
        )
        .fetch_all(&self.0)
        .await
        .map_err(Error::from)?;

        Ok(rows
            .iter()
            .map(|r| (r.id.to_string(), r.to_organization()))
            .collect::<HashMap<_, _>>())
    }
}
