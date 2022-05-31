use async_graphql::dataloader::*;
use async_graphql::types::ID;
use async_graphql::SimpleObject;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;
use std::sync::Arc;

use super::shared::uuids;
use crate::schema::Link;

#[derive(sqlx::FromRow, Clone, Debug, SimpleObject)]
pub struct Row {
    id: Uuid,
    parent_topic_ids: Vec<Uuid>,
    title: String,
    url: String,
}

impl Row {
    fn to_link(&self) -> Link {
        let parent_topic_ids = self.parent_topic_ids.iter().map(Uuid::to_string).collect();

        Link {
            id: ID(self.id.to_string()),
            parent_topic_ids,
            title: self.title.to_owned(),
            url: self.url.to_owned(),
        }
    }
}

pub struct LinkLoader(PgPool);

impl LinkLoader {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait::async_trait]
impl Loader<String> for LinkLoader {
    type Value = Link;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, ids: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        log::debug!("load links by batch {:?}", ids);

        let uuids = uuids(ids);
        let rows = sqlx::query_as!(
            Row,
            r#"select
                l.id as "id!: Uuid",
                l.title as "title!: String",
                l.url as "url!: String",
                array_remove(array_agg(distinct parent_topics.parent_id), null)
                    as "parent_topic_ids!"

            from links l
            left join link_topics parent_topics on l.id = parent_topics.child_id
            where l.id = any($1)
            group by l.id"#,
            &uuids,
        )
        .fetch_all(&self.0)
        .await;

        Ok(rows?
            .iter()
            .map(|r| (r.id.to_string(), r.to_link()))
            .collect::<HashMap<_, _>>())
    }
}
