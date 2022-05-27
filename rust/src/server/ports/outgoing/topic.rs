use async_graphql::Result;
use async_trait::async_trait;

use crate::schema::Topic;

#[async_trait]
pub trait Port {
    async fn child_topics(&self, topic_id: String) -> Result<Option<Vec<Topic>>>;
    async fn get(&self, topic_id: String) -> Result<Option<Topic>>;
    async fn parent_topics(&self, topic_id: String) -> Result<Option<Vec<Topic>>>;
}
