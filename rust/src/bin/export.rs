use geotime::Geotime;
use getopts::Options;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::collections::{BTreeSet, HashMap};
use std::env;
use std::path::{Path, PathBuf};

use digraph::config::Config;
use digraph::db;
use digraph::git::{
    activity, Client, DataRoot, IndexMode, Kind, Link, LinkDetails, LinkMetadata, Mutation,
    ParentTopic, Synonym, Topic, TopicChild, TopicDetails, TopicMetadata,
};
use digraph::prelude::*;
use digraph::redis;
use digraph::types::{sha256_base64, Timerange, TimerangePrefixFormat, Timespec};

struct Opts {
    root: PathBuf,
}

fn sha256_path(login: &str, id: &str) -> Result<RepoId> {
    let id = sha256_base64(id);
    let path = format!("/{}/{}", login, id);
    RepoId::try_from(&path)
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
    login: String,
    name: String,
    root: bool,
    synonyms: serde_json::Value,
    timerange_prefix_format: Option<String>,
    timerange_starts: Option<Timestamp>,
}

#[derive(Clone, FromRow)]
struct ParentTopicRow {
    id: String,
    login: String,
    name: String,
}

impl TryFrom<&ParentTopicRow> for ParentTopic {
    type Error = Error;

    fn try_from(row: &ParentTopicRow) -> Result<Self> {
        let path = sha256_path(&row.login, &row.id)?;
        Ok(Self { path: path.inner })
    }
}

#[derive(Clone, FromRow)]
struct LinkMetadataRow {
    added: Timestamp,
    login: String,
    link_id: String,
    title: String,
    url: String,
}

impl TryFrom<&LinkMetadataRow> for LinkMetadata {
    type Error = Error;

    fn try_from(row: &LinkMetadataRow) -> Result<Self> {
        let path = sha256_path(&row.login, &row.url)?;
        Ok(Self {
            added: row.added,
            path: path.inner,
            details: Some(LinkDetails {
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
    login: String,
    name: String,
    url: String,
}

impl TryFrom<&TopicChildRow> for TopicChild {
    type Error = Error;

    fn try_from(row: &TopicChildRow) -> Result<Self> {
        let path = match row.kind.as_str() {
            "Link" => sha256_path(&row.login, &row.url)?,
            "Topic" => sha256_path(&row.login, &row.id)?,
            _ => sha256_path(&row.login, &row.id)?,
        };

        Ok(Self {
            added: row.added,
            kind: Kind::from(&row.kind).unwrap(),
            path: path.inner,
        })
    }
}

#[derive(Debug)]
struct Topics {
    topic: Topic,
    topics: HashMap<RepoName, Topic>,
}

impl Topics {
    fn new(topic: Topic) -> Self {
        let repo = topic.path().unwrap().repo;
        let mut topics: HashMap<RepoName, Topic> = HashMap::new();
        topics.insert(repo, topic.clone());
        Self { topic, topics }
    }

    fn get_mut(&mut self, repo: &RepoName) -> &mut Topic {
        let topics = &mut self.topics;

        topics.entry(repo.to_owned()).or_insert_with(|| {
            let id = &self.topic.path().unwrap().short_id;
            let path = repo.path(id).unwrap();

            Topic {
                api_version: API_VERSION.to_string(),
                metadata: TopicMetadata {
                    added: self.topic.added(),
                    path: path.inner,
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
    repo: RepoName,
    topic: &mut Topic,
    parent_topics: &Vec<ParentTopicRow>,
    children: &Vec<TopicChildRow>,
) -> Result<()> {
    let mut child_links = vec![];
    let mut child_topics = vec![];

    for child in children {
        let TopicChildRow {
            kind,
            id,
            login,
            name,
            url,
            ..
        } = child;

        if repo.org_login() != login {
            continue;
        }

        match kind.as_str() {
            "Topic" => {
                let path = sha256_path(login, id)?;
                child_topics.push(activity::TopicInfo::from((
                    Locale::EN,
                    name.to_owned(),
                    path.inner,
                )));
            }
            "Link" => {
                let path = sha256_path(login, url)?;
                child_links.push(activity::LinkInfo {
                    title: name.to_owned(),
                    url: url.to_owned(),
                    path: path.inner,
                    deleted: false,
                });
            }
            _ => {}
        };
    }

    let mut parents = vec![];
    for topic in parent_topics {
        if topic.login != repo.org_login() {
            continue;
        }

        let path = sha256_path(&topic.login, &topic.id)?;
        parents.push(activity::TopicInfo::from((
            Locale::EN,
            topic.name.to_owned(),
            path.inner.to_owned(),
        )));
    }

    let change = activity::Change::ImportTopic(activity::ImportTopic {
        actor_id: "461c87c8-fb8f-11e8-9cbc-afde6c54d881".to_owned(),
        date: chrono::Utc::now(),
        imported_topic: activity::TopicInfo::from(&*topic),
        child_links: activity::LinkInfoList::from(&child_links),
        child_topics: activity::TopicInfoList::from(&child_topics),
        id: activity::Change::new_id(),
        parent_topics: activity::TopicInfoList::from(&parents),
    });

    mutation.save_topic(&topic.path()?, topic)?;
    mutation.add_change(&change)
}

// 1. Don't place paths for items in private repos under /wiki/
fn persist_topics(
    mutation: &mut Mutation,
    topic_path: &RepoId,
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

    let topic = Topic {
        api_version: API_VERSION.to_string(),
        metadata: TopicMetadata {
            added: meta.added,
            path: topic_path.inner.to_owned(),
            details: Some(TopicDetails {
                root: meta.root,
                synonyms: synonyms.iter().map(Synonym::from).collect(),
                timerange,
            }),
        },
        parent_topics: BTreeSet::new(),
        children: BTreeSet::new(),
    };

    let mut topics = Topics::new(topic);

    for parent in parent_topics {
        let repo = RepoName::from_login(&parent.login)?;
        let topic = topics.get_mut(&repo);
        topic.parent_topics.insert(parent.try_into()?);
    }

    for child in children {
        let parent_repo = RepoName::from_login(&child.login)?;
        let topic = topics.get_mut(&parent_repo);
        topic.children.insert(child.try_into()?);
    }

    let repos = topics.topics.keys().cloned().collect::<Vec<RepoName>>();

    for repo in repos {
        let topic = topics.get_mut(&repo);
        persist_topic(mutation, repo, topic, parent_topics, children)?;
    }

    Ok(())
}

async fn export_topics(builder: &mut Mutation, pool: &PgPool) -> Result<()> {
    log::info!("saving topic blobs");

    let rows = sqlx::query_as::<_, TopicMetadataRow>(
        r#"select
            t.name,
            o.login,
            t.id::varchar,
            t.synonyms,
            t.root,
            t.created_at added,
            tr.starts_at timerange_starts,
            tr.prefix_format timerange_prefix_format

        from topics t
        join organizations o on o.id = t.organization_id
        left join timeranges tr on tr.id = t.timerange_id"#,
    )
    .fetch_all(pool)
    .await?;

    for meta in &rows {
        let topic_path = sha256_path(&meta.login, &meta.id)?;

        let parent_topics = sqlx::query_as::<_, ParentTopicRow>(
            r#"select
                o.login,
                tt.parent_id::varchar id,
                t.name

            from topic_topics tt
            join topics t on t.id = tt.parent_id
            join organizations o on o.id = t.organization_id
            left join timeranges tr on tr.id = t.timerange_id
            where tt.child_id = $1::uuid
            order by t.name"#,
        )
        .bind(&meta.id)
        .fetch_all(pool)
        .await?;

        let children = sqlx::query_as::<_, TopicChildRow>(
            r#"(
                select
                    t.created_at added,
                    'Topic' kind,
                    o.login,
                    t.id::varchar,
                    t.name,
                    'url' as url

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
                    'Link' kind,
                    o.login,
                    l.id::varchar,
                    l.title as name,
                    l.url

                from link_topics tt
                join links l on l.id = tt.child_id
                join organizations o on o.id = l.organization_id
                where tt.parent_id = $1::uuid
                order by l.created_at desc
            )"#,
        )
        .bind(&meta.id)
        .fetch_all(pool)
        .await?;

        persist_topics(builder, &topic_path, meta, &parent_topics, &children)?;
    }

    Ok(())
}

#[derive(Debug)]
struct Links {
    link: Link,
    links: HashMap<RepoName, Link>,
}

impl Links {
    fn new(link: Link) -> Self {
        let repo = link.path().unwrap().repo;
        let mut links: HashMap<RepoName, Link> = HashMap::new();
        links.insert(repo, link.clone());
        Self { link, links }
    }

    fn get_mut(&mut self, repo: &RepoName) -> &mut Link {
        let links = &mut self.links;

        links.entry(repo.to_owned()).or_insert_with(|| {
            let id = &self.link.path().unwrap().short_id;
            let path = repo.path(id).unwrap();

            Link {
                api_version: API_VERSION.to_string(),
                metadata: LinkMetadata {
                    added: self.link.added(),
                    path: path.inner,
                    details: None,
                },
                parent_topics: BTreeSet::new(),
            }
        })
    }
}

fn persist_link(
    mutation: &mut Mutation,
    link: &mut Link,
    parent_topics: &Vec<ParentTopicRow>,
) -> Result<()> {
    let repo = link.path()?.repo;
    let mut topics = BTreeSet::new();

    for parent in parent_topics {
        if repo.org_login() != parent.login {
            continue;
        }

        let path = sha256_path(&parent.login, &parent.id).unwrap();
        topics.insert(activity::TopicInfo::from((
            Locale::EN,
            parent.name.to_owned(),
            path.inner.to_owned(),
        )));
    }

    let change = activity::Change::ImportLink(activity::ImportLink {
        actor_id: "461c87c8-fb8f-11e8-9cbc-afde6c54d881".to_owned(),
        date: chrono::Utc::now(),
        id: activity::Change::new_id(),
        imported_link: activity::LinkInfo::from(&*link),
        parent_topics: activity::TopicInfoList::from(&topics),
    });

    mutation.save_link(&link.path().unwrap(), link).unwrap();
    mutation.add_change(&change).unwrap();

    Ok(())
}

fn persist_links(
    mutation: &mut Mutation,
    meta: &LinkMetadataRow,
    parent_topics: &Vec<ParentTopicRow>,
) -> Result<()> {
    let link = Link {
        api_version: API_VERSION.to_string(),
        metadata: LinkMetadata::try_from(meta).unwrap(),
        parent_topics: BTreeSet::new(),
    };

    let mut links = Links::new(link);

    for parent in parent_topics {
        let repo = RepoName::from_login(&parent.login).unwrap();
        let link = links.get_mut(&repo);
        link.parent_topics.insert(parent.try_into().unwrap());
    }

    let repos = links.links.keys().cloned().collect::<Vec<RepoName>>();

    for repo in repos {
        let link = links.get_mut(&repo);
        persist_link(mutation, link, parent_topics).unwrap();
    }

    Ok(())
}

async fn export_links(mutation: &mut Mutation, pool: &PgPool) -> Result<()> {
    log::info!("saving link blobs");

    let rows = sqlx::query_as::<_, LinkMetadataRow>(
        r#"select
            l.created_at added,
            o.login,
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
                o.login,
                t.id::varchar,
                t.name,
                t.root

            from link_topics lt
            join topics t on t.id = lt.parent_id
            join organizations o on o.id = t.organization_id
            where lt.child_id = $1::uuid"#,
        )
        .bind(&link_id)
        .fetch_all(pool)
        .await?;

        persist_links(mutation, &meta, &parent_topics).unwrap();
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
    let client = Client::new(&Viewer::service_account(), &root, Timespec);
    let mut mutation = client.mutation(IndexMode::Replace)?;

    export_topics(&mut mutation, &pool).await.unwrap();
    export_links(&mut mutation, &pool).await.unwrap();

    log::info!("saving trees and indexes");
    mutation.write(&redis::Noop)?;

    log::info!("exported database to {}", root);
    Ok(())
}
