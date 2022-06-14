use async_graphql::dataloader::*;
use async_graphql::types::ID;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;

use super::queries::{LINK_FIELDS, LINK_JOINS};
use crate::http::{repo_url::Url, Page};
use crate::prelude::*;
use crate::schema::{
    Alert, Link, SearchResultItem, UpdateLinkTopicsInput, UpsertLinkInput, Viewer,
};

const PUBLIC_ROOT_TOPIC_ID: &str = "df63295e-ee02-11e8-9e36-17d56b662bc8";

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Row {
    id: Uuid,
    parent_topic_ids: Vec<Uuid>,
    repository_id: Uuid,
    title: String,
    url: String,
}

impl Row {
    fn to_link(&self, newly_added: bool) -> Link {
        let parent_topic_ids = self.parent_topic_ids.iter().map(Uuid::to_string).collect();

        Link {
            id: ID(self.id.to_string()),
            newly_added,
            parent_topic_ids,
            title: self.title.to_owned(),
            repository_id: ID(self.repository_id.to_string()),
            url: self.url.to_owned(),
        }
    }

    pub fn to_search_result_item(&self) -> SearchResultItem {
        SearchResultItem::Link(self.to_link(false))
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
            .map(|r| (r.id.to_string(), r.to_link(false)))
            .collect::<HashMap<_, _>>())
    }
}

pub struct FetchChildLinksForTopic {
    viewer_ids: Vec<String>,
    parent_topic_id: String,
    limit: i32,
}

impl FetchChildLinksForTopic {
    pub fn new(viewer_ids: Vec<String>, parent_topic_id: String) -> Self {
        Self {
            viewer_ids,
            parent_topic_id,
            limit: 100,
        }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<Vec<Link>> {
        log::debug!("fetching linkes for topic: {}", self.parent_topic_id);

        let query = format!(
            r#"
            select
                {LINK_FIELDS}
                {LINK_JOINS}
                where parent_topics.parent_id = $1::uuid
                    and om.user_id = any($2::uuid[])
                group by l.id, l.created_at
                order by l.created_at desc
                limit $3
            "#
        );

        let rows = sqlx::query_as::<_, Row>(&query)
            .bind(&self.parent_topic_id)
            .bind(&self.viewer_ids)
            .bind(self.limit)
            .fetch_all(pool)
            .await?;

        Ok(rows.iter().map(|r| r.to_link(false)).collect())
    }
}

pub struct DeleteLink {
    link_id: String,
}

pub struct DeleteLinkResult {
    pub deleted_link_id: String,
}

impl DeleteLink {
    pub fn new(link_id: String) -> Self {
        Self { link_id }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<DeleteLinkResult> {
        sqlx::query("delete from links where id = $1::uuid")
            .bind(&self.link_id)
            .execute(pool)
            .await?;
        Ok(DeleteLinkResult {
            deleted_link_id: self.link_id.clone(),
        })
    }
}

pub struct UpdateLinkParentTopics {
    input: UpdateLinkTopicsInput,
}

pub struct UpdateLinkTopicsResult {
    pub link: Link,
}

impl UpdateLinkParentTopics {
    pub fn new(input: UpdateLinkTopicsInput) -> Self {
        Self { input }
    }

    async fn fetch_link(&self, link_id: &str, pool: &PgPool) -> Result<Link> {
        let query = format!(
            r#"
            select
                {LINK_FIELDS}
                {LINK_JOINS}
                where l.id = $1::uuid
                group by l.id
            "#
        );

        let row = sqlx::query_as::<_, Row>(&query)
            .bind(link_id)
            .fetch_one(pool)
            .await?;

        Ok(row.to_link(false))
    }

    pub async fn call(&self, pool: &PgPool) -> Result<UpdateLinkTopicsResult> {
        let link_id = self.input.link_id.as_str();

        let mut topic_ids = self
            .input
            .parent_topic_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>();

        if topic_ids.is_empty() {
            topic_ids.push(PUBLIC_ROOT_TOPIC_ID.to_string());
        }

        let mut tx = pool.begin().await?;

        sqlx::query("delete from link_transitive_closure where child_id = $1::uuid")
            .bind(link_id)
            .execute(&mut tx)
            .await?;

        sqlx::query("delete from link_topics where child_id = $1::uuid")
            .bind(link_id)
            .execute(&mut tx)
            .await?;

        for topic_id in &topic_ids {
            sqlx::query("select add_topic_to_link($1::uuid, $2::uuid)")
                .bind(&topic_id)
                .bind(link_id)
                .execute(&mut tx)
                .await?;
        }

        tx.commit().await?;

        let link = self.fetch_link(link_id, pool).await?;
        Ok(UpdateLinkTopicsResult { link })
    }
}

pub struct UpsertLink {
    actor: Viewer,
    input: UpsertLinkInput,
}

pub struct UpsertLinkResult {
    pub alerts: Vec<Alert>,
    pub link: Option<Link>,
}

impl UpsertLink {
    pub fn new(actor: Viewer, input: UpsertLinkInput) -> Self {
        Self { actor, input }
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

        // TODO: Figure out how to pass a transaction around to helper methods
        let mut tx = pool.begin().await?;

        // Upsert link
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

            returning id, repository_id, organization_id, (xmax = 0) inserted
            "#;

        let (link_id, repository_id, organization_id, inserted) =
            sqlx::query_as::<_, (Uuid, Uuid, Uuid, bool)>(query)
                .bind(&self.input.organization_login)
                .bind(&self.input.repository_name)
                .bind(&url.normalized)
                .bind(&url.sha1)
                .bind(&title)
                .fetch_one(&mut tx)
                .await?;

        for topic_id in &self.input.add_parent_topic_ids {
            sqlx::query("select add_topic_to_link($1::uuid, $2::uuid)")
                .bind(topic_id.as_str())
                .bind(&link_id)
                .fetch_one(&mut tx)
                .await?;
        }

        // Upsert link activity
        let (user_link_id,) = sqlx::query_as::<_, (Uuid,)>(
            r#"insert into user_links
                (organization_id, repository_id, link_id, user_id, action)
                values ($1::uuid, $2::uuid, $3::uuid, $4::uuid, 'upsert_link')
                returning id
            "#,
        )
        .bind(&organization_id)
        .bind(&repository_id)
        .bind(&link_id)
        .bind(&self.actor.user_id)
        .fetch_one(&mut tx)
        .await?;

        for topic_id in &self.input.add_parent_topic_ids {
            sqlx::query(
                r#"insert into user_link_topics
                    (user_link_id, topic_id, action)
                    values ($1::uuid, $2::uuid, 'topic_added')
                "#,
            )
            .bind(&user_link_id)
            .bind(topic_id.as_str())
            .execute(&mut tx)
            .await?;
        }

        // Upsert user link review record
        // if err = m.addUserLinkReview(ctx, exec, link); err != nil {
        //     log.Printf("There was a problem creating a user link review record: %s", err)
        //     return nil, errors.Wrap(err, "services.UpsertLink")
        // }

        tx.commit().await?;

        let query = format!(
            r#"select
            {LINK_FIELDS}
            {LINK_JOINS}
            where l.id = $1::uuid
            group by l.id"#,
        );

        let row = sqlx::query_as::<_, Row>(&query)
            .bind(link_id)
            .fetch_one(pool)
            .await?;

        Ok(UpsertLinkResult {
            alerts: vec![],
            link: Some(row.to_link(inserted)),
        })
    }
}
