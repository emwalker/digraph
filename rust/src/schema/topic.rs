use async_graphql::connection::*;
use itertools::Itertools;

use super::{
    relay::conn, timerange::Prefix, LinkConnection, Repository, SearchResultItemConnection,
    Synonym, Synonyms,
};
use crate::prelude::*;
use crate::psql::Repo;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Topic {
    pub id: String,
    pub name: String,
    pub parent_topic_ids: Vec<String>,
    pub prefix: Prefix,
    pub repository_id: String,
    pub repository_is_private: bool,
    pub repository_owner_id: String,
    pub resource_path: String,
    pub root: bool,
    pub synonyms: Synonyms,
}

pub type TopicEdge = Edge<String, Topic, EmptyFields>;
pub type TopicConnection = Connection<String, Topic, EmptyFields, EmptyFields>;

#[Object]
impl Topic {
    #[allow(unused_variables)]
    async fn child_topics(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
        search_string: Option<String>,
    ) -> Result<TopicConnection> {
        conn(
            after,
            before,
            first,
            last,
            ctx.data_unchecked::<Repo>()
                .child_topics_for_topic(self.id.to_string())
                .await?,
        )
    }

    async fn description(&self) -> Option<String> {
        None
    }

    async fn display_name(&self) -> String {
        self.synonyms.display_name("en", &self.name, &self.prefix)
    }

    async fn id(&self) -> ID {
        ID(self.id.to_owned())
    }

    // TODO: rename to childLinks
    #[allow(unused_variables)]
    async fn links(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
        search_string: Option<String>,
    ) -> Result<LinkConnection> {
        conn(
            after,
            before,
            first,
            last,
            ctx.data::<Repo>()?
                .child_links_for_topic(self.id.to_string())
                .await?,
        )
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
                .parent_topics_for_topic(self.id.to_string())
                .await?,
        )
    }

    async fn repository(&self, ctx: &Context<'_>) -> Result<Option<Repository>> {
        ctx.data_unchecked::<Repo>()
            .repository(self.repository_id.clone())
            .await
            .map_err(|_e| Error::NotFound(format!("repo id {}", self.repository_id)))
    }

    async fn search(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        after: Option<String>,
        last: Option<i32>,
        before: Option<String>,
        search_string: String,
    ) -> Result<SearchResultItemConnection> {
        conn(
            after,
            before,
            first,
            last,
            ctx.data_unchecked::<Repo>()
                .search(self.clone(), search_string)
                .await?,
        )
    }

    async fn synonyms(&self) -> Vec<Synonym> {
        self.synonyms.into_iter().collect_vec()
    }

    async fn resource_path(&self) -> &str {
        self.resource_path.as_str()
    }

    async fn viewer_can_update(&self, ctx: &Context<'_>) -> Result<bool> {
        let repo = ctx.data_unchecked::<Repo>();
        if repo.viewer.is_guest() {
            log::debug!("viewer is guest");
            return Ok(false);
        }
        if self.repository_is_private {
            log::debug!(
                "viewer: {}, owner: {}",
                repo.viewer.user_id,
                self.repository_owner_id
            );
            return Ok(self.repository_owner_id == repo.viewer.user_id);
        }
        Ok(true)
    }
}
