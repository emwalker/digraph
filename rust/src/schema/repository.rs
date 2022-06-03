use super::organization::Organization;
use super::topic::Topic;
use crate::psql::Repo;
use async_graphql::*;

#[derive(Clone)]
pub struct Repository {
    pub id: ID,
    pub name: String,
    pub organization_id: String,
    pub root_topic_id: String,
}

#[Object]
impl Repository {
    async fn id(&self) -> ID {
        self.id.to_owned()
    }

    async fn name(&self) -> &str {
        self.name.as_str()
    }

    async fn organization(&self, ctx: &Context<'_>) -> Result<Option<Organization>> {
        ctx.data_unchecked::<Repo>()
            .organization(self.organization_id.clone())
            .await
    }

    async fn root_topic(&self, ctx: &Context<'_>) -> Result<Option<Topic>> {
        ctx.data_unchecked::<Repo>()
            .topic(self.root_topic_id.clone())
            .await
    }
}
