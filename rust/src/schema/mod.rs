use async_graphql::EmptySubscription;
use async_graphql::*;
use futures::lock::Mutex;
use sqlx::{postgres::PgPool, types::Uuid, FromRow};

use crate::psql::Repo;
mod activity;
pub use activity::*;
mod alert;
pub use alert::*;
mod relay;
pub use relay::*;
mod link;
pub use link::*;
mod organization;
pub use organization::*;
mod query;
pub use query::*;
mod repository;
pub use repository::*;
mod search;
pub use search::*;
mod session;
pub use session::*;
mod synonym;
pub use synonym::*;
pub mod timerange;
mod topic;
pub use topic::*;
mod user;
pub use user::*;
mod view;
pub use view::*;

pub struct MutationRoot;

pub struct QueryRoot;

pub type Schema = async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Clone, Debug)]
pub struct Viewer {
    pub query_ids: Vec<String>,
}

#[derive(FromRow, Clone, Debug)]
pub struct DatabaseSession {
    user_id: Uuid,
}

#[derive(Clone)]
pub struct State {
    pool: PgPool,
    pub schema: Schema,
}

impl State {
    pub fn new(pool: PgPool, schema: Schema) -> Self {
        Self { pool, schema }
    }

    pub fn create_repo(&self, viewer: Viewer) -> Repo {
        Repo::new(viewer, self.pool.clone())
    }

    pub async fn viewer(&self, session_id: String) -> Viewer {
        let session_id = session_id.as_bytes();
        let result = sqlx::query_as!(
            DatabaseSession,
            r#"select user_id from sessions where session_id = $1"#,
            &session_id,
        )
        .fetch_optional(&self.pool)
        .await;

        match result {
            Ok(row) => match row {
                Some(session) => Viewer {
                    query_ids: vec![GUEST_ID.to_string(), session.user_id.to_string()],
                },
                None => Viewer {
                    query_ids: vec![GUEST_ID.to_string()],
                },
            },
            Err(_) => {
                log::warn!("problem fetching session: {:?}", session_id);
                Viewer {
                    query_ids: vec![GUEST_ID.to_string()],
                }
            }
        }
    }
}

#[Object]
impl QueryRoot {
    async fn alerts(&self) -> Vec<Alert> {
        vec![]
    }

    async fn view(
        &self,
        ctx: &Context<'_>,
        viewer_id: ID,
        current_organization_login: String,
        current_repository_name: Option<String>,
        repository_ids: Option<Vec<ID>>,
        search_string: Option<String>,
    ) -> Result<View> {
        let view = View {
            current_organization_login,
            current_repository_name,
            repository_ids,
            search_string,
            viewer_id,
        };

        // Add the view to the context
        let mutex = ctx.data::<Mutex<Option<View>>>()?;
        *mutex.lock().await = Some(view.clone());

        Ok(view)
    }
}

#[derive(SimpleObject)]
pub struct UserEdge {
    cursor: String,
    node: User,
}

#[derive(SimpleObject)]
pub struct SessionEdge {
    cursor: String,
    node: Session,
}

#[derive(SimpleObject)]
pub struct CreateSessionPayload {
    alerts: Vec<Alert>,
    user_edge: Option<UserEdge>,
    session_edge: Option<SessionEdge>,
}

#[derive(InputObject, Debug)]
struct CreateGithubSessionInput {
    client_mutation_id: Option<String>,
    github_avatar_url: String,
    github_username: String,
    name: String,
    primary_email: String,
    server_secret: String,
}

#[Object]
impl MutationRoot {
    async fn create_github_session(
        &self,
        input: CreateGithubSessionInput,
    ) -> Result<CreateSessionPayload> {
        log::info!("creating GitHub session: {:?}", input);
        Ok(CreateSessionPayload {
            alerts: vec![],
            user_edge: None,
            session_edge: None,
        })
    }
}
