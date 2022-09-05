use sqlx::postgres::PgPool;

use crate::git;
use crate::prelude::*;

pub struct ReviewLink {
    pub actor: Viewer,
    pub repo_id: RepoId,
    pub link: git::Link,
    pub reviewed: bool,
}

pub struct ReviewLinkResult;

impl ReviewLink {
    pub async fn call(&self, _pool: &PgPool) -> Result<ReviewLinkResult> {
        self.actor.ensure_can_read(&self.repo_id)?;
        Ok(ReviewLinkResult)
    }
}
