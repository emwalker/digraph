use async_graphql::Object;

use super::{relay::connection, TopicConnection};
use crate::prelude::*;

pub struct QueryInfo {
    pub string_tokens: Vec<String>,
}

#[Object]
impl QueryInfo {
    async fn string_tokens(&self) -> Vec<String> {
        self.string_tokens.clone()
    }

    async fn topics(&self) -> Result<TopicConnection> {
        connection(None, None, None, None, vec![])
    }
}
