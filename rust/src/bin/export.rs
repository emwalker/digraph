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
    DataRoot, Link, LinkMetadata, ParentTopic, Synonym, Timerange, TimerangePrefixFormat, Topic,
    TopicChild, TopicMetadata,
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
        "d",
        "data-dir",
        "root directory of a repo to export to",
        "DIRNAME",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };
    let data_directory = matches.opt_str("d").expect("expected a data directory");
    let root = Path::new(&data_directory);

    Opts {
        root: PathBuf::from(root),
    }
}

#[derive(Clone, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
struct SynonymRow {
    added: DateTime<Utc>,
    locale: String,
    name: String,
}

impl From<&SynonymRow> for Synonym {
    fn from(row: &SynonymRow) -> Self {
        Self {
            added: row.added,
            locale: row.locale.clone(),
            name: row.name.clone(),
        }
    }
}

#[derive(FromRow)]
struct TopicMetadataRow {
    added: DateTime<Utc>,
    path: String,
    name: String,
    root: bool,
    synonyms: serde_json::Value,
    timerange_starts: Option<DateTime<Utc>>,
    timerange_prefix_format: Option<String>,
}

#[derive(Clone, FromRow)]
struct ParentTopicRow {
    path: String,
}

impl From<&ParentTopicRow> for ParentTopic {
    fn from(row: &ParentTopicRow) -> Self {
        Self { path: row.path.clone() }
    }
}

#[derive(Clone, FromRow)]
struct LinkMetadataRow {
    added: DateTime<Utc>,
    path: String,
    link_id: String,
    title: String,
    url: String,
}

impl From<&LinkMetadataRow> for LinkMetadata {
    fn from(row: &LinkMetadataRow) -> Self {
        Self {
            added: row.added,
            path: row.path.clone(),
            title: row.title.clone(),
            url: row.url.clone(),
        }
    }
}

#[derive(FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
struct TopicChildRow {
    added: DateTime<Utc>,
    path: String,
}

impl From<&TopicChildRow> for TopicChild {
    fn from(row: &TopicChildRow) -> Self {
        Self {
            added: row.added,
            path: row.path.clone(),
        }
    }
}

fn write_object<T: Serialize>(root: &DataRoot, id: &str, object: T) -> Result<()> {
    let filename = root.file_path(id)?;
    let dest = filename.parent().expect("expected a parent directory");
    fs::create_dir_all(&dest).ok();
    let s = serde_yaml::to_string(&object)?;
    log::debug!("saving {:?}", filename);
    fs::write(&filename, s)?;
    Ok(())
}

async fn save_topics(root: &DataRoot, pool: &PgPool) -> Result<()> {
    log::info!("saving topics");

    let rows = sqlx::query_as::<_, TopicMetadataRow>(
        r#"select
            t.name,
            concat('/', o.login, '/', t.id) path,
            t.synonyms,
            t.created_at added,
            t.root,
            tr.starts_at timerange_starts,
            tr.prefix_format timerange_prefix_format

        from topics t
        join organizations o on o.id = t.organization_id
        left join timeranges tr on tr.id = t.timerange_id"#,
    )
    .fetch_all(pool)
    .await?;

    for meta in &rows {
        let topic_path = RepoPath::from(&meta.path);
        let parent_topics = sqlx::query_as::<_, ParentTopicRow>(
            r#"select
                concat('/', o.login, '/', tt.parent_id) path
            from topic_topics tt
            join topics t on t.id = tt.parent_id
            join organizations o on o.id = t.organization_id
            left join timeranges tr on tr.id = t.timerange_id
            where tt.child_id = $1::uuid"#,
        )
        .bind(&topic_path.short_id)
        .fetch_all(pool)
        .await?;

        let children = sqlx::query_as::<_, TopicChildRow>(
            r#"(
                select
                    t.created_at added,
                    concat('/', o.login, '/', tt.child_id) path
                from topic_topics tt
                join topics t on t.id = tt.child_id
                join organizations o on o.id = t.organization_id
                where tt.parent_id = $1::uuid
                order by t.name
            )

            union all

            (
                select
                    l.created_at added,
                    concat('/', o.login, '/', l.id) path
                from link_topics tt
                join links l on l.id = tt.child_id
                join organizations o on o.id = l.organization_id
                where tt.parent_id = $1::uuid
                order by l.created_at desc
            )"#,
        )
        .bind(&topic_path.short_id)
        .fetch_all(pool)
        .await?;

        let deserialized =
            serde_json::from_value::<Vec<HashMap<String, String>>>(meta.synonyms.clone())?;

        let mut synonyms: Vec<SynonymRow> = vec![];
        for s in &deserialized {
            synonyms.push(SynonymRow {
                added: meta.added,
                locale: s.get("Locale").unwrap_or(&String::from("en")).to_string(),
                name: s.get("Name").unwrap_or(&meta.name).to_string(),
            });
        }

        let timerange = match (&meta.timerange_starts, &meta.timerange_prefix_format) {
            (Some(starts), Some(prefix_format)) => Some(Timerange {
                starts: starts.to_owned(),
                prefix_format: TimerangePrefixFormat::from(prefix_format.as_ref()),
            }),
            _ => None,
        };

        let topic = Topic {
            api_version: API_VERSION.to_string(),
            kind: "Topic".to_string(),
            metadata: TopicMetadata {
                added: meta.added,
                path: topic_path.path,
                root: meta.root,
                synonyms: synonyms.iter().map(Synonym::from).collect(),
                timerange,
            },
            parent_topics: parent_topics.iter().map(ParentTopic::from).collect(),
            children: children.iter().map(TopicChild::from).collect(),
        };

        write_object(root, &meta.path, topic)?;
    }

    Ok(())
}

async fn save_links(root: &DataRoot, pool: &PgPool) -> Result<()> {
    log::info!("saving links");

    let rows = sqlx::query_as::<_, LinkMetadataRow>(
        r#"select
            l.created_at added,
            concat('/', o.login, '/', l.id) path,
            l.id::varchar link_id,
            l.title,
            l.url
        from links l
        join organizations o on o.id = l.organization_id"#,
    )
    .fetch_all(pool)
    .await?;

    for meta in rows {
        let link_id = meta.link_id.clone();
        let parent_topics = sqlx::query_as::<_, ParentTopicRow>(
            r#"select
                concat('/', o.login, '/', lt.parent_id) path
            from link_topics lt
            join topics t on t.id = lt.parent_id
            join organizations o on o.id = t.organization_id
            where lt.child_id = $1::uuid"#,
        )
        .bind(&link_id)
        .fetch_all(pool)
        .await?;

        let link = Link {
            api_version: API_VERSION.to_string(),
            kind: "Link".to_string(),
            metadata: LinkMetadata::from(&meta),
            parent_topics: parent_topics.iter().map(ParentTopic::from).collect(),
        };

        write_object(root, &meta.path, link)?;
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
    let root = DataRoot::new(opts.root);

    save_topics(&root, &pool).await?;
    save_links(&root, &pool).await?;

    log::info!("exported database to {}", root);
    Ok(())
}
