use async_graphql::*;
use async_graphql::{EmptyMutation, EmptySubscription};
use futures::lock::Mutex;
use sqlx::postgres::PgPool;

use crate::psql::Repo;
use crate::schema::{Alert, View};

pub struct QueryRoot;

pub type Schema = async_graphql::Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(Clone)]
pub struct State {
    pool: PgPool,
    pub schema: Schema,
    pub view: Option<View>,
}

impl State {
    pub fn new(pool: PgPool, schema: Schema) -> Self {
        Self {
            pool,
            schema,
            view: None,
        }
    }

    pub fn create_repo(&self) -> Repo {
        Repo::new(self.pool.clone())
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
        repository_ids: Vec<ID>,
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
