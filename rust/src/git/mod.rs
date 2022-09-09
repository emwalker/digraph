use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};
use std::convert::TryInto;

mod account;
pub mod activity;
mod checks;
mod client;
pub mod core;
mod external;
mod index;
mod link;
mod repository;
mod search;
mod stats;
pub mod testing;
mod topic;

use crate::prelude::*;
use crate::types as core_types;
pub use account::*;
pub use client::*;
pub use external::*;
pub use index::*;
pub use link::*;
pub use repository::*;
pub use search::*;
pub use stats::*;
pub use topic::*;

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Kind {
    Link,
    Topic,
}

impl Kind {
    pub fn from(kind: &str) -> Result<Self> {
        match kind {
            "Link" => Ok(Self::Link),
            "Topic" => Ok(Self::Topic),
            _ => Err(Error::Repo(format!("unknown kind: {}", kind))),
        }
    }
}

impl std::cmp::Ord for Kind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        match (self, &other) {
            (Self::Topic, Self::Topic) => Ordering::Equal,
            (Self::Topic, Self::Link) => Ordering::Less,
            (Self::Link, Self::Topic) => Ordering::Greater,
            (Self::Link, Self::Link) => Ordering::Equal,
        }
    }
}

impl std::cmp::PartialOrd for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RepoLinkDetails {
    pub title: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoLinkMetadata {
    pub added: Timestamp,
    pub id: Oid,
    pub details: Option<RepoLinkDetails>,
}

impl RepoLinkMetadata {
    fn title(&self) -> &str {
        if let Some(details) = &self.details {
            &details.title
        } else {
            "no title"
        }
    }

    fn url(&self) -> &str {
        if let Some(details) = &self.details {
            &details.url
        } else {
            "no url"
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub struct RepoLink {
    pub api_version: String,
    pub metadata: RepoLinkMetadata,
    pub parent_topics: BTreeSet<ParentTopic>,
}

impl std::cmp::PartialEq for RepoLink {
    fn eq(&self, other: &Self) -> bool {
        self.metadata.id == other.metadata.id
    }
}

impl std::cmp::Eq for RepoLink {}

impl std::cmp::Ord for RepoLink {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.metadata.details, &self.metadata.id)
            .cmp(&(&other.metadata.details, &other.metadata.id))
    }
}

impl std::cmp::PartialOrd for RepoLink {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::hash::Hash for RepoLink {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.metadata.id.hash(state);
    }
}

impl TryFrom<RepoObject> for RepoLink {
    type Error = Error;

    fn try_from(value: RepoObject) -> Result<Self> {
        match value {
            RepoObject::Link(link) => Ok(link),
            _ => Err(Error::NotFound("no repo link".into())),
        }
    }
}

impl RepoLink {
    pub fn added(&self) -> Timestamp {
        self.metadata.added
    }

    pub fn id(&self) -> &Oid {
        &self.metadata.id
    }

    pub fn is_reference(&self) -> bool {
        self.metadata.details.is_none()
    }

    pub fn title(&self) -> &str {
        self.metadata.title()
    }

    pub fn to_search_entry(&self) -> SearchEntry {
        SearchEntry {
            id: self.metadata.id.to_owned(),
            kind: Kind::Link,
        }
    }

    pub fn to_topic_child(&self, added: Timestamp) -> TopicChild {
        TopicChild {
            added,
            kind: Kind::Link,
            id: self.metadata.id.to_owned(),
        }
    }

    pub fn url(&self) -> &str {
        self.metadata.url()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ParentTopic {
    pub id: Oid,
}

impl std::cmp::Ord for ParentTopic {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl std::cmp::PartialOrd for ParentTopic {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ParentTopic {
    pub fn fetch(&self, repo: &RepoId, mutation: &Mutation) -> Result<Option<RepoTopic>> {
        Ok(mutation.fetch_topic(repo, &self.id))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicChild {
    pub added: Timestamp,
    pub kind: Kind,
    pub id: Oid,
}

impl std::cmp::PartialEq for TopicChild {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::cmp::Eq for TopicChild {}

impl std::cmp::PartialOrd for TopicChild {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for TopicChild {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.kind, &self.id).cmp(&(&other.kind, &other.id))
    }
}

impl std::hash::Hash for TopicChild {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.id.hash(state);
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Synonym {
    pub added: Timestamp,
    pub locale: Locale,
    pub name: String,
}

impl std::hash::Hash for Synonym {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.locale.hash(state);
        self.name.hash(state);
    }
}

impl std::cmp::PartialEq for Synonym {
    fn eq(&self, other: &Self) -> bool {
        self.locale == other.locale && self.name == other.name
    }
}

impl std::cmp::Eq for Synonym {}

impl std::cmp::Ord for Synonym {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.locale, &self.name).cmp(&(&other.locale, &other.name))
    }
}

impl std::cmp::PartialOrd for Synonym {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RepoTopicDetails {
    pub root: bool,
    pub synonyms: Vec<Synonym>,
    pub timerange: Option<Timerange>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoTopicMetadata {
    pub added: Timestamp,
    pub id: Oid,
    pub details: Option<RepoTopicDetails>,
}

impl RepoTopicMetadata {
    pub fn name(&self, locale: Locale) -> String {
        for synonym in self.synonyms() {
            if synonym.locale == locale {
                return synonym.name.clone();
            }
        }
        "Missing name".into()
    }

    fn synonyms(&self) -> &[Synonym] {
        match &self.details {
            Some(details) => &details.synonyms,
            None => &[],
        }
    }

    fn timerange(&self) -> &Option<Timerange> {
        match &self.details {
            Some(details) => &details.timerange,
            None => &None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub struct RepoTopic {
    pub api_version: String,
    pub metadata: RepoTopicMetadata,
    pub parent_topics: BTreeSet<ParentTopic>,
    pub children: BTreeSet<TopicChild>,
}

impl std::cmp::PartialEq for RepoTopic {
    fn eq(&self, other: &Self) -> bool {
        self.metadata.id == other.metadata.id
    }
}

impl std::cmp::Eq for RepoTopic {}

impl std::cmp::Ord for RepoTopic {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.metadata.id.cmp(&other.metadata.id)
    }
}

impl std::cmp::PartialOrd for RepoTopic {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::hash::Hash for RepoTopic {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.metadata.id.hash(state);
    }
}

impl RepoTopic {
    pub fn added(&self) -> Timestamp {
        self.metadata.added
    }

    pub fn has_child(&self, id: &Oid) -> bool {
        self.children.iter().any(|child| &child.id == id)
    }

    pub fn id(&self) -> &Oid {
        &self.metadata.id
    }

    pub fn name(&self, locale: Locale) -> String {
        self.prefix().format(&self.metadata.name(locale))
    }

    fn prefix(&self) -> core_types::TimerangePrefix {
        match &self.metadata.details {
            Some(details) => core_types::TimerangePrefix::from(&details.timerange),
            None => core_types::TimerangePrefix::from(&None),
        }
    }

    pub fn prefixed_synonyms(&self) -> Vec<Synonym> {
        let mut synonyms = vec![];
        let prefix = self.prefix();
        for Synonym {
            added,
            locale,
            name,
        } in self.metadata.synonyms()
        {
            synonyms.push(Synonym {
                added: *added,
                locale: *locale,
                name: prefix.format(name),
            })
        }
        synonyms
    }

    pub fn root(&self) -> bool {
        match &self.metadata.details {
            Some(details) => details.root,
            None => false,
        }
    }

    pub fn synonyms(&self) -> &[Synonym] {
        self.metadata.synonyms()
    }

    pub fn relative_url(&self) -> String {
        format!("/topics/{}", self.id())
    }

    pub fn timerange(&self) -> &Option<Timerange> {
        self.metadata.timerange()
    }

    pub fn to_parent_topic(&self) -> ParentTopic {
        ParentTopic {
            id: self.metadata.id.to_owned(),
        }
    }

    pub fn to_search_entry(&self) -> SearchEntry {
        SearchEntry {
            id: self.metadata.id.to_owned(),
            kind: Kind::Topic,
        }
    }

    pub fn to_topic_child(&self, added: Timestamp) -> TopicChild {
        TopicChild {
            added,
            kind: Kind::Topic,
            id: self.metadata.id.to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum RepoObject {
    Topic(RepoTopic),
    Link(RepoLink),
}

impl TryFrom<RepoObject> for RepoTopic {
    type Error = Error;

    fn try_from(value: RepoObject) -> Result<Self> {
        match value {
            RepoObject::Topic(topic) => Ok(topic),
            _ => Err(Error::NotFound("no repo topic".into())),
        }
    }
}

impl TryFrom<&RepoObject> for RepoLink {
    type Error = Error;

    fn try_from(value: &RepoObject) -> Result<Self> {
        value.to_owned().try_into()
    }
}

impl TryFrom<&RepoObject> for RepoTopic {
    type Error = Error;

    fn try_from(value: &RepoObject) -> Result<Self> {
        value.to_owned().try_into()
    }
}

impl RepoObject {
    pub fn accept<V>(&self, mut visitor: V) -> Result<()>
    where
        V: Visitor,
    {
        match self {
            Self::Topic(topic) => {
                visitor.visit_topic(topic)?;
            }
            Self::Link(link) => {
                visitor.visit_link(link)?;
            }
        }

        Ok(())
    }

    pub fn kind(&self) -> Kind {
        match self {
            RepoObject::Topic(_) => Kind::Topic,
            RepoObject::Link(_) => Kind::Link,
        }
    }

    pub fn parent_topics(&self) -> &BTreeSet<ParentTopic> {
        match self {
            RepoObject::Topic(topic) => &topic.parent_topics,
            RepoObject::Link(link) => &link.parent_topics,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoTopicReferenceMetadata {
    pub path: String,
    pub path_tracked: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoTopicReference {
    api_version: String,
    pub metadata: RepoTopicReferenceMetadata,
    // pub parent_topics: BTreeSet<ParentTopic>,
    pub children: BTreeSet<TopicChild>,
}

pub trait Visitor {
    fn visit_topic(&mut self, topic: &RepoTopic) -> Result<()>;
    fn visit_link(&mut self, link: &RepoLink) -> Result<()>;
}

#[derive(Debug)]
pub struct TopicDownsetIter {
    client: Client,
    repo: RepoId,
    seen: HashSet<TopicChild>,
    stack: Vec<TopicChild>,
}

impl Iterator for TopicDownsetIter {
    type Item = RepoTopic;

    fn next(&mut self) -> Option<Self::Item> {
        log::debug!("next() with {} stack elements", self.stack.len());

        while !self.stack.is_empty() {
            match self.stack.pop() {
                Some(topic_child) => {
                    if self.seen.contains(&topic_child) {
                        log::debug!("topic already seen, skipping: {}", topic_child.id);
                        continue;
                    }
                    self.seen.insert(topic_child.clone());

                    let topic_id = &topic_child.id;

                    if let Some(topic) = self.client.fetch_topic(&self.repo, topic_id) {
                        for child in &topic.children {
                            if child.kind != Kind::Topic {
                                break;
                            }
                            self.stack.push(child.clone());
                        }
                        log::debug!("yielding topic {}", topic_child.id);
                        return Some(topic);
                    };
                }

                None => {
                    log::error!("expected a topic, continuing");
                }
            }
        }

        None
    }
}

impl TopicDownsetIter {
    fn new(client: Client, repo: RepoId, topic: Option<RepoTopic>) -> Self {
        let mut stack = vec![];
        if let Some(topic) = &topic {
            stack.push(topic.to_topic_child(chrono::Utc::now()));
        }

        Self {
            client,
            seen: HashSet::new(),
            repo,
            stack,
        }
    }
}

#[derive(Debug)]
pub struct DownsetIter {
    iter: TopicDownsetIter,
    links: Vec<Oid>,
}

impl Iterator for DownsetIter {
    type Item = Oid;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.links.is_empty() {
            return self.links.pop();
        }

        if let Some(topic) = self.iter.next() {
            for child in &topic.children {
                if child.kind == Kind::Link {
                    self.links.push(child.id.to_owned());
                }
            }

            return Some(topic.id().to_owned());
        }

        None
    }
}

impl DownsetIter {
    fn new(client: Client, repo: RepoId, topic: Option<RepoTopic>) -> Self {
        Self {
            links: vec![],
            iter: TopicDownsetIter::new(client, repo, topic),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::testing::*;
    use super::*;

    #[test]
    fn parse_path_works() {
        let result = parse_path("../../12345/12/34/5678/object.yaml");
        assert!(matches!(result, Err(Error::Repo(_))));

        let (root, repo_id, oid) =
            parse_path("../../32212616-fc1b-11e8-8eda-b70af6d8d09f/objects/12/34/5678/object.yaml")
                .unwrap();
        assert_eq!(root.path, PathBuf::from("../.."));
        assert_eq!(
            repo_id.to_string(),
            "32212616-fc1b-11e8-8eda-b70af6d8d09f".to_owned()
        );
        assert_eq!(oid.to_string(), "12345678".to_owned());

        let (root, repo_id, oid) = parse_path(
            "../../32212616-fc1b-11e8-8eda-b70af6d8d09f/objects/q-/ZZ/meNzLnZvgk_QGVjqPIpSgkADx71iWZrapMTphpQ/object.yaml",
        )
        .unwrap();
        assert_eq!(root.path, PathBuf::from("../.."));
        assert_eq!(
            repo_id.to_string(),
            "32212616-fc1b-11e8-8eda-b70af6d8d09f".to_owned()
        );
        assert_eq!(
            oid.to_string(),
            "q-ZZmeNzLnZvgk_QGVjqPIpSgkADx71iWZrapMTphpQ".to_owned(),
        );
    }

    #[test]
    fn topic_child_equality_ignores_timestamps() {
        let child1 = TopicChild {
            added: chrono::Utc::now(),
            id: "00001".try_into().unwrap(),
            kind: Kind::Link,
        };

        let child2 = TopicChild {
            added: chrono::Utc::now(),
            id: "00001".try_into().unwrap(),
            kind: Kind::Link,
        };

        let child3 = TopicChild {
            added: chrono::Utc::now(),
            id: "00002".try_into().unwrap(),
            kind: Kind::Link,
        };

        assert_eq!(child1, child2);
        assert_ne!(child1, child3);
        assert_ne!(child2, child3);
    }

    #[test]
    fn deduping_topic_children() {
        let mut set = BTreeSet::new();

        let a = TopicChild {
            added: chrono::Utc::now(),
            id: "00001".try_into().unwrap(),
            kind: Kind::Link,
        };
        assert!(set.insert(&a));
        assert_eq!(set.len(), 1);

        let b = TopicChild {
            added: chrono::Utc::now(),
            id: "00001".try_into().unwrap(),
            kind: Kind::Link,
        };
        assert!(set.contains(&b));
        assert_eq!(&a, &b);

        assert!(!set.insert(&b));
    }

    #[test]
    fn topic_display_name() {
        let date = timerange_epoch();

        let mut topic = topic("Climate change");
        match &mut topic.metadata.details {
            Some(details) => {
                details.timerange = Some(Timerange {
                    starts: date.into(),
                    prefix_format: TimerangePrefixFormat::StartYear,
                });
            }

            None => {}
        }

        assert_eq!(topic.name(Locale::EN), "1970 Climate change");
    }
}
