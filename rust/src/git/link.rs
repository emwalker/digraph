use chrono::Utc;
use std::collections::{BTreeSet, HashMap, HashSet};

use crate::git::{
    activity, Kind, ParentTopic, RepoLink, RepoLinkMetadata, RepoTopic, SaveChangesForPrefix,
    TopicChild, API_VERSION,
};
use crate::http::{self, RepoUrl};
use crate::prelude::*;

use super::activity::TopicInfoList;
use super::{Mutation, RepoLinkDetails};

#[derive(Clone)]
pub struct Link(HashSet<(RepoId, Option<RepoLink>)>);

impl From<HashSet<(&RepoId, Option<RepoLink>)>> for Link {
    fn from(source: HashSet<(&RepoId, Option<RepoLink>)>) -> Self {
        let mut dest = HashSet::new();

        for (repo_id, repo_link) in source {
            dest.insert((repo_id.to_owned(), repo_link.to_owned()));
        }

        Self(dest)
    }
}

#[derive(Clone)]
pub struct Links(pub(crate) HashMap<Oid, Link>);

impl From<HashMap<&Oid, HashSet<(&RepoId, Option<RepoLink>)>>> for Links {
    fn from(source: HashMap<&Oid, HashSet<(&RepoId, Option<RepoLink>)>>) -> Self {
        let mut dest = HashMap::new();

        for (link_id, set) in source {
            dest.insert(link_id.to_owned(), Link::from(set));
        }

        Self(dest)
    }
}

impl Links {
    pub fn to_hash(self) -> HashMap<Oid, Link> {
        self.0
    }
}

pub struct DeleteLink {
    pub actor: Viewer,
    pub repo: RepoId,
    pub link_id: Oid,
}

pub struct DeleteLinkResult {
    pub alerts: Vec<Alert>,
    pub deleted_link_id: Oid,
}

impl DeleteLink {
    pub fn call<S>(&self, mut mutation: Mutation, store: &S) -> Result<DeleteLinkResult>
    where
        S: SaveChangesForPrefix,
    {
        let link = mutation.fetch_link(&self.repo, &self.link_id);
        if link.is_none() {
            return Err(Error::NotFound(format!("not found: {}", self.link_id)));
        }
        let link = link.unwrap();

        // Not actually used
        let date = chrono::Utc::now();
        let child = link.to_topic_child(date);
        let mut parent_topics = vec![];

        mutation.mark_deleted(&self.repo, &self.link_id)?;

        for ParentTopic { id: parent_id, .. } in &link.parent_topics {
            if let Some(mut parent) = mutation.fetch_topic(&self.repo, parent_id) {
                parent.children.remove(&child);
                mutation.save_topic(&self.repo, &parent)?;
                parent_topics.push(parent);
            }
        }

        let change = self.change(&link, &parent_topics, date);

        mutation.remove_link(&self.repo, &self.link_id, &link)?;
        mutation.add_change(&self.repo, &change)?;
        mutation.write(store)?;

        Ok(DeleteLinkResult {
            alerts: vec![],
            deleted_link_id: self.link_id.clone(),
        })
    }

    fn change(
        &self,
        link: &RepoLink,
        parent_topics: &Vec<RepoTopic>,
        date: Timestamp,
    ) -> activity::Change {
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
    pub repo_id: RepoId,
    pub link_id: Oid,
    pub parent_topic_ids: BTreeSet<Oid>,
}

pub struct UpdateLinkParentTopicsResult {
    pub alerts: Vec<Alert>,
    pub link: RepoLink,
}

impl UpdateLinkParentTopics {
    pub fn call<S>(&self, mut mutation: Mutation, store: &S) -> Result<UpdateLinkParentTopicsResult>
    where
        S: SaveChangesForPrefix,
    {
        self.validate()?;
        log::info!("updating parent topics for {}", self.link_id);

        let date = chrono::Utc::now();
        let link = mutation.fetch_link(&self.repo_id, &self.link_id);
        if link.is_none() {
            return Err(Error::NotFound(format!("not found: {}", self.link_id)));
        }
        let mut link = link.unwrap();

        let parent_topics = self
            .parent_topic_ids
            .iter()
            .map(|id| ParentTopic { id: id.to_owned() })
            .collect::<BTreeSet<ParentTopic>>();

        let mut added = vec![];
        for parent in parent_topics.difference(&link.parent_topics) {
            if let Some(topic) = parent.fetch(&self.repo_id, &mutation)? {
                added.push(topic);
            }
        }

        for topic in &mut added {
            topic.children.insert(link.to_topic_child(date));
            link.parent_topics.insert(topic.to_parent_topic());
        }

        let mut removed = vec![];
        for parent in link.parent_topics.difference(&parent_topics) {
            if let Some(link) = parent.fetch(&self.repo_id, &mutation)? {
                removed.push(link);
            }
        }

        for topic in &mut removed {
            topic.children.remove(&link.to_topic_child(date));
            link.parent_topics.remove(&topic.to_parent_topic());
        }

        let change = self.change(&link, &added, &removed, date);

        for topic in &added {
            mutation.save_topic(&self.repo_id, topic)?;
        }

        for topic in &removed {
            mutation.save_topic(&self.repo_id, topic)?;
        }

        mutation.save_link(&self.repo_id, &link)?;
        mutation.add_change(&self.repo_id, &change)?;
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
        link: &RepoLink,
        added: &Vec<RepoTopic>,
        removed: &Vec<RepoTopic>,
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
    pub add_parent_topic_id: Option<Oid>,
    #[derivative(Debug = "ignore")]
    pub fetcher: Box<dyn http::Fetch + Send + Sync>,
    pub repo_id: RepoId,
    pub title: Option<String>,
    pub url: String,
}

pub struct UpsertLinkResult {
    pub alerts: Vec<Alert>,
    pub link: Option<RepoLink>,
}

impl UpsertLink {
    pub fn call<S>(&self, mut mutation: Mutation, store: &S) -> Result<UpsertLinkResult>
    where
        S: SaveChangesForPrefix,
    {
        log::info!("upserting link: {}", self.url);
        let url = RepoUrl::parse(&self.url)?;
        let link_id = url.id()?;
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
                id: path.to_owned(),
            })
            .collect::<BTreeSet<ParentTopic>>();

        let link_id = link.id();
        for topic in parent_topics.values_mut() {
            topic.children.insert(TopicChild {
                added: date,
                kind: Kind::Link,
                id: link_id.to_owned(),
            });
            mutation.save_topic(&self.repo_id, topic)?;
        }

        mutation.save_link(&self.repo_id, &link)?;
        mutation.add_change(&self.repo_id, &change)?;
        mutation.write(store)?;

        Ok(UpsertLinkResult {
            alerts: vec![],
            link: Some(link),
        })
    }

    fn maybe_topic(
        &self,
        mutation: &mut Mutation,
        link: &RepoLink,
    ) -> Result<(Option<RepoTopic>, HashMap<Oid, RepoTopic>)> {
        let mut parent_topics = HashMap::new();

        for parent in &link.parent_topics {
            if let Some(topic) = mutation.fetch_topic(&self.repo_id, &parent.id) {
                parent_topics.insert(parent.id.to_owned(), topic);
            }
        }

        let topic = if let Some(topic_id) = &self.add_parent_topic_id {
            let topic = mutation.fetch_topic(&self.repo_id, topic_id);
            if let Some(topic) = &topic {
                parent_topics.insert(topic_id.to_owned(), topic.to_owned());
            }
            topic
        } else {
            None
        };

        if parent_topics.is_empty() {
            // There's a client error if we get to this point.
            log::warn!("no topic found, placing under root topic");
            if let Some(topic) = mutation.fetch_topic(&self.repo_id, &Oid::root_topic()) {
                parent_topics.insert(topic.id().to_owned(), topic);
            }
        }

        Ok((topic, parent_topics))
    }

    fn change(
        &self,
        link: &RepoLink,
        parent_topic: &Option<RepoTopic>,
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
                .map(|parent| parent.id.to_owned())
                .collect::<BTreeSet<Oid>>(),
            previous_title: previous_title.to_owned(),
            upserted_link: activity::LinkInfo::from(link),
        })
    }

    fn make_link(
        &self,
        mutation: &Mutation,
        link_id: &Oid,
        url: &RepoUrl,
    ) -> Result<(RepoLink, Option<String>)> {
        if mutation.exists(&self.repo_id, link_id)? {
            if let Some(link) = mutation.fetch_link(&self.repo_id, link_id) {
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
        if let Some(id) = &self.add_parent_topic_id {
            parent_topics.insert(ParentTopic { id: id.to_owned() });
        }

        let link = RepoLink {
            api_version: API_VERSION.into(),
            parent_topics,
            metadata: RepoLinkMetadata {
                added: chrono::Utc::now(),
                id: link_id.to_owned(),
                details: Some(RepoLinkDetails {
                    title,
                    url: url.normalized.to_owned(),
                }),
            },
        };

        Ok((link, None))
    }
}
