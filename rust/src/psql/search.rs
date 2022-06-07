use super::{
    queries::{LiveTopicQuery, SearchQuery},
    QuerySpec,
};
use crate::prelude::*;
use crate::schema::{SearchResultItem, Topic};
use sqlx::postgres::PgPool;

#[allow(unused_variables, dead_code)]
pub struct LiveSearchTopics {
    viewer_ids: Vec<String>,
    search_string: Option<String>,
}

impl LiveSearchTopics {
    pub fn new(viewer_ids: Vec<String>, search_string: Option<String>) -> Self {
        Self {
            viewer_ids,
            search_string,
        }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<Vec<Topic>> {
        let spec = QuerySpec::parse(self.search_string.clone().unwrap_or_default().as_str())?;
        log::debug!("running live search: {}", spec);

        if spec.is_empty() {
            return Ok(vec![]);
        }

        LiveTopicQuery::from(self.viewer_ids.clone(), spec)
            .execute(pool)
            .await
    }
}

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

    pub async fn call(&self, pool: &PgPool) -> Result<Vec<SearchResultItem>> {
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
