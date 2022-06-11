use async_graphql::dataloader::*;
use async_graphql::types::ID;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;

use super::queries::{TopicQuery, TOPIC_FIELDS, TOPIC_JOINS};
use crate::prelude::*;
use crate::schema::{timerange, SearchResultItem, Synonyms, Topic, Viewer};

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Row {
    pub id: Uuid,
    pub name: String,
    pub parent_topic_ids: Vec<Uuid>,
    pub prefix_format: Vec<String>,
    pub repository_id: Uuid,
    pub resource_path: String,
    pub starts_at: Vec<DateTime<Utc>>,
    pub synonyms: serde_json::Value,
}

impl Row {
    pub fn to_topic(&self) -> Topic {
        let parent_topic_ids = self.parent_topic_ids.iter().map(Uuid::to_string).collect();
        let synonyms = Synonyms::from_json(&self.synonyms);

        let starts_at = self.starts_at.first().map(DateTime::to_owned);
        let prefix_format = self.prefix_format.first().map(String::as_ref);
        let prefix = timerange::Prefix::new(prefix_format, starts_at);

        Topic {
            id: ID(self.id.to_string()),
            name: self.name.to_owned(),
            parent_topic_ids,
            prefix,
            repository_id: self.repository_id.to_string(),
            resource_path: self.resource_path.to_owned(),
            synonyms,
        }
    }

    pub fn to_search_result_item(&self) -> SearchResultItem {
        SearchResultItem::Topic(self.to_topic())
    }
}

pub struct TopicLoader {
    viewer: Viewer,
    pool: PgPool,
}

impl TopicLoader {
    pub fn new(viewer: Viewer, pool: PgPool) -> Self {
        Self { viewer, pool }
    }
}

#[async_trait::async_trait]
impl Loader<String> for TopicLoader {
    type Value = Topic;
    type Error = Error;

    async fn load(&self, ids: &[String]) -> Result<HashMap<String, Self::Value>> {
        log::debug!("load topics by batch {:?}", ids);
        let topics = TopicQuery::from(self.viewer.query_ids.clone(), ids.to_vec())
            .execute(&self.pool)
            .await?;
        Ok(topics
            .iter()
            .map(|t| (t.id.to_string(), t.clone()))
            .collect::<HashMap<_, _>>())
    }
}

pub struct FetchTopics {
    viewer_ids: Vec<String>,
    parent_topic_id: String,
    limit: i32,
}

impl FetchTopics {
    pub fn new(viewer_ids: Vec<String>, parent_topic_id: String) -> Self {
        Self {
            viewer_ids,
            parent_topic_id,
            limit: 100,
        }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<Vec<Topic>> {
        log::debug!("fetching linkes for topic: {}", self.parent_topic_id);

        let query = format!(
            r#"
            select
                {TOPIC_FIELDS}
                {TOPIC_JOINS}
                where parent_topics.parent_id = $1::uuid
                    and om.user_id = any($2::uuid[])
                group by t.id, t.name, o.login
                order by t.name asc
                limit $3
            "#
        );

        let rows = sqlx::query_as::<_, Row>(&query)
            .bind(&self.parent_topic_id)
            .bind(&self.viewer_ids)
            .bind(self.limit)
            .fetch_all(pool)
            .await?;
        Ok(rows.iter().map(Row::to_topic).collect())
    }
}
