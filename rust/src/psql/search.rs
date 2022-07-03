use super::{queries::SearchQuery, QuerySpec};
use crate::graphql::{Topic, TopicChild};
use crate::prelude::*;
use sqlx::postgres::PgPool;

pub struct Search {
    viewer_ids: Vec<String>,
    parent_topic: Topic,
    search_string: String,
}

impl Search {
    pub fn new(viewer_ids: Vec<String>, parent_topic: Topic, search_string: String) -> Self {
        Self {
            viewer_ids,
            parent_topic,
            search_string,
        }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<Vec<TopicChild>> {
        let spec = QuerySpec::parse(self.search_string.clone().as_str())?;
        log::debug!("running query: {}", spec);

        if spec.is_empty() {
            return Ok(vec![]);
        }

        SearchQuery::from(self.viewer_ids.clone(), self.parent_topic.clone(), spec)
            .execute(pool)
            .await
    }
}
