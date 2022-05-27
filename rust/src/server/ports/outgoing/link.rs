use async_graphql::Result;
use async_trait::async_trait;

use crate::schema::Link;

#[async_trait]
pub trait Port {
    async fn child_links(&self, topic_id: String) -> Result<Option<Vec<Link>>>;
}
