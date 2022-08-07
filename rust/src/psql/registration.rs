use sqlx::types::Uuid;

use super::{user, PgTransaction};
use crate::prelude::*;

pub struct CompleteRegistration {
    login: String,
    user: user::Row,
}

impl CompleteRegistration {
    pub fn new(user: user::Row, login: String) -> Self {
        Self { user, login }
    }

    pub async fn call<'t>(&self, mut tx: PgTransaction<'t>) -> Result<PgTransaction<'t>> {
        let (user_id, name) = self.user_info();
        log::info!("completing registration for {}", name);

        log::info!("creating default org {} for {}", self.login, name);
        let (organization_id,) = sqlx::query_as::<_, (Uuid,)>(
            "insert into organizations
                (login, name, public, repo_prefix, owner_id)
                values ($1, $2, false, concat('/', $1, '/'), $3)
                returning id",
        )
        .bind(&self.login)
        .bind(DEFAULT_ORGANIZATION_NAME)
        .bind(user_id)
        .fetch_one(&mut tx)
        .await?;

        let repository_name = format!("{}/{}", self.login, DEFAULT_REPOSITORY_NAME);
        let root_topic_path = format!("/{}/{}", self.login, DEFAULT_ROOT_TOPIC_ID);

        log::info!("creating default repo {} for {}", repository_name, name);
        sqlx::query(
            "insert into repositories
                (
                    organization_id,
                    name,
                    owner_id,
                    private,
                    prefix,
                    root_topic_path
                )
                values (
                    $1::uuid,
                    $2,
                    $3,
                    't',
                    concat('/', $4, '/'),
                    $5
                )",
        )
        .bind(&organization_id)
        .bind(&repository_name)
        .bind(&user_id)
        .bind(&self.login)
        .bind(&root_topic_path)
        .fetch_one(&mut tx)
        .await?;

        log::info!("marking user {} as registered", name);
        sqlx::query(
            "update users
                set
                    registered_at = now(),
                    login = $1,
                    write_prefixes = array['/wiki/', concat('/', $1, '/')]
                where id = $2",
        )
        .bind(&self.login)
        .bind(&user_id)
        .execute(&mut tx)
        .await?;

        Ok(tx)
    }

    fn user_info(&self) -> (&Uuid, &str) {
        (&self.user.id, &self.user.name)
    }
}
