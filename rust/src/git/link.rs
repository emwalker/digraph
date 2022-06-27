use async_graphql::dataloader::*;
use chrono::Utc;
use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};

use crate::git::ParentTopic;
use crate::http::{repo_url, Response};
use crate::prelude::*;
use crate::schema::{Alert, Link, WIKI_REPOSITORY_ID, WIKI_ROOT_TOPIC_PATH};
use crate::{
    git,
    git::{Git, Indexer, API_VERSION},
};

impl From<&git::Link> for Link {
    fn from(link: &git::Link) -> Self {
        let meta = &link.metadata;
        let parent_topic_paths = link
            .parent_topics
            .iter()
            .map(|topic| RepoPath::from(&topic.path))
            .collect::<Vec<RepoPath>>();

        Self {
            path: RepoPath::from(&meta.path),
            newly_added: false,
            parent_topic_paths,
            repository_id: WIKI_REPOSITORY_ID.into(),
            viewer_review: None,
            title: meta.title.clone(),
            url: meta.url.clone(),
        }
    }
}

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
            let link = match &self.git.get(path)? {
                git::Object::Link(link) => Link::from(link),
                other => {
                    return Err(Error::Repo(format!("expected a link: {:?}", other)));
                }
            };
            map.insert(path.to_owned(), link);
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
    pub alerts: Vec<Alert>,
    pub link: Option<Link>,
}

impl UpsertLink {
    pub async fn call(&self, git: &Git) -> Result<UpsertLinkResult> {
        log::info!("upserting link: {}", self.url);
        let url = repo_url::Url::parse(&self.url)?;
        let path = url.path(&self.prefix);

        let mut link = if git.exists(&path)? {
            let object = git.get(&path.inner)?;
            match object {
                git::Object::Link(link) => link,
                other => return Err(Error::Repo(format!("expected a link: {:?}", other))),
            }
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
                .collect_vec();

            git::Link {
                api_version: API_VERSION.into(),
                kind: "Link".into(),
                parent_topics,
                metadata: git::LinkMetadata {
                    added: Utc::now(),
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
            .collect_vec();

        let mut indexer = Indexer::new(git);
        git.save_link(&path, &link, &mut indexer)?;
        indexer.save()?;

        Ok(UpsertLinkResult {
            alerts: vec![],
            link: Some(Link::from(&link)),
        })
    }
}
