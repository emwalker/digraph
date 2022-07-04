use async_graphql::types::ID;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;

use super::queries::{LINK_FIELDS, LINK_JOINS};
use crate::graphql::DateTime;
use crate::graphql::{Link, LinkReview, Viewer};
use crate::prelude::*;

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
