use async_graphql::dataloader::*;
use async_graphql::ID;
use serde_json::json;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::{HashMap, HashSet};

use super::queries::{TopicQuery, TOPIC_FIELDS, TOPIC_GROUP_BY, TOPIC_JOINS};
use crate::http::repo_url::Url;
use crate::prelude::*;
use crate::schema::{
    alert, Alert, AlertType, DateTime, Prefix, SearchResultItem, Synonyms, TimeRange,
    TimeRangePrefixFormat, Topic, UpsertTopicInput, UpsertTopicTimeRangeInput, Viewer,
};

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Row {
    pub id: Uuid,
    pub name: String,
    pub parent_topic_ids: Vec<Uuid>,
    pub repository_id: Uuid,
    pub repository_is_private: bool,
    pub repository_owner_id: Uuid,
    pub resource_path: String,
    pub root: bool,
    pub synonyms: serde_json::Value,
    pub timerange_id: Option<Uuid>,
    pub timerange_prefix_format: Option<String>,
    pub timerange_starts_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Row {
    pub fn to_topic(&self) -> Topic {
        let parent_topic_ids = self.parent_topic_ids.iter().map(Uuid::to_string).collect();
        let synonyms = Synonyms::from_json(&self.synonyms);
        let time_range = self.time_range();
        let prefix = Prefix::from(&time_range);

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

    pub fn time_range(&self) -> Option<TimeRange> {
        match (
            self.timerange_id,
            self.timerange_starts_at,
            self.timerange_prefix_format.clone(),
        ) {
            (Some(id), Some(starts_at), Some(prefix_format)) => {
                let prefix_format = match prefix_format.as_str() {
                    "NONE" => TimeRangePrefixFormat::None,
                    "START_YEAR" => TimeRangePrefixFormat::StartYear,
                    "START_YEAR_MONTH" => TimeRangePrefixFormat::StartYearMonth,
                    _ => TimeRangePrefixFormat::None,
                };

                Some(TimeRange {
                    id: ID(id.to_string()),
                    starts_at: DateTime(starts_at),
                    ends_at: None,
                    prefix_format,
                })
            }
            _ => None,
        }
    }
}

pub async fn fetch_topic(pool: &PgPool, topic_id: &String) -> Result<Row> {
    let query = format!(
        r#"select
        {TOPIC_FIELDS}
        {TOPIC_JOINS}
        where t.id = $1::uuid
        {TOPIC_GROUP_BY}"#,
    );

    Ok(sqlx::query_as::<_, Row>(&query)
        .bind(&topic_id)
        .fetch_one(pool)
        .await?)
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
                {TOPIC_GROUP_BY}
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

pub struct DeleteTopic {
    pub topic_id: String,
}

pub struct DeleteTopicResult {
    pub alerts: Vec<Alert>,
    pub deleted_topic_id: Option<String>,
}

impl DeleteTopic {
    pub fn new(topic_id: String) -> Self {
        Self { topic_id }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<DeleteTopicResult> {
        log::info!("attempting to delete topic: {}", self.topic_id);
        let topic_id = self.topic_id.clone();
        let topic = fetch_topic(pool, &topic_id).await?;
        let mut alerts: Vec<Alert> = vec![];

        if topic.root {
            log::warn!(
                "attempting to delete root topic, bailing: {}",
                self.topic_id
            );
            alerts.push(alert::warning("Cannot delete root topic".into()));
            return Ok(DeleteTopicResult {
                alerts,
                deleted_topic_id: None,
            });
        }

        let parent_topic_ids: Vec<String> = sqlx::query_as::<_, (Uuid,)>(
            r#"select parent_id from topic_topics where child_id = $1::uuid"#,
        )
        .bind(&topic_id)
        .fetch_all(pool)
        .await?
        .iter()
        .map(|t| t.0.to_string())
        .collect();

        let child_topic_ids: Vec<String> = sqlx::query_as::<_, (Uuid,)>(
            r#"select child_id from topic_topics where parent_id = $1::uuid"#,
        )
        .bind(&topic_id)
        .fetch_all(pool)
        .await?
        .iter()
        .map(|t| t.0.to_string())
        .collect();

        let child_link_ids: Vec<String> = sqlx::query_as::<_, (Uuid,)>(
            r#"select child_id from link_topics where parent_id = $1::uuid"#,
        )
        .bind(&topic_id)
        .fetch_all(pool)
        .await?
        .iter()
        .map(|t| t.0.to_string())
        .collect();

        let mut tx = pool.begin().await?;

        for parent_topic_id in parent_topic_ids {
            for child_topic_id in &child_topic_ids {
                sqlx::query("select add_topic_to_topic($1::uuid, $2::uuid)")
                    .bind(&parent_topic_id)
                    .bind(&child_topic_id)
                    .execute(&mut tx)
                    .await?;
            }

            for child_link_id in &child_link_ids {
                sqlx::query("select add_topic_to_link($1::uuid, $2::uuid)")
                    .bind(&parent_topic_id)
                    .bind(&child_link_id)
                    .execute(&mut tx)
                    .await?;
            }
        }

        sqlx::query("delete from topics where id = $1::uuid")
            .bind(&topic_id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;

        Ok(DeleteTopicResult {
            alerts,
            deleted_topic_id: Some(topic_id),
        })
    }
}

pub struct DeleteTopicTimeRange {
    pub topic_id: String,
}

pub struct DeleteTopicTimeRangeResult {
    pub topic: Topic,
    pub deleted_time_range_id: Option<String>,
}

impl DeleteTopicTimeRange {
    pub fn new(topic_id: String) -> Self {
        Self { topic_id }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<DeleteTopicTimeRangeResult> {
        log::info!("deleting topic time range: {}", self.topic_id);

        let topic = fetch_topic(pool, &self.topic_id).await?.to_topic();
        let mut deleted_time_range_id = None;

        if let Some(time_range) = topic.time_range {
            deleted_time_range_id = Some(time_range.id.to_string());
            let mut tx = pool.begin().await?;
            let name = topic
                .synonyms
                .0
                .first()
                .map(|s| s.name.clone())
                .unwrap_or_else(|| "Missing name".into());

            sqlx::query(
                r#"
                update topics
                    set timerange_id = null, name = $1
                where id = $2::uuid
                "#,
            )
            .bind(&name)
            .bind(&self.topic_id)
            .execute(&mut tx)
            .await?;

            sqlx::query("delete from timeranges where id = $1::uuid")
                .bind(&time_range.id.to_string())
                .execute(&mut tx)
                .await?;

            tx.commit().await?;
        }

        let topic = fetch_topic(pool, &self.topic_id).await?.to_topic();

        Ok(DeleteTopicTimeRangeResult {
            topic,
            deleted_time_range_id,
        })
    }
}

pub struct UpdateTopicParentTopics {
    pub topic_id: String,
    pub parent_topic_ids: Vec<String>,
}

pub struct UpdateTopicParentTopicsResult {
    pub alerts: Vec<Alert>,
    pub topic: Topic,
}

impl UpdateTopicParentTopics {
    pub fn new(topic_id: String, parent_topic_ids: Vec<String>) -> Self {
        Self {
            topic_id,
            parent_topic_ids,
        }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<UpdateTopicParentTopicsResult> {
        let topic_id = &self.topic_id;

        let topic = fetch_topic(pool, topic_id).await?.to_topic();
        let before = topic
            .parent_topic_ids
            .iter()
            .map(String::to_owned)
            .collect::<HashSet<String>>();

        let (update, mut alerts) = self.valid_parent_topic_ids(&topic, pool).await?;

        if update.is_empty() {
            let alert = alert::warning("at least one valid topic is needed".to_string());
            alerts.push(alert);
            return Ok(UpdateTopicParentTopicsResult { topic, alerts });
        }

        if &update == &before {
            log::info!("no change in parent topics, skipping update of {}", topic.name);
            return Ok(UpdateTopicParentTopicsResult { topic, alerts });
        }

        let mut tx = pool.begin().await?;

        sqlx::query("delete from topic_transitive_closure where child_id = $1::uuid")
            .bind(topic_id)
            .execute(&mut tx)
            .await?;

        sqlx::query("delete from topic_topics where child_id = $1::uuid")
            .bind(topic_id)
            .execute(&mut tx)
            .await?;

        for parent_topic_id in &update {
            sqlx::query("select add_topic_to_topic($1::uuid, $2::uuid)")
                .bind(parent_topic_id.as_str())
                .bind(&topic_id)
                .execute(&mut tx)
                .await?;
        }

        let removed = &before - &update;

        for removed_topic_id in &removed {
            sqlx::query("delete from link_transitive_closure where parent_id = $1::uuid")
                .bind(removed_topic_id)
                .execute(&mut tx)
                .await?;

            sqlx::query("select upsert_link_down_set($1::uuid)")
                .bind(removed_topic_id)
                .execute(&mut tx)
                .await?;
        }

        tx.commit().await?;

        let topic = fetch_topic(pool, topic_id).await?.to_topic();
        Ok(UpdateTopicParentTopicsResult { topic, alerts })
    }

    async fn valid_parent_topic_ids(
        &self,
        topic: &Topic,
        pool: &PgPool,
    ) -> Result<(HashSet<String>, Vec<Alert>)> {
        let mut valid: HashSet<String> = HashSet::new();
        let mut alerts = vec![];
        let desired = self
            .parent_topic_ids
            .iter()
            .map(String::to_owned)
            .collect::<HashSet<String>>();

        for parent_topic_id in &desired {
            if parent_topic_id == &topic.id {
                let alert = alert::warning("cannot add a topic to itself".to_string());
                alerts.push(alert);
                continue;
            }

            // If the topic whose parents are being updated is itself a parent topic of a desired
            // parent topic, we must skip that desired parent topic to avoid a cycle.
            let (count,) = sqlx::query_as::<_, (i64,)>(
                r#"select count(*) match_count
                    from topic_down_set($1::uuid) tds
                    where tds.child_id = $2::uuid"#,
            )
            .bind(&topic.id)
            .bind(&parent_topic_id)
            .fetch_one(pool)
            .await?;

            if count.is_positive() {
                let alert = alert::warning(format!(
                    r#""{}" is a descendant of "{}" and cannot be added as a parent topic"#,
                    parent_topic_id, topic.name,
                ));
                alerts.push(alert);
                continue;
            }

            valid.insert(parent_topic_id.to_string());
        }

        Ok((valid, alerts))
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
        let mut alerts = vec![];
        let name = self.input.name.to_owned();

        if name.is_empty() || Url::is_valid_url(&name) {
            let result = UpsertTopicResult {
                alerts: vec![Alert {
                    text: format!("Not a valid topic name: {}", name),
                    alert_type: AlertType::Warn,
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

        let (topic_id,) = sqlx::query_as::<_, (Uuid,)>(query)
            .bind(&self.input.organization_login)
            .bind(&self.input.repository_name)
            .bind(&name)
            .bind(&synonyms)
            .fetch_one(&mut tx)
            .await?;

        for parent_topic_id in &self.input.topic_ids {
            let parent_topic_id = parent_topic_id.to_string();
            let (count,) = sqlx::query_as::<_, (i64,)>(
                r#"select count(*) match_count
                    from topic_down_set($1) tds
                    where tds.child_id = $2"#,
            )
            .bind(&parent_topic_id)
            .bind(&topic_id)
            .fetch_one(&mut tx)
            .await?;

            if count.is_positive() {
                let alert = alert::warning(format!(
                    r#""{}" is a descendant of "{}" and cannot be added as a parent topic"#,
                    parent_topic_id, name,
                ));
                alerts.push(alert);
                continue;
            }

            sqlx::query("select add_topic_to_topic($1::uuid, $2::uuid)")
                .bind(parent_topic_id.as_str())
                .bind(&topic_id)
                .fetch_one(&mut tx)
                .await?;
        }

        tx.commit().await?;

        let row = fetch_topic(pool, &topic_id.to_string()).await?;

        Ok(UpsertTopicResult {
            alerts,
            topic: Some(row.to_topic()),
        })
    }
}

pub struct UpsertTopicTimeRange {
    input: UpsertTopicTimeRangeInput,
}

pub struct UpsertTopicTimeRangeResult {
    pub alerts: Vec<Alert>,
    pub time_range: TimeRange,
    pub topic: Topic,
}

impl UpsertTopicTimeRange {
    pub fn new(input: UpsertTopicTimeRangeInput) -> Self {
        Self { input }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<UpsertTopicTimeRangeResult> {
        log::info!("upserting time range for topic: {:?}", self.input);

        let topic_id = self.input.topic_id.to_string();
        let topic = fetch_topic(pool, &topic_id).await?.to_topic();
        let prefix_format = format!("{}", self.input.prefix_format);
        let mut tx = pool.begin().await?;

        let time_range_id = if let Some(time_range) = &topic.time_range {
            sqlx::query(
                r#"
                update timeranges set starts_at = $1, prefix_format = $2
                    where id = $3::uuid
                "#,
            )
            .bind(&self.input.starts_at.0)
            .bind(&prefix_format)
            .bind(&time_range.id.to_string())
            .execute(&mut tx)
            .await?;

            time_range.id.to_string()
        } else {
            let row = sqlx::query_as::<_, (Uuid,)>(
                r#"
                insert into timeranges (starts_at, prefix_format)
                    values ($1, $2)
                returning id
                "#,
            )
            .bind(&self.input.starts_at.0)
            .bind(&prefix_format)
            .fetch_one(&mut tx)
            .await?;
            let time_range_id = row.0.to_string();

            sqlx::query("update topics set timerange_id = $1::uuid where id = $2::uuid")
                .bind(&time_range_id)
                .bind(&topic_id)
                .execute(&mut tx)
                .await?;

            time_range_id
        };

        let time_range = TimeRange {
            id: ID(time_range_id),
            starts_at: self.input.starts_at.clone(),
            ends_at: None,
            prefix_format: self.input.prefix_format,
        };

        let prefix = Prefix::from(&Some(time_range.clone()));
        let synonym = topic
            .synonyms
            .first()
            .map(|s| s.name.clone())
            .unwrap_or_else(|| "Missing name".into());
        let name = prefix.display(&synonym);

        sqlx::query("update topics set name = $1 where id = $2::uuid")
            .bind(&name)
            .bind(&topic_id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;

        // Reload to pick up changes
        let topic = fetch_topic(pool, &topic_id).await?.to_topic();

        Ok(UpsertTopicTimeRangeResult {
            alerts: vec![],
            time_range,
            topic,
        })
    }
}
