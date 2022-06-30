use async_graphql::dataloader::*;
use chrono::Utc;
use std::collections::{BTreeSet, HashMap};

use crate::git::{
    Git, IndexMode, Indexer, Kind, Link, LinkMetadata, ParentTopic, TopicChild, API_VERSION,
};
use crate::http::{repo_url, Response};
use crate::prelude::*;

#[allow(dead_code)]
pub struct LinkLoader {
    viewer: Viewer,
    git: Git,
}

impl LinkLoader {
    pub fn new(viewer: Viewer, git: Git) -> Self {
        Self { viewer, git }
    }
}

#[async_trait::async_trait]
impl Loader<String> for LinkLoader {
    type Value = Link;
    type Error = Error;

    async fn load(&self, paths: &[String]) -> Result<HashMap<String, Self::Value>> {
        log::debug!("batch links: {:?}", paths);
        let mut map: HashMap<_, _> = HashMap::new();

        for path in paths {
            let link = &self.git.fetch_link(path)?;
            map.insert(path.to_owned(), link.to_owned());
        }

        Ok(map)
    }
}

pub trait Fetch {
    fn fetch(&self, url: &repo_url::Url) -> Result<Response>;
}

pub struct UpsertLink {
    pub actor: Viewer,
    pub add_parent_topic_paths: Vec<RepoPath>,
    pub prefix: String,
    pub title: Option<String>,
    pub url: String,
    pub fetcher: Box<dyn Fetch>,
}

pub struct UpsertLinkResult {
    pub alerts: Vec<String>,
    pub link: Option<Link>,
}

impl UpsertLink {
    pub fn call(&self, git: &Git) -> Result<UpsertLinkResult> {
        log::info!("upserting link: {}", self.url);
        let url = repo_url::Url::parse(&self.url)?;
        let path = url.path(&self.prefix);
        let added = Utc::now();

        let mut link = if git.exists(&path)? {
            git.fetch_link(&path.inner)?
        } else {
            let title = if let Some(title) = &self.title {
                title.clone()
            } else {
                let response = self.fetcher.fetch(&url)?;
                response.title().unwrap_or_else(|| "Missing title".into())
            };

            let parent_topics = self
                .add_parent_topic_paths
                .iter()
                .map(|path| ParentTopic {
                    path: path.to_string(),
                })
                .collect::<BTreeSet<ParentTopic>>();

            Link {
                api_version: API_VERSION.into(),
                kind: "Link".into(),
                parent_topics,
                metadata: LinkMetadata {
                    added,
                    path: path.to_string(),
                    title,
                    url: url.normalized,
                },
            }
        };

        if let Some(title) = &self.title {
            link.metadata.title = title.clone();
        }

        let mut parent_topics: BTreeSet<String> = BTreeSet::new();

        for topic in &link.parent_topics {
            parent_topics.insert(topic.path.to_owned());
        }

        for path in &self.add_parent_topic_paths {
            parent_topics.insert(path.to_string());
        }

        if parent_topics.is_empty() {
            parent_topics.insert(WIKI_ROOT_TOPIC_PATH.into());
        }

        link.parent_topics = parent_topics
            .iter()
            .map(|path| ParentTopic { path: path.into() })
            .collect::<BTreeSet<ParentTopic>>();

        let mut indexer = Indexer::new(git, IndexMode::Update);

        for path in &parent_topics {
            let mut topic = git.fetch_topic(path)?;
            topic.children.insert(TopicChild {
                added,
                kind: Kind::Topic,
                path: link.metadata.path.to_owned(),
            });
            git.save_topic(&RepoPath::from(path), &topic, &mut indexer)?;
        }

        git.save_link(&path, &link, &mut indexer)?;

        indexer.save()?;

        Ok(UpsertLinkResult {
            alerts: vec![],
            link: Some(link),
        })
    }
}
