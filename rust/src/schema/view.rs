use async_graphql::*;

use super::{Topic, User};
use crate::psql::Repo;

pub struct View {
    pub viewer_id: ID,
}

#[Object]
impl View {
    async fn topic(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Topic Id")] id: ID,
    ) -> async_graphql::Result<Option<Topic>> {
        ctx.data_unchecked::<Repo>().topic(id.to_string()).await
    }

    async fn viewer(&self, ctx: &Context<'_>) -> Result<User> {
        Ok(ctx
            .data_unchecked::<Repo>()
            .user(self.viewer_id.to_string())
            .await?
            .unwrap_or(User::Guest))
    }
}
