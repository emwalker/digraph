use sqlx::postgres::PgPool;

use crate::git;
use crate::prelude::*;

pub struct ReviewLink {
    pub actor: Viewer,
    pub link: git::Link,
    pub reviewed: bool,
}

pub struct ReviewLinkResult {
    pub link: git::Link,
}

impl ReviewLink {
    pub fn new(actor: Viewer, link: git::Link, reviewed: bool) -> Self {
        Self {
            actor,
            link,
            reviewed,
        }
    }

    pub async fn call(&self, _pool: &PgPool) -> Result<ReviewLinkResult> {
        self.actor.ensure_can_read(&self.link.path()?)?;

        Ok(ReviewLinkResult {
            link: self.link.to_owned(),
        })
    }
}
