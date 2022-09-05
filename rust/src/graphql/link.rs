use async_graphql::connection::*;
use async_graphql::{Context, Object, SimpleObject};
use itertools::Itertools;

use super::{relay, time, LiveSearchTopicsPayload, TopicConnection, User};
use crate::git;
use crate::prelude::*;
use crate::store::Store;

impl TryFrom<Option<Link>> for Link {
    type Error = Error;

    fn try_from(value: Option<Link>) -> Result<Self> {
        match value {
            Some(link) => Ok(link),
            None => Err(Error::NotFound("link not found".into())),
        }
    }
}

pub struct LinkDetail(pub(crate) git::LinkDetail);

#[Object]
impl LinkDetail {
    async fn available_parent_topics(
        &self,
        ctx: &Context<'_>,
        search_string: Option<String>,
    ) -> Result<LiveSearchTopicsPayload> {
        let result = ctx
            .data_unchecked::<Store>()
            .search_topics(search_string)
            .await?;
        Ok(LiveSearchTopicsPayload(result))
    }

    async fn display_color(&self) -> &str {
        if self.0.repo_id.is_wiki() {
            ""
        } else {
            DEFAULT_PRIVATE_COLOR
        }
    }

    async fn link_id(&self) -> &str {
        self.0.link_id().as_str()
    }

    async fn title(&self) -> &str {
        self.0.title()
    }

    async fn url(&self) -> &str {
        self.0.url()
    }

    async fn viewer_can_update(&self, ctx: &Context<'_>) -> bool {
        ctx.data_unchecked::<Store>()
            .viewer
            .write_repo_ids
            .include(&self.0.repo_id)
    }

    async fn parent_topics(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<TopicConnection> {
        let topics = ctx
            .data_unchecked::<Store>()
            .fetch_topics(self.0.parent_topic_ids(), 50)
            .await?;
        relay::topics(after, before, first, last, topics)
    }
}

#[derive(Debug)]
pub struct Link(pub(crate) git::Link);

#[derive(SimpleObject)]
pub struct LinkConnectionFields {
    pub total_count: i64,
}

pub type LinkEdge = Edge<String, Link, EmptyFields>;
pub type LinkConnection = Connection<String, Link, LinkConnectionFields, EmptyFields>;

#[Object]
impl Link {
    async fn display_parent_topics(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<TopicConnection> {
        let topics = ctx
            .data_unchecked::<Store>()
            .fetch_topics(self.0.parent_topic_ids(), 50)
            .await?;
        relay::topics(after, before, first, last, topics)
    }

    async fn display_color(&self) -> &str {
        self.0.display_color()
    }

    async fn display_title(&self) -> &str {
        self.0.display_title()
    }

    async fn display_url(&self) -> &str {
        self.0.display_url()
    }

    async fn details(&self) -> Vec<LinkDetail> {
        self.0.details.iter().map(LinkDetail::from).collect_vec()
    }

    async fn loading(&self) -> bool {
        false
    }

    async fn id(&self) -> String {
        self.0.id.to_string()
    }

    // Used by the JS client to highlight a link that was just added
    async fn newly_added(&self) -> bool {
        false
    }

    async fn viewer_can_update(&self, ctx: &Context<'_>) -> bool {
        let viewer = &ctx.data_unchecked::<Store>().viewer;
        self.0.can_update(&viewer.write_repo_ids)
    }

    async fn viewer_review(&self) -> &Option<LinkReview> {
        &None
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct LinkReview {
    pub reviewed_at: Option<time::DateTime>,
    pub user_id: String,
}

#[Object]
impl LinkReview {
    async fn reviewed_at(&self) -> Option<time::DateTime> {
        self.reviewed_at.clone()
    }

    async fn user(&self, ctx: &Context<'_>) -> Result<User> {
        let user = ctx
            .data_unchecked::<Store>()
            .user(self.user_id.clone())
            .await?;

        match user {
            Some(user) => Ok(user),
            None => Err(Error::NotFound(format!("user not found: {}", self.user_id))),
        }
    }
}
