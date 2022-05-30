use async_graphql::connection::*;
use async_graphql::*;

use super::link::LinkConnection;
use super::synonym::{Synonym, Synonyms};
use crate::state::State;

#[derive(Clone, Debug)]
pub struct Topic {
    pub id: ID,
    pub name: String,
    pub resource_path: String,
    pub synonyms: Synonyms,
}

pub type TopicConnection = Connection<usize, Topic, EmptyFields, EmptyFields>;

#[Object]
impl Topic {
    async fn child_links(&self, ctx: &Context<'_>) -> Result<LinkConnection> {
        ctx.data::<State>()?
            .links
            .child_links_by_topic_id(self.id.to_string())
            .await
            .into()
    }

    async fn child_topics(&self, ctx: &Context<'_>) -> Result<TopicConnection> {
        ctx.data_unchecked::<State>()
            .topics
            .child_topics_by_id(self.id.to_string())
            .await
            .into()
    }

    async fn display_name(&self) -> String {
        self.synonyms.display_name("en", &self.name, None)
    }

    async fn id(&self) -> &str {
        self.id.as_str()
    }

    async fn name(&self) -> &str {
        self.name.as_str()
    }

    async fn parent_topics(&self, ctx: &Context<'_>) -> Result<TopicConnection> {
        ctx.data_unchecked::<State>()
            .topics
            .parent_topics_by_id(self.id.to_string())
            .await
            .into()
    }

    async fn synonyms(&self) -> Vec<Synonym> {
        self.synonyms.to_vec()
    }

    async fn resource_path(&self) -> &str {
        self.resource_path.as_str()
    }
}
