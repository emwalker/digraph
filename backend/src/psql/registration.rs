use sqlx::types::Uuid;

use super::{user, PgTransaction};
use crate::prelude::*;

pub struct CompleteRegistration<'s> {
    login: &'s str,
    user: &'s user::Row,
}

impl<'s> CompleteRegistration<'s> {
    pub fn new(user: &'s user::Row, login: &'s str) -> Self {
        Self { user, login }
    }

    pub async fn call<'t>(&self, tx: &mut PgTransaction<'t>) -> Result<()> {
        let (user_id, name) = self.user_info();
        log::info!("completing registration for {}", name);

        log::info!("creating default org {} for {}", self.login, name);
        let (organization_id,) = sqlx::query_as::<_, (Uuid,)>(
            "insert into organizations
                (login, name, public, owner_id)
                values ($1, $2, false, $3)
                returning id",
        )
        .bind(self.login)
        .bind(DEFAULT_ORGANIZATION_NAME)
        .bind(user_id)
        .fetch_one(&mut **tx)
        .await?;

        log::info!(
            "creating default repo {} for {}",
            DEFAULT_REPOSITORY_NAME,
            name
        );
        let (repository_id,) = sqlx::query_as::<_, (Uuid,)>(
            "insert into repositories
                (
                    organization_id,
                    name,
                    owner_id,
                    private
                )
                values (
                    $1::uuid,
                    $2,
                    $3,
                    't',
                )
                returning id",
        )
        .bind(organization_id)
        .bind(DEFAULT_REPOSITORY_NAME)
        .bind(user_id)
        .fetch_one(&mut **tx)
        .await?;

        // Add permissions for personal repo
        sqlx::query(
            "insert into users_repositories (user_id, repository_id, can_read, can_write)
                values ($1::uuid, $2, 't', 't')
                on conflict do nothing",
        )
        .bind(organization_id)
        .bind(repository_id)
        .execute(&mut **tx)
        .await?;

        // Add permissions for wiki repo
        sqlx::query(
            "insert into users_repositories (user_id, repository_id, can_read, can_write)
                select u.id, r.id, 't', 't'
                from users u
                cross join repositories r
                join organizations o on r.organization_id = o.id
                where o.login = 'wiki' and u.id = $1
                on conflict do nothing",
        )
        .bind(user_id)
        .execute(&mut **tx)
        .await?;

        log::info!("marking user {} as registered", name);
        sqlx::query(
            "update users
                set
                    registered_at = now(),
                    login = $1
                where id = $2",
        )
        .bind(self.login)
        .bind(user_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    fn user_info(&self) -> (&Uuid, &str) {
        (&self.user.id, &self.user.name)
    }
}
