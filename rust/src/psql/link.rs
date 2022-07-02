use async_graphql::types::ID;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;

use super::queries::{LINK_FIELDS, LINK_JOINS};
use crate::graphql::DateTime;
use crate::graphql::{Link, LinkReview, TopicChild, UpdateLinkTopicsInput, Viewer};
use crate::prelude::*;

const PUBLIC_ROOT_TOPIC_PATH: &str = "/wiki/df63295e-ee02-11e8-9e36-17d56b662bc8";

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Row {
    path: String,
    parent_topic_paths: Vec<String>,
    repository_id: Uuid,
    reviewed_at: Option<chrono::DateTime<chrono::Utc>>,
    viewer_id: String,
    title: String,
    url: String,
}

impl Row {
    fn to_link(&self, newly_added: bool) -> Link {
        let viewer_review = LinkReview {
            reviewed_at: self.reviewed_at.map(DateTime),
            user_id: self.viewer_id.clone(),
        };

        Link {
            path: RepoPath::from(&self.path),
            newly_added,
            parent_topic_paths: self.parent_topic_paths.iter().map(RepoPath::from).collect(),
            title: self.title.to_owned(),
            repository_id: ID(self.repository_id.to_string()),
            url: self.url.to_owned(),
            viewer_review: Some(viewer_review),
        }
    }

    pub fn to_search_result_item(&self) -> TopicChild {
        TopicChild::Link(self.to_link(false))
    }
}

async fn fetch_link(query_ids: &Vec<String>, pool: &PgPool, link_path: &RepoPath) -> Result<Row> {
    let query = format!(
        r#"select
        {LINK_FIELDS}
        {LINK_JOINS}
        where l.id = $1::uuid
            and om.user_id = any($2::uuid[])
        group by l.id, o.login"#,
    );

    let row = sqlx::query_as::<_, Row>(&query)
        .bind(&link_path.short_id)
        .bind(query_ids)
        .fetch_one(pool)
        .await?;

    Ok(row)
}

pub struct ReviewLink {
    pub actor: Viewer,
    pub link: RepoPath,
    pub reviewed: bool,
}

pub struct ReviewLinkResult {
    pub link: Link,
}

impl ReviewLink {
    pub fn new(actor: Viewer, link: RepoPath, reviewed: bool) -> Self {
        Self {
            actor,
            link,
            reviewed,
        }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<ReviewLinkResult> {
        fetch_link(&self.actor.mutation_ids, pool, &self.link).await?;

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
        .bind(&self.link.short_id)
        .bind(&self.actor.user_id)
        .bind(reviewed_at)
        .execute(pool)
        .await?;

        let row = fetch_link(&self.actor.mutation_ids, pool, &self.link).await?;
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
        let link_path = RepoPath::from(&self.input.link_path);

        // Verify that we can update the link
        fetch_link(&self.actor.mutation_ids, pool, &link_path).await?;

        let mut parent_topic_paths = self
            .input
            .parent_topic_paths
            .iter()
            .map(RepoPath::from)
            .collect::<Vec<RepoPath>>();

        if parent_topic_paths.is_empty() {
            parent_topic_paths.push(RepoPath::from(PUBLIC_ROOT_TOPIC_PATH));
        }

        let mut tx = pool.begin().await?;

        sqlx::query("delete from link_transitive_closure where child_id = $1::uuid")
            .bind(&link_path.short_id)
            .execute(&mut tx)
            .await?;

        sqlx::query("delete from link_topics where child_id = $1::uuid")
            .bind(&link_path.short_id)
            .execute(&mut tx)
            .await?;

        for topic_path in &parent_topic_paths {
            sqlx::query("select add_topic_to_link($1::uuid, $2::uuid)")
                .bind(&topic_path.short_id)
                .bind(&link_path.short_id)
                .execute(&mut tx)
                .await?;
        }

        tx.commit().await?;

        let row = fetch_link(&self.actor.mutation_ids, pool, &link_path).await?;
        Ok(UpdateLinkTopicsResult {
            link: row.to_link(false),
        })
    }
}
