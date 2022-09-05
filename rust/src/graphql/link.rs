use async_graphql::connection::*;
use async_graphql::{Context, Object, SimpleObject};

use super::{relay::conn, DateTime, LiveSearchTopicsPayload, SynonymMatch, TopicConnection, User};
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

#[derive(Clone, Debug)]
pub struct LinkDetail {
    pub color: String,
    pub link_id: Oid,
    pub parent_topic_ids: Vec<Oid>,
    pub repo_id: RepoId,
    pub title: String,
    pub url: String,
}

#[Object]
impl LinkDetail {
    async fn available_parent_topics(
        &self,
        ctx: &Context<'_>,
        search_string: Option<String>,
    ) -> Result<LiveSearchTopicsPayload> {
        let git::FetchTopicLiveSearchResult { synonym_matches } = ctx
            .data_unchecked::<Store>()
            .search_topics(search_string)
            .await?;
        let synonym_matches = synonym_matches
            .iter()
            .map(SynonymMatch::from)
            .collect::<Vec<SynonymMatch>>();
        Ok(LiveSearchTopicsPayload { synonym_matches })
    }

    async fn display_color(&self) -> &str {
        if self.repo_id.is_wiki() {
            ""
        } else {
            DEFAULT_PRIVATE_COLOR
        }
    }

    async fn link_id(&self) -> &str {
        self.link_id.as_str()
    }

    async fn title(&self) -> &str {
        &self.title
    }

    async fn url(&self) -> &str {
        &self.url
    }

    async fn viewer_can_update(&self, ctx: &Context<'_>) -> bool {
        ctx.data_unchecked::<Store>()
            .viewer
            .write_repo_ids
            .include(&self.repo_id)
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
            .fetch_topics(&self.parent_topic_ids, 50)
            .await?;
        conn(after, before, first, last, topics)
    }
}

#[derive(Debug)]
pub struct Link {
    pub details: Vec<LinkDetail>,
    pub display_detail: LinkDetail,
    pub id: Oid,
    pub newly_added: bool,
    pub viewer_review: Option<LinkReview>,
}

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
        // FIXME
        self.display_detail
            .parent_topics(ctx, after, before, first, last)
            .await
    }

    async fn display_color(&self) -> &str {
        &self.display_detail.color
    }

    async fn display_title(&self) -> &str {
        &self.display_detail.title
    }

    async fn display_url(&self) -> &str {
        &self.display_detail.url
    }

    async fn details(&self) -> &Vec<LinkDetail> {
        &self.details
    }

    async fn loading(&self) -> bool {
        false
    }

    async fn id(&self) -> String {
        self.id.to_string()
    }

    async fn newly_added(&self) -> bool {
        self.newly_added
    }

    async fn viewer_can_update(&self, ctx: &Context<'_>) -> Result<bool> {
        for details in &self.details {
            if details.viewer_can_update(ctx).await? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    async fn viewer_review(&self) -> &Option<LinkReview> {
        &self.viewer_review
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
            .data_unchecked::<Store>()
            .user(self.user_id.clone())
            .await?;

        match user {
            Some(user) => Ok(user),
            None => Err(Error::NotFound(format!("user not found: {}", self.user_id))),
        }
    }
}
