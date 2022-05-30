use async_graphql::connection::*;
use async_graphql::*;

use super::topic::TopicConnection;
use crate::state::State;

#[derive(Clone)]
pub struct Link {
    pub id: ID,
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
        ctx.data_unchecked::<State>()
            .topics
            .parent_topics_for_link_id(self.id.to_string())
            .await
            .into()
    }

    async fn title(&self) -> String {
        self.title.to_owned()
    }

    async fn url(&self) -> String {
        self.url.to_owned()
    }
}
