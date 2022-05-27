use async_graphql::*;

#[derive(Clone, SimpleObject)]
pub struct User {
    pub handle: String,
    pub id: ID,
}
