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

        log::info!("adding {} ({}) to the Wiki org", name, user_id);
        sqlx::query(
            r#"insert into organization_members
                (organization_id, user_id)
                values ($1, $2::uuid)"#,
        )
        .bind(WIKI_ORGANIZATION_ID)
        .bind(&user_id)
        .execute(&mut tx)
        .await?;

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

        log::info!("adding {} ({}) to org {}", name, user_id, organization_id);
        sqlx::query(
            r#"insert into organization_members
                (organization_id, user_id)
                values ($1::uuid, $2)"#,
        )
        .bind(&organization_id.to_string())
        .bind(&user_id)
        .execute(&mut tx)
        .await?;

        let repository_name = format!("{}/{}", self.login, DEFAULT_REPOSITORY_NAME);
        log::info!("creating default repo {} for {}", repository_name, name);
        let (repository_id,) = sqlx::query_as::<_, (Uuid,)>(
            r#"insert into repositories
                (organization_id, name, owner_id, system)
                values ($1::uuid, $2, $3, 't')
                returning id"#,
        )
        .bind(&organization_id)
        .bind(&repository_name)
        .bind(&user_id)
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
                set registered_at = now(), login = $1
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
