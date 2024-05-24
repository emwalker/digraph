use async_graphql::connection::*;
use async_graphql::*;

use crate::prelude::*;

#[derive(SimpleObject)]
pub struct ActivityLineItem {
    pub description: String,
    pub created_at: Timestamp,
}

pub type ActivityLineItemConnection =
    Connection<String, ActivityLineItem, EmptyFields, EmptyFields>;
