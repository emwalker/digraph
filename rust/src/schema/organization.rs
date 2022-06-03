use async_graphql::*;

#[derive(Clone, SimpleObject)]
pub struct Organization {
    pub id: ID,
    pub name: String,
    pub login: String,
}
