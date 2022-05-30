use async_graphql::Result;
use std::convert::From;
use std::sync::Arc;

use crate::{
    schema::{conn, Link, LinkConnection},
    server::ports::outgoing::link,
};

pub struct Service {
    pub repo: Arc<dyn link::Port + Send + Sync>,
}

pub struct Conn(pub Result<Option<Vec<Link>>>);

impl From<Conn> for Result<LinkConnection> {
    fn from(c: Conn) -> Self {
        conn(c.0?.unwrap_or_default())
    }
}

impl Service {
    pub async fn child_links_by_topic_id(&self, topic_id: String) -> Conn {
        Conn(self.repo.child_links(topic_id).await)
    }
}
