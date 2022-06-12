use sqlx::postgres::PgPool;

use super::{link, topic, QuerySpec, TopicSpec};
use crate::prelude::*;
use crate::schema::{SearchResultItem, Topic, Viewer};

pub const TOPIC_FIELDS: &str = r#"
    t.id,
    t.name,
    concat('/', o.login, '/topics/', t.id) resource_path,
    t.synonyms,
    t.repository_id,
    t.root,
    (r.system and (r.name = 'system:default')) repository_is_private,
    r.owner_id repository_owner_id,
    array_remove(array_agg(distinct parent_topics.parent_id), null) parent_topic_ids,
    array_remove(array_agg(distinct tr.starts_at), null) starts_at,
    array_remove(array_agg(distinct tr.prefix_format), null) prefix_format
"#;

pub const TOPIC_JOINS: &str = r#"
    from topics t
    join organizations o on o.id = t.organization_id
    join organization_members om on om.organization_id = t.organization_id
    join repositories r on r.id = t.repository_id
    left join timeranges tr on tr.id = t.timerange_id
    left join topic_topics parent_topics on t.id = parent_topics.child_id
"#;

pub const LINK_FIELDS: &str = r#"
    l.id,
    l.title,
    l.url,
    l.repository_id,
    array_remove(array_agg(distinct parent_topics.parent_id), null)
        parent_topic_ids
"#;

pub const LINK_JOINS: &str = r#"
    from links l
    join repositories r on r.id = l.repository_id
    join organization_members om on om.organization_id = r.organization_id
    left join link_topics parent_topics on l.id = parent_topics.child_id
"#;

pub struct TopicQuery {
    viewer: Viewer,
    topic_ids: Vec<String>,
    limit: i32,
}

impl TopicQuery {
    pub fn from(viewer: Viewer, topic_ids: Vec<String>) -> Self {
        Self {
            viewer,
            topic_ids,
            limit: 10000,
        }
    }

    pub async fn execute(&self, pool: &PgPool) -> Result<Vec<Topic>> {
        let sql = self.topic_sql();
        let rows = sqlx::query_as::<_, topic::Row>(&sql)
            .bind(&self.topic_ids)
            .bind(&self.viewer.query_ids)
            .bind(self.limit)
            .fetch_all(pool)
            .await?;

        Ok(rows.iter().map(topic::Row::to_topic).collect())
    }

    fn topic_sql(&self) -> String {
        let mut wheres: Vec<String> = Vec::new();
        let mut index = 1;

        wheres.push(format!("t.id = any(${index}::uuid[])"));
        index += 1;

        wheres.push(format!("om.user_id = any(${index}::uuid[])"));
        index += 1;

        let where_clause = wheres.join(" and ");

        format!(
            r#"select
            {TOPIC_FIELDS}
            {TOPIC_JOINS}
            where {where_clause}
            group by t.id, o.login, r.system, r.name, r.owner_id
            limit ${index}"#
        )
    }
}

pub struct LiveTopicQuery {
    viewer_ids: Vec<String>,
    query_spec: QuerySpec,
    limit: i32,
}

impl LiveTopicQuery {
    pub fn from(viewer_ids: Vec<String>, query_spec: QuerySpec) -> Self {
        Self {
            viewer_ids,
            query_spec,
            limit: 15,
        }
    }

    pub async fn execute(&self, pool: &PgPool) -> Result<Vec<Topic>> {
        let tokens = self.query_spec.wildcard_tokens();

        // FIXME: Use a purpose-specific struct for these results, which show the synonyms
        let sql = self.topic_sql();
        let rows = sqlx::query_as::<_, topic::Row>(&sql)
            .bind(tokens)
            .bind(&self.viewer_ids)
            .bind(self.limit)
            .fetch_all(pool)
            .await?;

        Ok(rows.iter().map(topic::Row::to_topic).collect())
    }

    pub fn topic_sql(&self) -> String {
        let mut index = 1;
        let mut wheres = vec![format!("t.name ~~* all(${index})")];
        index += 1;

        wheres.push(format!("om.user_id = any(${index}::uuid[])"));
        index += 1;

        let where_clause = wheres.join(" and ");

        format!(
            r#"select
            {TOPIC_FIELDS}
            {TOPIC_JOINS}
            where {where_clause}
            group by t.id, o.login, r.system, r.name, r.owner_id
            order by char_length(t.name), t.name
            limit ${index}"#
        )
    }
}

pub struct SearchQuery {
    viewer_ids: Vec<String>,
    parent_topic: Topic,
    query_spec: QuerySpec,
    limit: i32,
}

impl SearchQuery {
    pub fn from(viewer_ids: Vec<String>, parent_topic: Topic, query_spec: QuerySpec) -> Self {
        Self {
            viewer_ids,
            parent_topic,
            query_spec,
            limit: 100,
        }
    }

    pub async fn execute(&self, pool: &PgPool) -> Result<Vec<SearchResultItem>> {
        let topic_ids = self.topic_ids();
        let tokens = self.query_spec.wildcard_tokens();

        let (mut results, limit) = self
            .fetch_topics(pool, self.limit, &topic_ids, &tokens)
            .await?;

        let (links, _limit) = self.fetch_links(pool, limit, &topic_ids, &tokens).await?;
        results.extend(links);

        Ok(results)
    }

    fn topic_sql(&self, topic_ids: &Vec<String>) -> String {
        let mut wheres: Vec<String> = vec![];
        let mut joins: Vec<String> = vec![];
        let mut index = 1;

        for i in 1..=topic_ids.len() {
            let (join_clause, where_clause) = self.topic_clauses(i);
            joins.push(join_clause);
            wheres.push(where_clause);
            index = i + 1
        }

        if !self.query_spec.tokens.is_empty() {
            wheres.push(format!("t.name ~~* all(${index})"));
            index += 1;
        }

        let order_by =
            format!(r#"order by t.id = any (${index}::uuid[]) desc, char_length(t.name), t.name"#);
        index += 1;

        wheres.push(format!("om.user_id = any(${index}::uuid[])"));
        index += 1;

        let where_clauses = wheres.join(" and ");
        let join_clauses = joins.join("\n");

        format!(
            r#"select
            {TOPIC_FIELDS}
            {TOPIC_JOINS}
            {join_clauses}
            where {where_clauses}
            group by t.id, o.login, r.system, r.name, r.owner_id
            {order_by}
            limit ${index}"#
        )
    }

    async fn fetch_topics(
        &self,
        pool: &PgPool,
        limit: i32,
        topic_ids: &Vec<String>,
        tokens: &Vec<String>,
    ) -> Result<(Vec<SearchResultItem>, i32)> {
        log::debug!(
            "filtering on topic ids {:?}, tokens {:?} and limit {:?}",
            topic_ids,
            tokens,
            limit
        );

        let sql = self.topic_sql(topic_ids);
        let mut q = sqlx::query_as::<_, topic::Row>(&sql);
        for topic_id in topic_ids {
            q = q.bind(topic_id);
        }
        let rows = q
            .bind(tokens)
            .bind(topic_ids)
            .bind(&self.viewer_ids)
            .bind(limit)
            .fetch_all(pool)
            .await?;

        let results: Vec<SearchResultItem> =
            rows.iter().map(topic::Row::to_search_result_item).collect();

        let found = results.len() as i32;
        Ok((results, limit - found))
    }

    fn link_sql(&self, topic_ids: &Vec<String>) -> String {
        let mut wheres: Vec<String> = vec![];
        let mut joins: Vec<String> = vec![];
        let mut index = 1;

        for i in 1..=topic_ids.len() {
            let (join_clause, where_clause) = self.link_clauses(i);
            joins.push(join_clause);
            wheres.push(where_clause);
            index = i + 1
        }

        if !self.query_spec.tokens.is_empty() {
            wheres.push(format!("l.title ~~* all(${index})"));
            index += 1;
        }

        wheres.push(format!("om.user_id = any(${index}::uuid[])"));
        index += 1;

        let where_clauses = wheres.join(" and ");
        let join_clauses = joins.join("\n");

        format!(
            r#"select
            {LINK_FIELDS}
            {LINK_JOINS}
            {join_clauses}
            where {where_clauses}
            group by l.id
            order by l.created_at desc
            limit ${index}"#
        )
    }

    async fn fetch_links(
        &self,
        pool: &PgPool,
        limit: i32,
        topic_ids: &Vec<String>,
        tokens: &Vec<String>,
    ) -> Result<(Vec<SearchResultItem>, i32)> {
        let sql = self.link_sql(topic_ids);

        let mut q = sqlx::query_as::<_, link::Row>(&sql);
        for topic_id in topic_ids {
            q = q.bind(topic_id);
        }
        let rows = q
            .bind(tokens)
            .bind(&self.viewer_ids)
            .bind(limit)
            .fetch_all(pool)
            .await?;

        let links: Vec<SearchResultItem> =
            rows.iter().map(link::Row::to_search_result_item).collect();

        Ok((links, limit))
    }

    fn topic_clauses(&self, index: usize) -> (String, String) {
        let join_clause =
            format!("join topic_transitive_closure ttc{index} on t.id = ttc{index}.child_id");
        let where_clause = format!("ttc{index}.parent_id = ${index}::uuid");
        (join_clause, where_clause)
    }

    fn link_clauses(&self, index: usize) -> (String, String) {
        let join_clause =
            format!("join link_transitive_closure ltc{index} on l.id = ltc{index}.child_id");
        let where_clause = format!("ltc{index}.parent_id = ${index}::uuid");
        (join_clause, where_clause)
    }

    // FIXME: Account for the org and repo as well?
    fn topic_ids(&self) -> Vec<String> {
        let mut topic_ids: Vec<String> = vec![self.parent_topic.id.to_string()];
        topic_ids.extend(self.query_spec.topics.iter().map(TopicSpec::topic_id));
        topic_ids
    }
}
