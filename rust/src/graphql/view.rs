use async_graphql::{Context, Object, SimpleObject, ID};

use super::{
    relay::conn, ActivityLineItem, ActivityLineItemConnection, Link, LiveSearchTopicsPayload,
    Organization, QueryInfo, Repository, SynonymMatch, Topic, User, WIKI_REPOSITORY_ID,
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
    pub current_organization_login: String,
    pub current_repository_name: Option<String>,
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
        topic_path: Option<String>,
    ) -> Result<ActivityLineItemConnection> {
        let repo = ctx.data_unchecked::<Store>();
        let activity = repo.activity(topic_path, first.unwrap_or(3)).await?;

        let mut results = vec![];
        for change in activity {
            let actor = repo.user_loader.load_one(change.actor_id()).await?;
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

    async fn current_organization(&self, ctx: &Context<'_>) -> Result<Organization> {
        Ok(ctx
            .data_unchecked::<Store>()
            .organization_by_login(self.current_organization_login.to_string())
            .await?
            .unwrap_or_default())
    }

    async fn current_repository(&self, ctx: &Context<'_>) -> Result<Repository> {
        match &self.current_repository_name {
            Some(name) => ctx
                .data_unchecked::<Store>()
                .repository_by_name(name.to_string())
                .await?
                .ok_or_else(|| Error::NotFound(format!("repo name {}", name))),

            None => ctx
                .data_unchecked::<Store>()
                .repository(WIKI_REPOSITORY_ID.to_string())
                .await?
                .ok_or_else(|| Error::NotFound(format!("repo id {}", WIKI_REPOSITORY_ID))),
        }
    }

    async fn link(&self, ctx: &Context<'_>, path: String) -> Result<Option<Link>> {
        let path = RepoId::try_from(&path.to_string())?;
        ctx.data_unchecked::<Store>().link(&path).await
    }

    async fn query_info(&self) -> QueryInfo {
        QueryInfo {
            string_tokens: vec![],
        }
    }

    async fn topic(&self, ctx: &Context<'_>, path: String) -> Result<Option<Topic>> {
        ctx.data_unchecked::<Store>()
            .topic(&RepoId::try_from(&path)?)
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
