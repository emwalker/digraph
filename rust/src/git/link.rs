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
    pub link_id: RepoId,
}

pub struct DeleteLinkResult {
    pub alerts: Vec<Alert>,
    pub deleted_link_path: RepoId,
}

impl DeleteLink {
    pub fn call<S>(&self, mut mutation: Mutation, store: &S) -> Result<DeleteLinkResult>
    where
        S: SaveChangesForPrefix,
    {
        let link = mutation.fetch_link(&self.link_id.repo, &self.link_id);
        if link.is_none() {
            return Err(Error::NotFound(format!("not found: {}", self.link_id)));
        }
        let link = link.unwrap();

        // Not actually used
        let date = chrono::Utc::now();
        let child = link.to_topic_child(date);
        let mut parent_topics = vec![];

        mutation.mark_deleted(&self.link_id)?;

        for ParentTopic { path, .. } in &link.parent_topics {
            let parent_id = RepoId::try_from(path)?;
            if let Some(mut parent) = mutation.fetch_topic(&parent_id.repo, &parent_id) {
                parent.children.remove(&child);
                mutation.save_topic(&parent_id.repo, &parent_id, &parent)?;
                parent_topics.push(parent);
            }
        }

        let change = self.change(&link, &parent_topics, date);

        mutation.remove_link(&self.link_id, &link)?;
        mutation.add_change(&change)?;
        mutation.write(store)?;

        Ok(DeleteLinkResult {
            alerts: vec![],
            deleted_link_path: self.link_id.clone(),
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
    pub link_id: RepoId,
    pub parent_topic_ids: BTreeSet<RepoId>,
}

pub struct UpdateLinkParentTopicsResult {
    pub alerts: Vec<Alert>,
    pub link: Link,
}

impl UpdateLinkParentTopics {
    pub fn call<S>(&self, mut mutation: Mutation, store: &S) -> Result<UpdateLinkParentTopicsResult>
    where
        S: SaveChangesForPrefix,
    {
        self.validate()?;

        let date = chrono::Utc::now();
        let link = mutation.fetch_link(&self.link_id.repo, &self.link_id);
        if link.is_none() {
            return Err(Error::NotFound(format!("not found: {}", self.link_id)));
        }
        let mut link = link.unwrap();

        let parent_topics = self
            .parent_topic_ids
            .iter()
            .map(|path| ParentTopic {
                path: path.to_string(),
            })
            .collect::<BTreeSet<ParentTopic>>();

        let mut added = vec![];
        for parent in parent_topics.difference(&link.parent_topics) {
            if let Some(topic) = parent.fetch(&mutation)? {
                added.push(topic);
            }
        }

        for topic in &mut added {
            topic.children.insert(link.to_topic_child(date));
            link.parent_topics.insert(topic.to_parent_topic());
        }

        let mut removed = vec![];
        for parent in link.parent_topics.difference(&parent_topics) {
            if let Some(link) = parent.fetch(&mutation)? {
                removed.push(link);
            }
        }

        for topic in &mut removed {
            topic.children.remove(&link.to_topic_child(date));
            link.parent_topics.remove(&topic.to_parent_topic());
        }

        let change = self.change(&link, &added, &removed, date);

        for topic in &added {
            let topic_id = &topic.path()?;
            mutation.save_topic(&topic_id.repo, topic_id, topic)?;
        }

        for topic in &removed {
            let topic_id = &topic.path()?;
            mutation.save_topic(&topic_id.repo, &topic_id, topic)?;
        }

        let link_id = link.path()?;
        mutation.save_link(&link_id.repo, &link_id, &link)?;
        mutation.add_change(&change)?;
        mutation.write(store)?;

        Ok(UpdateLinkParentTopicsResult {
            alerts: vec![],
            link,
        })
    }

    fn validate(&self) -> Result<()> {
        if self.parent_topic_ids.is_empty() {
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
        let link_id = url.path(&self.repo)?;
        let date = Utc::now();

        let (mut link, previous_title) = self.make_link(&mutation, &link_id, &url)?;

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

        for (parent_id, topic) in &mut parent_topics {
            topic.children.insert(TopicChild {
                added: date,
                kind: Kind::Link,
                path: link.metadata.path.to_owned(),
            });
            let topic_id = RepoId::try_from(parent_id)?;
            mutation.save_topic(&self.repo, &topic_id, topic)?;
        }

        mutation.save_link(&self.repo, &link_id, &link)?;
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
            if let Some(topic) = mutation.fetch_topic(&self.repo, &RepoId::try_from(&parent.path)?)
            {
                parent_topics.insert(parent.path.to_owned(), topic);
            }
        }

        let topic = if let Some(topic_path) = &self.add_parent_topic_path {
            let topic_path = link.path()?.repo.path(&topic_path.short_id)?;
            let topic = mutation.fetch_topic(&self.repo, &topic_path);
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
                mutation.fetch_topic(&self.repo, &RepoId::try_from(WIKI_ROOT_TOPIC_PATH).unwrap())
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
        link_id: &RepoId,
        url: &RepoUrl,
    ) -> Result<(Link, Option<String>)> {
        if mutation.exists(&link_id.repo, link_id)? {
            if let Some(link) = mutation.fetch_link(&self.repo, link_id) {
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
            let path = link_id.repo.path(&path.short_id)?;
            parent_topics.insert(ParentTopic {
                path: path.to_string(),
            });
        }

        let link = Link {
            api_version: API_VERSION.into(),
            parent_topics,
            metadata: LinkMetadata {
                added: chrono::Utc::now(),
                path: link_id.to_string(),
                details: Some(LinkDetails {
                    title,
                    url: url.normalized.to_owned(),
                }),
            },
        };

        Ok((link, None))
    }
}
