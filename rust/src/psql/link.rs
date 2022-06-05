use async_graphql::dataloader::*;
use async_graphql::types::ID;
use async_graphql::SimpleObject;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;
use std::sync::Arc;

use super::queries::{LINK_FIELDS, LINK_JOINS};

use crate::schema::{Link, SearchResultItem};

#[derive(sqlx::FromRow, Clone, Debug, SimpleObject)]
pub struct Row {
    id: Uuid,
    parent_topic_ids: Vec<Uuid>,
    repository_id: Uuid,
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
            repository_id: ID(self.repository_id.to_string()),
            url: self.url.to_owned(),
        }
    }

    pub fn to_search_result_item(&self) -> SearchResultItem {
        SearchResultItem::Link(self.to_link())
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
        log::debug!("batch links: {:?}", ids);

        let query = format!(
            r#"select
                {LINK_FIELDS}
                {LINK_JOINS}
            where l.id = any($1::uuid[])
            group by l.id"#,
        );

        let rows = sqlx::query_as::<_, Row>(&query)
            .bind(ids)
            .fetch_all(&self.0)
            .await;

        Ok(rows?
            .iter()
            .map(|r| (r.id.to_string(), r.to_link()))
            .collect::<HashMap<_, _>>())
    }
}
