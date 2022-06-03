use async_graphql::*;

use super::{relay::conn, TopicConnection};

pub struct QueryInfo {
    pub string_tokens: Vec<String>,
}

#[Object]
impl QueryInfo {
    async fn string_tokens(&self) -> Vec<String> {
        self.string_tokens.clone()
    }

    async fn topics(&self) -> Result<TopicConnection> {
        conn(vec![])
    }
}
