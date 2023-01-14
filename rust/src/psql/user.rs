use async_graphql::dataloader::*;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;

use super::{CompleteRegistration, PgTransaction};
use crate::graphql::CreateGithubSessionInput;
use crate::prelude::*;

pub const USER_FIELDS: &str = r#"
    u.id,
    u.login,
    u.name,
    u.avatar_url,
    u.selected_repository_id,
    array_agg(ur.repository_id) write_repository_ids
"#;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Row {
    pub avatar_url: String,
    pub id: Uuid,
    pub login: Option<String>,
    pub name: String,
    pub selected_repository_id: Option<Uuid>,
    pub write_repository_ids: Vec<Uuid>,
}

pub async fn fetch_user(_read_prefixes: &RepoIds, user_id: &String, pool: &PgPool) -> Result<Row> {
    // TODO: Filter on query_ids
    let query = format!(
        "select
            {USER_FIELDS}
         from users u
         left join users_repositories ur on u.id = ur.user_id
         where u.id = $1::uuid
         group by u.id"
    );
    let row = sqlx::query_as::<_, Row>(&query)
        .bind(user_id)
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
    type Value = Row;
    type Error = Error;

    async fn load(&self, ids: &[String]) -> Result<HashMap<String, Self::Value>> {
        log::debug!("batch load users: {:?}", ids);

        let query = format!(
            "select
                {USER_FIELDS}
             from users u
             left join users_repositories ur on u.id = ur.user_id
             where u.id = any($1::uuid[])
             group by u.id",
        );
        let rows = sqlx::query_as::<_, Row>(&query)
            .bind(ids)
            .fetch_all(&self.pool)
            .await;

        Ok(rows?
            .iter()
            .map(|r| (r.id.to_string(), r.to_owned()))
            .collect::<HashMap<_, _>>())
    }
}

pub struct UpsertRegisteredUser {
    // TODO: generalize
    input: CreateGithubSessionInput,
}

pub struct UpsertUserResult {
    pub user: Row,
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
            left join users_repositories ur on u.id = ur.user_id
            join github_accounts ga on u.id = ga.user_id
            where ga.username = $1
            group by u.id"#
        );
        let result = sqlx::query_as::<_, Row>(&query)
            .bind(username)
            .fetch_optional(pool)
            .await?;

        let user = match result {
            Some(row) => row,
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
        &'t self,
        mut tx: PgTransaction<'t>,
    ) -> Result<(PgTransaction<'t>, Row)> {
        let row = sqlx::query_as::<_, Row>(
            r#"insert into users
                (name, avatar_url, primary_email, github_username, github_avatar_url)
                values ($1, $2, $3, $4, $5)
                returning id,
                    avatar_url,
                    name,
                    null as login,
                    null as selected_repository_id,
                    write_prefixes
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
        .bind(row.id)
        .bind(&self.input.github_username)
        .execute(&mut tx)
        .await?;

        Ok((tx, row))
    }
}

pub struct FetchAccountInfo {
    pub viewer: Viewer,
    pub user_id: String,
}

pub struct FetchAccountInfoResult {
    pub personal_repos: RepoIds,
}

impl FetchAccountInfo {
    pub async fn call(&self, pool: &PgPool) -> Result<FetchAccountInfoResult> {
        #[derive(sqlx::FromRow, Clone, Debug)]
        struct AccountInfo {
            pub personal_prefixes: Vec<String>,
        }

        let row = sqlx::query_as::<_, AccountInfo>(
            "select personal_prefixes from users where id = $1::uuid",
        )
        .bind(&self.user_id)
        .fetch_one(pool)
        .await?;

        Ok(FetchAccountInfoResult {
            personal_repos: (&row.personal_prefixes).try_into()?,
        })
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
        let row = fetch_user(&self.actor.write_repo_ids, &self.user_id, pool).await?;

        let mut tx = pool.begin().await?;

        sqlx::query("insert into deleted_users (user_id) values ($1::uuid)")
            .bind(&self.user_id)
            .execute(&mut tx)
            .await?;

        if let Some(login) = row.login {
            log::warn!("deleting default organization {}", login);
            sqlx::query("delete from organizations where login = $1")
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

        let alert = Alert::Success("Your account has been deleted".into());
        Ok(DeleteAccountResult {
            alerts: vec![alert],
            deleted_user_id: self.user_id.clone(),
        })
    }
}
