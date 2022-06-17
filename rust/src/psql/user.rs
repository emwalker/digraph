use async_graphql::dataloader::*;
use async_graphql::types::ID;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;

use super::{CompleteRegistration, PgTransaction};
use crate::prelude::*;
use crate::schema::{alert, Alert, CreateGithubSessionInput, User, Viewer};

pub const USER_FIELDS: &str = r#"
    u.id,
    u.login,
    u.name,
    u.avatar_url,
    u.selected_repository_id
"#;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Row {
    pub avatar_url: String,
    pub id: Uuid,
    pub login: Option<String>,
    pub name: String,
    pub selected_repository_id: Option<Uuid>,
}

impl Row {
    pub fn to_user(&self) -> User {
        User::Registered {
            id: ID(self.id.to_string()),
            name: self.name.to_owned(),
            avatar_url: self.avatar_url.to_owned(),
            selected_repository_id: self.selected_repository_id.map(|uuid| ID(uuid.to_string())),
        }
    }
}

#[allow(unused_variables)]
pub async fn fetch_user(query_ids: &[String], user_id: &String, pool: &PgPool) -> Result<Row> {
    // TODO: Filter on query_ids
    let query = format!(
        r#"select
            {USER_FIELDS}
        from users u
        where u.id = $1::uuid"#,
    );
    let row = sqlx::query_as::<_, Row>(&query)
        .bind(&user_id)
        .fetch_one(pool)
        .await?;
    Ok(row)
}

pub struct UserLoader {
    pool: PgPool,
    #[allow(dead_code)]
    viewer: Viewer,
}

impl UserLoader {
    pub fn new(viewer: Viewer, pool: PgPool) -> Self {
        Self { viewer, pool }
    }
}

#[async_trait::async_trait]
impl Loader<String> for UserLoader {
    type Value = User;
    type Error = Error;

    async fn load(&self, ids: &[String]) -> Result<HashMap<String, Self::Value>> {
        log::debug!("batch load users: {:?}", ids);

        let query = format!(
            r#"select
                {USER_FIELDS}
            from users u
            where u.id = any($1::uuid[])"#,
        );
        let rows = sqlx::query_as::<_, Row>(&query)
            .bind(&ids)
            .fetch_all(&self.pool)
            .await;

        Ok(rows?
            .iter()
            .map(|r| (r.id.to_string(), r.to_user()))
            .collect::<HashMap<_, _>>())
    }
}

pub struct UpsertRegisteredUser {
    // TODO: generalize
    input: CreateGithubSessionInput,
}

pub struct UpsertUserResult {
    pub user: User,
}

impl UpsertRegisteredUser {
    pub fn new(input: CreateGithubSessionInput) -> Self {
        Self { input }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<UpsertUserResult> {
        let username = &self.input.github_username;

        let query = format!(
            r#"select
                {USER_FIELDS}
            from users u
            join github_accounts ga on u.id = ga.user_id
            where ga.username = $1"#
        );
        let result = sqlx::query_as::<_, Row>(&query)
            .bind(username)
            .fetch_optional(pool)
            .await?;

        let user = match result {
            Some(row) => row.to_user(),
            None => {
                log::info!("user {} not found, creating", username);
                let tx = pool.begin().await?;
                let (tx, user) = self.create_github_user(tx).await?;
                let tx =
                    CompleteRegistration::new(user.clone(), self.input.github_username.clone())
                        .call(tx)
                        .await?;
                tx.commit().await?;
                user
            }
        };

        Ok(UpsertUserResult { user })
    }

    async fn create_github_user<'t>(
        &self,
        mut tx: PgTransaction<'t>,
    ) -> Result<(PgTransaction<'t>, User)> {
        let row = sqlx::query_as::<_, Row>(
            r#"insert into users
                (name, avatar_url, primary_email, github_username, github_avatar_url)
                values ($1, $2, $3, $4, $5)
                returning id, avatar_url, name, null as login, null as selected_repository_id
            "#,
        )
        .bind(&self.input.name)
        .bind(&self.input.github_avatar_url)
        .bind(&self.input.primary_email)
        .bind(&self.input.github_username)
        .bind(&self.input.github_avatar_url)
        .fetch_one(&mut tx)
        .await?;

        sqlx::query(
            r#"insert into github_accounts
                (avatar_url, name, primary_email, user_id, username)
                values ($1, $2, $3, $4::uuid, $5)"#,
        )
        .bind(&self.input.name)
        .bind(&self.input.github_avatar_url)
        .bind(&self.input.primary_email)
        .bind(&row.id)
        .bind(&self.input.github_username)
        .execute(&mut tx)
        .await?;

        Ok((tx, row.to_user()))
    }
}

pub struct DeleteAccount {
    actor: Viewer,
    user_id: String,
}

pub struct DeleteAccountResult {
    pub alerts: Vec<Alert>,
    pub deleted_user_id: String,
}

impl DeleteAccount {
    pub fn new(actor: Viewer, user_id: String) -> Self {
        Self { actor, user_id }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<DeleteAccountResult> {
        if self.user_id != self.actor.user_id {
            return Err(Error::RBAC("not allowed to delete account".into()));
        }

        log::warn!("deleting account {}", self.user_id);
        let row = fetch_user(&self.actor.mutation_ids, &self.user_id, pool).await?;

        let mut tx = pool.begin().await?;

        sqlx::query("insert into deleted_users (user_id) values ($1::uuid)")
            .bind(&self.user_id)
            .execute(&mut tx)
            .await?;

        if let Some(login) = row.login {
            log::warn!("deleting default organization {}", login);
            sqlx::query(r#"delete from organizations where login = $1 and system"#)
                .bind(&login)
                .bind(&self.user_id)
                .execute(&mut tx)
                .await?;
        }

        sqlx::query("delete from users where id = $1::uuid")
            .bind(&self.user_id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        log::warn!("account {} has been deleted", self.user_id);

        let alert = alert::success("Your account has been deleted".into());
        Ok(DeleteAccountResult {
            alerts: vec![alert],
            deleted_user_id: self.user_id.clone(),
        })
    }
}
