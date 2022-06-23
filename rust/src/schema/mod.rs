use async_graphql::EmptySubscription;
use sqlx::postgres::PgPool;

use crate::git;
use crate::prelude::*;
use crate::repo::Repo;
mod activity;
pub use activity::*;
pub mod alert;
pub use alert::*;
mod relay;
pub use relay::*;
mod link;
pub use link::*;
mod mutation;
pub use mutation::*;
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
mod timerange;
pub use timerange::*;
mod topic;
pub use topic::*;
mod user;
pub use user::*;
mod view;
pub use view::*;

pub struct QueryRoot;

pub type Schema = async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Clone, Debug)]
pub struct Viewer {
    pub query_ids: Vec<String>,
    pub mutation_ids: Vec<String>,
    pub session_id: Option<String>,
    pub user_id: String,
}

impl Viewer {
    pub fn guest() -> Self {
        let user_id = GUEST_ID.to_string();
        Viewer {
            mutation_ids: vec![],
            query_ids: vec![user_id.clone()],
            session_id: None,
            user_id,
        }
    }

    pub fn is_guest(&self) -> bool {
        self.session_id.is_none()
    }
}

#[derive(Clone)]
pub struct State {
    pub git: git::Git,
    pub pool: PgPool,
    pub schema: Schema,
    pub server_secret: String,
}

impl State {
    pub fn new(pool: PgPool, schema: Schema, server_secret: String, git: git::Git) -> Self {
        Self {
            git,
            pool,
            schema,
            server_secret,
        }
    }

    pub fn create_repo(&self, viewer: Viewer) -> Repo {
        Repo::new(
            viewer,
            self.git.clone(),
            self.pool.clone(),
            self.server_secret.clone(),
        )
    }

    pub async fn authenticate(&self, user_info: Option<(String, String)>) -> Viewer {
        match user_info {
            Some((user_id, session_id)) => {
                let result = sqlx::query_as::<_, (i64,)>(
                    r#"select count(*)
                    from sessions
                    where user_id = $1::uuid and session_id = decode($2, 'hex')"#,
                )
                .bind(&user_id)
                .bind(&session_id)
                .fetch_one(&self.pool)
                .await;

                match result {
                    Ok((count,)) => {
                        if count == 0 {
                            log::warn!(
                                "no user session found in database, proceeding as guest: {}, {}",
                                user_id,
                                session_id
                            );
                            return Viewer::guest();
                        }
                        log::info!("found user and session in database: {}", user_id);
                        Viewer {
                            mutation_ids: vec![user_id.clone()],
                            query_ids: vec![user_id.clone(), GUEST_ID.to_string()],
                            session_id: Some(session_id),
                            user_id,
                        }
                    }
                    Err(err) => {
                        log::warn!("failed to fetch session info, proceeding as guest: {}", err);
                        Viewer::guest()
                    }
                }
            }

            None => {
                log::info!("no session info provided, proceeding as guest");
                Viewer::guest()
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

        Ok(view)
    }
}
