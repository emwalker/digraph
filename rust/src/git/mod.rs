use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};

mod account;
pub mod activity;
mod checks;
mod client;
pub mod core;
mod index;
mod link;
mod repository;
mod search;
mod stats;
pub mod testing;
mod topic;

use crate::prelude::*;
use crate::types;
pub use account::*;
pub use client::*;
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
pub struct LinkDetails {
    pub title: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkMetadata {
    pub added: Timestamp,
    pub path: String,
    pub details: Option<LinkDetails>,
}

impl LinkMetadata {
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
pub struct Link {
    pub api_version: String,
    pub metadata: LinkMetadata,
    pub parent_topics: BTreeSet<ParentTopic>,
}

impl std::cmp::PartialEq for Link {
    fn eq(&self, other: &Self) -> bool {
        self.metadata.path == other.metadata.path
    }
}

impl std::cmp::Eq for Link {}

impl std::cmp::Ord for Link {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.metadata.details, &self.metadata.path)
            .cmp(&(&other.metadata.details, &other.metadata.path))
    }
}

impl std::cmp::PartialOrd for Link {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Link {
    pub fn added(&self) -> Timestamp {
        self.metadata.added
    }

    pub fn is_reference(&self) -> bool {
        self.metadata.details.is_none()
    }

    pub fn path(&self) -> Result<RepoId> {
        RepoId::try_from(&self.metadata.path)
    }

    pub fn title(&self) -> &str {
        self.metadata.title()
    }

    pub fn to_search_entry(&self) -> SearchEntry {
        SearchEntry {
            path: self.metadata.path.to_owned(),
            kind: Kind::Link,
        }
    }

    pub fn to_topic_child(&self, added: Timestamp) -> TopicChild {
        TopicChild {
            added,
            kind: Kind::Link,
            path: self.metadata.path.to_owned(),
        }
    }

    pub fn url(&self) -> &str {
        self.metadata.url()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ParentTopic {
    // RepoPath is hard to serialize and deserialize, so let's keep this a string
    pub path: String,
}

impl std::cmp::Ord for ParentTopic {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.path.cmp(&other.path)
    }
}

impl std::cmp::PartialOrd for ParentTopic {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ParentTopic {
    pub fn fetch(&self, mutation: &Mutation) -> Result<Option<Topic>> {
        let topic_id = RepoId::try_from(&self.path)?;
        Ok(mutation.fetch_topic(&topic_id.repo, &topic_id))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicChild {
    pub added: Timestamp,
    pub kind: Kind,
    pub path: String,
}

impl std::cmp::PartialEq for TopicChild {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
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
        (&self.kind, &self.path).cmp(&(&other.kind, &other.path))
    }
}

impl std::hash::Hash for TopicChild {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.path.hash(state);
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
pub struct TopicDetails {
    pub root: bool,
    pub synonyms: Vec<Synonym>,
    pub timerange: Option<Timerange>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicMetadata {
    pub added: Timestamp,
    pub path: String,
    pub details: Option<TopicDetails>,
}

impl TopicMetadata {
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
pub struct Topic {
    pub api_version: String,
    pub metadata: TopicMetadata,
    pub parent_topics: BTreeSet<ParentTopic>,
    pub children: BTreeSet<TopicChild>,
}

impl std::cmp::PartialEq for Topic {
    fn eq(&self, other: &Self) -> bool {
        self.metadata.path == other.metadata.path
    }
}

impl std::cmp::Eq for Topic {}

impl std::cmp::Ord for Topic {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.metadata.path.cmp(&other.metadata.path)
    }
}

impl std::cmp::PartialOrd for Topic {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::hash::Hash for Topic {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.metadata.path.hash(state);
    }
}

impl Topic {
    pub fn added(&self) -> Timestamp {
        self.metadata.added
    }

    pub fn has_child(&self, path: &RepoId) -> bool {
        self.children.iter().any(|child| child.path == path.inner)
    }

    pub fn name(&self, locale: Locale) -> String {
        self.prefix().format(&self.metadata.name(locale))
    }

    pub fn path(&self) -> Result<RepoId> {
        RepoId::try_from(&self.metadata.path)
    }

    fn prefix(&self) -> types::TimerangePrefix {
        match &self.metadata.details {
            Some(details) => types::TimerangePrefix::from(&details.timerange),
            None => types::TimerangePrefix::from(&None),
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

    pub fn timerange(&self) -> &Option<Timerange> {
        self.metadata.timerange()
    }

    pub fn to_parent_topic(&self) -> ParentTopic {
        ParentTopic {
            path: self.metadata.path.to_owned(),
        }
    }

    pub fn to_search_entry(&self) -> SearchEntry {
        SearchEntry {
            path: self.metadata.path.to_owned(),
            kind: Kind::Topic,
        }
    }

    pub fn to_topic_child(&self, added: Timestamp) -> TopicChild {
        TopicChild {
            added,
            kind: Kind::Topic,
            path: self.metadata.path.to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum Object {
    Topic(Topic),
    Link(Link),
}

impl Object {
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

    fn display_string(&self, locale: Locale) -> String {
        match self {
            Self::Link(link) => link.metadata.title().to_owned(),
            Self::Topic(topic) => topic.name(locale),
        }
    }

    pub fn kind(&self) -> Kind {
        match self {
            Object::Topic(_) => Kind::Topic,
            Object::Link(_) => Kind::Link,
        }
    }

    pub fn parent_topics(&self) -> &BTreeSet<ParentTopic> {
        match self {
            Object::Topic(topic) => &topic.parent_topics,
            Object::Link(link) => &link.parent_topics,
        }
    }

    fn search_string(&self, locale: Locale) -> Phrase {
        Phrase::parse(&self.display_string(locale))
    }

    pub fn to_search_match(self, locale: Locale, search: &Search) -> SearchMatch {
        let normalized = &search.normalized;
        let display_string = self.display_string(locale);
        let search_string = self.search_string(locale);

        match &self {
            Self::Link(_) => SearchMatch {
                sort_key: SortKey(Kind::Link, &search_string != normalized, display_string),
                object: self,
            },
            Self::Topic(topic) => {
                let path = &topic.metadata.path;
                let explicit_in_search = search.path_specs.iter().any(|s| &s.path.inner == path);
                SearchMatch {
                    sort_key: SortKey(
                        Kind::Topic,
                        !explicit_in_search && &search_string != normalized,
                        display_string,
                    ),
                    object: self,
                }
            }
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicReferenceMetadata {
    pub path: String,
    pub path_tracked: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicReference {
    api_version: String,
    pub metadata: TopicReferenceMetadata,
    // pub parent_topics: BTreeSet<ParentTopic>,
    pub children: BTreeSet<TopicChild>,
}

pub trait Visitor {
    fn visit_topic(&mut self, topic: &Topic) -> Result<()>;
    fn visit_link(&mut self, link: &Link) -> Result<()>;
}

#[derive(Debug)]
pub struct TopicDownsetIter {
    client: Client,
    seen: HashSet<TopicChild>,
    stack: Vec<TopicChild>,
}

impl Iterator for TopicDownsetIter {
    type Item = Topic;

    fn next(&mut self) -> Option<Self::Item> {
        log::debug!("next() with {} stack elements", self.stack.len());

        while !self.stack.is_empty() {
            match self.stack.pop() {
                Some(topic_child) => {
                    if self.seen.contains(&topic_child) {
                        log::debug!("topic already seen, skipping: {}", topic_child.path);
                        continue;
                    }
                    self.seen.insert(topic_child.clone());

                    let topic_id = match RepoId::try_from(&topic_child.path) {
                        Ok(path) => path,
                        Err(err) => {
                            log::debug!("error parsing path, skipping topic: {}", err);
                            continue;
                        }
                    };

                    if let Some(topic) = self.client.fetch_topic(&topic_id.repo, &topic_id) {
                        for child in &topic.children {
                            if child.kind != Kind::Topic {
                                break;
                            }
                            self.stack.push(child.clone());
                        }
                        log::debug!("yielding topic {}", topic_child.path);
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
    fn new(client: Client, topic: Option<Topic>) -> Self {
        let mut stack = vec![];
        if let Some(topic) = &topic {
            stack.push(topic.to_topic_child(chrono::Utc::now()));
        }

        Self {
            client,
            seen: HashSet::new(),
            stack,
        }
    }
}

#[derive(Debug)]
pub struct DownsetIter {
    iter: TopicDownsetIter,
    links: Vec<String>,
}

impl Iterator for DownsetIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.links.is_empty() {
            return self.links.pop();
        }

        if let Some(topic) = self.iter.next() {
            for child in &topic.children {
                if child.kind == Kind::Link {
                    self.links.push(child.path.to_owned());
                }
            }

            return Some(topic.metadata.path);
        }

        None
    }
}

impl DownsetIter {
    fn new(client: Client, topic: Option<Topic>) -> Self {
        Self {
            links: vec![],
            iter: TopicDownsetIter::new(client, topic),
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
        let result = parse_path("../../wiki/12/34/5678/object.yaml");
        assert!(matches!(result, Err(Error::Repo(_))));

        let (root, path) = parse_path("../../wiki/objects/12/34/5678/object.yaml").unwrap();
        assert_eq!(root.path, PathBuf::from("../.."));
        assert_eq!(path.inner, "/wiki/12345678".to_owned());

        let (root, path) = parse_path(
            "../../wiki/objects/q-/ZZ/meNzLnZvgk_QGVjqPIpSgkADx71iWZrapMTphpQ/object.yaml",
        )
        .unwrap();
        assert_eq!(root.path, PathBuf::from("../.."));
        assert_eq!(
            path.inner,
            "/wiki/q-ZZmeNzLnZvgk_QGVjqPIpSgkADx71iWZrapMTphpQ".to_owned(),
        );
    }

    #[test]
    fn topic_child_equality_ignores_timestamps() {
        let child1 = TopicChild {
            added: chrono::Utc::now(),
            path: "/wiki/00001".to_owned(),
            kind: Kind::Link,
        };

        let child2 = TopicChild {
            added: chrono::Utc::now(),
            path: "/wiki/00001".to_owned(),
            kind: Kind::Link,
        };

        let child3 = TopicChild {
            added: chrono::Utc::now(),
            path: "/wiki/00002".to_owned(),
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
            path: "/wiki/00001".to_owned(),
            kind: Kind::Link,
        };
        assert!(set.insert(&a));
        assert_eq!(set.len(), 1);

        let b = TopicChild {
            added: chrono::Utc::now(),
            path: "/wiki/00001".to_owned(),
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
