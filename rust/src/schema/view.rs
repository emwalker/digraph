use async_graphql::*;

use super::topic::Topic;
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
}
