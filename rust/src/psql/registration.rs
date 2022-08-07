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
            r#"insert into organizations
                (login, name, public, system)
                values ($1, $2, false, true)
                returning id"#,
        )
        .bind(&self.login)
        .bind(DEFAULT_ORGANIZATION_NAME)
        .fetch_one(&mut tx)
        .await?;

        let repository_name = format!("{}/{}", self.login, DEFAULT_REPOSITORY_NAME);
        let root_topic_path = format!("/{}/{}", self.login, DEFAULT_ROOT_TOPIC_ID);

        log::info!("creating default repo {} for {}", repository_name, name);
        let (repository_id,) = sqlx::query_as::<_, (Uuid,)>(
            r#"insert into repositories
                (
                    organization_id,
                    name,
                    owner_id,
                    system,
                    private,
                    prefix,
                    root_topic_path
                )
                values (
                    $1::uuid,
                    $2,
                    $3,
                    't',
                    't',
                    concat('/', $4, '/'),
                    $5
                )
                returning id"#,
        )
        .bind(&organization_id)
        .bind(&repository_name)
        .bind(&user_id)
        .bind(&self.login)
        .bind(&root_topic_path)
        .fetch_one(&mut tx)
        .await?;

        log::info!("creating root topic for {}", repository_name);
        sqlx::query(
            r#"insert into topics
                (organization_id, repository_id, name, root)
                values ($1::uuid, $2::uuid, $3, 't')"#,
        )
        .bind(&organization_id)
        .bind(&repository_id)
        .bind(DEFAULT_ROOT_TOPIC_NAME)
        .execute(&mut tx)
        .await?;

        log::info!("marking user {} as registered", name);
        sqlx::query(
            r#"update users
                set
                    registered_at = now(),
                    login = $1,
                    write_prefixes = array['/wiki/', concat('/', $1, '/')]
                where id = $2"#,
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
