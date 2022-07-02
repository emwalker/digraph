use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashSet;

use super::queries::{TOPIC_FIELDS, TOPIC_GROUP_BY, TOPIC_JOINS};
use crate::graphql::{
    DateTime, Prefix, Synonyms, Timerange, TimerangePrefixFormat, Topic, TopicChild, Viewer,
};
use crate::prelude::*;
use crate::Alert;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Row {
    pub path: String,
    pub name: String,
    pub parent_topic_paths: Vec<String>,
    pub root: bool,
    pub synonyms: serde_json::Value,
    pub timerange_prefix_format: Option<String>,
    pub timerange_starts_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Row {
    pub fn to_topic(&self) -> Topic {
        let synonyms = Synonyms::from_json(&self.synonyms);
        let timerange = self.timerange();
        let prefix = Prefix::from(&timerange);

        Topic {
            child_paths: vec![],
            path: RepoPath::from(&self.path),
            name: self.name.to_owned(),
            parent_topic_paths: self.parent_topic_paths.iter().map(RepoPath::from).collect(),
            prefix,
            root: self.root,
            synonyms,
            timerange,
        }
    }

    pub fn to_search_result_item(&self) -> TopicChild {
        TopicChild::Topic(self.to_topic())
    }

    pub fn timerange(&self) -> Option<Timerange> {
        match (
            self.timerange_starts_at,
            self.timerange_prefix_format.clone(),
        ) {
            (Some(starts_at), Some(prefix_format)) => {
                let prefix_format = match prefix_format.as_str() {
                    "NONE" => TimerangePrefixFormat::None,
                    "START_YEAR" => TimerangePrefixFormat::StartYear,
                    "START_YEAR_MONTH" => TimerangePrefixFormat::StartYearMonth,
                    _ => TimerangePrefixFormat::None,
                };

                Some(Timerange {
                    starts_at: DateTime(starts_at),
                    ends_at: None,
                    prefix_format,
                })
            }
            _ => None,
        }
    }
}

pub async fn fetch_topic(
    query_ids: &Vec<String>,
    pool: &PgPool,
    topic_path: &RepoPath,
) -> Result<Row> {
    let query = format!(
        r#"select
        {TOPIC_FIELDS}
        {TOPIC_JOINS}
        where t.id = $1::uuid
            and om.user_id = any($2::uuid[])
        {TOPIC_GROUP_BY}"#,
    );

    let row = sqlx::query_as::<_, Row>(&query)
        .bind(&topic_path.short_id)
        .bind(query_ids)
        .fetch_one(pool)
        .await?;
    Ok(row)
}

pub struct DeleteTopic {
    actor: Viewer,
    topic: RepoPath,
}

pub struct DeleteTopicResult {
    pub alerts: Vec<Alert>,
    pub deleted_topic_path: Option<String>,
}

impl DeleteTopic {
    pub fn new(actor: Viewer, topic: RepoPath) -> Self {
        Self { actor, topic }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<DeleteTopicResult> {
        log::info!("attempting to delete topic: {}", self.topic);
        let topic = fetch_topic(&self.actor.mutation_ids, pool, &self.topic).await?;
        let mut alerts: Vec<Alert> = vec![];

        if topic.root {
            log::warn!("attempting to delete root topic, bailing: {}", self.topic);
            alerts.push(Alert::Warning("Cannot delete root topic".into()));
            return Ok(DeleteTopicResult {
                alerts,
                deleted_topic_path: None,
            });
        }

        let parent_topic_ids: Vec<String> = sqlx::query_as::<_, (Uuid,)>(
            r#"select parent_id from topic_topics where child_id = $1::uuid"#,
        )
        .bind(&self.topic.short_id)
        .fetch_all(pool)
        .await?
        .iter()
        .map(|t| t.0.to_string())
        .collect();

        let child_topic_ids: Vec<String> = sqlx::query_as::<_, (Uuid,)>(
            r#"select child_id from topic_topics where parent_id = $1::uuid"#,
        )
        .bind(&self.topic.short_id)
        .fetch_all(pool)
        .await?
        .iter()
        .map(|t| t.0.to_string())
        .collect();

        let child_link_ids: Vec<String> = sqlx::query_as::<_, (Uuid,)>(
            r#"select child_id from link_topics where parent_id = $1::uuid"#,
        )
        .bind(&self.topic.short_id)
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
            .bind(&self.topic.short_id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;

        Ok(DeleteTopicResult {
            alerts,
            deleted_topic_path: Some(self.topic.to_string()),
        })
    }
}

pub struct UpdateTopicParentTopics {
    actor: Viewer,
    topic: RepoPath,
    parent_topics: Vec<RepoPath>,
}

pub struct UpdateTopicParentTopicsResult {
    pub alerts: Vec<Alert>,
    pub topic: Topic,
}

impl UpdateTopicParentTopics {
    pub fn new(actor: Viewer, topic: RepoPath, parent_topics: Vec<RepoPath>) -> Self {
        Self {
            actor,
            parent_topics,
            topic,
        }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<UpdateTopicParentTopicsResult> {
        let topic_path = &self.topic;

        let topic = fetch_topic(&self.actor.mutation_ids, pool, topic_path)
            .await?
            .to_topic();
        let before = topic
            .parent_topic_paths
            .iter()
            .map(|p| p.short_id.to_owned())
            .collect::<HashSet<String>>();

        let (update, mut alerts) = self.valid_parent_topic_ids(&topic, pool).await?;

        if update.is_empty() {
            let alert = Alert::Warning("at least one valid topic is needed".to_string());
            alerts.push(alert);
            return Ok(UpdateTopicParentTopicsResult { topic, alerts });
        }

        if update == before {
            log::info!(
                "no change in parent topics, skipping update of {}",
                topic.name
            );
            return Ok(UpdateTopicParentTopicsResult { topic, alerts });
        }

        let mut tx = pool.begin().await?;

        sqlx::query("delete from topic_transitive_closure where child_id = $1::uuid")
            .bind(&topic_path.short_id)
            .execute(&mut tx)
            .await?;

        sqlx::query("delete from topic_topics where child_id = $1::uuid")
            .bind(&topic_path.short_id)
            .execute(&mut tx)
            .await?;

        for parent_topic_id in &update {
            sqlx::query("select add_topic_to_topic($1::uuid, $2::uuid)")
                .bind(parent_topic_id.as_str())
                .bind(&topic_path.short_id)
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

        let topic = fetch_topic(&self.actor.mutation_ids, pool, topic_path)
            .await?
            .to_topic();
        Ok(UpdateTopicParentTopicsResult { topic, alerts })
    }

    async fn valid_parent_topic_ids(
        &self,
        topic: &Topic,
        pool: &PgPool,
    ) -> Result<(HashSet<String>, Vec<Alert>)> {
        let topic_id = &topic.path.short_id;
        let mut valid: HashSet<String> = HashSet::new();
        let mut alerts = vec![];
        let desired = self
            .parent_topics
            .iter()
            .map(RepoPath::to_string)
            .collect::<HashSet<String>>();

        for parent_topic_id in &desired {
            if parent_topic_id == &topic.path.short_id {
                let alert = Alert::Warning("cannot add a topic to itself".to_string());
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
            .bind(&topic_id)
            .bind(&parent_topic_id)
            .fetch_one(pool)
            .await?;

            if count.is_positive() {
                let alert = Alert::Warning(format!(
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
