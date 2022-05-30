use async_graphql::*;
use std::convert::From;
use std::sync::Arc;

use crate::{
    schema::{conn, Topic, TopicConnection},
    server::ports::outgoing::topic,
};

pub struct Service {
    pub repo: Arc<dyn topic::Port + Send + Sync>,
}

pub struct Conn(pub Result<Option<Vec<Topic>>>);

impl From<Conn> for Result<TopicConnection> {
    fn from(c: Conn) -> Self {
        conn(c.0?.unwrap_or_default())
    }
}

impl Service {
    pub async fn by_id(&self, id: String) -> Result<Option<Topic>> {
        self.repo.get(id).await
    }

    pub async fn child_topics_by_id(&self, topic_id: String) -> Conn {
        Conn(self.repo.child_topics(topic_id).await)
    }

    pub async fn parent_topics_by_id(&self, topic_id: String) -> Conn {
        Conn(self.repo.parent_topics(topic_id).await)
    }

    pub async fn parent_topics_for_link_id(&self, link_id: String) -> Conn {
        Conn(self.repo.parent_topics_for_link(link_id).await)
    }
}
