use async_graphql::types::ID;
use async_graphql::Result;
use async_graphql::SimpleObject;
use async_trait::async_trait;
use dataloader::cached::Loader;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;

use super::{
    loader,
    loader::{ChildTopicsValue, ParentTopicsValue, TopicValue},
};
use crate::psql::shared::unload;
use crate::psql::shared::BatchKey;
use crate::schema::Topic;
use crate::server::ports::outgoing::topic;

#[derive(sqlx::FromRow, Clone, Debug, SimpleObject)]
pub struct TopicRow {
    pub id: Uuid,
    pub name: String,
    pub resource_path: String,
}

impl BatchKey for TopicRow {
    fn batch_key(&self) -> String {
        self.id.to_string()
    }
}

pub fn row_to_topic(row: &TopicRow) -> Topic {
    Topic {
        id: ID(row.id.to_string()),
        name: row.name.to_owned(),
        resource_path: row.resource_path.to_owned(),
    }
}

#[derive(sqlx::FromRow, Clone, Debug, SimpleObject)]
pub struct ChildTopicRow {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Uuid,
    pub resource_path: String,
}

impl BatchKey for ChildTopicRow {
    fn batch_key(&self) -> String {
        self.parent_id.to_string()
    }
}

pub fn child_row_to_topic(row: &ChildTopicRow) -> Topic {
    Topic {
        id: ID(row.id.to_string()),
        name: row.name.to_owned(),
        resource_path: row.resource_path.to_owned(),
    }
}

#[derive(sqlx::FromRow, Clone, Debug, SimpleObject)]
pub struct ParentTopicRow {
    pub child_id: Uuid,
    pub id: Uuid,
    pub name: String,
    pub resource_path: String,
}

impl BatchKey for ParentTopicRow {
    fn batch_key(&self) -> String {
        self.child_id.to_string()
    }
}

pub fn parent_row_to_topic(row: &ParentTopicRow) -> Topic {
    Topic {
        id: ID(row.id.to_string()),
        name: row.name.to_owned(),
        resource_path: row.resource_path.to_owned(),
    }
}

pub struct Repo {
    topics: Loader<String, TopicValue, loader::Topics>,
    child_topics: Loader<String, ChildTopicsValue, loader::ChildTopics>,
    parent_topics: Loader<String, ParentTopicsValue, loader::ParentTopics>,
}

impl Repo {
    pub fn new(pool: PgPool) -> Self {
        Self {
            child_topics: Loader::new(loader::ChildTopics::new(pool.clone())),
            topics: Loader::new(loader::Topics::new(pool.clone())),
            parent_topics: Loader::new(loader::ParentTopics::new(pool)),
        }
    }
}

#[async_trait]
impl topic::Port for Repo {
    async fn get(&self, topic_id: String) -> Result<Option<Topic>> {
        unload(self.topics.try_load(topic_id).await?, |row| {
            row_to_topic(&row)
        })
    }

    async fn child_topics(&self, topic_id: String) -> Result<Option<Vec<Topic>>> {
        unload(self.child_topics.try_load(topic_id).await?, |rows| {
            rows.iter().map(child_row_to_topic).collect::<Vec<Topic>>()
        })
    }

    async fn parent_topics(&self, topic_id: String) -> Result<Option<Vec<Topic>>> {
        unload(self.parent_topics.try_load(topic_id).await?, |rows| {
            rows.iter().map(parent_row_to_topic).collect::<Vec<Topic>>()
        })
    }
}
