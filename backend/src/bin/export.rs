use geotime::Geotime;
use getopts::Options;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::collections::{BTreeSet, HashMap};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use uuid::Uuid;

use digraph::config::Config;
use digraph::db;
use digraph::git::{
    activity, Client, DataRoot, IndexMode, Kind, Mutation, ParentTopic, RepoLink, RepoLinkDetails,
    RepoLinkMetadata, RepoTopic, RepoTopicDetails, RepoTopicMetadata, Synonym, TopicChild,
};
use digraph::prelude::*;
use digraph::redis;
use digraph::types::{sha256_base64, Timerange, TimerangePrefixFormat, Timespec};

struct Opts {
    root: PathBuf,
}

fn sha256_id(id: &str) -> ExternalId {
    let id = sha256_base64(id);
    ExternalId::try_from(&id).unwrap()
}

fn parse_args() -> Opts {
    let args: Vec<String> = env::args().collect();
    // let program = args[0].clone();

    let mut opts = Options::new();
    opts.reqopt(
        "d",
        "data-dir",
        "root directory of a repo_id to export to",
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

fn prefer_private_repo(repo_1: RepoId, repo_2: RepoId) -> RepoId {
    if repo_1.is_wiki() {
        repo_2
    } else {
        repo_1
    }
}

#[derive(Clone, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
struct SynonymRow {
    added: Timestamp,
    locale: String,
    name: String,
}

impl From<&SynonymRow> for Synonym {
    fn from(row: &SynonymRow) -> Self {
        use std::str::FromStr;
        Self {
            added: row.added,
            locale: Locale::from_str(&row.locale).unwrap_or(Locale::EN),
            name: row.name.clone(),
        }
    }
}

#[derive(FromRow)]
struct TopicMetadataRow {
    added: Timestamp,
    id: String,
    name: String,
    repository_id: Uuid,
    root: bool,
    synonyms: serde_json::Value,
    timerange_prefix_format: Option<String>,
    timerange_starts: Option<Timestamp>,
}

#[derive(Clone, FromRow)]
struct ParentTopicRow {
    id: String,
    repository_id: Uuid,
    name: String,
}

impl TryFrom<&ParentTopicRow> for ParentTopic {
    type Error = Error;

    fn try_from(row: &ParentTopicRow) -> Result<Self> {
        let id = sha256_id(&row.id);
        Ok(Self { id })
    }
}

#[derive(Clone, FromRow)]
struct LinkMetadataRow {
    added: Timestamp,
    repository_id: Uuid,
    link_id: String,
    title: String,
    url: String,
}

impl TryFrom<&LinkMetadataRow> for RepoLinkMetadata {
    type Error = Error;

    fn try_from(row: &LinkMetadataRow) -> Result<Self> {
        let id = sha256_id(&row.url);
        Ok(Self {
            added: row.added,
            id,
            details: Some(RepoLinkDetails {
                title: row.title.clone(),
                url: row.url.clone(),
            }),
        })
    }
}

#[derive(FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
struct TopicChildRow {
    added: Timestamp,
    id: String,
    kind: String,
    repository_id: Uuid,
    name: String,
    url: String,
}

impl TryFrom<&TopicChildRow> for TopicChild {
    type Error = Error;

    fn try_from(row: &TopicChildRow) -> Result<Self> {
        let id = match row.kind.as_str() {
            "Link" => sha256_id(&row.url),
            "Topic" => sha256_id(&row.id),
            _ => sha256_id(&row.id),
        };

        Ok(Self {
            added: row.added,
            kind: Kind::from(&row.kind).unwrap(),
            id,
        })
    }
}

#[derive(Debug)]
struct RepoTopics {
    repo_topic: RepoTopic,
    references: HashMap<RepoId, RepoTopic>,
}

impl RepoTopics {
    fn new(repo_id: RepoId, repo_topic: RepoTopic) -> Self {
        let mut references: HashMap<RepoId, RepoTopic> = HashMap::new();
        references.insert(repo_id, repo_topic.clone());
        Self {
            repo_topic,
            references,
        }
    }

    fn get_mut(&mut self, repo_id: RepoId) -> &mut RepoTopic {
        let topics = &mut self.references;

        topics.entry(repo_id).or_insert_with(|| {
            let id = self.repo_topic.topic_id().to_owned();

            RepoTopic {
                api_version: API_VERSION.to_string(),
                metadata: RepoTopicMetadata {
                    added: self.repo_topic.added(),
                    id,
                    details: None,
                },
                parent_topics: BTreeSet::new(),
                children: BTreeSet::new(),
            }
        })
    }
}

fn persist_topic(
    mutation: &mut Mutation,
    repo_topic_repo_id: RepoId,
    repo_topic: &mut RepoTopic,
    parent_topics: &Vec<ParentTopicRow>,
    children: &Vec<TopicChildRow>,
) -> Result<()> {
    let mut child_links = vec![];
    let mut child_topics = vec![];

    for child in children {
        let TopicChildRow {
            kind,
            id,
            repository_id,
            name,
            url,
            ..
        } = child;
        let child_repo_id = RepoId::try_from(repository_id).unwrap();

        if repo_topic_repo_id != child_repo_id {
            continue;
        }

        match kind.as_str() {
            "Topic" => {
                let id = sha256_id(id);
                child_topics.push(activity::TopicInfo::from((Locale::EN, name.to_owned(), id)));
            }
            "Link" => {
                let id = sha256_id(url);
                child_links.push(activity::LinkInfo {
                    title: name.to_owned(),
                    url: url.to_owned(),
                    id,
                    deleted: false,
                });
            }
            _ => {}
        };
    }

    let mut parents = vec![];
    for topic in parent_topics {
        let topic_repo_id = RepoId::try_from(&topic.repository_id).unwrap();
        if topic_repo_id != repo_topic_repo_id {
            continue;
        }

        let id = sha256_id(&topic.id);
        parents.push(activity::TopicInfo::from((
            Locale::EN,
            topic.name.to_owned(),
            id,
        )));
    }

    let change = activity::Change::ImportTopic(activity::ImportTopic {
        actor_id: "461c87c8-fb8f-11e8-9cbc-afde6c54d881".to_owned(),
        date: chrono::Utc::now(),
        imported_topic: activity::TopicInfo::from(&*repo_topic),
        child_links: activity::LinkInfoList::from(&child_links),
        child_topics: activity::TopicInfoList::from(&child_topics),
        id: activity::Change::new_id(),
        parent_topics: activity::TopicInfoList::from(&parents),
    });

    mutation.save_topic(repo_topic_repo_id, repo_topic)?;
    mutation.add_change(repo_topic_repo_id, &change)
}

// 1. Don't place paths for items in private repos under /wiki/
fn persist_topics(
    mutation: &mut Mutation,
    repo_topic_repo_id: RepoId,
    topic_id: &ExternalId,
    meta: &TopicMetadataRow,
    parent_topics: &Vec<ParentTopicRow>,
    children: &Vec<TopicChildRow>,
) -> Result<()> {
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
            starts: Geotime::from(starts).into(),
            prefix_format: TimerangePrefixFormat::from(prefix_format.as_str()),
        }),
        _ => None,
    };

    let repo_topic = RepoTopic {
        api_version: API_VERSION.to_string(),
        metadata: RepoTopicMetadata {
            added: meta.added,
            id: topic_id.to_owned(),
            details: Some(RepoTopicDetails {
                root: meta.root,
                synonyms: synonyms.iter().map(Synonym::from).collect(),
                timerange,
            }),
        },
        parent_topics: BTreeSet::new(),
        children: BTreeSet::new(),
    };

    let mut topics = RepoTopics::new(repo_topic_repo_id, repo_topic);

    for parent in parent_topics {
        let parent_repo_id = RepoId::try_from(&parent.repository_id).unwrap();
        let repo_id = prefer_private_repo(repo_topic_repo_id, parent_repo_id);
        let topic = topics.get_mut(repo_id);
        topic.parent_topics.insert(parent.try_into()?);
    }

    for child in children {
        let parent_repo_id = RepoId::try_from(&child.repository_id).unwrap();
        let repo_id = prefer_private_repo(repo_topic_repo_id, parent_repo_id);
        let topic = topics.get_mut(repo_id);
        topic.children.insert(child.try_into()?);
    }

    let repo_ids = topics.references.keys().cloned().collect::<Vec<RepoId>>();

    for repo_id in repo_ids {
        let topic = topics.get_mut(repo_id);
        persist_topic(mutation, repo_id, topic, parent_topics, children)?;
    }

    Ok(())
}

async fn export_topics(builder: &mut Mutation, pool: &PgPool) -> Result<()> {
    log::info!("saving topic blobs");

    let rows = sqlx::query_as::<_, TopicMetadataRow>(
        r#"select
            t.name,
            t.id::varchar,
            t.synonyms,
            t.root,
            t.created_at added,
            t.repository_id,
            tr.starts_at timerange_starts,
            tr.prefix_format timerange_prefix_format

        from topics t
        left join timeranges tr on tr.id = t.timerange_id"#,
    )
    .fetch_all(pool)
    .await
    .unwrap();

    for meta in &rows {
        let repo_id = RepoId::try_from(&meta.repository_id.to_string()).unwrap();
        let topic_id = sha256_id(&meta.id);

        let parent_topics = sqlx::query_as::<_, ParentTopicRow>(
            r#"select
                t.repository_id,
                tt.parent_id::varchar id,
                t.name

            from topic_topics tt
            join topics t on t.id = tt.parent_id
            left join timeranges tr on tr.id = t.timerange_id
            where tt.child_id = $1::uuid
            order by t.name"#,
        )
        .bind(&meta.id)
        .fetch_all(pool)
        .await
        .unwrap();

        let children = sqlx::query_as::<_, TopicChildRow>(
            r#"(
                select
                    t.created_at added,
                    'Topic' kind,
                    t.repository_id,
                    t.id::varchar,
                    t.name,
                    'url' as url

                from topic_topics tt
                join topics t on t.id = tt.child_id
                where tt.parent_id = $1::uuid
                order by t.name
            )

            union all

            (
                select
                    l.created_at added,
                    'Link' kind,
                    l.repository_id,
                    l.id::varchar,
                    l.title as name,
                    l.url

                from link_topics tt
                join links l on l.id = tt.child_id
                where tt.parent_id = $1::uuid
                order by l.created_at desc
            )"#,
        )
        .bind(&meta.id)
        .fetch_all(pool)
        .await
        .unwrap();

        persist_topics(builder, repo_id, &topic_id, meta, &parent_topics, &children).unwrap();
    }

    Ok(())
}

#[derive(Debug)]
struct RepoLinks {
    link: RepoLink,
    links: HashMap<RepoId, RepoLink>,
}

impl RepoLinks {
    fn new(repo_id: RepoId, link: RepoLink) -> Self {
        let mut links: HashMap<RepoId, RepoLink> = HashMap::new();
        links.insert(repo_id, link.clone());
        Self { link, links }
    }

    fn get_mut(&mut self, repo_id: RepoId) -> &mut RepoLink {
        let links = &mut self.links;

        links.entry(repo_id).or_insert_with(|| RepoLink {
            api_version: API_VERSION.to_string(),
            metadata: RepoLinkMetadata {
                added: self.link.added(),
                id: self.link.id().to_owned(),
                details: None,
            },
            parent_topics: BTreeSet::new(),
        })
    }
}

fn persist_link(
    mutation: &mut Mutation,
    repo_link_repo_id: RepoId,
    repo_link: &mut RepoLink,
    parent_topics: &Vec<ParentTopicRow>,
) -> Result<()> {
    let mut topics = BTreeSet::new();

    for parent in parent_topics {
        let parent_repo_id = RepoId::try_from(&parent.repository_id).unwrap();
        if repo_link_repo_id != parent_repo_id {
            continue;
        }

        let id = sha256_id(&parent.id);
        topics.insert(activity::TopicInfo::from((
            Locale::EN,
            parent.name.to_owned(),
            id,
        )));
    }

    let change = activity::Change::ImportLink(activity::ImportLink {
        actor_id: "461c87c8-fb8f-11e8-9cbc-afde6c54d881".to_owned(),
        date: chrono::Utc::now(),
        id: activity::Change::new_id(),
        imported_link: activity::LinkInfo::from(&*repo_link),
        parent_topics: activity::TopicInfoList::from(topics),
    });

    mutation.save_link(repo_link_repo_id, repo_link).unwrap();
    mutation.add_change(repo_link_repo_id, &change).unwrap();

    Ok(())
}

fn persist_links(
    mutation: &mut Mutation,
    repo_link_repo_id: RepoId,
    repo_link_meta: &LinkMetadataRow,
    parent_topics: &Vec<ParentTopicRow>,
) -> Result<()> {
    let repo_link = RepoLink {
        api_version: API_VERSION.to_string(),
        metadata: RepoLinkMetadata::try_from(repo_link_meta).unwrap(),
        parent_topics: BTreeSet::new(),
    };

    let mut repo_links = RepoLinks::new(repo_link_repo_id, repo_link);

    for parent in parent_topics {
        let parent_repo_id = RepoId::try_from(&parent.repository_id).unwrap();
        let repo_id = prefer_private_repo(repo_link_repo_id, parent_repo_id);
        let reference = repo_links.get_mut(repo_id);
        reference.parent_topics.insert(parent.try_into().unwrap());
    }

    let repo_ids = repo_links.links.keys().cloned().collect::<Vec<RepoId>>();

    for repo_id in repo_ids {
        let link = repo_links.get_mut(repo_id);
        persist_link(mutation, repo_id, link, parent_topics).unwrap();
    }

    Ok(())
}

async fn export_links(mutation: &mut Mutation, pool: &PgPool) -> Result<()> {
    log::info!("saving link blobs");

    let rows = sqlx::query_as::<_, LinkMetadataRow>(
        r#"select
            l.created_at added,
            l.repository_id,
            l.id::varchar link_id,
            l.title,
            l.url

        from links l"#,
    )
    .fetch_all(pool)
    .await?;

    for meta in rows {
        let repo_id = RepoId::try_from(&meta.repository_id).expect("failed to parse repo_id");
        let link_id = meta.link_id.clone();
        let parent_topics = sqlx::query_as::<_, ParentTopicRow>(
            r#"select
                t.repository_id,
                t.id::varchar,
                t.name,
                t.root

            from link_topics lt
            join topics t on t.id = lt.parent_id
            where lt.child_id = $1::uuid and t.repository_id = $2::uuid"#,
        )
        .bind(&link_id)
        .bind(meta.repository_id)
        .fetch_all(pool)
        .await?;

        persist_links(mutation, repo_id, &meta, &parent_topics).unwrap();
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;
    env_logger::init();
    let pool = db::db_connection(&config).await?;
    let opts = parse_args();

    if !opts.root.exists() {
        return Err(Error::NotFound(format!("{:?}", opts.root)));
    }
    let root = DataRoot::new(opts.root);
    let actor = Arc::new(Viewer::service_account());
    let client = Client::new(actor, &root, Timespec);
    let mut mutation = client.mutation(IndexMode::Replace)?;

    export_topics(&mut mutation, &pool).await.unwrap();
    export_links(&mut mutation, &pool).await.unwrap();

    log::info!("saving trees and indexes");
    mutation.write(&redis::Noop)?;

    log::info!("exported database to {}", root);
    Ok(())
}
