use async_graphql::connection::*;
use async_graphql::{Context, Object, SimpleObject, Union};
use itertools::Itertools;

use super::timerange;
use super::{relay::conn, ActivityLineItem, Link, LinkConnection, Synonym, Synonyms};
use super::{ActivityLineItemConnection, LinkConnectionFields};
use crate::store::Store;
use crate::{git, prelude::*};

#[derive(Debug, SimpleObject)]
pub struct SynonymMatch {
    pub display_name: String,
    pub id: String,
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

#[derive(Clone, Debug)]
pub struct TopicDetail {
    pub child_ids: Vec<Oid>,
    pub color: String,
    pub parent_topic_ids: Vec<Oid>,
    pub name: String,
    pub repo_id: RepoId,
    pub synonyms: Synonyms,
    pub timerange: Option<timerange::Timerange>,
    pub topic_id: Oid,
}

#[Object]
impl TopicDetail {
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
        Ok(ctx
            .data_unchecked::<Store>()
            .viewer
            .write_repo_ids
            .include(&self.repo_id))
    }
}

#[derive(Debug)]
pub struct Topic {
    pub details: Vec<TopicDetail>,
    pub display_detail: TopicDetail,
    pub newly_added: bool,
    pub id: Oid,
    pub root: bool,
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
        let topic_id = Some(self.id.to_owned());
        let store = ctx.data_unchecked::<Store>();

        let activity = store
            .activity(&RepoId::wiki(), &topic_id.to_owned(), first.unwrap_or(3))
            .await?;

        let mut results = vec![];
        for change in activity {
            let actor = store.user_loader.load_one(change.actor_id()).await?;
            let actor_name = actor
                .map(|user| user.name)
                .unwrap_or_else(|| "[missing user]".to_owned());

            results.push(ActivityLineItem {
                created_at: change.date(),
                description: change.markdown(Locale::EN, &actor_name, topic_id.as_ref()),
            });
        }

        conn(after, before, first, last, results)
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
            .topic_children(&self.id)
            .await?;

        let mut results = vec![];
        for child in iter {
            results.push(TopicChild::try_from(&child)?);
        }

        conn(after, before, first, last, results)
    }

    #[allow(unused_variables, clippy::too_many_arguments)]
    async fn child_links(
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
                    .child_links_for_topic(&self.id, reviewed)
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

    async fn description(&self) -> Option<String> {
        None
    }

    async fn display_color(&self) -> &str {
        &self.display_detail.color
    }

    async fn display_name(&self) -> &str {
        &self.display_detail.name
    }

    async fn display_parent_topics(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<TopicConnection> {
        self.display_detail
            .parent_topics(ctx, after, before, first, last)
            .await
    }

    async fn display_synonyms(&self, ctx: &Context<'_>) -> Result<Vec<Synonym>> {
        Ok(self.display_detail.synonyms(ctx).await?)
    }

    async fn loading(&self) -> bool {
        false
    }

    async fn id(&self) -> &str {
        self.id.as_str()
    }

    async fn name(&self) -> &str {
        &self.display_detail.name
    }

    async fn newly_added(&self) -> bool {
        self.newly_added
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
                .search(self, search_string)
                .await?,
        )
    }

    async fn viewer_can_update(&self, ctx: &Context<'_>) -> Result<bool> {
        for details in &self.details {
            if details.viewer_can_update(ctx).await? {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
