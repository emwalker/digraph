use async_graphql::connection::*;
use async_graphql::*;

use super::link::LinkConnection;
use super::relay::conn;
use crate::state::State;

#[derive(Clone, Debug)]
pub struct Topic {
    pub id: ID,
    pub name: String,
    pub resource_path: String,
}

pub type TopicConnection = Connection<usize, Topic, EmptyFields, EmptyFields>;

#[Object]
impl Topic {
    async fn child_links(&self, ctx: &Context<'_>) -> Result<LinkConnection> {
        conn(
            ctx.data::<State>()?
                .links
                .child_links_by_topic_id(self.id.to_string())
                .await?
                .unwrap_or_default(),
        )
        .await
    }

    async fn child_topics(&self, ctx: &Context<'_>) -> Result<TopicConnection> {
        conn(
            ctx.data_unchecked::<State>()
                .topics
                .child_topics_by_id(self.id.to_string())
                .await?
                .unwrap_or_default(),
        )
        .await
    }

    async fn id(&self) -> &str {
        self.id.as_str()
    }

    async fn name(&self) -> &str {
        self.name.as_str()
    }

    async fn parent_topics(&self, ctx: &Context<'_>) -> Result<TopicConnection> {
        conn(
            ctx.data_unchecked::<State>()
                .topics
                .parent_topics_by_id(self.id.to_string())
                .await?
                .unwrap_or_default(),
        )
        .await
    }

    async fn resource_path(&self) -> &str {
        self.resource_path.as_str()
    }
}
