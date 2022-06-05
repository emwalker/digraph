use super::{
    queries::{LiveTopicQuery, SearchQuery},
    QuerySpec,
};
use crate::prelude::*;
use crate::schema::{SearchResultItem, Topic, View};
use sqlx::postgres::PgPool;

#[allow(unused_variables, dead_code)]
pub struct LiveSearchTopics {
    view: View,
    search_string: Option<String>,
}

impl LiveSearchTopics {
    pub fn new(view: View, search_string: Option<String>) -> Self {
        Self {
            view,
            search_string,
        }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<Vec<Topic>> {
        let spec = QuerySpec::parse(self.search_string.clone().unwrap_or_default().as_str())?;
        log::debug!("running live search: {}", spec);

        if spec.is_empty() {
            return Ok(vec![]);
        }

        LiveTopicQuery::from(spec).execute(pool).await
    }
}

pub struct Search {
    parent_topic: Topic,
    search_string: String,
}

impl Search {
    pub fn new(parent_topic: Topic, search_string: String) -> Self {
        Self {
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

        SearchQuery::from(self.parent_topic.clone(), spec)
            .execute(pool)
            .await
    }
}
