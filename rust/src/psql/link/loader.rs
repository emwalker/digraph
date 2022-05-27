use async_trait::async_trait;
use dataloader::BatchFn;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;

use super::repo::ChildLinkRow;
use crate::psql::shared::{collect_relations, uuids, Value};

pub type ChildLinksValue = Value<Vec<ChildLinkRow>>;

pub struct ChildLinks(PgPool);

impl ChildLinks {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl BatchFn<String, ChildLinksValue> for ChildLinks {
    async fn load(&mut self, ids: &[String]) -> HashMap<String, ChildLinksValue> {
        log::debug!("load child topics by batch {:?}", ids);

        let uuids = uuids(ids);
        let rows = sqlx::query_as!(
            ChildLinkRow,
            r#"select
                lt.parent_id as "parent_id!: Uuid",
                l.id as "id!: Uuid",
                l.url as "url!: String",
                l.title as "title!: String"
            from links l
            join link_topics lt on l.id = lt.child_id
            where lt.parent_id = any($1)"#,
            &uuids
        )
        .fetch_all(&self.0)
        .await;

        collect_relations(ids, rows)
    }
}
