use async_graphql::types::ID;
use async_graphql::Result;
use async_graphql::SimpleObject;
use async_trait::async_trait;
use dataloader::cached::Loader;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;

use super::{loader, loader::ChildLinksValue};
use crate::psql::shared::unload;
use crate::psql::shared::BatchKey;
use crate::schema::Link;
use crate::server::ports::outgoing::link;

#[derive(sqlx::FromRow, Clone, Debug, SimpleObject)]
pub struct ChildLinkRow {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub title: String,
    pub url: String,
}

impl BatchKey for ChildLinkRow {
    fn batch_key(&self) -> String {
        self.parent_id.to_string()
    }
}

pub fn child_row_to_link(row: &ChildLinkRow) -> Link {
    Link {
        id: ID(row.id.to_string()),
        url: row.url.to_owned(),
        title: row.title.to_owned(),
    }
}

pub struct Repo {
    child_links: Loader<String, ChildLinksValue, loader::ChildLinks>,
}

impl Repo {
    pub fn new(pool: PgPool) -> Self {
        Self {
            child_links: Loader::new(loader::ChildLinks::new(pool)),
        }
    }
}

#[async_trait]
impl link::Port for Repo {
    async fn child_links(&self, topic_id: String) -> Result<Option<Vec<Link>>> {
        unload(self.child_links.try_load(topic_id).await?, |rows| {
            rows.iter().map(child_row_to_link).collect::<Vec<Link>>()
        })
    }
}
