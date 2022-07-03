use chrono::Utc;
use std::collections::BTreeSet;

use crate::git::{
    Git, IndexMode, Indexer, Kind, Link, LinkMetadata, ParentTopic, Topic, TopicChild, API_VERSION,
};
use crate::http::{self, repo_url};
use crate::prelude::*;
use crate::Alert;

pub struct DeleteLink {
    pub actor: Viewer,
    pub link_path: RepoPath,
}

pub struct DeleteLinkResult {
    pub alerts: Vec<Alert>,
    pub deleted_link_path: RepoPath,
}

impl DeleteLink {
    pub fn call(&self, git: &Git) -> Result<DeleteLinkResult> {
        let link = git.fetch_link(&self.link_path.inner)?;
        // Not actually used
        let added = chrono::Utc::now();
        let child = link.to_topic_child(added);
        let mut indexer = Indexer::new(git, IndexMode::Update);

        for ParentTopic { path, .. } in &link.parent_topics {
            let mut parent = git.fetch_topic(path)?;
            parent.children.remove(&child);
            git.save_topic(&RepoPath::from(path), &parent, &mut indexer)?;
        }

        git.remove_link(&self.link_path, &link, &mut indexer)?;
        indexer.save()?;

        Ok(DeleteLinkResult {
            alerts: vec![],
            deleted_link_path: self.link_path.clone(),
        })
    }
}

pub struct UpdateLinkParentTopics {
    pub actor: Viewer,
    pub link_path: RepoPath,
    pub parent_topic_paths: BTreeSet<RepoPath>,
}

pub struct UpdateLinkParentTopicsResult {
    pub alerts: Vec<Alert>,
    pub link: Link,
}

impl UpdateLinkParentTopics {
    pub fn call(&self, git: &Git) -> Result<UpdateLinkParentTopicsResult> {
        self.validate()?;

        let now = chrono::Utc::now();
        let mut indexer = Indexer::new(git, IndexMode::Update);
        let mut link = git.fetch_link(&self.link_path.inner)?;
        let mut updates: Vec<Topic> = vec![];
        let parent_topics = self
            .parent_topic_paths
            .iter()
            .map(|path| ParentTopic {
                path: path.to_string(),
            })
            .collect::<BTreeSet<ParentTopic>>();

        let added = parent_topics.difference(&link.parent_topics);
        for parent in &added.cloned().collect::<Vec<ParentTopic>>() {
            let mut topic = parent.fetch(git)?;
            topic.children.insert(link.to_topic_child(now));
            link.parent_topics.insert(parent.to_owned());
            updates.push(topic);
        }

        let deleted = link.parent_topics.difference(&parent_topics);
        for parent in &deleted.cloned().collect::<Vec<ParentTopic>>() {
            let mut topic = parent.fetch(git)?;
            topic.children.remove(&link.to_topic_child(now));
            link.parent_topics.remove(parent);
            updates.push(topic);
        }

        for topic in updates {
            git.save_topic(&topic.path(), &topic, &mut indexer)?;
        }
        git.save_link(&link.path(), &link, &mut indexer)?;
        indexer.save()?;

        Ok(UpdateLinkParentTopicsResult {
            alerts: vec![],
            link,
        })
    }

    fn validate(&self) -> Result<()> {
        if self.parent_topic_paths.is_empty() {
            return Err(Error::Repo(
                "at least one parent topic must be provided".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct UpsertLink {
    pub actor: Viewer,
    pub add_parent_topic_paths: Vec<RepoPath>,
    pub prefix: String,
    pub title: Option<String>,
    pub url: String,
    #[derivative(Debug = "ignore")]
    pub fetcher: Box<dyn http::Fetch + Send + Sync>,
}

pub struct UpsertLinkResult {
    pub alerts: Vec<Alert>,
    pub link: Option<Link>,
}

impl UpsertLink {
    pub async fn call(&self, git: &Git) -> Result<UpsertLinkResult> {
        log::info!("upserting link: {:?}", self);
        let url = repo_url::Url::parse(&self.url)?;
        let path = url.path(&self.prefix);
        let added = Utc::now();

        let mut link = if git.exists(&path)? {
            git.fetch_link(&path.inner)?
        } else {
            let title = if let Some(title) = &self.title {
                title.clone()
            } else {
                let response = self.fetcher.fetch(&url).await?;
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
                kind: Kind::Link,
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
