use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashSet};

use super::{ChangeReference, Client, RepoLink, RepoTopic, Synonym};
use crate::prelude::*;
use crate::types::TimerangePrefix;

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct SynonymList(pub BTreeMap<Locale, String>);

impl SynonymList {
    pub fn get(&self, locale: &Locale) -> Option<&String> {
        self.0.get(locale)
    }
}

impl From<&Vec<Synonym>> for SynonymList {
    fn from(synonyms: &Vec<Synonym>) -> Self {
        let mut map = BTreeMap::new();
        for synonym in synonyms {
            map.entry(synonym.locale)
                .or_insert_with(|| synonym.name.to_owned());
        }
        Self(map)
    }
}

impl From<&HashSet<(String, Locale)>> for SynonymList {
    fn from(synonyms: &HashSet<(String, Locale)>) -> Self {
        let mut map = BTreeMap::new();
        for (name, locale) in synonyms {
            map.entry(locale.to_owned())
                .or_insert_with(|| name.to_owned());
        }
        Self(map)
    }
}

impl Default for SynonymList {
    fn default() -> Self {
        Self::new()
    }
}

impl SynonymList {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn markdown(&self) -> String {
        fn quote(name: &String) -> String {
            format!("\"{}\"", name)
        }

        match self.len() {
            0 => "[missing synonym]".to_owned(),
            1 => self.0.values().map(quote).join(""),

            2 => self.0.values().map(quote).join(" and "),

            3 => {
                let mut result = String::new();

                for (i, (_locale, name)) in self.0.iter().enumerate() {
                    match i {
                        0 => {
                            result.push_str(&quote(name));
                            result.push_str(", ");
                        }
                        1 => {
                            result.push_str(&quote(name));
                            result.push_str(" and ");
                        }
                        2 => {
                            result.push_str(&quote(name));
                        }
                        _ => unreachable!("unexpected index"),
                    }
                }

                result
            }

            _ => "several synonyms".to_owned(),
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct TopicInfo {
    pub deleted: bool,
    pub id: ExternalId,
    pub synonyms: SynonymList,
}

impl std::cmp::Ord for TopicInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl std::cmp::PartialOrd for TopicInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TopicInfo {
    fn name(&self, locale: Locale) -> String {
        if let Some(name) = self.synonyms.get(&locale) {
            return name.to_owned();
        }

        if let Some(name) = self.synonyms.get(&Locale::EN) {
            return name.to_owned();
        }

        "[missing topic]".to_owned()
    }

    fn mark_deleted(&mut self, id: &ExternalId) {
        if &self.id == id {
            self.deleted = true;
        }
    }
}

impl From<&RepoTopic> for TopicInfo {
    fn from(topic: &RepoTopic) -> Self {
        let mut synonyms = BTreeMap::new();

        for synonym in topic.metadata.synonyms() {
            if synonyms.contains_key(&synonym.locale) {
                continue;
            }
            synonyms.insert(synonym.locale, synonym.name.to_owned());
        }

        Self {
            synonyms: SynonymList(synonyms),
            id: topic.topic_id().to_owned(),
            deleted: false,
        }
    }
}

impl From<(Locale, String, ExternalId)> for TopicInfo {
    fn from(info: (Locale, String, ExternalId)) -> Self {
        let (locale, name, id) = info;
        let synonyms = SynonymList(BTreeMap::from([(locale, name)]));
        Self {
            synonyms,
            id,
            deleted: false,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct TopicInfoList(BTreeSet<TopicInfo>);

impl Default for TopicInfoList {
    fn default() -> Self {
        Self::new()
    }
}

impl TopicInfoList {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<&Vec<RepoTopic>> for TopicInfoList {
    fn from(topics: &Vec<RepoTopic>) -> Self {
        let mut set = BTreeSet::new();
        for topic in topics {
            set.insert(TopicInfo::from(topic));
        }
        Self(set)
    }
}

impl From<&Vec<TopicInfo>> for TopicInfoList {
    fn from(topics: &Vec<TopicInfo>) -> Self {
        let mut set = BTreeSet::new();
        for topic in topics {
            set.insert(topic.to_owned());
        }
        Self(set)
    }
}

impl From<&RepoTopic> for TopicInfoList {
    fn from(topic: &RepoTopic) -> Self {
        Self(BTreeSet::from([TopicInfo::from(topic)]))
    }
}

impl From<&BTreeSet<TopicInfo>> for TopicInfoList {
    fn from(set: &BTreeSet<TopicInfo>) -> Self {
        Self(set.clone())
    }
}

impl TopicInfoList {
    fn mark_deleted(&mut self, id: &ExternalId) {
        let mut updated = BTreeSet::new();
        for topic in &self.0 {
            let mut topic = topic.to_owned();
            topic.mark_deleted(id);
            updated.insert(topic);
        }
        self.0 = updated;
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct LinkInfo {
    pub deleted: bool,
    pub id: ExternalId,
    pub title: String,
    pub url: String,
}

impl std::cmp::Ord for LinkInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.id, &self.url, &self.title).cmp(&(&other.id, &other.url, &other.title))
    }
}

impl std::cmp::PartialOrd for LinkInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&RepoLink> for LinkInfo {
    fn from(link: &RepoLink) -> Self {
        Self {
            deleted: false,
            id: link.id().to_owned(),
            title: link.title().to_owned(),
            url: link.url().to_owned(),
        }
    }
}

impl LinkInfo {
    fn mark_deleted(&mut self, id: &ExternalId) {
        if &self.id == id {
            self.deleted = true;
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct LinkInfoList(pub BTreeSet<LinkInfo>);

impl LinkInfoList {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<&Vec<RepoLink>> for LinkInfoList {
    fn from(links: &Vec<RepoLink>) -> Self {
        let mut set = BTreeSet::new();
        for link in links {
            set.insert(LinkInfo::from(link));
        }
        Self(set)
    }
}

impl From<&Vec<LinkInfo>> for LinkInfoList {
    fn from(links: &Vec<LinkInfo>) -> Self {
        let mut set = BTreeSet::new();
        for link in links {
            set.insert(link.to_owned());
        }
        Self(set)
    }
}

impl LinkInfoList {
    fn mark_deleted(&mut self, id: &ExternalId) {
        let mut updated = BTreeSet::new();
        for link in &self.0 {
            let mut link = link.to_owned();
            link.mark_deleted(id);
            updated.insert(link);
        }
        self.0 = updated;
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "kind")]
pub enum Change {
    DeleteLink(DeleteLink),
    DeleteTopic(DeleteTopic),
    ImportLink(ImportLink),
    ImportTopic(ImportTopic),
    RemoveTopicTimerange(RemoveTopicTimerange),
    UpdateLinkParentTopics(UpdateLinkParentTopics),
    UpdateTopicSynonyms(UpdateTopicSynonyms),
    UpdateTopicParentTopics(UpdateTopicParentTopics),
    UpsertLink(UpsertLink),
    UpsertTopic(UpsertTopic),
    UpsertTopicTimerange(UpsertTopicTimerange),
}

impl std::cmp::Ord for Change {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse chronological order
        other.date().cmp(&self.date())
    }
}

impl std::cmp::PartialOrd for Change {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Change {
    pub fn new_id() -> ExternalId {
        ExternalId::make()
    }

    pub fn actor_id(&self) -> String {
        match self {
            Self::DeleteLink(inner) => inner.actor_id.to_owned(),
            Self::DeleteTopic(inner) => inner.actor_id.to_owned(),
            Self::ImportLink(inner) => inner.actor_id.to_owned(),
            Self::ImportTopic(inner) => inner.actor_id.to_owned(),
            Self::RemoveTopicTimerange(inner) => inner.actor_id.to_owned(),
            Self::UpdateLinkParentTopics(inner) => inner.actor_id.to_owned(),
            Self::UpdateTopicParentTopics(inner) => inner.actor_id.to_owned(),
            Self::UpdateTopicSynonyms(inner) => inner.actor_id.to_owned(),
            Self::UpsertLink(inner) => inner.actor_id.to_owned(),
            Self::UpsertTopic(inner) => inner.actor_id.to_owned(),
            Self::UpsertTopicTimerange(inner) => inner.actor_id.to_owned(),
        }
    }

    pub fn date(&self) -> Timestamp {
        match self {
            Self::DeleteLink(inner) => inner.date,
            Self::DeleteTopic(inner) => inner.date,
            Self::ImportLink(inner) => inner.date,
            Self::ImportTopic(inner) => inner.date,
            Self::RemoveTopicTimerange(inner) => inner.date,
            Self::UpdateLinkParentTopics(inner) => inner.date,
            Self::UpdateTopicParentTopics(inner) => inner.date,
            Self::UpdateTopicSynonyms(inner) => inner.date,
            Self::UpsertLink(inner) => inner.date,
            Self::UpsertTopic(inner) => inner.date,
            Self::UpsertTopicTimerange(inner) => inner.date,
        }
    }

    pub fn id(&self) -> ExternalId {
        match self {
            Self::DeleteLink(inner) => inner.id.to_owned(),
            Self::DeleteTopic(inner) => inner.id.to_owned(),
            Self::ImportLink(inner) => inner.id.to_owned(),
            Self::ImportTopic(inner) => inner.id.to_owned(),
            Self::RemoveTopicTimerange(inner) => inner.id.to_owned(),
            Self::UpdateLinkParentTopics(inner) => inner.id.to_owned(),
            Self::UpdateTopicParentTopics(inner) => inner.id.to_owned(),
            Self::UpdateTopicSynonyms(inner) => inner.id.to_owned(),
            Self::UpsertLink(inner) => inner.id.to_owned(),
            Self::UpsertTopic(inner) => inner.id.to_owned(),
            Self::UpsertTopicTimerange(inner) => inner.id.to_owned(),
        }
    }

    pub fn markdown(
        &self,
        locale: Locale,
        actor_name: &str,
        context: Option<&ExternalId>,
    ) -> String {
        use crate::git::activity::markdown::Markdown;
        match self {
            Self::DeleteLink(inner) => inner.markdown(locale, actor_name, context),
            Self::DeleteTopic(inner) => inner.markdown(locale, actor_name, context),
            Self::ImportLink(inner) => inner.markdown(locale, actor_name, context),
            Self::ImportTopic(inner) => inner.markdown(locale, actor_name, context),
            Self::RemoveTopicTimerange(inner) => inner.markdown(locale, actor_name, context),
            Self::UpdateLinkParentTopics(inner) => inner.markdown(locale, actor_name, context),
            Self::UpdateTopicParentTopics(inner) => inner.markdown(locale, actor_name, context),
            Self::UpdateTopicSynonyms(inner) => inner.markdown(locale, actor_name, context),
            Self::UpsertLink(inner) => inner.markdown(locale, actor_name, context),
            Self::UpsertTopic(inner) => inner.markdown(locale, actor_name, context),
            Self::UpsertTopicTimerange(inner) => inner.markdown(locale, actor_name, context),
        }
    }

    pub fn ids(&self) -> HashSet<&ExternalId> {
        match self {
            Self::DeleteLink(inner) => inner.ids(),
            Self::DeleteTopic(inner) => inner.ids(),
            Self::ImportLink(inner) => inner.ids(),
            Self::ImportTopic(inner) => inner.ids(),
            Self::RemoveTopicTimerange(inner) => inner.ids(),
            Self::UpdateLinkParentTopics(inner) => inner.ids(),
            Self::UpdateTopicParentTopics(inner) => inner.ids(),
            Self::UpdateTopicSynonyms(inner) => inner.ids(),
            Self::UpsertLink(inner) => inner.ids(),
            Self::UpsertTopic(inner) => inner.ids(),
            Self::UpsertTopicTimerange(inner) => inner.paths(),
        }
    }

    pub fn mark_deleted(&mut self, id: &ExternalId) {
        match self {
            Self::DeleteLink(inner) => inner.mark_deleted(id),
            Self::DeleteTopic(inner) => inner.mark_deleted(id),
            Self::ImportLink(inner) => inner.mark_deleted(id),
            Self::ImportTopic(inner) => inner.mark_deleted(id),
            Self::RemoveTopicTimerange(inner) => inner.mark_deleted(id),
            Self::UpdateLinkParentTopics(inner) => inner.mark_deleted(id),
            Self::UpdateTopicParentTopics(inner) => inner.mark_deleted(id),
            Self::UpdateTopicSynonyms(inner) => inner.mark_deleted(id),
            Self::UpsertLink(inner) => inner.mark_deleted(id),
            Self::UpsertTopic(inner) => inner.mark_deleted(id),
            Self::UpsertTopicTimerange(inner) => inner.mark_deleted(id),
        }
    }

    pub fn to_reference(&self) -> ChangeReference {
        ChangeReference::new(self)
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteLink {
    pub actor_id: String,
    pub id: ExternalId,
    pub date: Timestamp,
    pub deleted_link: LinkInfo,
    pub parent_topics: TopicInfoList,
}

impl DeleteLink {
    fn ids(&self) -> HashSet<&ExternalId> {
        let mut ids = HashSet::from([&self.deleted_link.id]);
        for topic in &self.parent_topics.0 {
            ids.insert(&topic.id);
        }
        ids
    }

    fn mark_deleted(&mut self, id: &ExternalId) {
        self.parent_topics.mark_deleted(id);
        self.deleted_link.mark_deleted(id);
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTopic {
    pub actor_id: String,
    pub id: ExternalId,
    pub child_links: LinkInfoList,
    pub child_topics: TopicInfoList,
    pub date: Timestamp,
    pub deleted_topic: TopicInfo,
    pub parent_topics: TopicInfoList,
}

impl DeleteTopic {
    fn ids(&self) -> HashSet<&ExternalId> {
        let mut paths = HashSet::from([&self.deleted_topic.id]);

        for link in &self.child_links.0 {
            paths.insert(&link.id);
        }

        for topic in &self.child_topics.0 {
            paths.insert(&topic.id);
        }

        for topic in &self.parent_topics.0 {
            paths.insert(&topic.id);
        }

        paths
    }

    fn mark_deleted(&mut self, id: &ExternalId) {
        self.deleted_topic.mark_deleted(id);
        self.child_links.mark_deleted(id);
        self.parent_topics.mark_deleted(id);
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImportLink {
    pub actor_id: String,
    pub id: ExternalId,
    pub date: Timestamp,
    pub imported_link: LinkInfo,
    pub parent_topics: TopicInfoList,
}

impl ImportLink {
    fn ids(&self) -> HashSet<&ExternalId> {
        let mut ids = HashSet::from([&self.imported_link.id]);
        for topic in &self.parent_topics.0 {
            ids.insert(&topic.id);
        }
        ids
    }

    fn mark_deleted(&mut self, id: &ExternalId) {
        self.imported_link.mark_deleted(id);
        self.parent_topics.mark_deleted(id);
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImportTopic {
    pub actor_id: String,
    pub id: ExternalId,
    pub child_links: LinkInfoList,
    pub child_topics: TopicInfoList,
    pub date: Timestamp,
    pub imported_topic: TopicInfo,
    pub parent_topics: TopicInfoList,
}

impl ImportTopic {
    fn ids(&self) -> HashSet<&ExternalId> {
        let mut ids = HashSet::from([&self.imported_topic.id]);

        for link in &self.child_links.0 {
            ids.insert(&link.id);
        }

        for topic in &self.child_topics.0 {
            ids.insert(&topic.id);
        }

        for topic in &self.parent_topics.0 {
            ids.insert(&topic.id);
        }

        ids
    }

    fn mark_deleted(&mut self, id: &ExternalId) {
        self.imported_topic.mark_deleted(id);
        self.parent_topics.mark_deleted(id);
        self.child_topics.mark_deleted(id);
        self.child_links.mark_deleted(id);
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveTopicTimerange {
    pub actor_id: String,
    pub date: Timestamp,
    pub id: ExternalId,
    pub parent_topics: BTreeSet<ExternalId>,
    // The RemoveTopicTimerange is idempotent, so the timerange might already have been removed.
    pub previous_timerange: Option<Timerange>,
    pub updated_topic: TopicInfo,
}

impl RemoveTopicTimerange {
    fn ids(&self) -> HashSet<&ExternalId> {
        HashSet::from([&self.updated_topic.id])
    }

    fn mark_deleted(&mut self, id: &ExternalId) {
        self.parent_topics.remove(id);
        self.updated_topic.mark_deleted(id);
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLinkParentTopics {
    pub actor_id: String,
    pub added_parent_topics: TopicInfoList,
    pub date: Timestamp,
    pub id: ExternalId,
    pub removed_parent_topics: TopicInfoList,
    pub updated_link: LinkInfo,
}

impl UpdateLinkParentTopics {
    fn ids(&self) -> HashSet<&ExternalId> {
        let mut ids = HashSet::from([&self.updated_link.id]);

        for topic in &self.added_parent_topics.0 {
            ids.insert(&topic.id);
        }

        for topic in &self.removed_parent_topics.0 {
            ids.insert(&topic.id);
        }

        ids
    }

    fn mark_deleted(&mut self, id: &ExternalId) {
        self.added_parent_topics.mark_deleted(id);
        self.removed_parent_topics.mark_deleted(id);
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTopicParentTopics {
    pub actor_id: String,
    pub added_parent_topics: TopicInfoList,
    pub id: ExternalId,
    pub date: Timestamp,
    pub parent_topic_ids: BTreeSet<ExternalId>,
    pub removed_parent_topics: TopicInfoList,
    pub updated_topic: TopicInfo,
}

impl UpdateTopicParentTopics {
    fn ids(&self) -> HashSet<&ExternalId> {
        let mut ids = HashSet::from([&self.updated_topic.id]);

        for topic in &self.added_parent_topics.0 {
            ids.insert(&topic.id);
        }

        for topic in &self.removed_parent_topics.0 {
            ids.insert(&topic.id);
        }

        for parent_id in &self.parent_topic_ids {
            ids.insert(parent_id);
        }

        ids
    }

    fn mark_deleted(&mut self, id: &ExternalId) {
        self.added_parent_topics.mark_deleted(id);
        self.removed_parent_topics.mark_deleted(id);
        self.updated_topic.mark_deleted(id);
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTopicSynonyms {
    pub actor_id: String,
    pub added_synonyms: SynonymList,
    pub date: Timestamp,
    pub id: ExternalId,
    pub parent_topics: BTreeSet<ExternalId>,
    pub removed_synonyms: SynonymList,
    pub reordered: bool,
    pub updated_topic: TopicInfo,
}

impl UpdateTopicSynonyms {
    fn ids(&self) -> HashSet<&ExternalId> {
        let mut ids = HashSet::from([&self.updated_topic.id]);
        for parent_id in &self.parent_topics {
            ids.insert(parent_id);
        }
        ids
    }

    fn mark_deleted(&mut self, id: &ExternalId) {
        self.updated_topic.mark_deleted(id);
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpsertLink {
    pub actor_id: String,
    pub add_parent_topic: Option<TopicInfo>,
    pub date: Timestamp,
    pub id: ExternalId,
    pub parent_topics: BTreeSet<ExternalId>,
    pub previous_title: Option<String>,
    pub upserted_link: LinkInfo,
}

impl UpsertLink {
    fn ids(&self) -> HashSet<&ExternalId> {
        let mut ids = HashSet::from([&self.upserted_link.id]);

        if let Some(topic) = &self.add_parent_topic {
            ids.insert(&topic.id);
        }

        for parent_id in &self.parent_topics {
            ids.insert(parent_id);
        }

        ids
    }

    fn mark_deleted(&mut self, id: &ExternalId) {
        if let Some(topic) = &mut self.add_parent_topic {
            topic.mark_deleted(id);
        }

        self.upserted_link.mark_deleted(id);
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpsertTopic {
    pub actor_id: String,
    pub id: ExternalId,
    pub date: Timestamp,
    pub parent_topic: TopicInfo,
    pub parent_topic_ids: BTreeSet<ExternalId>,
    pub upserted_topic: TopicInfo,
}

impl UpsertTopic {
    fn ids(&self) -> HashSet<&ExternalId> {
        let mut ids = HashSet::from([&self.upserted_topic.id, &self.parent_topic.id]);
        for parent_id in &self.parent_topic_ids {
            ids.insert(parent_id);
        }
        ids
    }

    fn mark_deleted(&mut self, path: &ExternalId) {
        self.parent_topic.mark_deleted(path);
        self.upserted_topic.mark_deleted(path);
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpsertTopicTimerange {
    pub actor_id: String,
    pub id: ExternalId,
    pub date: Timestamp,
    pub previous_timerange: Option<Timerange>,
    // Show change logs under parent topics as well
    pub parent_topics: BTreeSet<ExternalId>,
    pub updated_timerange: Timerange,
    pub updated_topic: TopicInfo,
}

impl UpsertTopicTimerange {
    fn paths(&self) -> HashSet<&ExternalId> {
        let mut ids = HashSet::from([&self.updated_topic.id]);
        for parent_id in &self.parent_topics {
            ids.insert(parent_id);
        }
        ids
    }

    fn mark_deleted(&mut self, id: &ExternalId) {
        let parents = &mut self.parent_topics;
        parents.remove(id);
        self.updated_topic.mark_deleted(id);
    }
}

mod markdown {
    use super::*;

    pub trait Markdown {
        fn markdown(
            &self,
            locale: Locale,
            actor_name: &str,
            context: Option<&ExternalId>,
        ) -> String;
    }

    impl LinkInfo {
        fn markdown(&self) -> String {
            if self.deleted {
                // TODO: Add ~~strikethrough~~ markup once the JS client supports it
                format!("[{}]({}) (deleted)", self.title, self.url)
            } else {
                format!("[{}]({})", self.title, self.url)
            }
        }
    }

    impl TopicInfo {
        fn markdown(&self, locale: Locale) -> String {
            if self.deleted {
                // TODO: Add ~~strikethrough~~ markup once the JS client supports it
                format!(r#""{}" (deleted)"#, self.name(locale))
            } else {
                format!("[{}](/topics/{})", self.name(locale), self.id)
            }
        }
    }

    impl TopicInfoList {
        fn markdown(&self, locale: Locale) -> String {
            let topics = &self.0.iter().collect::<Vec<&TopicInfo>>();

            match topics.len() {
                0 => "".to_owned(),
                1 => topics.iter().map(|topic| topic.markdown(locale)).join(""),
                2 => topics
                    .iter()
                    .map(|topic| topic.markdown(locale))
                    .join(" and "),
                3 => {
                    let mut markdown = topics
                        .get(0..topics.len() - 1)
                        .unwrap_or_default()
                        .iter()
                        .map(|topic| topic.markdown(locale))
                        .join(", ");

                    match topics.last() {
                        Some(topic) => {
                            markdown.push_str(" and ");
                            markdown.push_str(&topic.markdown(locale));
                            markdown
                        }
                        None => "[missing topic]".to_owned(),
                    }
                }
                _ => "a number of topics".to_owned(),
            }
        }
    }

    impl LinkInfoList {
        fn markdown(&self) -> String {
            let links = &self.0;

            match links.len() {
                0 => "".to_owned(),
                1 => links.iter().map(LinkInfo::markdown).join(""),
                2 => links.iter().map(LinkInfo::markdown).join(" and "),
                _ => "a number of links".to_owned(),
            }
        }
    }

    impl Markdown for DeleteLink {
        fn markdown(
            &self,
            locale: Locale,
            actor_name: &str,
            _context: Option<&ExternalId>,
        ) -> String {
            if self.parent_topics.is_empty() {
                format!("{} deleted {}", actor_name, self.deleted_link.markdown())
            } else {
                format!(
                    "{} deleted {}, removing it from {}",
                    actor_name,
                    self.deleted_link.markdown(),
                    self.parent_topics.markdown(locale),
                )
            }
        }
    }

    impl Markdown for DeleteTopic {
        fn markdown(
            &self,
            locale: Locale,
            actor_name: &str,
            _context: Option<&ExternalId>,
        ) -> String {
            let mut markdown = format!(
                "{} deleted {}",
                actor_name,
                self.deleted_topic.markdown(locale)
            );
            let mut children = String::new();

            match (self.child_links.is_empty(), self.child_topics.is_empty()) {
                (true, true) => {}
                (false, true) => {
                    children.push_str(&self.child_links.markdown());
                }
                (true, false) => {
                    children.push_str(&self.child_topics.markdown(locale));
                }
                (false, false) => {
                    children.push_str(&self.child_links.markdown());
                    children.push_str(" and ");
                    children.push_str(&self.child_topics.markdown(locale));
                }
            }

            match (children.is_empty(), self.parent_topics.is_empty()) {
                (true, true) => {}
                (true, false) => {
                    markdown.push_str(", removing it from ");
                    markdown.push_str(&self.parent_topics.markdown(locale));
                }
                (false, true) => {
                    markdown.push_str(", removing ");
                    markdown.push_str(&children);
                    markdown.push_str(" from it");
                }
                (false, false) => {
                    markdown.push_str(", removing ");
                    markdown.push_str(&children);
                    markdown.push_str(" from it, and removing it from ");
                    markdown.push_str(&self.parent_topics.markdown(locale));
                }
            }

            markdown
        }
    }

    impl Markdown for ImportLink {
        fn markdown(
            &self,
            locale: Locale,
            actor_name: &str,
            _context: Option<&ExternalId>,
        ) -> String {
            format!(
                "{} added {} to {}",
                actor_name,
                self.imported_link.markdown(),
                self.parent_topics.markdown(locale),
            )
        }
    }

    impl Markdown for ImportTopic {
        fn markdown(
            &self,
            locale: Locale,
            actor_name: &str,
            _context: Option<&ExternalId>,
        ) -> String {
            let mut children = vec![];

            if !self.child_topics.is_empty() {
                children.push(self.child_topics.markdown(locale));
            }

            if !self.child_links.is_empty() {
                children.push(self.child_links.markdown());
            }
            let children = children.join(" and ");

            let markdown = match (children.is_empty(), self.parent_topics.is_empty()) {
                (true, true) => {
                    format!(
                        "updated {}, but the details are missing",
                        self.imported_topic.markdown(locale)
                    )
                }

                (true, false) => format!(
                    "added {} to {}",
                    self.imported_topic.markdown(locale),
                    self.parent_topics.markdown(locale),
                ),

                (false, true) => format!(
                    "added {} to {}",
                    children,
                    self.imported_topic.markdown(locale)
                ),

                (false, false) => {
                    let topic = self.imported_topic.markdown(locale);
                    format!(
                        "added {} to {}, and added {} to {}",
                        children,
                        topic,
                        topic,
                        self.parent_topics.markdown(locale),
                    )
                }
            };

            format!("{} {}", actor_name, markdown)
        }
    }

    impl Markdown for RemoveTopicTimerange {
        fn markdown(
            &self,
            locale: Locale,
            actor_name: &str,
            _context: Option<&ExternalId>,
        ) -> String {
            let topic = self.updated_topic.markdown(locale);

            match &self.previous_timerange {
                Some(timerange) => {
                    let prefix = TimerangePrefix::from(timerange);
                    match prefix.prefix() {
                        Some(prefix) => {
                            format!(
                                r#"{} removed the time prefix "{}" from {}"#,
                                actor_name, prefix, topic,
                            )
                        }
                        None => {
                            format!(
                                r#"{} removed the start time from {}, but no change will be seen"#,
                                actor_name, topic,
                            )
                        }
                    }
                }
                None => {
                    format!(
                        r#"{} removed the start time from {}, but it was already blank"#,
                        actor_name, topic,
                    )
                }
            }
        }
    }

    impl Markdown for UpdateLinkParentTopics {
        fn markdown(
            &self,
            locale: Locale,
            actor_name: &str,
            _context: Option<&ExternalId>,
        ) -> String {
            if self.added_parent_topics.is_empty() && self.removed_parent_topics.is_empty() {
                return format!(
                    "{} updated {}, but information about the change has been lost",
                    actor_name,
                    self.updated_link.markdown()
                );
            }

            let mut markdown = actor_name.to_owned();
            markdown.push(' ');
            let mut changes = vec![];

            if !self.added_parent_topics.is_empty() {
                changes.push(format!(
                    "added {} to",
                    self.added_parent_topics.markdown(locale)
                ));
            }

            if !self.removed_parent_topics.is_empty() {
                changes.push(format!(
                    "removed {} from",
                    self.removed_parent_topics.markdown(locale)
                ));
            }

            let changes = changes.join(" and ");

            markdown.push_str(&changes);
            markdown.push(' ');
            markdown.push_str(&self.updated_link.markdown());
            markdown
        }
    }

    impl Markdown for UpdateTopicParentTopics {
        fn markdown(
            &self,
            locale: Locale,
            actor_name: &str,
            _context: Option<&ExternalId>,
        ) -> String {
            let mut actions = vec![];
            let updated_topic = self.updated_topic.markdown(locale);

            if !self.added_parent_topics.is_empty() {
                actions.push(format!(
                    "placed {} under {}",
                    updated_topic,
                    self.added_parent_topics.markdown(locale)
                ));
            }

            if !self.removed_parent_topics.is_empty() {
                actions.push(format!(
                    "removed {} from {}",
                    updated_topic,
                    self.removed_parent_topics.markdown(locale)
                ));
            }

            if actions.is_empty() {
                actions.push(format!("mysteriously updated {}", updated_topic));
            }

            format!("{} {}", actor_name, actions.join(" and "))
        }
    }

    impl Markdown for UpdateTopicSynonyms {
        fn markdown(
            &self,
            locale: Locale,
            actor_name: &str,
            _context: Option<&ExternalId>,
        ) -> String {
            let mut actions = vec![];

            if !self.added_synonyms.is_empty() {
                let markdown = format!("added {} to", self.added_synonyms.markdown());
                actions.push(markdown);
            }

            if !self.removed_synonyms.is_empty() {
                let markdown = format!("removed {} from", self.removed_synonyms.markdown());
                actions.push(markdown);
            }

            if actions.is_empty() {
                if self.reordered {
                    actions.push("reordered the synonyms for".to_owned());
                } else {
                    actions.push("mysteriousy updated".to_owned());
                }
            }

            format!(
                "{} {} {}",
                actor_name,
                actions.join(" and "),
                self.updated_topic.markdown(locale)
            )
        }
    }

    impl Markdown for UpsertLink {
        fn markdown(
            &self,
            locale: Locale,
            actor_name: &str,
            _context: Option<&ExternalId>,
        ) -> String {
            match &self.add_parent_topic {
                Some(topic) => format!(
                    "{} added {} to {}",
                    actor_name,
                    self.upserted_link.markdown(),
                    topic.markdown(locale)
                ),
                None => {
                    if let Some(title) = &self.previous_title {
                        format!(
                            r#"{} changed the title of [{}]({}) to "{}""#,
                            actor_name, title, self.upserted_link.url, self.upserted_link.title,
                        )
                    } else {
                        format!("{} updated {}", actor_name, self.upserted_link.markdown())
                    }
                }
            }
        }
    }

    impl Markdown for UpsertTopic {
        fn markdown(
            &self,
            locale: Locale,
            actor_name: &str,
            _context: Option<&ExternalId>,
        ) -> String {
            format!(
                "{} added {} to {}",
                actor_name,
                self.upserted_topic.markdown(locale),
                self.parent_topic.markdown(locale),
            )
        }
    }

    impl Markdown for UpsertTopicTimerange {
        fn markdown(
            &self,
            locale: Locale,
            actor_name: &str,
            _context: Option<&ExternalId>,
        ) -> String {
            let topic = self.updated_topic.markdown(locale);
            let prefix = TimerangePrefix::from(&self.updated_timerange);

            match prefix.prefix() {
                Some(prefix) => {
                    format!(
                        r#"{} updated the time prefix for {} to be "{}""#,
                        actor_name, topic, prefix
                    )
                }
                None => format!(
                    "{} updated the start time for {} to be {}, but it will not be shown",
                    actor_name,
                    topic,
                    prefix.date_string()
                ),
            }
        }
    }
}

pub struct FetchActivity {
    pub actor: Viewer,
    pub first: usize,
    pub path: Option<(RepoId, ExternalId)>,
}

pub struct FetchActivityResult {
    pub changes: Vec<Change>,
}

pub trait ActivityForPrefix {
    fn fetch_activity(&self, prefix: &RepoId, first: usize) -> Result<Vec<Change>>;
}

impl FetchActivity {
    pub fn call<F>(&self, git: &Client, fetch: &F) -> Result<FetchActivityResult>
    where
        F: ActivityForPrefix,
    {
        let changes = match &self.path {
            Some((repo, id)) => git.fetch_activity(repo, id, self.first)?,

            // Fetch the top-level activity feed from Redis rather than Git so as to avoid
            // write contention on a single file for every update.  This could show up in the form
            // of merge conflicts when commits are being saved to Git.
            None => fetch.fetch_activity(&RepoId::wiki(), self.first)?,
        };

        Ok(FetchActivityResult { changes })
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use std::collections::BTreeSet;

    use super::super::testing::*;
    use super::*;
    use crate::git::RepoTopic;
    use crate::prelude::Locale;

    #[test]
    fn update_link_parent_topics() {
        let topic1 = topic("Climate change");
        let topic2 = topic("Weather");

        let change = Change::UpdateLinkParentTopics(UpdateLinkParentTopics {
            actor_id: "2".to_owned(),
            date: chrono::Utc::now(),
            id: Change::new_id(),
            updated_link: LinkInfo {
                deleted: false,
                id: "some-path".try_into().unwrap(),
                title: "Reddit".to_owned(),
                url: "http://www.reddit.com".to_owned(),
            },
            added_parent_topics: TopicInfoList(BTreeSet::from([TopicInfo::from(&topic1)])),
            removed_parent_topics: TopicInfoList(BTreeSet::from([TopicInfo::from(&topic2)])),
        });

        let markdown = format!(
            "Gnusto added [Climate change]({}) to and removed [Weather]({}) from \
            [Reddit](http://www.reddit.com)",
            topic1.relative_url(),
            topic2.relative_url()
        );

        let context = ExternalId::try_from("00001").unwrap();
        assert_eq!(
            change.markdown(Locale::EN, "Gnusto", Some(&context)),
            markdown
        );
    }

    #[test]
    fn delete_link() {
        let link = link("Reddit", "http://www.reddit.com");
        let topic1 = topic("Climate change");
        let topic2 = topic("Weather");

        let change = Change::DeleteLink(DeleteLink {
            actor_id: "2".to_owned(),
            date: chrono::Utc::now(),
            deleted_link: LinkInfo {
                deleted: false,
                id: link.id().to_owned(),
                title: link.title().to_owned(),
                url: link.url().to_owned(),
            },
            id: Change::new_id(),
            parent_topics: TopicInfoList(BTreeSet::from([
                TopicInfo::from(&topic1),
                TopicInfo::from(&topic2),
            ])),
        });

        assert_eq!(
            change.markdown(Locale::EN, "Gnusto", None),
            format!(
                "Gnusto deleted [Reddit](http://www.reddit.com), removing it from [Climate change]({}) and \
                [Weather]({})",
                topic1.relative_url(),
                topic2.relative_url(),
            )
        );
    }

    #[test]
    fn delete_topic() {
        let link = link("Reddit", "http://www.reddit.com");
        let topic1 = topic("Climate change");
        let topic2 = topic("Weather");

        let change = Change::DeleteTopic(DeleteTopic {
            actor_id: "2".to_owned(),
            child_links: LinkInfoList(BTreeSet::from([LinkInfo::from(&link)])),
            child_topics: TopicInfoList(BTreeSet::new()),
            date: chrono::Utc::now(),
            deleted_topic: TopicInfo::from(&topic1),
            id: Change::new_id(),
            parent_topics: TopicInfoList(BTreeSet::from([TopicInfo::from(&topic2)])),
        });

        assert_eq!(
            change.markdown(Locale::EN, "Gnusto", None),
            format!(
                "Gnusto deleted [Climate change]({}), removing [Reddit](http://www.reddit.com) from it, and \
                removing it from [Weather]({})",
                topic1.relative_url(),
                topic2.relative_url(),
            )
        );
    }

    mod update_topic_parent_topics {
        use super::*;

        #[test]
        fn simple_case() {
            let topic1 = topic("Climate change");
            let topic2 = topic("Climate");
            let topic3 = topic("Weather");

            let change = Change::UpdateTopicParentTopics(UpdateTopicParentTopics {
                actor_id: "2".to_owned(),
                added_parent_topics: TopicInfoList::from(&topic2),
                date: chrono::Utc::now(),
                id: Change::new_id(),
                parent_topic_ids: BTreeSet::from([topic3.topic_id().to_owned()]),
                removed_parent_topics: TopicInfoList::new(),
                updated_topic: TopicInfo::from(&topic1),
            });

            assert_eq!(
                change.markdown(Locale::EN, "Gnusto", None),
                format!(
                    "Gnusto placed [Climate change]({}) under [Climate]({})",
                    topic1.relative_url(),
                    topic2.relative_url()
                ),
            );
        }

        #[test]
        fn paths() {
            let topic1 = topic("Climate change");
            let topic2 = topic("Climate");
            let topic3 = topic("Weather");

            let change = Change::UpdateTopicParentTopics(UpdateTopicParentTopics {
                actor_id: "2".to_owned(),
                added_parent_topics: TopicInfoList::from(&topic2),
                date: chrono::Utc::now(),
                id: Change::new_id(),
                parent_topic_ids: BTreeSet::from([topic3.topic_id().to_owned()]),
                removed_parent_topics: TopicInfoList::new(),
                updated_topic: TopicInfo::from(&topic1),
            });

            assert_eq!(
                change.ids(),
                HashSet::from([topic1.topic_id(), topic2.topic_id(), topic3.topic_id()])
            );
        }
    }

    mod update_topic_synonyms {
        use super::*;

        #[test]
        fn added_and_removed() {
            let topic1 = topic("Climate change");

            let change = Change::UpdateTopicSynonyms(UpdateTopicSynonyms {
                actor_id: "2".to_owned(),
                added_synonyms: SynonymList(BTreeMap::from([(
                    Locale::EN,
                    "Added synonym".to_owned(),
                )])),
                date: chrono::Utc::now(),
                id: Change::new_id(),
                parent_topics: BTreeSet::new(),
                removed_synonyms: SynonymList(BTreeMap::from([(
                    Locale::EN,
                    "Removed synonym".to_owned(),
                )])),
                reordered: false,
                // parent_topic: TopicInfo::from(&topic2),
                updated_topic: TopicInfo::from(&topic1),
            });

            assert_eq!(
                change.markdown(Locale::EN, "Gnusto", None),
                format!(
                    "Gnusto added \"Added synonym\" to and removed \"Removed synonym\" \
                    from [Climate change]({})",
                    topic1.relative_url(),
                ),
            );
        }

        #[test]
        fn removed() {
            let topic1 = topic("Climate change");

            let change = Change::UpdateTopicSynonyms(UpdateTopicSynonyms {
                actor_id: "2".to_owned(),
                added_synonyms: SynonymList(BTreeMap::new()),
                date: chrono::Utc::now(),
                id: Change::new_id(),
                parent_topics: BTreeSet::new(),
                updated_topic: TopicInfo::from(&topic1),
                removed_synonyms: SynonymList(BTreeMap::from([(
                    Locale::EN,
                    "Removed synonym".to_owned(),
                )])),
                reordered: false,
            });

            assert_eq!(
                change.markdown(Locale::EN, "Gnusto", None),
                format!(
                    "Gnusto removed \"Removed synonym\" from [Climate change]({})",
                    topic1.relative_url(),
                ),
            );
        }

        #[test]
        fn reordered() {
            let topic1 = topic("Climate change");

            let change = Change::UpdateTopicSynonyms(UpdateTopicSynonyms {
                actor_id: "2".to_owned(),
                added_synonyms: SynonymList::new(),
                date: chrono::Utc::now(),
                id: Change::new_id(),
                parent_topics: BTreeSet::new(),
                removed_synonyms: SynonymList::new(),
                reordered: true,
                updated_topic: TopicInfo::from(&topic1),
            });

            assert_eq!(
                change.markdown(Locale::EN, "Gnusto", None),
                format!(
                    "Gnusto reordered the synonyms for [Climate change]({})",
                    topic1.relative_url(),
                ),
            );
        }

        #[test]
        fn paths() {
            let topic1 = topic("Climate change");
            let topic2 = topic("Climate");

            let change = Change::UpdateTopicSynonyms(UpdateTopicSynonyms {
                actor_id: "2".to_owned(),
                added_synonyms: SynonymList(BTreeMap::new()),
                date: chrono::Utc::now(),
                id: Change::new_id(),
                parent_topics: BTreeSet::from([topic2.topic_id().to_owned()]),
                updated_topic: TopicInfo::from(&topic1),
                removed_synonyms: SynonymList::new(),
                reordered: false,
            });

            assert_eq!(
                change.ids(),
                HashSet::from([topic1.topic_id(), topic2.topic_id()])
            );
        }
    }

    mod remove_topic_timerange {
        use geotime::Geotime;

        use super::*;

        fn date() -> Geotime {
            let ts = chrono::Utc.ymd(1970, 1, 1).and_hms(0, 0, 1);
            Geotime::from(&ts)
        }

        fn change(topic: &RepoTopic, previous: Option<Timerange>) -> Change {
            let mut parent_topics = BTreeSet::new();
            for parent in &topic.parent_topics {
                parent_topics.insert(parent.id.to_owned());
            }

            Change::RemoveTopicTimerange(RemoveTopicTimerange {
                actor_id: "2".to_owned(),
                date: chrono::Utc::now(),
                id: Change::new_id(),
                parent_topics: parent_topics.iter().map(|path| path.to_owned()).collect(),
                previous_timerange: previous,
                updated_topic: TopicInfo::from(topic),
            })
        }

        #[test]
        fn simple_case() {
            let topic1 = topic("Climate change");
            let change = change(
                &topic1,
                Some(Timerange {
                    starts: date().into(),
                    prefix_format: TimerangePrefixFormat::StartYear,
                }),
            );

            assert_eq!(
                change.markdown(Locale::EN, "Gnusto", None),
                format!(
                    "Gnusto removed the time prefix \"1970\" from [Climate change]({})",
                    topic1.relative_url()
                )
            );
        }

        #[test]
        fn parent_topics() {
            let topic1 = topic("Climate change");
            let change = change(
                &topic1,
                Some(Timerange {
                    starts: date().into(),
                    prefix_format: TimerangePrefixFormat::StartYear,
                }),
            );

            if let Change::RemoveTopicTimerange(RemoveTopicTimerange { parent_topics, .. }) = change
            {
                assert!(!parent_topics.is_empty());
            } else {
                unreachable!("expected RemoveTopicTimerange");
            }
        }
    }

    mod upsert_link {
        use super::*;

        #[test]
        fn simple_case() {
            let topic1 = topic("Climate change");
            let link = link("Reddit", "https://www.reddit.com");

            let change = Change::UpsertLink(UpsertLink {
                actor_id: "2".to_owned(),
                date: chrono::Utc::now(),
                id: Change::new_id(),
                parent_topics: BTreeSet::from([topic1.topic_id().to_owned()]),
                previous_title: None,
                upserted_link: LinkInfo::from(&link),
                add_parent_topic: Some(TopicInfo::from(&topic1)),
            });

            assert_eq!(
                change.markdown(Locale::EN, "Gnusto", None),
                format!(
                    "Gnusto added [Reddit](https://www.reddit.com) to [Climate change]({})",
                    topic1.relative_url()
                ),
            );
        }

        #[test]
        fn updated_title() {
            let topic1 = topic("Climate change");
            let link = link("Reddit", "https://www.reddit.com");

            let change = Change::UpsertLink(UpsertLink {
                actor_id: "2".to_owned(),
                date: chrono::Utc::now(),
                id: Change::new_id(),
                parent_topics: BTreeSet::from([topic1.topic_id().to_owned()]),
                previous_title: Some("Redis".to_owned()),
                upserted_link: LinkInfo::from(&link),
                add_parent_topic: None,
            });

            assert_eq!(
                change.markdown(Locale::EN, "Gnusto", None),
                format!(
                    r#"Gnusto changed the title of [Redis](https://www.reddit.com) to "Reddit""#,
                ),
            );
        }

        #[test]
        fn paths() {
            let topic1 = topic("Climate change");
            let link = link("Reddit", "https://www.reddit.com");

            let change = Change::UpsertLink(UpsertLink {
                actor_id: "2".to_owned(),
                date: chrono::Utc::now(),
                id: Change::new_id(),
                parent_topics: BTreeSet::from([topic1.topic_id().to_owned()]),
                previous_title: Some("Redis".to_owned()),
                upserted_link: LinkInfo::from(&link),
                add_parent_topic: None,
            });

            assert_eq!(change.ids(), HashSet::from([link.id(), topic1.topic_id()]));
        }
    }

    mod upsert_topic {
        use super::*;

        #[test]
        fn simple_case() {
            let topic1 = topic("Climate change");
            let topic2 = topic("Climate");

            let change = Change::UpsertTopic(UpsertTopic {
                actor_id: "2".to_owned(),
                date: chrono::Utc::now(),
                id: Change::new_id(),
                parent_topic: TopicInfo::from(&topic2),
                parent_topic_ids: BTreeSet::from([topic2.topic_id().to_owned()]),
                upserted_topic: TopicInfo::from(&topic1),
            });

            assert_eq!(
                change.markdown(Locale::EN, "Gnusto", None),
                format!(
                    "Gnusto added [Climate change]({}) to [Climate]({})",
                    topic1.relative_url(),
                    topic2.relative_url()
                ),
            );
        }

        #[test]
        fn paths() {
            let topic1 = topic("Climate change");
            let topic2 = topic("Climate");
            let topic3 = topic("Weather");

            let change = Change::UpsertTopic(UpsertTopic {
                actor_id: "2".to_owned(),
                date: chrono::Utc::now(),
                id: Change::new_id(),
                parent_topic: TopicInfo::from(&topic2),
                parent_topic_ids: BTreeSet::from([topic3.topic_id().to_owned()]),
                upserted_topic: TopicInfo::from(&topic1),
            });

            assert_eq!(
                change.ids(),
                HashSet::from([topic1.topic_id(), topic2.topic_id(), topic3.topic_id()]),
            );
        }
    }

    mod upsert_topic_timerange {
        use geotime::Geotime;

        use super::*;

        fn change(topic: &RepoTopic, format: TimerangePrefixFormat) -> Change {
            let dt = chrono::Utc.ymd(1970, 1, 1).and_hms_milli(0, 0, 1, 444);
            let ts = Geotime::from(&dt);

            let mut parent_topics = BTreeSet::new();
            for parent in &topic.parent_topics {
                parent_topics.insert(parent.id.to_owned());
            }

            Change::UpsertTopicTimerange(UpsertTopicTimerange {
                actor_id: "2".to_owned(),
                date: chrono::Utc::now(),
                id: Change::new_id(),
                parent_topics: parent_topics.iter().map(|path| path.to_owned()).collect(),
                previous_timerange: None,
                updated_timerange: Timerange {
                    starts: ts.into(),
                    prefix_format: format,
                },
                updated_topic: TopicInfo::from(topic),
            })
        }

        #[test]
        fn start_year_format() {
            let topic1 = topic("Climate change");
            let change = change(&topic1, TimerangePrefixFormat::StartYear);

            assert_eq!(
                change.markdown(Locale::EN, "Gnusto", None),
                format!(
                    r#"Gnusto updated the time prefix for [Climate change]({}) to be "1970""#,
                    topic1.relative_url()
                )
            );
        }

        #[test]
        fn start_year_month_format() {
            let topic1 = topic("Climate change");
            let change = change(&topic1, TimerangePrefixFormat::StartYearMonth);

            assert_eq!(
                change.markdown(Locale::EN, "Gnusto", None),
                format!(
                    r#"Gnusto updated the time prefix for [Climate change]({}) to be "1970-01""#,
                    topic1.relative_url()
                )
            );
        }

        #[test]
        fn none_format() {
            let topic1 = topic("Climate change");
            let change = change(&topic1, TimerangePrefixFormat::None);

            assert_eq!(
                change.markdown(Locale::EN, "Gnusto", None),
                format!(
                    "Gnusto updated the start time for [Climate change]({}) to be 1970-01-01, \
                    but it will not be shown",
                    topic1.relative_url()
                )
            );
        }

        #[test]
        fn parent_references() {
            // Show the change in the activity log for any parent topics, even though the parent
            // topics aren't being updated with the change.
            let topic1 = topic("Climate change");
            let change = change(&topic1, TimerangePrefixFormat::None);

            if let Change::UpsertTopicTimerange(UpsertTopicTimerange { parent_topics, .. }) = change
            {
                assert!(!parent_topics.is_empty());
            } else {
                unreachable!("expected UpsertTopicTimerange");
            }
        }
    }
}
