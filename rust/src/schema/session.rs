use serde::Deserialize;

use crate::prelude::*;

#[derive(Clone, Debug, Deserialize, SimpleObject)]
pub struct Session {
    pub id: String,
}

#[derive(Debug, SimpleObject)]
pub struct SessionEdge {
    pub cursor: String,
    pub node: Session,
}
