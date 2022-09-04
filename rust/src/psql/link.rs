use sqlx::postgres::PgPool;

use crate::git;
use crate::prelude::*;

pub struct ReviewLink {
    pub actor: Viewer,
    pub repo: RepoId,
    pub link: git::RepoLink,
    pub reviewed: bool,
}

pub struct ReviewLinkResult {
    pub link: git::RepoLink,
}

impl ReviewLink {
    pub async fn call(&self, _pool: &PgPool) -> Result<ReviewLinkResult> {
        self.actor.ensure_can_read(&self.repo)?;

        Ok(ReviewLinkResult {
            link: self.link.to_owned(),
        })
    }
}
