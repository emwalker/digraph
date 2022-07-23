use async_graphql::connection::*;
use async_graphql::{Context, Object, SimpleObject, ID};

use super::{
    relay::conn, DateTime, LiveSearchTopicsPayload, Repository, SynonymMatch, TopicConnection, User,
};
use crate::git;
use crate::prelude::*;
use crate::store::Store;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Link {
    pub path: RepoPath,
    pub newly_added: bool,
    pub parent_topic_paths: Vec<RepoPath>,
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
        if self.path.starts_with(WIKI_REPO_PREFIX) {
            ""
        } else {
            DEFAULT_PRIVATE_COLOR
        }
    }

    async fn loading(&self) -> bool {
        false
    }

    async fn id(&self) -> String {
        self.path.to_string()
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
        let topics = ctx
            .data_unchecked::<Store>()
            .parent_topics_for_link(&self.path)
            .await?;
        conn(after, before, first, last, topics)
    }

    async fn path(&self) -> String {
        self.path.to_string()
    }

    async fn repository(&self, ctx: &Context<'_>) -> Result<Repository> {
        ctx.data_unchecked::<Store>()
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

    async fn viewer_can_update(&self, ctx: &Context<'_>) -> Result<bool> {
        let repo = ctx.data_unchecked::<Store>();
        if repo.viewer.is_guest() {
            return Ok(false);
        }

        // TODO: Narrow down write permissions to a specific topics and their subtopics
        Ok(repo.viewer.write_prefixes.include(&self.path))
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
            .data_unchecked::<Store>()
            .user(self.user_id.clone())
            .await?;

        match user {
            Some(user) => Ok(user),
            None => Err(Error::NotFound(format!("user not found: {}", self.user_id))),
        }
    }
}
