use sqlx::postgres::PgPool;

use crate::graphql;
use crate::prelude::*;

pub struct ReviewLink {
    pub actor: Viewer,
    pub repo: RepoId,
    pub link: graphql::Link,
    pub reviewed: bool,
}

pub struct ReviewLinkResult;

impl ReviewLink {
    pub async fn call(&self, _pool: &PgPool) -> Result<ReviewLinkResult> {
        self.actor.ensure_can_read(&self.repo)?;
        Ok(ReviewLinkResult)
    }
}
