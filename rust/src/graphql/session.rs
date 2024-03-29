pub use async_graphql::{Context, Object, SimpleObject, ID};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, SimpleObject)]
pub struct Session {
    pub id: String,
}

#[derive(Debug, SimpleObject)]
pub struct SessionEdge {
    pub cursor: String,
    pub node: Session,
}
