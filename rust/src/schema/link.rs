use async_graphql::connection::*;

use super::{relay::conn, DateTime, Repository, TopicConnection, User};
use crate::prelude::*;
use crate::repo::Repo;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Link {
    pub id: ID,
    pub newly_added: bool,
    pub parent_topic_ids: Vec<String>,
    pub repository_id: ID,
    pub viewer_review: Option<LinkReview>,
    pub title: String,
    pub url: String,
}

#[derive(SimpleObject)]
pub struct LinkConnectionFields {
    pub total_count: i64,
}

pub type LinkEdge = Edge<String, Link, EmptyFields>;
pub type LinkConnection = Connection<String, Link, LinkConnectionFields, EmptyFields>;

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

    async fn viewer_review(&self) -> Option<LinkReview> {
        self.viewer_review.clone()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct LinkReview {
    pub reviewed_at: Option<DateTime>,
    pub user_id: String,
}

#[Object]
impl LinkReview {
    async fn reviewed_at(&self) -> Option<DateTime> {
        self.reviewed_at.clone()
    }

    async fn user(&self, ctx: &Context<'_>) -> Result<User> {
        let user = ctx
            .data_unchecked::<Repo>()
            .user(self.user_id.clone())
            .await?;

        match user {
            Some(user) => Ok(user),
            None => Err(Error::NotFound(format!("user not found: {}", self.user_id))),
        }
    }
}
