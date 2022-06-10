use async_graphql::EmptySubscription;
use async_graphql::*;
use futures::lock::Mutex;
use sqlx::postgres::PgPool;

use crate::psql::Repo;
mod activity;
pub use activity::*;
mod alert;
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
pub mod timerange;
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

    pub async fn viewer(&self, user_id: Option<String>) -> Viewer {
        let mut query_ids = vec![GUEST_ID.to_string()];
        if user_id.is_some() {
            query_ids.push(user_id.unwrap_or_default());
        }
        Viewer { query_ids }
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
