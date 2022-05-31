use async_graphql::connection::*;
use async_graphql::*;
use itertools::Itertools;

use super::link::LinkConnection;
use super::relay::conn;
use super::synonym::{Synonym, Synonyms};
use crate::psql::Repo;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Topic {
    pub child_link_ids: Vec<String>,
    pub child_topic_ids: Vec<String>,
    pub id: ID,
    pub name: String,
    pub parent_topic_ids: Vec<String>,
    pub resource_path: String,
    pub synonyms: Synonyms,
}

pub type TopicConnection = Connection<usize, Topic, EmptyFields, EmptyFields>;

#[Object]
impl Topic {
    async fn child_links(&self, ctx: &Context<'_>) -> Result<LinkConnection> {
        conn(
            ctx.data::<Repo>()?
                .child_links_for_topic(self.id.to_string())
                .await?,
        )
    }

    async fn child_topics(&self, ctx: &Context<'_>) -> Result<TopicConnection> {
        conn(
            ctx.data_unchecked::<Repo>()
                .child_topics_for_topic(self.id.to_string())
                .await?,
        )
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
        conn(
            ctx.data_unchecked::<Repo>()
                .parent_topics_for_topic(self.id.to_string())
                .await?,
        )
    }

    async fn synonyms(&self) -> Vec<Synonym> {
        self.synonyms.into_iter().collect_vec()
    }

    async fn resource_path(&self) -> &str {
        self.resource_path.as_str()
    }
}
