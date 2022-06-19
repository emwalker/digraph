use chrono::{DateTime, Utc};
use getopts::Options;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use digraph::config::Config;
use digraph::db;
use digraph::git::{
    Link, LinkMetadata, ParentTopic, RepoPath, Synonym, Topic, TopicChild, TopicMetadata,
};
use digraph::prelude::*;

const API_VERSION: &str = "objects/v1";

struct Opts {
    root: PathBuf,
}

fn parse_args() -> Opts {
    let args: Vec<String> = env::args().collect();
    // let program = args[0].clone();

    let mut opts = Options::new();
    opts.reqopt(
        "o",
        "output",
        "root directory of a repo to export to",
        "DIRNAME",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };
    let dirname = matches
        .opt_str("o")
        .expect("expected a repo root directory");
    let root = Path::new(&dirname);

    Opts {
        root: PathBuf::from(root),
    }
}

#[derive(Clone, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
struct SynonymRow {
    added_timestamp: DateTime<Utc>,
    locale: String,
    name: String,
}

impl From<&SynonymRow> for Synonym {
    fn from(row: &SynonymRow) -> Self {
        Self {
            added_timestamp: row.added_timestamp,
            locale: row.locale.clone(),
            name: row.name.clone(),
        }
    }
}

#[derive(FromRow)]
struct TopicMetadataRow {
    added_timestamp: DateTime<Utc>,
    id: String,
    name: String,
    synonyms: serde_json::Value,
}

#[derive(Clone, FromRow)]
struct ParentTopicRow {
    id: String,
}

impl From<&ParentTopicRow> for ParentTopic {
    fn from(row: &ParentTopicRow) -> Self {
        Self { id: row.id.clone() }
    }
}

#[derive(Clone, FromRow)]
struct LinkMetadataRow {
    added_timestamp: DateTime<Utc>,
    id: String,
    link_id: String,
    title: String,
    url: String,
}

impl From<&LinkMetadataRow> for LinkMetadata {
    fn from(row: &LinkMetadataRow) -> Self {
        Self {
            added_timestamp: row.added_timestamp,
            id: row.id.clone(),
            title: row.title.clone(),
            url: row.url.clone(),
        }
    }
}

#[derive(FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
struct TopicChildRow {
    added_timestamp: DateTime<Utc>,
    id: String,
}

impl From<&TopicChildRow> for TopicChild {
    fn from(row: &TopicChildRow) -> Self {
        Self {
            added_timestamp: row.added_timestamp,
            id: row.id.clone(),
        }
    }
}

fn write_object<T: Serialize>(root: &RepoPath, id: &str, object: T) -> Result<()> {
    let filename = root.path(id)?;
    let dest = filename.parent().expect("expected a parent directory");
    fs::create_dir_all(&dest).ok();
    let s = serde_yaml::to_string(&object)?;
    log::debug!("saving {:?}", filename);
    fs::write(&filename, s)?;
    Ok(())
}

async fn save_topics(root: &RepoPath, pool: &PgPool) -> Result<()> {
    log::info!("saving topics");

    let rows = sqlx::query_as::<_, TopicMetadataRow>(
        r#"select
            t.name,
            concat('/wiki/', t.id) id,
            t.synonyms,
            t.created_at added_timestamp
        from topics t"#,
    )
    .fetch_all(pool)
    .await?;

    for metadata in &rows {
        let topic_id = metadata.id.split('/').last().expect("expected a uuid");
        let parent_topics = sqlx::query_as::<_, ParentTopicRow>(
            r#"select
                concat('/wiki/', tt.parent_id) id
            from topic_topics tt
            where tt.child_id = $1::uuid"#,
        )
        .bind(&topic_id)
        .fetch_all(pool)
        .await?;

        let children = sqlx::query_as::<_, TopicChildRow>(
            r#"(
                select
                    t.created_at added_timestamp,
                    concat('/wiki/', tt.child_id) id
                from topic_topics tt
                join topics t on t.id = tt.child_id
                where tt.parent_id = $1::uuid
                order by t.name
            )

            union all

            (
                select
                    l.created_at added_timestamp,
                    concat('/wiki/', encode(digest(l.url, 'sha256'), 'hex')) id
                from link_topics tt
                join links l on l.id = tt.child_id
                where tt.parent_id = $1::uuid
                order by l.created_at desc
            )"#,
        )
        .bind(&topic_id)
        .fetch_all(pool)
        .await?;

        let deserialized =
            serde_json::from_value::<Vec<HashMap<String, String>>>(metadata.synonyms.clone())?;

        let mut synonyms: Vec<SynonymRow> = vec![];
        for s in &deserialized {
            synonyms.push(SynonymRow {
                added_timestamp: metadata.added_timestamp,
                locale: s.get("Locale").unwrap_or(&String::from("en")).to_string(),
                name: s.get("Name").unwrap_or(&metadata.name).to_string(),
            });
        }

        let topic = Topic {
            api_version: API_VERSION.to_string(),
            kind: "Topic".to_string(),
            metadata: TopicMetadata {
                id: metadata.id.clone(),
                added_timestamp: metadata.added_timestamp,
                synonyms: synonyms.iter().map(Synonym::from).collect(),
            },
            parent_topics: parent_topics.iter().map(ParentTopic::from).collect(),
            children: children.iter().map(TopicChild::from).collect(),
        };

        let id = metadata.id.split('/').last().expect("expected an id");
        write_object(root, id, topic)?;
    }

    Ok(())
}

async fn save_links(root: &RepoPath, pool: &PgPool) -> Result<()> {
    log::info!("saving links");

    let rows = sqlx::query_as::<_, LinkMetadataRow>(
        r#"select
            l.created_at added_timestamp,
            concat('/wiki/', encode(digest(l.url, 'sha256'), 'hex')) id,
            l.id::varchar link_id,
            l.title,
            l.url
        from links l"#,
    )
    .fetch_all(pool)
    .await?;

    for metadata in rows {
        let link_id = metadata.link_id.clone();
        let parent_topics = sqlx::query_as::<_, ParentTopicRow>(
            r#"select
                concat('/wiki/', lt.parent_id) id
            from link_topics lt
            where lt.child_id = $1::uuid"#,
        )
        .bind(&link_id)
        .fetch_all(pool)
        .await?;

        let link = Link {
            api_version: API_VERSION.to_string(),
            kind: "Link".to_string(),
            metadata: LinkMetadata::from(&metadata),
            parent_topics: parent_topics.iter().map(ParentTopic::from).collect(),
        };

        let id = metadata.id.split('/').last().expect("expected an id");
        write_object(root, id, link)?;
    }

    Ok(())
}

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::load()?;
    env_logger::init();
    let pool = db::db_connection(&config).await?;
    let opts = parse_args();

    if !opts.root.exists() {
        return Err(Error::NotFound(format!("{:?}", opts.root)));
    }
    let root = RepoPath::new(opts.root, None);

    save_topics(&root, &pool).await?;
    save_links(&root, &pool).await?;

    log::info!("exported database to {}", root);
    Ok(())
}
