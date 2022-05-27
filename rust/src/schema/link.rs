use async_graphql::*;
use async_graphql::connection::*;

#[derive(Clone, SimpleObject)]
pub struct Link {
    pub id: ID,
    pub title: String,
    pub url: String,
}

pub type LinkConnection = Connection<usize, Link, EmptyFields, EmptyFields>;
