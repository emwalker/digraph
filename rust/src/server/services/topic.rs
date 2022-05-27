use async_graphql::Result;
use std::sync::Arc;

use crate::{schema::Topic, server::ports::outgoing::topic};

pub struct Service {
    pub repo: Arc<dyn topic::Port + Send + Sync>,
}

impl Service {
    pub async fn by_id(&self, id: String) -> Result<Option<Topic>> {
        self.repo.get(id).await
    }

    pub async fn child_topics_by_id(&self, topic_id: String) -> Result<Option<Vec<Topic>>> {
        self.repo.child_topics(topic_id).await
    }

    pub async fn parent_topics_by_id(&self, topic_id: String) -> Result<Option<Vec<Topic>>> {
        self.repo.parent_topics(topic_id).await
    }
}
