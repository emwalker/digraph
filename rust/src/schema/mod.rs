use async_graphql::EmptySubscription;
use async_graphql::*;
use futures::lock::Mutex;
use sqlx::postgres::PgPool;

use crate::psql::Repo;
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
    pub user_id: String,
    pub query_ids: Vec<String>,
}

impl Viewer {
    pub fn is_guest(&self) -> bool {
        self.user_id == GUEST_ID
    }
}

#[derive(Clone)]
pub struct State {
    pool: PgPool,
    pub schema: Schema,
    pub server_secret: String,
}

impl State {
    pub fn new(pool: PgPool, schema: Schema, server_secret: String) -> Self {
        Self {
            pool,
            schema,
            server_secret,
        }
    }

    pub fn create_repo(&self, viewer: Viewer) -> Repo {
        Repo::new(viewer, self.pool.clone(), self.server_secret.clone())
    }

    pub async fn viewer(&self, user_id: Option<String>) -> Viewer {
        log::debug!("loading viewer from user id: {:?}", user_id);
        let not_guest = user_id.is_some();
        let user_id = user_id.unwrap_or_else(|| GUEST_ID.to_string());
        let mut query_ids = vec![user_id.clone()];
        if not_guest {
            query_ids.push(GUEST_ID.to_string());
        }
        Viewer { user_id, query_ids }
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
