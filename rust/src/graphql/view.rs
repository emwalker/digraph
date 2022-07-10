use super::{
    relay::conn, ActivityLineItem, ActivityLineItemConnection, Link, LiveSearchTopicsPayload,
    Organization, QueryInfo, Repository, SynonymMatch, Topic, User, WIKI_REPOSITORY_ID,
};
use crate::repo::Repo;
use crate::{git, prelude::*};

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
    ) -> Result<ActivityLineItemConnection> {
        let repo = ctx.data_unchecked::<Repo>();
        let activity = repo.activity(None, first.unwrap_or(3)).await?;

        let mut results = vec![];
        for change in activity {
            let actor = repo.user_loader.load_one(change.user_id()).await?;
            let actor_name = actor
                .map(|user| user.name.to_owned())
                .unwrap_or_else(|| "[missing user]".to_owned());

            let markdown = change.markdown(Locale::EN, &actor_name, None)?;
            results.push(ActivityLineItem {
                created_at: change.date(),
                description: markdown,
            });
        }

        conn(after, before, first, last, results)
    }

    async fn current_organization(&self, ctx: &Context<'_>) -> Result<Organization> {
        Ok(ctx
            .data_unchecked::<Repo>()
            .organization_by_login(self.current_organization_login.to_string())
            .await?
            .unwrap_or_default())
    }

    async fn current_repository(&self, ctx: &Context<'_>) -> Result<Repository> {
        match &self.current_repository_name {
            Some(name) => ctx
                .data_unchecked::<Repo>()
                .repository_by_name(name.to_string())
                .await?
                .ok_or_else(|| Error::NotFound(format!("repo name {}", name))),

            None => ctx
                .data_unchecked::<Repo>()
                .repository(WIKI_REPOSITORY_ID.to_string())
                .await?
                .ok_or_else(|| Error::NotFound(format!("repo id {}", WIKI_REPOSITORY_ID))),
        }
    }

    async fn link(&self, ctx: &Context<'_>, path: String) -> Result<Option<Link>> {
        let path = RepoPath::from(&path.to_string());
        ctx.data_unchecked::<Repo>().link(&path).await
    }

    async fn link_count(&self, ctx: &Context<'_>) -> Result<i64> {
        ctx.data_unchecked::<Repo>().link_count().await
    }

    async fn query_info(&self) -> QueryInfo {
        QueryInfo {
            string_tokens: vec![],
        }
    }

    async fn topic(&self, ctx: &Context<'_>, path: String) -> Result<Option<Topic>> {
        ctx.data_unchecked::<Repo>()
            .topic(&RepoPath::from(&path))
            .await
    }

    async fn topic_count(&self, ctx: &Context<'_>) -> Result<i64> {
        ctx.data_unchecked::<Repo>().topic_count().await
    }

    async fn topic_live_search(
        &self,
        ctx: &Context<'_>,
        search_string: Option<String>,
    ) -> Result<LiveSearchTopicsPayload> {
        let git::FetchTopicLiveSearchResult { synonym_matches } = ctx
            .data_unchecked::<Repo>()
            .search_topics(search_string)
            .await?;
        let synonym_matches = synonym_matches.iter().map(SynonymMatch::from).collect();
        Ok(LiveSearchTopicsPayload { synonym_matches })
    }

    async fn viewer(&self, ctx: &Context<'_>) -> Result<User> {
        let user = match self.viewer_id.to_string().as_str() {
            "" => User::Guest,
            id => ctx
                .data_unchecked::<Repo>()
                .user(id.to_string())
                .await?
                .unwrap_or_default(),
        };

        Ok(user)
    }
}
