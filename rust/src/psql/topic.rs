use async_graphql::dataloader::*;
use serde_json::json;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;

use super::queries::{TopicQuery, TOPIC_FIELDS, TOPIC_JOINS};
use crate::http::repo_url::Url;
use crate::prelude::*;
use crate::schema::{
    Alert, AlertType, DateTime, Prefix, SearchResultItem, Synonyms, TimeRange, Topic,
    UpsertTopicInput, Viewer,
};

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Row {
    pub id: Uuid,
    pub name: String,
    pub parent_topic_ids: Vec<Uuid>,
    pub prefix_format: Vec<String>,
    pub repository_id: Uuid,
    pub repository_is_private: bool,
    pub repository_owner_id: Uuid,
    pub resource_path: String,
    pub root: bool,
    pub starts_at: Vec<chrono::DateTime<chrono::Utc>>,
    pub synonyms: serde_json::Value,
}

impl Row {
    pub fn to_topic(&self) -> Topic {
        let parent_topic_ids = self.parent_topic_ids.iter().map(Uuid::to_string).collect();
        let synonyms = Synonyms::from_json(&self.synonyms);

        let starts_at = self.starts_at.first().map(chrono::DateTime::to_owned);
        let prefix_format = self.prefix_format.first().map(String::as_ref);
        let prefix = Prefix::new(prefix_format, starts_at);

        let time_range = starts_at.map(|starts_at| TimeRange {
            starts_at: DateTime(starts_at),
            ends_at: None,
            prefix_format: prefix.to_format(),
        });

        Topic {
            id: self.id.to_string(),
            name: self.name.to_owned(),
            parent_topic_ids,
            prefix,
            repository_id: self.repository_id.to_string(),
            repository_is_private: self.repository_is_private,
            repository_owner_id: self.repository_owner_id.to_string(),
            resource_path: self.resource_path.to_owned(),
            root: self.root,
            synonyms,
            time_range,
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
        let topics = TopicQuery::from(self.viewer.clone(), ids.to_vec())
            .execute(&self.pool)
            .await?;
        Ok(topics
            .iter()
            .map(|t| (t.id.to_string(), t.clone()))
            .collect::<HashMap<_, _>>())
    }
}

pub struct FetchChildTopicsForTopic {
    viewer_ids: Vec<String>,
    parent_topic_id: String,
    limit: i32,
}

impl FetchChildTopicsForTopic {
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
                group by t.id, o.login, r.system, r.name, r.owner_id
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

pub struct UpsertTopic {
    input: UpsertTopicInput,
}

pub struct UpsertTopicResult {
    pub alerts: Vec<Alert>,
    pub topic: Option<Topic>,
}

impl UpsertTopic {
    pub fn new(input: UpsertTopicInput) -> Self {
        Self { input }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<UpsertTopicResult> {
        let name = self.input.name.to_owned();

        if name.is_empty() || Url::is_valid_url(&name) {
            let result = UpsertTopicResult {
                alerts: vec![Alert {
                    text: format!("Not a valid topic name: {}", name),
                    alert_type: AlertType::Warning,
                    id: String::from("0"),
                }],
                topic: None,
            };
            return Ok(result);
        }

        let synonyms = json!([
            { "Locale": "en", "Name": name },
        ])
        .to_string();

        let mut tx = pool.begin().await?;

        let query = r#"
            insert
            into topics
                (organization_id, repository_id, name, synonyms)
                select
                    o.id, r.id, $3, $4::jsonb
                from organizations o
                join repositories r on o.id = r.organization_id
                where o.login = $1 and r.name = $2

            on conflict on constraint topics_repository_name_idx do
                -- No-op to ensure that an id is returned
                update set name = $3
            returning id
            "#;

        let row = sqlx::query_as::<_, (Uuid,)>(query)
            .bind(&self.input.organization_login)
            .bind(&self.input.repository_name)
            .bind(&name)
            .bind(&synonyms)
            .fetch_one(&mut tx)
            .await?;

        for topic_id in &self.input.topic_ids {
            sqlx::query("select add_topic_to_topic($1::uuid, $2::uuid)")
                .bind(topic_id.as_str())
                .bind(&row.0)
                .fetch_one(&mut tx)
                .await?;
        }

        tx.commit().await?;

        let query = format!(
            r#"select
            {TOPIC_FIELDS}
            {TOPIC_JOINS}
            where t.id = $1::uuid
            group by t.id, o.login"#,
        );

        let row = sqlx::query_as::<_, Row>(&query)
            .bind(row.0)
            .fetch_one(pool)
            .await?;

        Ok(UpsertTopicResult {
            alerts: vec![],
            topic: Some(row.to_topic()),
        })
    }
}
