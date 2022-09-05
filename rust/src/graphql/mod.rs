use async_graphql::EmptySubscription;
use sqlx::postgres::PgPool;

use crate::prelude::*;
use crate::redis;
use crate::store::Store;
use crate::types::Timespec;

mod activity;
pub use activity::*;
pub mod alert;
pub use alert::*;
mod git;
pub use git::*;
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
mod psql;
mod repository;
pub use repository::*;
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

#[derive(Clone)]
pub struct State {
    pub pool: PgPool,
    pub redis: redis::Redis,
    pub root: git::DataRoot,
    pub schema: Schema,
    pub server_secret: String,
}

impl State {
    pub fn new(
        pool: PgPool,
        root: git::DataRoot,
        schema: Schema,
        server_secret: String,
        redis: redis::Redis,
    ) -> Self {
        Self {
            pool,
            root,
            redis,
            schema,
            server_secret,
        }
    }

    pub fn store(&self, viewer: &Viewer, timespec: &Timespec) -> Store {
        Store::new(
            viewer.to_owned(),
            git::Client::new(viewer, &self.root, timespec.to_owned()),
            self.pool.clone(),
            self.server_secret.clone(),
            self.redis.clone(),
        )
    }

    pub async fn authenticate(&self, user_info: Option<(String, String)>) -> Viewer {
        match user_info {
            Some((user_id, session_id)) => {
                let result = sqlx::query_as::<_, (Vec<sqlx::types::Uuid>,)>(
                    "select array_agg(ur.repository_id)
                    from sessions s
                    join users u on s.user_id = u.id
                    join users_repositories ur on u.id = ur.user_id
                    where s.user_id = $1::uuid and s.session_id = decode($2, 'hex')
                        and ur.can_write
                    group by u.id",
                )
                .bind(&user_id)
                .bind(&session_id)
                .fetch_optional(&self.pool)
                .await;

                match result {
                    Ok(row) => match &row {
                        Some((repo_ids,)) => {
                            log::info!("found user and session in database: {}", user_id);
                            match RepoIds::try_from(repo_ids) {
                                Ok(repo_ids) => Viewer {
                                    write_repo_ids: repo_ids.clone(),
                                    read_repo_ids: repo_ids,
                                    session_id: Some(session_id),
                                    super_user: false,
                                    user_id,
                                },

                                Err(err) => {
                                    log::error!("problem decoding write prefixes: {}", err);
                                    Viewer::guest()
                                }
                            }
                        }

                        None => {
                            log::warn!(
                                "no user session found in database, proceeding as guest: {}, {}",
                                user_id,
                                session_id
                            );
                            Viewer::guest()
                        }
                    },

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
    async fn alerts(&self) -> Vec<alert::Alert> {
        vec![]
    }

    async fn view(
        &self,
        viewer_id: ID,
        repository_ids: Option<Vec<ID>>,
        search_string: Option<String>,
    ) -> Result<View> {
        let view = View {
            repository_ids,
            search_string,
            viewer_id,
        };

        Ok(view)
    }
}
