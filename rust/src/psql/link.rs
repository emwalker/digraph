use async_graphql::dataloader::*;
use async_graphql::types::ID;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;

use super::queries::{LINK_FIELDS, LINK_JOINS};
use crate::http::{repo_url::Url, Page};
use crate::prelude::*;
use crate::schema::{Link, SearchResultItem, UpsertLinkInput, Viewer};

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Row {
    id: Uuid,
    parent_topic_ids: Vec<Uuid>,
    repository_id: Uuid,
    title: String,
    url: String,
}

impl Row {
    fn to_link(&self) -> Link {
        let parent_topic_ids = self.parent_topic_ids.iter().map(Uuid::to_string).collect();

        Link {
            id: ID(self.id.to_string()),
            parent_topic_ids,
            title: self.title.to_owned(),
            repository_id: ID(self.repository_id.to_string()),
            url: self.url.to_owned(),
        }
    }

    pub fn to_search_result_item(&self) -> SearchResultItem {
        SearchResultItem::Link(self.to_link())
    }
}

pub struct LinkLoader {
    viewer: Viewer,
    pool: PgPool,
}

impl LinkLoader {
    pub fn new(viewer: Viewer, pool: PgPool) -> Self {
        Self { viewer, pool }
    }
}

#[async_trait::async_trait]
impl Loader<String> for LinkLoader {
    type Value = Link;
    type Error = Error;

    async fn load(&self, ids: &[String]) -> Result<HashMap<String, Self::Value>> {
        log::debug!("batch links: {:?}", ids);

        let query = format!(
            r#"select
            {LINK_FIELDS}
            {LINK_JOINS}
            where l.id = any($1::uuid[]) and om.user_id = any($2::uuid[])
            group by l.id"#,
        );

        let rows = sqlx::query_as::<_, Row>(&query)
            .bind(ids)
            .bind(&self.viewer.query_ids)
            .fetch_all(&self.pool)
            .await;

        Ok(rows?
            .iter()
            .map(|r| (r.id.to_string(), r.to_link()))
            .collect::<HashMap<_, _>>())
    }
}

pub struct UpsertLink {
    input: UpsertLinkInput,
}

pub struct UpsertLinkResult {
    pub link: Link,
}

impl UpsertLink {
    pub fn new(input: UpsertLinkInput) -> Self {
        Self { input }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<UpsertLinkResult> {
        let url = Url::parse(self.input.url.as_ref())?;

        let title = match &self.input.title {
            Some(title) => title.to_owned(),
            None => {
                let response = Page::from(url.clone()).fetch().await?;
                response
                    .title()
                    .unwrap_or_else(|| String::from("Missing title"))
            }
        };

        let mut tx = pool.begin().await?;

        let query = r#"
            insert
            into links
                (organization_id, repository_id, url, sha1, title)
                select
                    o.id, r.id, $3, $4, $5
                from organizations o
                join repositories r on o.id = r.organization_id
                where o.login = $1 and r.name = $2

            on conflict on constraint links_repository_sha1_idx do
                update set title = $5

            returning id
            "#;

        let row = sqlx::query_as::<_, (Uuid,)>(query)
            .bind(&self.input.organization_login)
            .bind(&self.input.repository_name)
            .bind(&url.normalized)
            .bind(&url.sha1)
            .bind(&title)
            .fetch_one(&mut tx)
            .await?;

        for topic_id in &self.input.add_parent_topic_ids {
            sqlx::query("select add_topic_to_link($1::uuid, $2::uuid)")
                .bind(&topic_id)
                .bind(&row.0)
                .fetch_one(&mut tx)
                .await?;
        }

        tx.commit().await?;

        let query = format!(
            r#"select
            {LINK_FIELDS}
            {LINK_JOINS}
            where l.id = $1::uuid
            group by l.id"#,
        );

        let row = sqlx::query_as::<_, Row>(&query)
            .bind(row.0)
            .fetch_one(pool)
            .await?;

        Ok(UpsertLinkResult {
            link: row.to_link(),
        })
    }
}
