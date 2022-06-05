use async_graphql::dataloader::*;
use async_graphql::types::ID;
use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;

use super::queries::TopicQuery;
use crate::prelude::*;
use crate::schema::{timerange, SearchResultItem, Synonyms, Topic};

#[derive(sqlx::FromRow, Clone, Debug, SimpleObject)]
pub struct Row {
    pub child_link_ids: Vec<Uuid>,
    pub child_topic_ids: Vec<Uuid>,
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
        let child_link_ids = self.child_link_ids.iter().map(Uuid::to_string).collect();
        let child_topic_ids = self.child_topic_ids.iter().map(Uuid::to_string).collect();
        let parent_topic_ids = self.parent_topic_ids.iter().map(Uuid::to_string).collect();
        let synonyms = Synonyms::from_json(&self.synonyms);

        let starts_at = self.starts_at.first().map(DateTime::to_owned);
        let prefix_format = self.prefix_format.first().map(String::as_ref);
        let prefix = timerange::Prefix::new(prefix_format, starts_at);

        Topic {
            child_link_ids,
            child_topic_ids,
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

pub struct TopicLoader(PgPool);

impl TopicLoader {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait::async_trait]
impl Loader<String> for TopicLoader {
    type Value = Topic;
    type Error = Error;

    async fn load(&self, ids: &[String]) -> Result<HashMap<String, Self::Value>> {
        log::debug!("load topics by batch {:?}", ids);
        let topics = TopicQuery::from(ids.to_vec()).execute(&self.0).await?;
        Ok(topics
            .iter()
            .map(|t| (t.id.to_string(), t.clone()))
            .collect::<HashMap<_, _>>())
    }
}
