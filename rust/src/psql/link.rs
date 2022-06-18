use async_graphql::dataloader::*;
use async_graphql::types::ID;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use std::collections::HashMap;

use super::{
    fetch_topic,
    queries::{LINK_FIELDS, LINK_JOINS},
};
use crate::prelude::*;
use crate::schema::{
    Alert, Link, LinkReview, SearchResultItem, UpdateLinkTopicsInput, UpsertLinkInput, Viewer,
};
use crate::{
    http::{repo_url::Url, Page},
    schema::DateTime,
};

const PUBLIC_ROOT_TOPIC_ID: &str = "df63295e-ee02-11e8-9e36-17d56b662bc8";

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Row {
    id: Uuid,
    parent_topic_ids: Vec<Uuid>,
    repository_id: Uuid,
    reviewed_at: Option<chrono::DateTime<chrono::Utc>>,
    viewer_id: String,
    title: String,
    url: String,
}

impl Row {
    fn to_link(&self, newly_added: bool) -> Link {
        let parent_topic_ids = self.parent_topic_ids.iter().map(Uuid::to_string).collect();

        let viewer_review = LinkReview {
            reviewed_at: self.reviewed_at.map(DateTime),
            user_id: self.viewer_id.clone(),
        };

        Link {
            id: ID(self.id.to_string()),
            newly_added,
            parent_topic_ids,
            title: self.title.to_owned(),
            repository_id: ID(self.repository_id.to_string()),
            url: self.url.to_owned(),
            viewer_review: Some(viewer_review),
        }
    }

    pub fn to_search_result_item(&self) -> SearchResultItem {
        SearchResultItem::Link(self.to_link(false))
    }
}

async fn fetch_link(query_ids: &Vec<String>, pool: &PgPool, link_id: &String) -> Result<Row> {
    let query = format!(
        r#"select
        {LINK_FIELDS}
        {LINK_JOINS}
        where l.id = $1::uuid
            and om.user_id = any($2::uuid[])
        group by l.id"#,
    );

    let row = sqlx::query_as::<_, Row>(&query)
        .bind(link_id)
        .bind(query_ids)
        .fetch_one(pool)
        .await?;

    Ok(row)
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
    limit: i32,
    parent_topic_id: String,
    reviewed: Option<bool>,
    viewer: Viewer,
}

impl FetchChildLinksForTopic {
    pub fn new(viewer: Viewer, parent_topic_id: String, reviewed: Option<bool>) -> Self {
        Self {
            limit: 100,
            parent_topic_id,
            reviewed,
            viewer,
        }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<Vec<Link>> {
        log::debug!("fetching linkes for topic: {}", self.parent_topic_id);

        let mut index = 1;
        let mut link_fields: Vec<String> = Vec::new();
        let mut reviewed_joins: Vec<String> = Vec::new();
        let mut reviewed_wheres: Vec<String> = vec!["true".into()];
        let mut group_by: Vec<String> = vec!["l.id".into(), "l.created_at".into()];

        if let Some(reviewed) = self.reviewed {
            link_fields.push(format!(", ulr.reviewed_at, ${index} viewer_id"));
            reviewed_joins.push(format!(
                r#"left join user_link_reviews ulr
                        on l.id = ulr.link_id and ulr.user_id = ${index}::uuid"#
            ));
            index += 1;

            if reviewed {
                reviewed_wheres.push("ulr.reviewed_at is not null".into());
            } else {
                reviewed_wheres.push("ulr.reviewed_at is null".into());
            }

            group_by.push("ulr.reviewed_at".into());
        }

        let link_fields = link_fields.join(" ");
        let reviewed_joins = reviewed_joins.join("\n");
        let reviewed_wheres = reviewed_wheres.join(" and ");
        let group_by = group_by.join(", ");

        let param_parent_id = index;
        let param_om_user_ids = param_parent_id + 1;
        let param_limit = param_om_user_ids + 1;

        let query = format!(
            r#"select
                {LINK_FIELDS}
                {link_fields}
                {LINK_JOINS}
                {reviewed_joins}
                where parent_topics.parent_id = ${param_parent_id}::uuid
                    and om.user_id = any(${param_om_user_ids}::uuid[])
                    and {reviewed_wheres}
                group by {group_by}
                order by l.created_at desc
                limit ${param_limit}"#
        );

        let mut q = sqlx::query_as::<_, Row>(&query);

        if self.reviewed.is_some() {
            q = q.bind(&self.viewer.user_id);
        }

        let rows = q
            .bind(&self.parent_topic_id)
            .bind(&self.viewer.query_ids)
            .bind(self.limit)
            .fetch_all(pool)
            .await?;

        Ok(rows.iter().map(|r| r.to_link(false)).collect())
    }
}

pub struct DeleteLink {
    actor: Viewer,
    link_id: String,
}

pub struct DeleteLinkResult {
    pub deleted_link_id: String,
}

impl DeleteLink {
    pub fn new(actor: Viewer, link_id: String) -> Self {
        Self { actor, link_id }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<DeleteLinkResult> {
        // Ensure that the caller can modify the link
        fetch_link(&self.actor.mutation_ids, pool, &self.link_id).await?;

        sqlx::query("delete from links where id = $1::uuid")
            .bind(&self.link_id)
            .execute(pool)
            .await?;

        Ok(DeleteLinkResult {
            deleted_link_id: self.link_id.clone(),
        })
    }
}

pub struct ReviewLink {
    pub actor: Viewer,
    pub link_id: String,
    pub reviewed: bool,
}

pub struct ReviewLinkResult {
    pub link: Link,
}

impl ReviewLink {
    pub fn new(actor: Viewer, link_id: String, reviewed: bool) -> Self {
        Self {
            actor,
            link_id,
            reviewed,
        }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<ReviewLinkResult> {
        fetch_link(&self.actor.mutation_ids, pool, &self.link_id).await?;

        let reviewed_at = if self.reviewed {
            Some(chrono::Utc::now())
        } else {
            None
        };

        sqlx::query(
            r#"insert into user_link_reviews
                (link_id, user_id, reviewed_at)
                values ($1::uuid, $2::uuid, $3)
            on conflict on constraint user_link_reviews_user_id_link_id_key do
                update set reviewed_at = $3"#,
        )
        .bind(&self.link_id)
        .bind(&self.actor.user_id)
        .bind(reviewed_at)
        .execute(pool)
        .await?;

        let row = fetch_link(&self.actor.mutation_ids, pool, &self.link_id).await?;
        Ok(ReviewLinkResult {
            link: row.to_link(false),
        })
    }
}

pub struct UpdateLinkParentTopics {
    actor: Viewer,
    input: UpdateLinkTopicsInput,
}

pub struct UpdateLinkTopicsResult {
    pub link: Link,
}

impl UpdateLinkParentTopics {
    pub fn new(actor: Viewer, input: UpdateLinkTopicsInput) -> Self {
        Self { actor, input }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<UpdateLinkTopicsResult> {
        let link_id = self.input.link_id.to_string();

        // Verify that we can update the link
        fetch_link(&self.actor.mutation_ids, pool, &link_id).await?;

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
            .bind(&link_id)
            .execute(&mut tx)
            .await?;

        sqlx::query("delete from link_topics where child_id = $1::uuid")
            .bind(&link_id)
            .execute(&mut tx)
            .await?;

        for topic_id in &topic_ids {
            sqlx::query("select add_topic_to_link($1::uuid, $2::uuid)")
                .bind(&topic_id)
                .bind(&link_id)
                .execute(&mut tx)
                .await?;
        }

        tx.commit().await?;

        let row = fetch_link(&self.actor.mutation_ids, pool, &link_id).await?;
        Ok(UpdateLinkTopicsResult {
            link: row.to_link(false),
        })
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
            let topic_id = topic_id.to_string();
            // Verify that we can update the parent topic
            fetch_topic(&self.actor.mutation_ids, pool, &topic_id).await?;

            sqlx::query("select add_topic_to_link($1::uuid, $2::uuid)")
                .bind(&topic_id)
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

        let row = fetch_link(&self.actor.mutation_ids, pool, &link_id.to_string()).await?;
        Ok(UpsertLinkResult {
            alerts: vec![],
            link: Some(row.to_link(inserted)),
        })
    }
}
