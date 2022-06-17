use sqlx::{postgres::PgPool, types::Uuid, FromRow};

use super::user;
use crate::prelude::*;
use crate::schema::{Alert, CreateGithubSessionInput, User, Viewer};

#[derive(FromRow, Clone, Debug)]
pub struct DatabaseSession {
    pub user_id: Uuid,
    pub session_id: String,
}

pub struct CreateSessionResult {
    pub alerts: Vec<Alert>,
    pub user: User,
    pub session_id: String,
}

pub struct CreateGithubSession {
    input: CreateGithubSessionInput,
}

impl CreateGithubSession {
    pub fn new(input: CreateGithubSessionInput) -> Self {
        Self { input }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<CreateSessionResult> {
        // TODO: actor join?
        let user = sqlx::query_as!(
            user::Row,
            r#"select
                u.id "id!",
                u.name "name!",
                u.avatar_url "avatar_url!",
                u.selected_repository_id
            from users u
            join github_accounts ga on u.id = ga.user_id
            where ga.username = $1"#,
            &self.input.github_username,
        )
        .fetch_one(pool)
        .await?;

        let result = sqlx::query_as!(
            DatabaseSession,
            r#"insert into sessions (user_id) values ($1)
                returning encode(session_id, 'hex') "session_id!", user_id"#,
            &user.id,
        )
        .fetch_one(pool)
        .await?;

        log::debug!("session id for user {:?}: {:?}", user, result.session_id);

        Ok(CreateSessionResult {
            alerts: vec![],
            user: user.to_user(),
            session_id: result.session_id,
        })
    }
}

pub struct DeleteSession {
    viewer: Viewer,
    session_id: String,
}

pub struct DeleteSessionResult {
    pub deleted_session_id: String,
}

impl DeleteSession {
    pub fn new(viewer: Viewer, session_id: String) -> Self {
        Self { viewer, session_id }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<DeleteSessionResult> {
        // TODO: actor join?
        sqlx::query(
            r#"delete from sessions
                where session_id = decode($1, 'hex') and user_id = $2::uuid
                returning id"#,
        )
        .bind(&self.session_id)
        .bind(&self.viewer.user_id)
        .execute(pool)
        .await?;

        Ok(DeleteSessionResult {
            deleted_session_id: self.session_id.clone(),
        })
    }
}
