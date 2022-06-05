use super::TopicConnection;
use super::{
    relay::conn, ActivityLineItemConnection, Organization, QueryInfo, Repository, Topic, User,
};
use crate::prelude::*;
use crate::psql::Repo;

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
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<ActivityLineItemConnection> {
        conn(after, before, first, last, vec![])
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
            None => {
                log::info!("no repository specified, fetching default repo for org");
                self.current_organization(ctx)
                    .await?
                    .default_repository(ctx)
                    .await
            }
        }
    }

    async fn link_count(&self) -> i32 {
        45000
    }

    async fn query_info(&self) -> QueryInfo {
        QueryInfo {
            string_tokens: vec![],
        }
    }

    async fn topic(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Topic Id")] id: ID,
    ) -> Result<Option<Topic>> {
        ctx.data_unchecked::<Repo>().topic(id.to_string()).await
    }

    async fn topic_count(&self) -> i32 {
        12000
    }

    async fn topics(
        &self,
        ctx: &Context<'_>,
        search_string: Option<String>,
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
                .search_topics(self.clone(), search_string)
                .await?,
        )
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
