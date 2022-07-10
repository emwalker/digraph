use chrono::Utc;
use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::git::{
    activity, Git, IndexMode, Indexer, Kind, Link, LinkMetadata, ParentTopic,
    SaveChangesForPrefix, Topic, TopicChild, API_VERSION,
};
use crate::http::{self, RepoUrl};
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
    pub fn call<S>(&self, git: &Git, store: &S) -> Result<DeleteLinkResult>
    where
        S: SaveChangesForPrefix,
    {
        let link = git.fetch_link(&self.link_path.inner)?;
        // Not actually used
        let date = chrono::Utc::now();
        let child = link.to_topic_child(date);
        let mut indexer = Indexer::new(git, IndexMode::Update);
        let mut parent_topics = vec![];

        for ParentTopic { path, .. } in &link.parent_topics {
            let mut parent = git.fetch_topic(path)?;
            parent.children.remove(&child);
            git.save_topic(&RepoPath::from(path), &parent, &mut indexer)?;
            parent_topics.push(parent);
        }

        let change = self.change(&link, &parent_topics, date);

        git.remove_link(&self.link_path, &link, &mut indexer)?;
        indexer.add_change(&change)?;
        indexer.save(store)?;

        Ok(DeleteLinkResult {
            alerts: vec![],
            deleted_link_path: self.link_path.clone(),
        })
    }

    fn change(&self, link: &Link, parent_topics: &Vec<Topic>, date: Timestamp) -> activity::Change {
        let mut paths = BTreeMap::from([(
            self.link_path.inner.to_owned(),
            activity::Role::DeletedLink(activity::LinkInfo {
                title: link.metadata.title.to_owned(),
                url: link.metadata.url.to_owned(),
            }),
        )]);

        for parent in parent_topics {
            paths.insert(
                parent.metadata.path.to_owned(),
                activity::Role::RemovedParentTopic(activity::TopicInfo::from(parent)),
            );
        }

        activity::Change::DeleteLink(activity::DeleteLink(activity::Body {
            date,
            paths,
            user_id: self.actor.user_id.to_owned(),
        }))
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
    pub fn call<S>(&self, git: &Git, store: &S) -> Result<UpdateLinkParentTopicsResult>
    where
        S: SaveChangesForPrefix,
    {
        self.validate()?;

        let date = chrono::Utc::now();
        let mut indexer = Indexer::new(git, IndexMode::Update);
        let mut link = git.fetch_link(&self.link_path.inner)?;

        let parent_topics = self
            .parent_topic_paths
            .iter()
            .map(|path| ParentTopic {
                path: path.to_string(),
            })
            .collect::<BTreeSet<ParentTopic>>();

        let mut added = vec![];
        for parent in parent_topics.difference(&link.parent_topics) {
            let topic = parent.fetch(git)?;
            added.push(topic);
        }

        for topic in &mut added {
            topic.children.insert(link.to_topic_child(date));
            link.parent_topics.insert(topic.to_parent_topic());
        }

        let mut removed = vec![];
        for parent in link.parent_topics.difference(&parent_topics) {
            let link = parent.fetch(git)?;
            removed.push(link);
        }

        for topic in &mut removed {
            topic.children.remove(&link.to_topic_child(date));
            link.parent_topics.remove(&topic.to_parent_topic());
        }

        let change = self.change(&link, &added, &removed, date);

        for topic in &added {
            git.save_topic(&topic.path(), topic, &mut indexer)?;
        }

        for topic in &removed {
            git.save_topic(&topic.path(), topic, &mut indexer)?;
        }

        git.save_link(&link.path(), &link, &mut indexer)?;
        indexer.add_change(&change)?;
        indexer.save(store)?;

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

    fn change(
        &self,
        link: &Link,
        added: &Vec<Topic>,
        removed: &Vec<Topic>,
        date: Timestamp,
    ) -> activity::Change {
        let mut paths = BTreeMap::from([(
            self.link_path.inner.to_owned(),
            activity::Role::UpdatedLink(activity::LinkInfo {
                title: link.metadata.title.to_owned(),
                url: link.metadata.url.to_owned(),
            }),
        )]);

        for topic in added {
            paths.insert(
                topic.metadata.path.to_owned(),
                activity::Role::AddedParentTopic(activity::TopicInfo::from(topic)),
            );
        }

        for topic in removed {
            paths.insert(
                topic.metadata.path.to_owned(),
                activity::Role::RemovedParentTopic(activity::TopicInfo::from(topic)),
            );
        }

        activity::Change::UpdateLinkParentTopics(activity::UpdateLinkParentTopics(activity::Body {
            date,
            paths,
            user_id: self.actor.user_id.to_owned(),
        }))
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
    pub async fn call<S>(&self, git: &Git, store: &S) -> Result<UpsertLinkResult>
    where
        S: SaveChangesForPrefix,
    {
        log::info!("upserting link: {:?}", self);
        let url = RepoUrl::parse(&self.url)?;
        let path = url.path(&self.prefix);
        let date = Utc::now();

        let mut link = self.make_link(git, &url, &path).await?;
        if let Some(title) = &self.title {
            link.metadata.title = title.clone();
        }

        let mut parent_topics = HashMap::new();

        for parent in &link.parent_topics {
            let topic = git.fetch_topic(&parent.path)?;
            parent_topics.insert(parent.path.to_owned(), topic);
        }

        for path in &self.add_parent_topic_paths {
            let topic = git.fetch_topic(&path.inner)?;
            parent_topics.insert(path.inner.to_owned(), topic);
        }

        if parent_topics.is_empty() {
            let topic = git.fetch_topic(WIKI_ROOT_TOPIC_PATH)?;
            parent_topics.insert(WIKI_ROOT_TOPIC_PATH.to_owned(), topic);
        }

        let change = self.change(&link, &parent_topics, date);
        link.parent_topics = parent_topics
            .iter()
            .map(|(path, _topic)| ParentTopic { path: path.into() })
            .collect::<BTreeSet<ParentTopic>>();

        let mut indexer = Indexer::new(git, IndexMode::Update);

        for (path, topic) in &mut parent_topics {
            topic.children.insert(TopicChild {
                added: date,
                kind: Kind::Link,
                path: link.metadata.path.to_owned(),
            });
            git.save_topic(&RepoPath::from(path), &topic, &mut indexer)?;
        }

        git.save_link(&path, &link, &mut indexer)?;
        indexer.add_change(&change)?;
        indexer.save(store)?;

        Ok(UpsertLinkResult {
            alerts: vec![],
            link: Some(link),
        })
    }

    fn change(
        &self,
        link: &Link,
        parent_topics: &HashMap<String, Topic>,
        date: Timestamp,
    ) -> activity::Change {
        let mut paths = BTreeMap::from([(
            link.metadata.path.to_owned(),
            activity::Role::UpdatedLink(activity::LinkInfo {
                title: link.metadata.title.to_owned(),
                url: link.metadata.url.to_owned(),
            }),
        )]);

        for (path, topic) in parent_topics {
            paths.insert(
                path.to_owned(),
                activity::Role::AddedParentTopic(activity::TopicInfo::from(&topic)),
            );
        }

        activity::Change::UpsertLink(activity::UpsertLink(activity::Body {
            date,
            paths,
            user_id: self.actor.user_id.to_owned(),
        }))
    }

    async fn make_link(&self, git: &Git, url: &RepoUrl, path: &RepoPath) -> Result<Link> {
        let link = if git.exists(path)? {
            git.fetch_link(&path.inner)?
        } else {
            let title = if let Some(title) = &self.title {
                title.clone()
            } else {
                let response = self.fetcher.fetch(url).await?;
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
                parent_topics,
                metadata: LinkMetadata {
                    added: chrono::Utc::now(),
                    path: path.to_string(),
                    title,
                    url: url.normalized.to_owned(),
                },
            }
        };

        Ok(link)
    }
}
