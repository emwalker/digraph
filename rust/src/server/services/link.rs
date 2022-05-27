use async_graphql::Result;
use std::sync::Arc;

use crate::{schema::Link, server::ports::outgoing::link};

pub struct Service {
    pub repo: Arc<dyn link::Port + Send + Sync>,
}

impl Service {
    pub async fn child_links_by_topic_id(&self, topic_id: String) -> Result<Option<Vec<Link>>> {
        self.repo.child_links(topic_id).await
    }
}
