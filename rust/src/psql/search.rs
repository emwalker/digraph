use sqlx::postgres::PgPool;
// use sqlx::types::Uuid;

use super::topic::Row;
use crate::prelude::*;
use crate::schema::{Topic, View};

#[allow(unused_variables, dead_code)]
pub struct LiveSearchTopics {
    view: View,
    search_string: Option<String>,
}

// struct TopicSpec(String);

// struct QuerySpec {
//     input: String,
//     tokens: Vec<String>,
//     string_tokens: Vec<String>,
//     topics: Vec<TopicSpec>,
// }

impl LiveSearchTopics {
    pub fn new(view: View, search_string: Option<String>) -> Self {
        Self {
            view,
            search_string,
        }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<Vec<Topic>> {
        if self.search_string.is_none() {
            return Ok(Vec::default());
        }

        let tokens = &self.wildcard_tokens()[..];

        let query = r#"select
            t.id as "id",
            t.name as "name",
            concat('/', o.login, '/topics/', t.id) as "resource_path",
            t.synonyms as "synonyms",
            t.repository_id as "repository_id",
            array_remove(array_agg(distinct child_links.child_id), null)
                as "child_link_ids",
            array_remove(array_agg(distinct child_topics.child_id), null)
                as "child_topic_ids",
            array_remove(array_agg(distinct parent_topics.parent_id), null)
                as "parent_topic_ids",
            array_remove(array_agg(distinct tr.starts_at), null)
                as "starts_at",
            array_remove(array_agg(distinct tr.prefix_format), null)
                as "prefix_format"

        from topics t
        join organizations o on o.id = t.organization_id
        left join timeranges tr on tr.id = t.timerange_id
        left join link_topics child_links on t.id = child_links.parent_id
        left join topic_topics child_topics on t.id = child_topics.parent_id
        left join topic_topics parent_topics on t.id = parent_topics.child_id

        where t.name ~~* all($1)
        group by t.id, o.login
        limit 10"#;

        let rows = sqlx::query_as::<_, Row>(query)
            .bind(tokens)
            .fetch_all(pool)
            .await?;

        Ok(rows.iter().map(Row::to_topic).collect())
    }

    fn wildcard_tokens(&self) -> Vec<String> {
        match &self.search_string {
            Some(string) => string.split(' ').map(|s| format!("%{}%", s)).collect(),
            None => Vec::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_view() -> View {
        View {
            current_organization_login: "wiki".into(),
            current_repository_name: None,
            repository_ids: None,
            search_string: None,
            viewer_id: ID("1".into()),
        }
    }

    #[test]
    fn test_wildcard_tokens_empty_string() {
        let s = LiveSearchTopics::new(valid_view(), None);
        let expected: Vec<String> = vec![];
        assert_eq!(s.wildcard_tokens(), expected);
    }

    #[test]
    fn test_wildcard_tokens_simple_string() {
        let s = LiveSearchTopics::new(valid_view(), Some("one two".into()));
        let expected: Vec<String> = vec!["%one%".into(), "%two%".into()];
        assert_eq!(s.wildcard_tokens(), expected);
    }
}
