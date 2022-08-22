use chrono::Utc;
use std::collections::{BTreeSet, HashMap};

use crate::git::{
    activity, Kind, Link, LinkMetadata, ParentTopic, SaveChangesForPrefix, Topic, TopicChild,
    API_VERSION,
};
use crate::http::{self, RepoUrl};
use crate::prelude::*;

use super::activity::TopicInfoList;
use super::{LinkDetails, Mutation};

pub struct DeleteLink {
    pub actor: Viewer,
    pub link_path: RepoId,
}

pub struct DeleteLinkResult {
    pub alerts: Vec<Alert>,
    pub deleted_link_path: RepoId,
}

impl DeleteLink {
    pub fn call<S>(&self, mut update: Mutation, store: &S) -> Result<DeleteLinkResult>
    where
        S: SaveChangesForPrefix,
    {
        let link = update.fetch_link(&self.link_path);
        if link.is_none() {
            return Err(Error::NotFound(format!("not found: {}", self.link_path)));
        }
        let link = link.unwrap();

        // Not actually used
        let date = chrono::Utc::now();
        let child = link.to_topic_child(date);
        let mut parent_topics = vec![];

        update.mark_deleted(&self.link_path)?;

        for ParentTopic { path, .. } in &link.parent_topics {
            let path = RepoId::try_from(path)?;
            if let Some(mut parent) = update.fetch_topic(&path) {
                parent.children.remove(&child);
                update.save_topic(&path, &parent)?;
                parent_topics.push(parent);
            }
        }

        let change = self.change(&link, &parent_topics, date);

        update.remove_link(&self.link_path, &link)?;
        update.add_change(&change)?;
        update.write(store)?;

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

pub struct UpdateLinkParentTopics {
    pub actor: Viewer,
    pub link_path: RepoId,
    pub parent_topic_paths: BTreeSet<RepoId>,
}

pub struct UpdateLinkParentTopicsResult {
    pub alerts: Vec<Alert>,
    pub link: Link,
}

impl UpdateLinkParentTopics {
    pub fn call<S>(&self, mut update: Mutation, store: &S) -> Result<UpdateLinkParentTopicsResult>
    where
        S: SaveChangesForPrefix,
    {
        self.validate()?;

        let date = chrono::Utc::now();
        let link = update.fetch_link(&self.link_path);
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
            if let Some(topic) = parent.fetch(&update)? {
                added.push(topic);
            }
        }

        for topic in &mut added {
            topic.children.insert(link.to_topic_child(date));
            link.parent_topics.insert(topic.to_parent_topic());
        }

        let mut removed = vec![];
        for parent in link.parent_topics.difference(&parent_topics) {
            if let Some(link) = parent.fetch(&update)? {
                removed.push(link);
            }
        }

        for topic in &mut removed {
            topic.children.remove(&link.to_topic_child(date));
            link.parent_topics.remove(&topic.to_parent_topic());
        }

        let change = self.change(&link, &added, &removed, date);

        for topic in &added {
            update.save_topic(&topic.path()?, topic)?;
        }

        for topic in &removed {
            update.save_topic(&topic.path()?, topic)?;
        }

        update.save_link(&link.path()?, &link)?;
        update.add_change(&change)?;
        update.write(store)?;

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
    pub add_parent_topic_path: Option<RepoId>,
    #[derivative(Debug = "ignore")]
    pub fetcher: Box<dyn http::Fetch + Send + Sync>,
    pub repo: RepoName,
    pub title: Option<String>,
    pub url: String,
}

pub struct UpsertLinkResult {
    pub alerts: Vec<Alert>,
    pub link: Option<Link>,
}

impl UpsertLink {
    pub fn call<S>(&self, mut mutation: Mutation, store: &S) -> Result<UpsertLinkResult>
    where
        S: SaveChangesForPrefix,
    {
        log::info!("upserting link: {:?}", self);
        let url = RepoUrl::parse(&self.url)?;
        let link_path = url.path(&self.repo)?;
        let date = Utc::now();

        let (mut link, previous_title) = self.make_link(&mutation, &link_path, &url)?;

        if let Some(title) = &self.title {
            match &mut link.metadata.details {
                Some(extra) => {
                    extra.title = title.clone();
                }

                None => {
                    let msg = format!("tried to save the title of a reference: {:?}", self);
                    return Err(Error::Repo(msg));
                }
            }
        }

        let (topic, mut parent_topics) = self.maybe_topic(&mut mutation, &link)?;

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
            mutation.save_topic(&RepoId::try_from(path)?, topic)?;
        }

        mutation.save_link(&link_path, &link)?;
        mutation.add_change(&change)?;
        mutation.write(store)?;

        Ok(UpsertLinkResult {
            alerts: vec![],
            link: Some(link),
        })
    }

    fn maybe_topic(
        &self,
        mutation: &mut Mutation,
        link: &Link,
    ) -> Result<(Option<Topic>, HashMap<String, Topic>)> {
        let mut parent_topics = HashMap::new();

        for parent in &link.parent_topics {
            if let Some(topic) = mutation.fetch_topic(&RepoId::try_from(&parent.path)?) {
                parent_topics.insert(parent.path.to_owned(), topic);
            }
        }

        let topic = if let Some(topic_path) = &self.add_parent_topic_path {
            let topic_path = link.path()?.repo.path(&topic_path.short_id)?;
            let topic = mutation.fetch_topic(&topic_path);
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
            if let Some(topic) =
                mutation.fetch_topic(&RepoId::try_from(WIKI_ROOT_TOPIC_PATH).unwrap())
            {
                parent_topics.insert(topic.metadata.path.to_owned(), topic);
            }
        }

        Ok((topic, parent_topics))
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
        mutation: &Mutation,
        link_path: &RepoId,
        url: &RepoUrl,
    ) -> Result<(Link, Option<String>)> {
        if mutation.exists(link_path)? {
            if let Some(link) = mutation.fetch_link(link_path) {
                let title = link.metadata.title().to_owned();
                return Ok((link, Some(title)));
            }
        }

        let title = if let Some(title) = &self.title {
            title.clone()
        } else {
            let response = self.fetcher.fetch(url)?;
            response.title().unwrap_or_else(|| "Missing title".into())
        };

        let mut parent_topics = BTreeSet::new();
        if let Some(path) = &self.add_parent_topic_path {
            let path = link_path.repo.path(&path.short_id)?;
            parent_topics.insert(ParentTopic {
                path: path.to_string(),
            });
        }

        let link = Link {
            api_version: API_VERSION.into(),
            parent_topics,
            metadata: LinkMetadata {
                added: chrono::Utc::now(),
                path: link_path.to_string(),
                details: Some(LinkDetails {
                    title,
                    url: url.normalized.to_owned(),
                }),
            },
        };

        Ok((link, None))
    }
}
