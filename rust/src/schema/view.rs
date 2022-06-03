use async_graphql::*;

use super::{relay::conn, ActivityLineItemConnection, QueryInfo, Topic, User};
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
    #[allow(unused_variables)]
    async fn activity(
        &self,
        first: Option<i32>,
        after: Option<String>,
        last: Option<i32>,
        before: Option<String>,
    ) -> Result<ActivityLineItemConnection> {
        conn(vec![])
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
