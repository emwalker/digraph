use async_graphql::connection::*;
use async_graphql::{Context, Object, SimpleObject, Union};
use itertools::Itertools;

use super::timerange;
use super::{relay::conn, ActivityLineItem, Link, LinkConnection, Repository, Synonym, Synonyms};
use super::{ActivityLineItemConnection, LinkConnectionFields};
use crate::store::Store;
use crate::{git, prelude::*};

#[derive(Debug, SimpleObject)]
pub struct SynonymMatch {
    pub display_name: String,
    pub path: String,
}

#[derive(Debug, SimpleObject)]
pub struct LiveSearchTopicsPayload {
    pub synonym_matches: Vec<SynonymMatch>,
}

#[derive(Union)]
pub enum TopicChild {
    Link(Link),
    Topic(Topic),
}

pub type TopicChildConnection = Connection<String, TopicChild, EmptyFields, EmptyFields>;
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Topic {
    pub child_paths: Vec<RepoId>,
    pub path: RepoId,
    pub name: String,
    pub parent_topic_paths: Vec<RepoId>,
    pub root: bool,
    pub synonyms: Synonyms,
    pub timerange: Option<timerange::Timerange>,
}

pub type TopicEdge = Edge<String, Topic, EmptyFields>;
pub type TopicConnection = Connection<String, Topic, EmptyFields, EmptyFields>;

#[Object]
impl Topic {
    async fn activity(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<ActivityLineItemConnection> {
        let repo = ctx.data_unchecked::<Store>();
        let activity = repo
            .activity(Some(self.path.inner.clone()), first.unwrap_or(3))
            .await?;

        let mut results = vec![];
        for change in activity {
            let actor = repo.user_loader.load_one(change.actor_id()).await?;
            let actor_name = actor
                .map(|user| user.name)
                .unwrap_or_else(|| "[missing user]".to_owned());

            results.push(ActivityLineItem {
                created_at: change.date(),
                description: change.markdown(Locale::EN, &actor_name, Some(&self.path)),
            });
        }

        conn(after, before, first, last, results)
    }

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

    #[allow(unused_variables)]
    async fn children(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
        search_string: Option<String>,
    ) -> Result<TopicChildConnection> {
        let iter = ctx
            .data_unchecked::<Store>()
            .topic_children(&self.path)
            .await?;

        let mut results = vec![];
        for child in iter {
            results.push(TopicChild::try_from(&child)?);
        }

        conn(after, before, first, last, results)
    }

    async fn description(&self) -> Option<String> {
        None
    }

    async fn display_color(&self) -> &str {
        if self.path.starts_with(WIKI_REPO_PREFIX) {
            ""
        } else {
            DEFAULT_PRIVATE_COLOR
        }
    }

    async fn display_name(&self) -> &str {
        &self.name
    }

    // TODO: rename to childLinks
    #[allow(unused_variables, clippy::too_many_arguments)]
    async fn links(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
        search_string: Option<String>,
        reviewed: Option<bool>,
        descendants: Option<bool>,
    ) -> Result<LinkConnection> {
        query(
            after,
            before,
            first,
            last,
            |_after, _before, _first, _last| async move {
                let results = ctx
                    .data::<Store>()?
                    .child_links_for_topic(&self.path, reviewed)
                    .await?;
                let mut connection = Connection::with_additional_fields(
                    false,
                    false,
                    LinkConnectionFields {
                        total_count: results.len() as i64,
                    },
                );

                connection.edges.extend(
                    results
                        .into_iter()
                        .map(|n| Edge::with_additional_fields(String::from("0"), n, EmptyFields)),
                );

                Ok::<_, Error>(connection)
            },
        )
        .await
        .map_err(Error::Resolver)
    }

    async fn loading(&self) -> bool {
        false
    }

    async fn id(&self) -> String {
        self.path.to_string()
    }

    async fn name(&self) -> &str {
        &self.name
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
            ctx.data_unchecked::<Store>()
                .parent_topics_for_topic(&self.path)
                .await?,
        )
    }

    async fn path(&self) -> String {
        self.path.to_string()
    }

    async fn repository(&self, ctx: &Context<'_>) -> Result<Option<Repository>> {
        ctx.data_unchecked::<Store>()
            .repository_by_prefix(self.path.org_login.clone())
            .await
    }

    async fn resource_path(&self) -> &str {
        self.path.inner.as_str()
    }

    async fn search(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        after: Option<String>,
        last: Option<i32>,
        before: Option<String>,
        search_string: String,
    ) -> Result<TopicChildConnection> {
        conn(
            after,
            before,
            first,
            last,
            ctx.data_unchecked::<Store>()
                .search(self.clone(), search_string)
                .await?,
        )
    }

    async fn synonyms(&self) -> Vec<Synonym> {
        self.synonyms.into_iter().collect_vec()
    }

    async fn timerange(&self) -> Option<timerange::Timerange> {
        self.timerange.clone()
    }

    async fn viewer_can_delete_synonyms(&self, ctx: &Context<'_>) -> Result<bool> {
        if self.synonyms.len() < 2 {
            return Ok(false);
        }
        self.viewer_can_update(ctx).await
    }

    async fn viewer_can_update(&self, ctx: &Context<'_>) -> Result<bool> {
        let repo = ctx.data_unchecked::<Store>();
        if repo.viewer.is_guest() {
            return Ok(false);
        }

        // TODO: Narrow down write permissions to a specific topics and their subtopics
        Ok(repo.viewer.write_repos.include(&self.path))
    }
}
