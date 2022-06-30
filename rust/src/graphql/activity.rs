use async_graphql::connection::*;
use async_graphql::*;
use chrono::{DateTime, Utc};

#[derive(SimpleObject)]
pub struct ActivityLineItem {
    pub description: String,
    pub created_at: DateTime<Utc>,
}

pub type ActivityLineItemConnection =
    Connection<String, ActivityLineItem, EmptyFields, EmptyFields>;
