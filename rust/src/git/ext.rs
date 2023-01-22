use itertools::Itertools;
use lazy_static::lazy_static;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    convert::{TryFrom, TryInto},
};

use super::{
    Kind, Phrase, RepoLink, RepoLinkDetails, RepoObject, RepoTopic, RepoTopicDetails, Search,
    SearchMatch, SortKey, Synonym,
};
use crate::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct Map(BTreeMap<RepoId, RepoObject>);

impl Map {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, repo_id: RepoId, repo_obj: RepoObject) {
        self.0.insert(repo_id, repo_obj);
    }

    pub fn link_details(&self, context: RepoId) -> Result<(Vec<RepoLinkWrapper>, RepoLinkWrapper)> {
        let mut display_link: Option<RepoLinkWrapper> = match self.0.get(&context) {
            Some(repo_topic) => {
                if repo_topic.has_details() {
                    Some((context, repo_topic).try_into()?)
                } else {
                    None
                }
            }
            None => None,
        };

        let mut topics = vec![];

        for (&repo_id, object) in self.iter() {
            if display_link.is_none() && object.has_details() {
                display_link = Some((repo_id, object).try_into()?);
            }
            topics.push((repo_id, object).try_into()?);
        }

        if display_link.is_none() {
            return Err(Error::Repo("no display link".into()));
        }

        Ok((topics, display_link.unwrap()))
    }

    pub fn topic_details(
        &self,
        context: RepoId,
    ) -> Result<(Vec<RepoTopicWrapper>, RepoTopicWrapper)> {
        let mut display_topic: Option<RepoTopicWrapper> = match self.0.get(&context) {
            Some(repo_topic) => {
                if repo_topic.has_details() {
                    Some((context, repo_topic).try_into()?)
                } else {
                    None
                }
            }
            None => None,
        };

        let mut topics = vec![];

        for (&repo_id, object) in self.iter() {
            if display_topic.is_none() && object.has_details() {
                display_topic = Some((repo_id, object).try_into()?);
            }
            topics.push((repo_id, object).try_into()?);
        }

        if display_topic.is_none() {
            return Err(Error::Repo("no display topic".into()));
        }

        Ok((topics, display_topic.unwrap()))
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, RepoId, RepoObject> {
        self.0.iter()
    }
}

#[derive(Clone, Debug)]
pub enum ObjectBuilder {
    Topic { key: Okey, map: Map },
    Link { key: Okey, map: Map },
}

impl ObjectBuilder {
    fn insert(&mut self, repo_id: RepoId, repo_obj: RepoObject) {
        match self {
            Self::Topic { map, .. } => map.insert(repo_id, repo_obj),
            Self::Link { map, .. } => map.insert(repo_id, repo_obj),
        }
    }

    pub fn finalize(self, context: RepoId) -> Result<Object> {
        match self {
            Self::Topic { key, map } => {
                let (details, display_topic) = map.topic_details(context)?;
                Ok(Object::Topic(Topic {
                    key,
                    display_topic,
                    repo_topics: details,
                    _map: map,
                }))
            }

            Self::Link { key, map } => {
                let (details, display_link) = map.link_details(context)?;
                Ok(Object::Link(Link {
                    key,
                    display_link,
                    repo_links: details,
                    _map: map,
                }))
            }
        }
    }
}

#[derive(Clone, Default)]
pub struct ObjectBuilders(HashMap<Okey, ObjectBuilder>);

impl ObjectBuilders {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, key: Okey, repo_id: RepoId, repo_obj: RepoObject) {
        let builder = self
            .0
            .entry(key.to_owned())
            .or_insert_with(|| match repo_obj {
                RepoObject::Link(_) => ObjectBuilder::Link {
                    key,
                    map: Map::new(),
                },
                RepoObject::Topic(_) => ObjectBuilder::Topic {
                    key,
                    map: Map::new(),
                },
            });

        builder.insert(repo_id, repo_obj);
    }

    pub fn finalize(self) -> Result<Objects> {
        let mut map = HashMap::new();

        for (key, builder) in self.0 {
            match builder.finalize(key.1) {
                Ok(object) => {
                    map.insert(key, object);
                }

                Err(err) => {
                    log::warn!("problem finalizing {:?}: {:?}", key, err);
                }
            }
        }

        Ok(Objects(map))
    }
}

#[derive(Clone, Debug)]
pub struct Synonyms(Vec<Synonym>);

impl Synonyms {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Synonym> {
        self.0.iter()
    }
}

#[derive(Clone, Debug)]
pub struct RepoTopicWrapper {
    pub repo_topic: RepoTopic,
    pub repo_id: RepoId,
}

impl TryFrom<(RepoId, &RepoObject)> for RepoTopicWrapper {
    type Error = Error;

    fn try_from((repo_id, object): (RepoId, &RepoObject)) -> Result<Self> {
        let repo_topic: RepoTopic = object.try_into()?;
        Ok(RepoTopicWrapper {
            repo_id,
            repo_topic,
        })
    }
}

impl RepoTopicWrapper {
    pub fn child_ids(&self) -> Vec<ExternalId> {
        self.repo_topic
            .children
            .iter()
            .map(|child| child.id.to_owned())
            .collect_vec()
    }

    pub fn child_link_ids(&self) -> Vec<ExternalId> {
        self.repo_topic
            .children
            .iter()
            .flat_map(|child| match child.kind {
                Kind::Topic => None,
                Kind::Link => Some(child.id.to_owned()),
            })
            .collect_vec()
    }

    pub fn details(&self) -> Option<&RepoTopicDetails> {
        self.repo_topic.details()
    }

    pub fn parent_topic_ids(&self) -> Vec<ExternalId> {
        self.repo_topic
            .parent_topics
            .iter()
            .map(|parent| parent.id.to_owned())
            .collect_vec()
    }

    pub fn in_repo(&self, repo_id: RepoId) -> bool {
        self.repo_id == repo_id
    }

    pub fn in_wiki_repo(&self) -> bool {
        lazy_static! {
            static ref WIKI: RepoId = RepoId::wiki();
        }
        self.repo_id == *WIKI
    }

    pub fn display_name(&self, locale: Locale) -> String {
        self.repo_topic.name(locale)
    }

    pub fn display_color(&self) -> &str {
        if self.in_wiki_repo() {
            ""
        } else {
            DEFAULT_PRIVATE_COLOR
        }
    }

    pub fn synonyms(&self) -> &[Synonym] {
        self.repo_topic.synonyms()
    }

    pub fn timerange(&self) -> &Option<Timerange> {
        self.repo_topic.timerange()
    }

    pub fn topic_id(&self) -> &ExternalId {
        self.repo_topic.topic_id()
    }
}

#[derive(Clone, Debug)]
pub struct Topic {
    _map: Map,
    display_topic: RepoTopicWrapper,
    pub repo_topics: Vec<RepoTopicWrapper>,
    pub key: Okey,
}

impl TryFrom<Object> for Topic {
    type Error = Error;

    fn try_from(value: Object) -> Result<Self> {
        match value {
            Object::Topic(topic) => Ok(topic),
            _ => Err(Error::Repo(format!("not a topic: {:?}", value))),
        }
    }
}

impl TryFrom<&Object> for Topic {
    type Error = Error;

    fn try_from(value: &Object) -> Result<Self> {
        value.to_owned().try_into()
    }
}

impl TryFrom<Option<Object>> for Topic {
    type Error = Error;

    fn try_from(value: Option<Object>) -> Result<Self> {
        match value {
            Some(object) => object.try_into(),
            None => Err(Error::NotFound("object not found".into())),
        }
    }
}

impl TryFrom<Option<Topic>> for Topic {
    type Error = Error;

    fn try_from(value: Option<Topic>) -> Result<Self> {
        match value {
            Some(object) => Ok(object),
            None => Err(Error::NotFound("object not found".into())),
        }
    }
}

impl TryFrom<Option<&Object>> for Topic {
    type Error = Error;

    fn try_from(value: Option<&Object>) -> Result<Self> {
        value.to_owned().try_into()
    }
}

impl std::cmp::PartialEq for Topic {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl std::cmp::Eq for Topic {}

impl std::cmp::PartialOrd for Topic {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Topic {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // FIXME
        self.display_name(Locale::EN)
            .cmp(&other.display_name(Locale::EN))
    }
}

impl Topic {
    pub fn can_update(&self, write_repo_ids: &RepoIds) -> bool {
        self.repo_topics
            .iter()
            .any(|topic| write_repo_ids.include(topic.repo_id))
    }

    pub fn child_link_ids(&self) -> Vec<ExternalId> {
        self.repo_topics
            .iter()
            .flat_map(|topic| topic.child_link_ids())
            .collect_vec()
    }

    pub fn child_ids(&self) -> Vec<ExternalId> {
        self.repo_topics
            .iter()
            .flat_map(|topic| topic.child_ids())
            .collect_vec()
    }

    pub fn context_id(&self) -> RepoId {
        self.key.1
    }

    pub fn display_color(&self) -> &str {
        if self.display_topic.in_repo(RepoId::wiki()) {
            ""
        } else {
            DEFAULT_PRIVATE_COLOR
        }
    }

    pub fn display_name(&self, locale: Locale) -> String {
        self.display_topic.display_name(locale)
    }

    pub fn display_synonyms(&self) -> &[Synonym] {
        self.display_topic.synonyms()
    }

    pub fn parent_topic_ids(&self) -> Vec<ExternalId> {
        self.repo_topics
            .iter()
            .flat_map(|parent| parent.parent_topic_ids())
            .collect_vec()
    }
}

#[derive(Clone, Debug)]
pub struct RepoLinkWrapper {
    pub repo_id: RepoId,
    repo_link: RepoLink,
}

impl TryFrom<(RepoId, &RepoObject)> for RepoLinkWrapper {
    type Error = Error;

    fn try_from((repo_id, object): (RepoId, &RepoObject)) -> Result<Self> {
        let repo_link: RepoLink = object.try_into()?;
        Ok(RepoLinkWrapper { repo_id, repo_link })
    }
}

impl RepoLinkWrapper {
    pub fn added(&self) -> Timestamp {
        self.repo_link.added()
    }

    pub fn details(&self) -> Option<&RepoLinkDetails> {
        self.repo_link.details()
    }

    pub fn in_wiki_repo(&self) -> bool {
        self.repo_id.is_wiki()
    }

    pub fn link_id(&self) -> &ExternalId {
        self.repo_link.id()
    }

    pub fn parent_topic_ids(&self) -> Vec<ExternalId> {
        self.repo_link
            .parent_topics
            .iter()
            .map(|parent| parent.id.to_owned())
            .collect_vec()
    }

    pub fn title(&self) -> &str {
        self.repo_link.title()
    }

    pub fn url(&self) -> &str {
        self.repo_link.url()
    }
}

#[derive(Clone, Debug)]
pub struct Link {
    _map: Map,
    display_link: RepoLinkWrapper,
    pub repo_links: Vec<RepoLinkWrapper>,
    pub key: Okey,
}

impl TryFrom<Object> for Link {
    type Error = Error;

    fn try_from(value: Object) -> Result<Self> {
        match value {
            Object::Link(link) => Ok(link),
            _ => Err(Error::Repo(format!("not a link: {:?}", value))),
        }
    }
}

impl TryFrom<&Object> for Link {
    type Error = Error;

    fn try_from(value: &Object) -> Result<Self> {
        value.to_owned().try_into()
    }
}

impl TryFrom<Option<Object>> for Link {
    type Error = Error;

    fn try_from(value: Option<Object>) -> Result<Self> {
        match value {
            Some(object) => object.try_into(),
            None => Err(Error::NotFound("no link".into())),
        }
    }
}

impl TryFrom<Option<Link>> for Link {
    type Error = Error;

    fn try_from(value: Option<Link>) -> Result<Self> {
        match value {
            Some(link) => Ok(link),
            None => Err(Error::NotFound("no link".into())),
        }
    }
}

impl std::cmp::PartialEq for Link {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl std::cmp::Eq for Link {}

impl std::cmp::PartialOrd for Link {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Link {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse chronological order
        // Eventually: reverse chronological order of the date of the web page?
        other.added().cmp(&self.added())
    }
}

impl Link {
    pub fn added(&self) -> Timestamp {
        self.display_link.added()
    }

    pub fn can_update(&self, write_repo_ids: &RepoIds) -> bool {
        self.repo_links
            .iter()
            .any(|link| write_repo_ids.include(link.repo_id))
    }

    pub fn context_id(&self) -> RepoId {
        self.key.1
    }

    pub fn display_title(&self) -> &str {
        self.display_link.title()
    }

    pub fn display_url(&self) -> &str {
        self.display_link.url()
    }

    pub fn display_color(&self) -> &str {
        if self.in_repo(RepoId::wiki()) {
            ""
        } else {
            DEFAULT_PRIVATE_COLOR
        }
    }

    pub fn in_repo(&self, repo_id: RepoId) -> bool {
        self.repo_links.iter().any(|link| link.repo_id == repo_id)
    }

    pub fn parent_topic_ids(&self) -> Vec<ExternalId> {
        self.repo_links
            .iter()
            .flat_map(|parent| parent.parent_topic_ids())
            .collect_vec()
    }
}

#[derive(Clone, Debug)]
pub enum Object {
    Topic(Topic),
    Link(Link),
}

impl std::cmp::PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl std::cmp::Eq for Object {}

impl Object {
    pub fn display_string(&self, locale: Locale) -> String {
        match self {
            Self::Link(link) => link.display_title().to_owned(),
            Self::Topic(topic) => topic.display_name(locale),
        }
    }

    pub fn id(&self) -> &ExternalId {
        match self {
            Self::Topic(Topic { key, .. }) => &key.0,
            Self::Link(Link { key, .. }) => &key.0,
        }
    }

    pub fn to_search_match(self, locale: Locale, search: &Search) -> Result<SearchMatch> {
        let normalized = &search.normalized;
        let display_string = self.display_string(locale);
        let search_string = Phrase::parse(&display_string);

        match self {
            Self::Link { .. } => Ok(SearchMatch {
                sort_key: SortKey(
                    Kind::Link,
                    &search_string != normalized,
                    display_string,
                    self.id().to_owned(),
                ),
                kind: Kind::Link,
                object: self,
            }),

            Self::Topic { .. } => {
                let topic_id = self.id();
                let explicit_in_search = search.topic_specs.iter().any(|s| &s.id == topic_id);
                Ok(SearchMatch {
                    sort_key: SortKey(
                        Kind::Topic,
                        !explicit_in_search && &search_string != normalized,
                        display_string,
                        topic_id.to_owned(),
                    ),
                    kind: Kind::Topic,
                    object: self,
                })
            }
        }
    }
}

#[derive(Debug)]
pub struct Objects(HashMap<Okey, Object>);

impl Objects {
    pub fn into_matches(
        self,
        search: &Search,
        locale: Locale,
        take: usize,
    ) -> Result<BTreeSet<SearchMatch>> {
        let mut matches = BTreeSet::new();

        for (_key, object) in self.0.into_iter() {
            matches.insert(object.to_search_match(locale, search)?);
        }

        // We won't know what is at the top of the results (e.g., a matching topic) until we've
        // built the full btree set. So we wait to apply the limit until after this has been done.
        Ok(matches.into_iter().take(take).collect())
    }

    pub fn into_hash(self) -> HashMap<Okey, Object> {
        self.0
    }
}
