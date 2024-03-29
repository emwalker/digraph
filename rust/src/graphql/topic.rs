use async_graphql::connection::*;
use async_graphql::{Context, Object, Union};
use itertools::Itertools;
use std::collections::BTreeSet;

use super::{relay, time, ActivityLineItem, ActivityLineItemConnection, Link, Repository};
use crate::store::Store;
use crate::types::TimerangePrefix;
use crate::{git, prelude::*};

#[derive(Debug)]
pub struct Synonym<'s>(pub(crate) &'s git::Synonym);

#[Object]
impl<'s> Synonym<'s> {
    async fn name(&self) -> &str {
        &self.0.name
    }

    async fn locale(&self) -> String {
        self.0.locale.to_string()
    }
}

#[derive(Debug)]
pub struct SynonymEntry<'e>(pub(crate) &'e git::SynonymEntry);

#[Object]
impl<'e> SynonymEntry<'e> {
    async fn display_name(&self) -> &str {
        &self.0.name
    }

    async fn id(&self) -> &str {
        self.0.id.as_str()
    }
}

#[derive(Debug)]
pub struct LiveSearchTopicsPayload(pub(crate) git::FetchTopicLiveSearchResult);

#[Object]
impl LiveSearchTopicsPayload {
    async fn synonyms(&self) -> Vec<SynonymEntry<'_>> {
        self.0.synonyms.iter().map(SynonymEntry::from).collect()
    }
}

#[derive(Union)]
pub enum SearchMatch {
    Link(Link),
    Topic(Topic),
}

pub type SearchResultConnection = Connection<String, SearchMatch, EmptyFields, EmptyFields>;

pub struct RepoTopicDetails<'a>(pub(crate) &'a git::RepoTopicDetails);

#[Object]
impl<'a> RepoTopicDetails<'a> {
    async fn synonyms(&self) -> Vec<Synonym> {
        self.0.synonyms.iter().map(Synonym::from).collect_vec()
    }

    async fn timerange(&self) -> Result<Option<time::Timerange>> {
        match &self.0.timerange {
            Some(timerange) => Ok(Some(timerange.try_into()?)),
            None => Ok(None),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RepoTopic(pub(crate) git::RepoTopicWrapper);

#[Object]
impl RepoTopic {
    // FIXME - needs to be scoped somehow, perhaps to the repo id
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

    async fn details(&self) -> Option<RepoTopicDetails> {
        self.0.details().map(RepoTopicDetails)
    }

    async fn display_name(&self) -> String {
        self.0.display_name(Locale::EN) // FIXME
    }

    async fn display_color(&self) -> &str {
        self.0.display_color()
    }

    async fn id(&self) -> String {
        format!("{}:{}", self.0.topic_id(), self.0.repo_id)
    }

    async fn in_wiki_repo(&self) -> bool {
        self.0.repo_id.is_wiki()
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
        relay::topics(after, before, first, last, topics).await
    }

    async fn repo(&self, ctx: &Context<'_>) -> Result<Repository> {
        match ctx
            .data_unchecked::<Store>()
            .repo(self.0.repo_id.to_string())
            .await
        {
            Ok(Some(repo)) => Ok(repo),
            _ => Err(Error::NotFound(format!(
                "repo not found: {}",
                self.0.repo_id
            ))),
        }
    }

    async fn repo_id(&self) -> String {
        self.0.repo_id.to_string()
    }

    async fn timerange_prefix(&self) -> String {
        if let Some(timerange) = self.0.timerange() {
            let prefix: TimerangePrefix = timerange.into();
            return prefix.prefix().unwrap_or_default();
        }

        "".into()
    }

    async fn topic_id(&self) -> &str {
        self.0.topic_id().as_str()
    }

    async fn viewer_can_delete_synonyms(&self, ctx: &Context<'_>) -> Result<bool> {
        if self.0.synonyms().len() < 2 {
            return Ok(false);
        }
        self.viewer_can_update(ctx).await
    }

    async fn viewer_can_update(&self, ctx: &Context<'_>) -> Result<bool> {
        let viewer = &ctx.data_unchecked::<Store>().viewer;

        if viewer.context_repo_id != self.0.repo_id {
            return Ok(false);
        }

        Ok(viewer.write_repo_ids.include(self.0.repo_id))
    }
}

#[derive(Debug)]
pub struct Topic(pub(crate) git::Topic);

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
        let topic_id = Some(self.0.key.0.to_owned());
        let store = ctx.data_unchecked::<Store>();

        let activity = store
            .activity(RepoId::wiki(), &topic_id.to_owned(), first.unwrap_or(3))
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

        relay::connection(after, before, first, last, results).await
    }

    async fn children(
        &self,
        ctx: &Context<'_>,
        search_string: Option<String>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<SearchResultConnection> {
        let search = git::Search::parse(&search_string.unwrap_or_default())?;

        if !search.is_empty() {
            let git::FindMatchesResult { matches, .. } = ctx
                .data_unchecked::<Store>()
                .search(&self.0, &search)
                .await?;

            let mut results = vec![];
            for row in matches {
                results.push(row.try_into()?);
            }

            return relay::connection(after, before, first, last, results).await;
        }

        let objects = ctx
            .data_unchecked::<Store>()
            .fetch_objects_with_context(self.0.child_ids().to_owned(), 50, self.0.context_id())
            .await?
            .into_iter();
        let mut matches = BTreeSet::new();

        for object in objects {
            let row = object.to_search_match(Locale::EN, &search)?;
            matches.insert(row);
        }

        let mut results = vec![];

        for row in matches.into_iter() {
            results.push(row.try_into()?);
        }

        relay::connection(after, before, first, last, results).await
    }

    async fn display_name(&self) -> String {
        self.0.display_name(Locale::EN)
    }

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
            .fetch_topics_with_context(self.0.parent_topic_ids(), 50, self.0.context_id())
            .await?;
        relay::topics(after, before, first, last, topics).await
    }

    async fn display_synonyms<'s>(&'s self) -> Result<Vec<Synonym<'s>>> {
        Ok(self
            .0
            .display_synonyms()
            .iter()
            .map(Synonym::from)
            .collect())
    }

    async fn loading(&self) -> bool {
        false
    }

    async fn id(&self) -> &str {
        self.0.key.0.as_str()
    }

    async fn newly_added(&self) -> bool {
        false
    }

    async fn repo_topic(&self, repo_id: String) -> Result<Option<RepoTopic>> {
        let repo_id: RepoId = repo_id.try_into()?;
        Ok(self
            .0
            .repo_topics
            .iter()
            .find(|repo_topic| repo_topic.repo_id == repo_id)
            .map(|repo_topic| repo_topic.into()))
    }

    async fn repo_topics(&self) -> Vec<RepoTopic> {
        self.0.repo_topics.iter().map(RepoTopic::from).collect_vec()
    }

    async fn show_repo_ownership(&self) -> bool {
        self.0.repo_topics.iter().any(|topic| !topic.in_wiki_repo())
    }

    async fn viewer_can_update(&self, ctx: &Context<'_>) -> bool {
        let viewer = &ctx.data_unchecked::<Store>().viewer;
        self.0.can_update(&viewer.write_repo_ids)
    }
}
