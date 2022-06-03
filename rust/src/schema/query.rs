use super::{relay::conn, TopicConnection};
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
        conn(None, None, None, None, vec![])
    }
}
