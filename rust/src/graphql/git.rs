use async_graphql::dataloader::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::convert::TryInto;
use std::str::FromStr;

use super::{
    Link, RepoLink, RepoTopic, Synonym, SynonymEntry, SynonymInput, Topic, TopicChild, ViewStats,
};
use crate::git;
use crate::prelude::*;

pub use git::{Client, DataRoot};

pub struct ObjectLoader {
    viewer: Viewer,
    client: git::Client,
}

impl ObjectLoader {
    pub fn new(viewer: Viewer, client: git::Client) -> Self {
        Self { viewer, client }
    }
}

#[async_trait::async_trait]
impl Loader<Oid> for ObjectLoader {
    type Value = git::Object;
    type Error = Error;

    async fn load(&self, oids: &[Oid]) -> Result<HashMap<Oid, Self::Value>> {
        log::debug!("batch load topics: {:?}", oids);
        let context = &self.viewer.context_repo_id;
        Ok(self.client.fetch_all(oids).finalize(context)?.into_hash())
    }
}

impl From<git::Object> for TopicChild {
    fn from(value: git::Object) -> Self {
        match value {
            git::Object::Link(link) => TopicChild::Link(Link(link)),
            git::Object::Topic(topic) => TopicChild::Topic(Topic(topic)),
        }
    }
}

impl TryFrom<git::Object> for Link {
    type Error = Error;

    fn try_from(value: git::Object) -> Result<Self> {
        match value {
            git::Object::Link(link) => Ok(link.into()),
            _ => Err(Error::Repo("not an object".into())),
        }
    }
}

impl TryFrom<git::Object> for Topic {
    type Error = Error;

    fn try_from(value: git::Object) -> Result<Self> {
        match value {
            git::Object::Topic(topic) => Ok(topic.into()),
            _ => Err(Error::Repo("not an object".into())),
        }
    }
}

impl TryFrom<git::SearchMatch> for TopicChild {
    type Error = Error;

    fn try_from(item: git::SearchMatch) -> Result<Self> {
        let git::SearchMatch { object, kind, .. } = item;
        let child = match kind {
            git::Kind::Topic => TopicChild::Topic(object.try_into()?),
            git::Kind::Link => TopicChild::Link(object.try_into()?),
        };
        Ok(child)
    }
}

impl<'a> From<&'a git::SynonymEntry> for SynonymEntry<'a> {
    fn from(value: &'a git::SynonymEntry) -> Self {
        Self(value)
    }
}

impl TryFrom<&SynonymInput> for git::Synonym {
    type Error = Error;

    fn try_from(value: &SynonymInput) -> Result<Self> {
        Ok(Self {
            added: chrono::Utc::now(),
            name: value.name.to_owned(),
            locale: Locale::from_str(&value.locale)?,
        })
    }
}

impl From<&git::Synonyms> for Vec<Synonym> {
    fn from(value: &git::Synonyms) -> Self {
        value.iter().map(Synonym::from).collect_vec()
    }
}

impl From<git::Synonym> for Synonym {
    fn from(synonym: git::Synonym) -> Self {
        Self(synonym)
    }
}

impl From<&git::Synonym> for Synonym {
    fn from(synonym: &git::Synonym) -> Self {
        synonym.to_owned().into()
    }
}

impl From<git::Topic> for Topic {
    fn from(value: git::Topic) -> Self {
        Self(value)
    }
}

impl TryFrom<Option<git::Topic>> for Topic {
    type Error = Error;

    fn try_from(value: Option<git::Topic>) -> Result<Self> {
        match value {
            Some(value) => Ok(value.into()),
            None => Err(Error::NotFound("no topic".into())),
        }
    }
}

impl From<&git::Topic> for Topic {
    fn from(value: &git::Topic) -> Self {
        value.to_owned().into()
    }
}

impl From<git::RepoTopicWrapper> for RepoTopic {
    fn from(value: git::RepoTopicWrapper) -> Self {
        Self(value)
    }
}

impl From<&git::RepoTopicWrapper> for RepoTopic {
    fn from(value: &git::RepoTopicWrapper) -> Self {
        value.to_owned().into()
    }
}

impl From<git::RepoLinkWrapper> for RepoLink {
    fn from(value: git::RepoLinkWrapper) -> Self {
        Self(value)
    }
}

impl From<&git::RepoLinkWrapper> for RepoLink {
    fn from(value: &git::RepoLinkWrapper) -> Self {
        value.to_owned().into()
    }
}

impl From<git::Link> for Link {
    fn from(value: git::Link) -> Self {
        Self(value)
    }
}

impl TryFrom<Option<git::Link>> for Link {
    type Error = Error;

    fn try_from(value: Option<git::Link>) -> Result<Self> {
        match value {
            Some(value) => Ok(value.into()),
            None => Err(Error::NotFound("no link".into())),
        }
    }
}

impl TryFrom<git::FetchStatsResult> for ViewStats {
    type Error = Error;

    fn try_from(value: git::FetchStatsResult) -> Result<Self> {
        let stats = &value.stats;

        let link_count = match stats.link_count().try_into() {
            Ok(count) => count,

            Err(err) => {
                log::error!("failed to convert link count: {}", err);
                0
            }
        };

        let topic_count = match stats.topic_count().try_into() {
            Ok(count) => count,

            Err(err) => {
                log::error!("failed to convert topic count: {}", err);
                0
            }
        };

        Ok(Self {
            link_count: Some(link_count),
            topic_count: Some(topic_count),
        })
    }
}
