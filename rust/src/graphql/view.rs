use async_graphql::{Context, Object, SimpleObject, ID};

use super::{
    relay::conn, ActivityLineItem, ActivityLineItemConnection, Link, LiveSearchTopicsPayload,
    QueryInfo, SynonymMatch, Topic, User,
};
use crate::store::Store;
use crate::{git, prelude::*};

#[derive(SimpleObject)]
pub struct ViewStats {
    pub link_count: Option<i32>,
    pub topic_count: Option<i32>,
}

#[derive(Clone)]
pub struct View {
    pub repository_ids: Option<Vec<ID>>,
    pub search_string: Option<String>,
    pub viewer_id: ID,
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
        let repo = RepoId::wiki();
        let topic_id: Option<Oid> = match topic_id {
            Some(topic_id) => Some(topic_id.try_into()?),
            None => None,
        };

        let activity = store.activity(&repo, &topic_id, first.unwrap_or(3)).await?;

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

        conn(after, before, first, last, results)
    }

    async fn link(&self, ctx: &Context<'_>, id: String) -> Result<Option<Link>> {
        // FIXME
        let repo = RepoId::wiki();
        ctx.data_unchecked::<Store>()
            .link(&repo, &id.try_into()?)
            .await
    }

    async fn query_info(&self) -> QueryInfo {
        QueryInfo {
            string_tokens: vec![],
        }
    }

    async fn topic(&self, ctx: &Context<'_>, id: String) -> Result<Option<Topic>> {
        // FIXME
        let repo = RepoId::wiki();
        ctx.data_unchecked::<Store>()
            .topic(&repo, &id.try_into()?)
            .await
    }

    async fn topic_live_search(
        &self,
        ctx: &Context<'_>,
        search_string: Option<String>,
    ) -> Result<LiveSearchTopicsPayload> {
        let git::FetchTopicLiveSearchResult { synonym_matches } = ctx
            .data_unchecked::<Store>()
            .search_topics(search_string)
            .await?;
        let synonym_matches = synonym_matches.iter().map(SynonymMatch::from).collect();
        Ok(LiveSearchTopicsPayload { synonym_matches })
    }

    async fn stats(&self, ctx: &Context<'_>) -> Result<ViewStats> {
        ctx.data_unchecked::<Store>().view_stats().await
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
