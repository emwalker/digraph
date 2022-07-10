use sqlx::{postgres::PgPool, types::Uuid, FromRow};

use super::{user, UpsertRegisteredUser, UpsertUserResult};
use crate::graphql::{Alert, CreateGithubSessionInput, Viewer};
use crate::prelude::*;

#[derive(FromRow, Clone, Debug)]
pub struct DatabaseSession {
    pub user_id: Uuid,
    pub session_id: String,
}

pub struct CreateSessionResult {
    pub alerts: Vec<Alert>,
    pub user: user::Row,
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
        // The actor in this case is the downstream server, whose identity was verified by comparing
        // the server secret provided with the one we have.  In the future, perhaps it would be good
        // to use a service account here?

        let UpsertUserResult { user } = UpsertRegisteredUser::new(self.input.clone())
            .call(pool)
            .await?;

        let result = sqlx::query_as::<_, DatabaseSession>(
            r#"insert into sessions (user_id) values ($1)
                returning encode(session_id, 'hex') session_id, user_id"#,
        )
        .bind(&user.id)
        .fetch_one(pool)
        .await?;

        log::debug!(
            "session id for user {:?}: {:?}",
            user.name,
            result.session_id
        );

        Ok(CreateSessionResult {
            alerts: vec![],
            user,
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
        let result = sqlx::query_as::<_, (Uuid,)>(
            r#"delete from sessions
                where session_id = decode($1, 'hex') and user_id = $2::uuid
                returning id"#,
        )
        .bind(&self.session_id)
        .bind(&self.viewer.user_id)
        .fetch_optional(pool)
        .await;

        match result {
            Ok(row) => match row {
                Some((deleted_session_id,)) => {
                    log::info!("session deleted: {}", deleted_session_id);
                }
                None => {
                    log::warn!("no session {} found to delete", &self.session_id);
                }
            },
            Err(err) => {
                log::warn!("no session {} deleted: {}", &self.session_id, err);
            }
        }

        Ok(DeleteSessionResult {
            deleted_session_id: self.session_id.clone(),
        })
    }
}
