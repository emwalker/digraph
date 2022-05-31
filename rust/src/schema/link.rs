use async_graphql::connection::*;
use async_graphql::*;

use super::relay::conn;
use super::topic::TopicConnection;
use crate::psql::Repo;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Link {
    pub id: ID,
    pub parent_topic_ids: Vec<String>,
    pub title: String,
    pub url: String,
}

pub type LinkConnection = Connection<usize, Link, EmptyFields, EmptyFields>;

#[Object]
impl Link {
    async fn id(&self) -> ID {
        self.id.to_owned()
    }

    async fn loading(&self) -> bool {
        false
    }

    async fn newly_added(&self) -> bool {
        false
    }

    async fn parent_topics(&self, ctx: &Context<'_>) -> Result<TopicConnection> {
        conn(
            ctx.data_unchecked::<Repo>()
                .parent_topics_for_link(self.id.to_string())
                .await?,
        )
    }

    async fn title(&self) -> String {
        self.title.to_owned()
    }

    async fn url(&self) -> String {
        self.url.to_owned()
    }
}
