use async_graphql::connection::*;
use async_graphql::*;
use itertools::Itertools;

use super::{relay::conn, timerange::Prefix, LinkConnection, Repository, Synonym, Synonyms};
use crate::psql::Repo;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Topic {
    pub child_link_ids: Vec<String>,
    pub child_topic_ids: Vec<String>,
    pub id: ID,
    pub name: String,
    pub parent_topic_ids: Vec<String>,
    pub prefix: Prefix,
    pub repository_id: String,
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
        self.synonyms.display_name("en", &self.name, &self.prefix)
    }

    async fn id(&self) -> ID {
        self.id.to_owned()
    }

    async fn loading(&self) -> bool {
        false
    }

    async fn name(&self) -> &str {
        self.name.as_str()
    }

    async fn newly_added(&self) -> bool {
        false
    }

    async fn parent_topics(&self, ctx: &Context<'_>) -> Result<TopicConnection> {
        conn(
            ctx.data_unchecked::<Repo>()
                .parent_topics_for_topic(self.id.to_string())
                .await?,
        )
    }

    async fn repository(&self, ctx: &Context<'_>) -> Result<Option<Repository>> {
        ctx.data_unchecked::<Repo>()
            .repository(self.repository_id.clone())
            .await
    }

    async fn synonyms(&self) -> Vec<Synonym> {
        self.synonyms.into_iter().collect_vec()
    }

    async fn resource_path(&self) -> &str {
        self.resource_path.as_str()
    }

    async fn viewer_can_update(&self) -> bool {
        false
    }
}
