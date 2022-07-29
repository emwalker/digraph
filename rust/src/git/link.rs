use chrono::Utc;
use std::collections::{BTreeSet, HashMap};

use crate::git::{
    activity, Client, Kind, Link, LinkMetadata, ParentTopic, SaveChangesForPrefix, Topic,
    TopicChild, API_VERSION,
};
use crate::http::{self, RepoUrl};
use crate::prelude::*;

use super::activity::TopicInfoList;
use super::BatchUpdate;

pub struct DeleteLink {
    pub actor: Viewer,
    pub link_path: RepoPath,
}

pub struct DeleteLinkResult {
    pub alerts: Vec<Alert>,
    pub deleted_link_path: RepoPath,
}

impl DeleteLink {
    pub fn call<S>(&self, mut builder: BatchUpdate, store: &S) -> Result<DeleteLinkResult>
    where
        S: SaveChangesForPrefix,
    {
        let link = builder.fetch_link(&self.link_path);
        if link.is_none() {
            return Err(Error::NotFound(format!("not found: {}", self.link_path)));
        }
        let link = link.unwrap();

        // Not actually used
        let date = chrono::Utc::now();
        let child = link.to_topic_child(date);
        let mut parent_topics = vec![];

        builder.mark_deleted(&self.link_path)?;

        for ParentTopic { path, .. } in &link.parent_topics {
            let path = RepoPath::from(path);
            if let Some(mut parent) = builder.fetch_topic(&path) {
                parent.children.remove(&child);
                builder.save_topic(&path, &parent)?;
                parent_topics.push(parent);
            }
        }

        let change = self.change(&link, &parent_topics, date);

        builder.remove_link(&self.link_path, &link)?;
        builder.add_change(&change)?;
        builder.write(store)?;

        Ok(DeleteLinkResult {
            alerts: vec![],
            deleted_link_path: self.link_path.clone(),
        })
    }

    fn change(&self, link: &Link, parent_topics: &Vec<Topic>, date: Timestamp) -> activity::Change {
        let mut deleted_link = activity::LinkInfo::from(link);
        deleted_link.deleted = true;

        activity::Change::DeleteLink(activity::DeleteLink {
            actor_id: self.actor.user_id.to_owned(),
            date,
            deleted_link,
            id: activity::Change::new_id(),
            parent_topics: activity::TopicInfoList::from(parent_topics),
        })
    }
}

pub struct FetchLinkCount {
    pub actor: Viewer,
}

pub struct FetchLinkCountResult {
    pub count: usize,
}

impl FetchLinkCount {
    pub fn call(&self, _client: &Client) -> Result<FetchLinkCountResult> {
        Ok(FetchLinkCountResult { count: 1000 })
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
    pub fn call<S>(
        &self,
        mut builder: BatchUpdate,
        store: &S,
    ) -> Result<UpdateLinkParentTopicsResult>
    where
        S: SaveChangesForPrefix,
    {
        self.validate()?;

        let date = chrono::Utc::now();
        let link = builder.fetch_link(&self.link_path);
        if link.is_none() {
            return Err(Error::NotFound(format!("not found: {}", self.link_path)));
        }
        let mut link = link.unwrap();

        let parent_topics = self
            .parent_topic_paths
            .iter()
            .map(|path| ParentTopic {
                path: path.to_string(),
            })
            .collect::<BTreeSet<ParentTopic>>();

        let mut added = vec![];
        for parent in parent_topics.difference(&link.parent_topics) {
            if let Some(topic) = parent.fetch(&builder) {
                added.push(topic);
            }
        }

        for topic in &mut added {
            topic.children.insert(link.to_topic_child(date));
            link.parent_topics.insert(topic.to_parent_topic());
        }

        let mut removed = vec![];
        for parent in link.parent_topics.difference(&parent_topics) {
            if let Some(link) = parent.fetch(&builder) {
                removed.push(link);
            }
        }

        for topic in &mut removed {
            topic.children.remove(&link.to_topic_child(date));
            link.parent_topics.remove(&topic.to_parent_topic());
        }

        let change = self.change(&link, &added, &removed, date);

        for topic in &added {
            builder.save_topic(&topic.path(), topic)?;
        }

        for topic in &removed {
            builder.save_topic(&topic.path(), topic)?;
        }

        builder.save_link(&link.path(), &link)?;
        builder.add_change(&change)?;
        builder.write(store)?;

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
        activity::Change::UpdateLinkParentTopics(activity::UpdateLinkParentTopics {
            actor_id: self.actor.user_id.to_owned(),
            added_parent_topics: activity::TopicInfoList::from(added),
            date,
            id: activity::Change::new_id(),
            updated_link: activity::LinkInfo::from(link),
            removed_parent_topics: TopicInfoList::from(removed),
        })
    }
}

// TODO: Refactor so that there's an UpsertLink mutation, which requires at least one topic, and an
// UpdateLink mutation, which has a required link path field and doesn't modify the parent topics.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct UpsertLink {
    pub actor: Viewer,
    pub add_parent_topic_path: Option<RepoPath>,
    #[derivative(Debug = "ignore")]
    pub fetcher: Box<dyn http::Fetch + Send + Sync>,
    pub repo: RepoPrefix,
    pub title: Option<String>,
    pub url: String,
}

pub struct UpsertLinkResult {
    pub alerts: Vec<Alert>,
    pub link: Option<Link>,
}

impl UpsertLink {
    pub fn call<S>(&self, mut builder: BatchUpdate, store: &S) -> Result<UpsertLinkResult>
    where
        S: SaveChangesForPrefix,
    {
        log::info!("upserting link: {:?}", self);
        let url = RepoUrl::parse(&self.url)?;
        let path = url.path(&self.repo);
        let date = Utc::now();

        let (mut link, previous_title) = self.make_link(&builder, &url, &path)?;
        if let Some(title) = &self.title {
            link.metadata.title = title.clone();
        }

        let mut parent_topics = HashMap::new();

        for parent in &link.parent_topics {
            if let Some(topic) = builder.fetch_topic(&RepoPath::from(&parent.path)) {
                parent_topics.insert(parent.path.to_owned(), topic);
            }
        }

        let topic = if let Some(topic_path) = &self.add_parent_topic_path {
            let topic = builder.fetch_topic(topic_path);
            if let Some(topic) = &topic {
                parent_topics.insert(topic_path.to_string(), topic.to_owned());
            }
            topic
        } else {
            None
        };

        if parent_topics.is_empty() {
            // There's a client error if we get to this point.
            log::warn!("no topic found, placing under root topic for /wiki");
            if let Some(topic) = builder.fetch_topic(&RepoPath::from(WIKI_ROOT_TOPIC_PATH)) {
                parent_topics.insert(topic.metadata.path.to_owned(), topic);
            }
        }

        let change = self.change(&link, &topic, &previous_title, date);
        link.parent_topics = parent_topics
            .iter()
            .map(|(path, _topic)| ParentTopic {
                path: path.to_owned(),
            })
            .collect::<BTreeSet<ParentTopic>>();

        for (path, topic) in &mut parent_topics {
            topic.children.insert(TopicChild {
                added: date,
                kind: Kind::Link,
                path: link.metadata.path.to_owned(),
            });
            builder.save_topic(&RepoPath::from(path), topic)?;
        }

        builder.save_link(&path, &link)?;
        builder.add_change(&change)?;
        builder.write(store)?;

        Ok(UpsertLinkResult {
            alerts: vec![],
            link: Some(link),
        })
    }

    fn change(
        &self,
        link: &Link,
        parent_topic: &Option<Topic>,
        previous_title: &Option<String>,
        date: Timestamp,
    ) -> activity::Change {
        let add_parent_topic = parent_topic
            .as_ref()
            .map(|topic| activity::TopicInfo::from(&topic.to_owned()));

        activity::Change::UpsertLink(activity::UpsertLink {
            add_parent_topic,
            actor_id: self.actor.user_id.to_owned(),
            date,
            id: activity::Change::new_id(),
            parent_topics: link
                .parent_topics
                .iter()
                .map(|parent| parent.path.to_owned())
                .collect::<BTreeSet<String>>(),
            previous_title: previous_title.to_owned(),
            upserted_link: activity::LinkInfo::from(link),
        })
    }

    fn make_link(
        &self,
        builder: &BatchUpdate,
        url: &RepoUrl,
        path: &RepoPath,
    ) -> Result<(Link, Option<String>)> {
        if builder.exists(path)? {
            if let Some(link) = builder.fetch_link(path) {
                let title = link.metadata.title.to_owned();
                return Ok((link, Some(title)));
            }
        }

        let title = if let Some(title) = &self.title {
            title.clone()
        } else {
            let response = futures::executor::block_on(self.fetcher.fetch(url))?;
            response.title().unwrap_or_else(|| "Missing title".into())
        };

        let mut parent_topics = BTreeSet::new();
        if let Some(path) = &self.add_parent_topic_path {
            parent_topics.insert(ParentTopic {
                path: path.to_string(),
            });
        }

        let link = Link {
            api_version: API_VERSION.into(),
            parent_topics,
            metadata: LinkMetadata {
                added: chrono::Utc::now(),
                path: path.to_string(),
                title,
                url: url.normalized.to_owned(),
            },
        };

        Ok((link, None))
    }
}
