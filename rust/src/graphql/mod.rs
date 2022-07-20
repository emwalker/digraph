use async_graphql::EmptySubscription;
use sqlx::postgres::PgPool;

use crate::git;
use crate::prelude::*;
use crate::redis;
use crate::repo::Repo;

mod activity;
pub use activity::*;
pub mod alert;
pub use alert::*;
mod git_object;
pub use git_object::*;
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

    pub fn create_repo(&self, viewer: &Viewer) -> Repo {
        Repo::new(
            viewer.to_owned(),
            git::Git::new(viewer, &self.root),
            self.pool.clone(),
            self.server_secret.clone(),
            self.redis.clone(),
        )
    }

    pub async fn authenticate(&self, user_info: Option<(String, String)>) -> Viewer {
        match user_info {
            Some((user_id, session_id)) => {
                let result = sqlx::query_as::<_, (Vec<String>,)>(
                    r#"select u.write_prefixes
                    from sessions s
                    join users u on s.user_id = u.id
                    where s.user_id = $1::uuid and s.session_id = decode($2, 'hex')"#,
                )
                .bind(&user_id)
                .bind(&session_id)
                .fetch_optional(&self.pool)
                .await;

                match result {
                    Ok(row) => match &row {
                        Some((prefixes,)) => {
                            log::info!("found user and session in database: {}", user_id);
                            let prefixes = RepoPrefixList::from(prefixes);
                            Viewer {
                                write_prefixes: prefixes.clone(),
                                read_prefixes: prefixes,
                                session_id: Some(session_id),
                                super_user: false,
                                user_id,
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
