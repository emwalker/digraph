use async_graphql::connection::*;

use super::{relay::conn, Repository, TopicConnection};
use crate::prelude::*;
use crate::psql::Repo;
use crate::Result;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Link {
    pub id: ID,
    pub newly_added: bool,
    pub parent_topic_ids: Vec<String>,
    pub repository_id: ID,
    pub title: String,
    pub url: String,
}

pub type LinkEdge = Edge<String, Link, EmptyFields>;
pub type LinkConnection = Connection<String, Link, EmptyFields, EmptyFields>;

#[Object]
impl Link {
    async fn available_parent_topics(
        &self,
        ctx: &Context<'_>,
        search_string: Option<String>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<TopicConnection> {
        conn(
            after,
            before,
            first,
            last,
            ctx.data_unchecked::<Repo>()
                .search_topics(search_string)
                .await?,
        )
    }

    async fn id(&self) -> ID {
        self.id.to_owned()
    }

    async fn loading(&self) -> bool {
        false
    }

    async fn newly_added(&self) -> bool {
        self.newly_added
    }

    async fn parent_topics(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<TopicConnection> {
        conn(
            after,
            before,
            first,
            last,
            ctx.data_unchecked::<Repo>()
                .parent_topics_for_link(self.id.to_string())
                .await?,
        )
    }

    async fn repository(&self, ctx: &Context<'_>) -> Result<Repository> {
        ctx.data_unchecked::<Repo>()
            .repository(self.repository_id.to_string())
            .await?
            .ok_or_else(|| Error::NotFound(format!("repo id {}", *self.repository_id)))
    }

    async fn title(&self) -> String {
        self.title.to_owned()
    }

    async fn url(&self) -> String {
        self.url.to_owned()
    }
}