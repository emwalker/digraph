use async_graphql::*;
use async_graphql::{EmptyMutation, EmptySubscription};
use sqlx::postgres::PgPool;

use crate::psql::Repo;
use crate::schema::View;

pub struct QueryRoot;

pub type Schema = async_graphql::Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(Clone)]
pub struct State {
    pool: PgPool,
    pub schema: Schema,
}

impl State {
    pub fn new(pool: PgPool, schema: Schema) -> Self {
        Self { pool, schema }
    }

    pub fn create_repo(&self) -> Repo {
        Repo::new(self.pool.clone())
    }
}

#[Object]
impl QueryRoot {
    // viewerId: ID!
    // currentOrganizationLogin: String!,
    // currentRepositoryName: String,
    // repositoryIds: [ID!],
    // searchString: String,
    async fn view(&self, #[graphql(desc = "Viewer id")] viewer_id: ID) -> Result<View> {
        Ok(View { viewer_id })
    }
}
