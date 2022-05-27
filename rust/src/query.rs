use async_graphql::*;
use async_graphql::{EmptyMutation, EmptySubscription};

use crate::schema::View;

pub struct QueryRoot;

pub type Schema = async_graphql::Schema<QueryRoot, EmptyMutation, EmptySubscription>;

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
