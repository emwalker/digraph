use async_trait::async_trait;
use dataloader::BatchFn;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;

use super::repo::{ChildTopicRow, ParentTopicRow, TopicRow};
use crate::psql::shared::{collect, collect_relations, uuids, Value};

pub type TopicValue = Value<TopicRow>;
pub type ChildTopicsValue = Value<Vec<ChildTopicRow>>;
pub type ParentTopicsValue = Value<Vec<ParentTopicRow>>;

pub struct Topics(PgPool);

impl Topics {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl BatchFn<String, TopicValue> for Topics {
    async fn load(&mut self, ids: &[String]) -> HashMap<String, TopicValue> {
        log::debug!("load topics by batch {:?}", ids);

        let uuids = uuids(ids);
        let rows = sqlx::query_as!(
            TopicRow,
            r#"select
                t.id as "id!: Uuid",
                t.name as "name!: String",
                concat('/', o.login, '/topics/', t.id) as "resource_path!: String"
            from topics t
            join organizations o on o.id = t.organization_id
            where t.id = any($1)"#,
            &uuids,
        )
        .fetch_all(&self.0)
        .await;

        collect(ids, rows)
    }
}

pub struct ChildTopics(PgPool);

impl ChildTopics {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl BatchFn<String, ChildTopicsValue> for ChildTopics {
    async fn load(&mut self, ids: &[String]) -> HashMap<String, ChildTopicsValue> {
        log::debug!("load child topics by batch {:?}", ids);

        let uuids = uuids(ids);
        let rows = sqlx::query_as!(
            ChildTopicRow,
            r#"select
                tt.parent_id as "parent_id!: Uuid",
                t.id as "id!: Uuid",
                t.name as "name!: String",
                concat('/', o.login, '/topics/', t.id) as "resource_path!: String"
            from topics t
            join topic_topics tt on t.id = tt.child_id
            join organizations o on o.id = t.organization_id
            where tt.parent_id = any($1)"#,
            &uuids
        )
        .fetch_all(&self.0)
        .await;

        collect_relations(ids, rows)
    }
}

pub struct ParentTopics(PgPool);

impl ParentTopics {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl BatchFn<String, ParentTopicsValue> for ParentTopics {
    async fn load(&mut self, ids: &[String]) -> HashMap<String, ParentTopicsValue> {
        log::debug!("load parent topics by batch {:?}", ids);

        let uuids = uuids(ids);
        let rows = sqlx::query_as!(
            ParentTopicRow,
            r#"select
                tt.child_id as "child_id!: Uuid",
                t.id as "id!: Uuid",
                t.name as "name!: String",
                concat('/', o.login, '/topics/', t.id) as "resource_path!: String"
            from topics t
            join topic_topics tt on t.id = tt.parent_id
            join organizations o on o.id = t.organization_id
            where tt.child_id = any($1)"#,
            &uuids
        )
        .fetch_all(&self.0)
        .await;

        collect_relations(ids, rows)
    }
}
