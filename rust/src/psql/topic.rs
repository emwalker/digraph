use async_graphql::dataloader::*;
use async_graphql::types::ID;
use async_graphql::SimpleObject;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;
use std::sync::Arc;

use super::shared::uuids;
use crate::schema::{Synonyms, Topic};

#[derive(sqlx::FromRow, Clone, Debug, SimpleObject)]
pub struct Row {
    child_link_ids: Vec<Uuid>,
    child_topic_ids: Vec<Uuid>,
    id: Uuid,
    name: String,
    parent_topic_ids: Vec<Uuid>,
    resource_path: String,
    synonyms: serde_json::Value,
}

impl Row {
    fn to_topic(&self) -> Topic {
        let child_link_ids = self.child_link_ids.iter().map(Uuid::to_string).collect();
        let child_topic_ids = self.child_topic_ids.iter().map(Uuid::to_string).collect();
        let parent_topic_ids = self.parent_topic_ids.iter().map(Uuid::to_string).collect();
        let synonyms = Synonyms::from_json(&self.synonyms);

        Topic {
            child_link_ids,
            child_topic_ids,
            id: ID(self.id.to_string()),
            name: self.name.to_owned(),
            parent_topic_ids,
            resource_path: self.resource_path.to_owned(),
            synonyms,
        }
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
    type Error = Arc<sqlx::Error>;

    async fn load(&self, ids: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        log::debug!("load topics by batch {:?}", ids);

        let uuids = uuids(ids);
        let rows = sqlx::query_as!(
            Row,
            r#"select
                t.id as "id!: Uuid",
                t.name as "name!: String",
                concat('/', o.login, '/topics/', t.id) as "resource_path!: String",
                t.synonyms as "synonyms!",
                array_remove(array_agg(distinct child_links.child_id), null)
                    as "child_link_ids!",
                array_remove(array_agg(distinct child_topics.child_id), null)
                    as "child_topic_ids!",
                array_remove(array_agg(distinct parent_topics.parent_id), null)
                    as "parent_topic_ids!"

            from topics t
            join organizations o on o.id = t.organization_id
            left join link_topics child_links on t.id = child_links.parent_id
            left join topic_topics child_topics on t.id = child_topics.parent_id
            left join topic_topics parent_topics on t.id = parent_topics.child_id

            where t.id = any($1)
            group by t.id, o.login"#,
            &uuids,
        )
        .fetch_all(&self.0)
        .await;

        Ok(rows?
            .iter()
            .map(|r| (r.id.to_string(), r.to_topic()))
            .collect::<HashMap<_, _>>())
    }
}
