use std::convert::TryInto;

use async_graphql::{Context, Object, SimpleObject, ID};

use super::{
    relay, ActivityLineItem, ActivityLineItemConnection, Link, LiveSearchTopicsPayload, Topic, User,
};
use crate::git;
use crate::prelude::*;
use crate::store::Store;

#[derive(SimpleObject)]
pub struct ViewStats {
    pub link_count: Option<i32>,
    pub topic_count: Option<i32>,
}

#[derive(Clone)]
pub struct View {
    pub repo_ids: Option<Vec<ID>>,
    pub search_string: Option<String>,
    pub viewer_id: ID,
}

#[derive(SimpleObject)]
struct QueryInfoPayload {
    topics: Vec<Topic>,
    phrases: Vec<String>,
}

#[Object]
impl View {
    async fn activity(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
        topic_id: Option<String>,
    ) -> Result<ActivityLineItemConnection> {
        let store = ctx.data_unchecked::<Store>();
        let repo_id = RepoId::wiki();
        let topic_id: Option<ExternalId> = match topic_id {
            Some(topic_id) => Some(topic_id.try_into()?),
            None => None,
        };

        let activity = store
            .activity(repo_id, &topic_id, first.unwrap_or(3))
            .await?;

        let mut results = vec![];
        for change in activity {
            let actor = store.user_loader.load_one(change.actor_id()).await?;
            let actor_name = actor
                .map(|user| user.name)
                .unwrap_or_else(|| "[missing user]".to_owned());

            let markdown = change.markdown(Locale::EN, &actor_name, None);
            results.push(ActivityLineItem {
                created_at: change.date(),
                description: markdown,
            });
        }

        relay::connection(after, before, first, last, results).await
    }

    async fn link(&self, ctx: &Context<'_>, id: String) -> Result<Option<Link>> {
        Ok(ctx
            .data_unchecked::<Store>()
            .fetch_link(id.try_into()?)
            .await?
            .map(Link::from))
    }

    async fn topic(&self, ctx: &Context<'_>, id: String) -> Result<Option<Topic>> {
        Ok(ctx
            .data_unchecked::<Store>()
            .fetch_topic(id.try_into()?)
            .await?
            .map(Topic::from))
    }

    async fn topic_live_search(
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

    async fn stats(&self, ctx: &Context<'_>) -> Result<ViewStats> {
        ctx.data_unchecked::<Store>().view_stats().await?.try_into()
    }

    async fn query_info(&self, ctx: &Context<'_>) -> Result<QueryInfoPayload> {
        if let Some(search_string) = &self.search_string {
            let git::Search {
                topic_specs,
                tokens: phrases,
                ..
            } = git::Search::parse(search_string)?;

            let topics = ctx
                .data_unchecked::<Store>()
                .fetch_topics(topic_specs.into_iter().map(|spec| spec.id).collect(), 10)
                .await?
                .into_iter()
                .map(|topic| topic.into())
                .collect();

            return Ok(QueryInfoPayload {
                topics,
                phrases: phrases
                    .into_iter()
                    .map(|phrase| phrase.to_string())
                    .collect(),
            });
        }

        Ok(QueryInfoPayload {
            topics: vec![],
            phrases: vec![],
        })
    }

    async fn viewer(&self, ctx: &Context<'_>) -> Result<User> {
        let user = match self.viewer_id.to_string().as_str() {
            "" => User::Guest,
            id => ctx
                .data_unchecked::<Store>()
                .user(id.to_string())
                .await?
                .unwrap_or_default(),
        };

        Ok(user)
    }
}
