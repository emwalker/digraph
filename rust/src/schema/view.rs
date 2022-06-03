use super::{relay::conn, ActivityLineItemConnection, QueryInfo, Repository, Topic, User};
use crate::prelude::*;
use crate::psql::Repo;

#[derive(Clone)]
pub struct View {
    pub current_organization_login: String,
    pub current_repository_name: Option<String>,
    pub repository_ids: Vec<ID>,
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

    async fn current_repository(&self, ctx: &Context<'_>) -> Result<Option<Repository>> {
        match &self.current_repository_name {
            Some(name) => ctx
                .data_unchecked::<Repo>()
                .repository_by_name(name.to_string())
                .await
                .map_err(|_e| Error::NotFound),
            None => Ok(None),
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
    ) -> async_graphql::Result<Option<Topic>> {
        ctx.data_unchecked::<Repo>().topic(id.to_string()).await
    }

    async fn topic_count(&self) -> i32 {
        12000
    }

    async fn viewer(&self, ctx: &Context<'_>) -> Result<User> {
        let user = ctx
            .data_unchecked::<Repo>()
            .user(self.viewer_id.to_string())
            .await?
            .unwrap_or(User::Guest);

        Ok(user)
    }
}
