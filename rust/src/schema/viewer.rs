use super::{Repository, User};
use crate::prelude::*;
use crate::psql::Repo;

pub struct Viewer {
    pub user: User,
    pub selected_repository_id: Option<ID>,
}

#[Object]
impl Viewer {
    async fn avatar_url(&self, ctx: &Context<'_>) -> Result<Option<String>> {
        self.user.avatar_url(ctx).await.map_err(Error::from)
    }

    async fn id(&self, ctx: &Context<'_>) -> Result<ID> {
        self.user.id(ctx).await.map_err(Error::from)
    }

    async fn is_guest(&self, ctx: &Context<'_>) -> Result<bool> {
        self.user.is_guest(ctx).await.map_err(Error::from)
    }

    async fn name(&self, ctx: &Context<'_>) -> Result<String> {
        self.user.name(ctx).await.map_err(Error::from)
    }

    async fn selected_repository(&self, ctx: &Context<'_>) -> Result<Option<Repository>> {
        match &self.selected_repository_id {
            Some(id) => {
                ctx.data_unchecked::<Repo>()
                    .repository(id.to_string())
                    .await
            }
            None => Ok(None),
        }
    }
}
