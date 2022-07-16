use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use super::{Git, Link, Locale, RepoPath, Synonym, Timerange, TimerangePrefixFormat, Topic};
use crate::prelude::*;

#[derive(Clone, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ChangeId(pub String);

impl std::fmt::Display for ChangeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct TopicInfo {
    pub path: Option<String>,
    pub synonyms: SynonymList,
}

impl std::cmp::Ord for TopicInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.path.cmp(&other.path)
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

    fn path(&self) -> Option<RepoPath> {
        self.path.as_ref().map(RepoPath::from)
    }
}

impl From<&Topic> for TopicInfo {
    fn from(topic: &Topic) -> Self {
        let mut synonyms = BTreeMap::new();

        for synonym in &topic.metadata.synonyms {
            if synonyms.contains_key(&synonym.locale) {
                continue;
            }
            synonyms.insert(synonym.locale, synonym.name.to_owned());
        }

        Self {
            synonyms: SynonymList(synonyms),
            path: Some(topic.metadata.path.to_owned()),
        }
    }
}

impl From<(Locale, String, Option<String>)> for TopicInfo {
    fn from(info: (Locale, String, Option<String>)) -> Self {
        let (locale, name, path) = info;
        let synonyms = SynonymList(BTreeMap::from([(locale, name)]));
        Self { synonyms, path }
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct TopicInfoList(pub BTreeSet<TopicInfo>);

impl TopicInfoList {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<&Vec<Topic>> for TopicInfoList {
    fn from(topics: &Vec<Topic>) -> Self {
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

#[derive(Clone, Deserialize, Serialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct LinkInfo {
    pub path: Option<String>,
    pub title: String,
    pub url: String,
}

impl From<&Link> for LinkInfo {
    fn from(link: &Link) -> Self {
        Self {
            path: Some(link.metadata.path.to_owned()),
            title: link.title().to_owned(),
            url: link.url().to_owned(),
        }
    }
}

impl LinkInfo {
    fn path(&self) -> Option<RepoPath> {
        self.path.as_ref().map(RepoPath::from)
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct LinkInfoList(pub BTreeSet<LinkInfo>);

impl LinkInfoList {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<&Vec<Link>> for LinkInfoList {
    fn from(links: &Vec<Link>) -> Self {
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
    pub fn new_id() -> ChangeId {
        ChangeId(random_id())
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

    pub fn id(&self) -> ChangeId {
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

    pub fn markdown(&self, locale: Locale, actor_name: &str, context: Option<&RepoPath>) -> String {
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

    pub fn paths(&self) -> Vec<Option<RepoPath>> {
        match self {
            Self::DeleteLink(inner) => inner.paths(),
            Self::DeleteTopic(inner) => inner.paths(),
            Self::ImportLink(inner) => inner.paths(),
            Self::ImportTopic(inner) => inner.paths(),
            Self::RemoveTopicTimerange(inner) => inner.paths(),
            Self::UpdateLinkParentTopics(inner) => inner.paths(),
            Self::UpdateTopicParentTopics(inner) => inner.paths(),
            Self::UpdateTopicSynonyms(inner) => inner.paths(),
            Self::UpsertLink(inner) => inner.paths(),
            Self::UpsertTopic(inner) => inner.paths(),
            Self::UpsertTopicTimerange(inner) => inner.paths(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteLink {
    pub actor_id: String,
    pub id: ChangeId,
    pub date: Timestamp,
    pub deleted_link: LinkInfo,
    pub parent_topics: TopicInfoList,
}

impl DeleteLink {
    fn paths(&self) -> Vec<Option<RepoPath>> {
        let mut paths = vec![self.deleted_link.path()];
        for topic in &self.parent_topics.0 {
            paths.push(topic.path());
        }
        paths
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTopic {
    pub actor_id: String,
    pub id: ChangeId,
    pub child_links: LinkInfoList,
    pub child_topics: TopicInfoList,
    pub date: Timestamp,
    pub deleted_topic: TopicInfo,
    pub parent_topics: TopicInfoList,
}

impl DeleteTopic {
    fn paths(&self) -> Vec<Option<RepoPath>> {
        let mut paths = vec![self.deleted_topic.path()];

        for link in &self.child_links.0 {
            paths.push(link.path());
        }

        for topic in &self.child_topics.0 {
            paths.push(topic.path());
        }

        for topic in &self.parent_topics.0 {
            paths.push(topic.path());
        }

        paths
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImportLink {
    pub actor_id: String,
    pub id: ChangeId,
    pub date: Timestamp,
    pub imported_link: LinkInfo,
    pub parent_topics: TopicInfoList,
}

impl ImportLink {
    fn paths(&self) -> Vec<Option<RepoPath>> {
        let mut paths = vec![self.imported_link.path()];
        for topic in &self.parent_topics.0 {
            paths.push(topic.path());
        }
        paths
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImportTopic {
    pub actor_id: String,
    pub id: ChangeId,
    pub child_links: LinkInfoList,
    pub child_topics: TopicInfoList,
    pub date: Timestamp,
    pub imported_topic: TopicInfo,
    pub parent_topics: TopicInfoList,
}

impl ImportTopic {
    fn paths(&self) -> Vec<Option<RepoPath>> {
        let mut paths = vec![self.imported_topic.path()];

        for link in &self.child_links.0 {
            paths.push(link.path());
        }

        for topic in &self.child_topics.0 {
            paths.push(topic.path());
        }

        for topic in &self.parent_topics.0 {
            paths.push(topic.path());
        }

        paths
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveTopicTimerange {
    pub actor_id: String,
    pub date: Timestamp,
    pub id: ChangeId,
    // The RemoveTopicTimerange is idempotent, so the timerange might already have been removed.
    pub previous_timerange: Option<Timerange>,
    pub updated_topic: TopicInfo,
}

impl RemoveTopicTimerange {
    fn paths(&self) -> Vec<Option<RepoPath>> {
        vec![self.updated_topic.path()]
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLinkParentTopics {
    pub actor_id: String,
    pub added_parent_topics: TopicInfoList,
    pub date: Timestamp,
    pub id: ChangeId,
    pub removed_parent_topics: TopicInfoList,
    pub updated_link: LinkInfo,
}

impl UpdateLinkParentTopics {
    fn paths(&self) -> Vec<Option<RepoPath>> {
        let mut paths = vec![self.updated_link.path()];

        for topic in &self.added_parent_topics.0 {
            paths.push(topic.path());
        }

        for topic in &self.removed_parent_topics.0 {
            paths.push(topic.path());
        }

        paths
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTopicParentTopics {
    pub actor_id: String,
    pub added_parent_topics: TopicInfoList,
    pub id: ChangeId,
    pub date: Timestamp,
    pub removed_parent_topics: TopicInfoList,
    pub updated_topic: TopicInfo,
}

impl UpdateTopicParentTopics {
    fn paths(&self) -> Vec<Option<RepoPath>> {
        let mut paths = vec![self.updated_topic.path()];

        for topic in &self.added_parent_topics.0 {
            paths.push(topic.path());
        }

        for topic in &self.removed_parent_topics.0 {
            paths.push(topic.path());
        }

        paths
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTopicSynonyms {
    pub actor_id: String,
    pub added_synonyms: SynonymList,
    pub id: ChangeId,
    pub date: Timestamp,
    pub removed_synonyms: SynonymList,
    pub reordered: bool,
    pub updated_topic: TopicInfo,
}

impl UpdateTopicSynonyms {
    fn paths(&self) -> Vec<Option<RepoPath>> {
        vec![self.updated_topic.path()]
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpsertLink {
    pub add_parent_topic: Option<TopicInfo>,
    pub actor_id: String,
    pub id: ChangeId,
    pub date: Timestamp,
    pub upserted_link: LinkInfo,
}

impl UpsertLink {
    fn paths(&self) -> Vec<Option<RepoPath>> {
        let mut paths = vec![self.upserted_link.path()];
        if let Some(topic) = &self.add_parent_topic {
            paths.push(topic.path());
        }
        paths
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpsertTopic {
    pub actor_id: String,
    pub id: ChangeId,
    pub date: Timestamp,
    pub parent_topic: TopicInfo,
    pub upserted_topic: TopicInfo,
}

impl UpsertTopic {
    fn paths(&self) -> Vec<Option<RepoPath>> {
        vec![self.upserted_topic.path(), self.parent_topic.path()]
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpsertTopicTimerange {
    pub actor_id: String,
    pub id: ChangeId,
    pub date: Timestamp,
    pub previous_timerange: Option<Timerange>,
    pub updated_timerange: Timerange,
    pub updated_topic: TopicInfo,
}

impl UpsertTopicTimerange {
    fn paths(&self) -> Vec<Option<RepoPath>> {
        vec![self.updated_topic.path()]
    }
}

mod markdown {
    use super::*;

    pub trait Markdown {
        fn markdown(&self, locale: Locale, actor_name: &str, context: Option<&RepoPath>) -> String;
    }

    impl LinkInfo {
        fn markdown(&self) -> String {
            format!("[{}]({})", self.title, self.url)
        }
    }

    impl TopicInfo {
        fn markdown(&self, locale: Locale) -> String {
            match &self.path {
                Some(path) => format!("[{}]({})", self.name(locale), path),
                None => format!("{} (deleted)", self.name(locale)),
            }
        }
    }

    impl TopicInfoList {
        fn markdown(&self, locale: Locale) -> String {
            use itertools::Itertools;
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
            use itertools::Itertools;
            let links = &self.0;

            match links.len() {
                0 => "".to_owned(),
                1 => links.iter().map(LinkInfo::markdown).join(""),
                2 => links.iter().map(LinkInfo::markdown).join(" and "),
                _ => "a number of links".to_owned(),
            }
        }
    }

    impl Timerange {
        fn markdown(&self) -> String {
            match self.prefix_format {
                TimerangePrefixFormat::None => "(none)",
                TimerangePrefixFormat::StartYear => "(start-year)",
                TimerangePrefixFormat::StartYearMonth => "(start-year-month)",
            }
            .to_owned()
        }
    }

    impl Markdown for DeleteLink {
        fn markdown(
            &self,
            locale: Locale,
            actor_name: &str,
            _context: Option<&RepoPath>,
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
            _context: Option<&RepoPath>,
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
            _context: Option<&RepoPath>,
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
            _context: Option<&RepoPath>,
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
            _context: Option<&RepoPath>,
        ) -> String {
            match &self.previous_timerange {
                Some(timerange) => {
                    format!(
                        r#"{} removed the timerange prefix "{}" from {}"#,
                        actor_name,
                        timerange.markdown(),
                        self.updated_topic.markdown(locale),
                    )
                }
                None => {
                    format!(
                        r#"{} removed the timerange prefix from {}"#,
                        actor_name,
                        self.updated_topic.markdown(locale),
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
            _context: Option<&RepoPath>,
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
            _locale: Locale,
            actor_name: &str,
            _context: Option<&RepoPath>,
        ) -> String {
            format!("{} added TOPIC to TOPIC and TOPIC", actor_name)
        }
    }

    impl Markdown for UpdateTopicSynonyms {
        fn markdown(
            &self,
            _locale: Locale,
            actor_name: &str,
            _context: Option<&RepoPath>,
        ) -> String {
            format!("{} added NAME to and removed NAME from TOPIC", actor_name)
        }
    }

    impl Markdown for UpsertLink {
        fn markdown(
            &self,
            locale: Locale,
            actor_name: &str,
            _context: Option<&RepoPath>,
        ) -> String {
            match &self.add_parent_topic {
                Some(topic) => format!(
                    "{} added {} to {}",
                    actor_name,
                    self.upserted_link.markdown(),
                    topic.markdown(locale)
                ),
                None => format!("{} updated {}", actor_name, self.upserted_link.markdown()),
            }
        }
    }

    impl Markdown for UpsertTopic {
        fn markdown(
            &self,
            locale: Locale,
            actor_name: &str,
            _context: Option<&RepoPath>,
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
            _locale: Locale,
            actor_name: &str,
            _context: Option<&RepoPath>,
        ) -> String {
            format!("{} updated the timerange on to TOPIC to be", actor_name)
        }
    }
}

pub struct FetchActivity {
    pub actor: Viewer,
    pub first: usize,
    pub topic_path: Option<RepoPath>,
}

pub struct FetchActivityResult {
    pub changes: Vec<Change>,
}

pub trait ActivityForPrefix {
    fn fetch_activity(&self, prefix: &str, first: usize) -> Result<Vec<Change>>;
}

impl FetchActivity {
    pub fn call<F>(&self, git: &Git, fetch: &F) -> Result<FetchActivityResult>
    where
        F: ActivityForPrefix,
    {
        let changes = match &self.topic_path {
            Some(path) => git.fetch_activity(path, self.first)?,

            // Fetch the top-level activity feed from Redis rather than Git so as to avoid
            // write contention on a single file for every update.  This could show up in the form
            // of merge conflicts when commits are being saved to Git.
            None => fetch.fetch_activity(WIKI_REPO_PREFIX, self.first)?,
        };

        Ok(FetchActivityResult { changes })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use crate::git::{
        Link, LinkMetadata, Locale, ParentTopic, Synonym, Topic, TopicMetadata, API_VERSION,
    };

    fn topic(name: &str) -> Topic {
        let added = chrono::Utc::now();
        // Some unique path
        let path = format!("/wiki/{}", name);
        Topic {
            api_version: API_VERSION.to_owned(),
            metadata: TopicMetadata {
                added,
                path: path.to_owned(),
                root: false,
                timerange: None,
                synonyms: Vec::from([Synonym {
                    name: name.to_owned(),
                    locale: Locale::EN,
                    added,
                }]),
            },
            parent_topics: BTreeSet::from([ParentTopic { path }]),
            children: BTreeSet::new(),
        }
    }

    fn link(title: &str, url: &str) -> Link {
        let added = chrono::Utc::now();
        Link {
            api_version: API_VERSION.to_owned(),
            metadata: LinkMetadata {
                title: title.to_owned(),
                url: url.to_owned(),
                path: "/wiki/00002".to_owned(),
                added,
            },
            parent_topics: BTreeSet::from([ParentTopic {
                path: "/wiki/0001".to_owned(),
            }]),
        }
    }

    #[test]
    fn update_link_parent_topics() {
        let topic1 = topic("Climate change");
        let topic2 = topic("Weather");

        let change = Change::UpdateLinkParentTopics(UpdateLinkParentTopics {
            actor_id: "2".to_owned(),
            date: chrono::Utc::now(),
            id: Change::new_id(),
            updated_link: LinkInfo {
                title: "Reddit".to_owned(),
                url: "http://www.reddit.com".to_owned(),
                path: Some("some-path".to_owned()),
            },
            added_parent_topics: TopicInfoList(BTreeSet::from([TopicInfo::from(&topic1)])),
            removed_parent_topics: TopicInfoList(BTreeSet::from([TopicInfo::from(&topic2)])),
        });

        let markdown = format!(
            "Gnusto added [Climate change]({}) to and removed [Weather]({}) from \
            [Reddit](http://www.reddit.com)",
            topic1.path(),
            topic2.path()
        );

        let context = RepoPath::from("/wiki/00010");
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
                title: link.metadata.title.to_owned(),
                url: link.metadata.url,
                path: Some(link.metadata.path),
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
                topic1.metadata.path,
                topic2.metadata.path,
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
                topic1.metadata.path,
                topic2.metadata.path,
            )
        );
    }

    #[test]
    fn upsert_topic_timerange() {
        use chrono::TimeZone;

        let topic1 = topic("Climate change");
        let date = chrono::Utc.ymd(1970, 1, 1).and_hms_milli(0, 0, 1, 444);

        let change = Change::UpsertTopicTimerange(UpsertTopicTimerange {
            actor_id: "2".to_owned(),
            date: chrono::Utc::now(),
            id: Change::new_id(),
            previous_timerange: None,
            updated_timerange: Timerange {
                starts: date,
                prefix_format: TimerangePrefixFormat::StartYear,
            },
            updated_topic: TopicInfo::from(&topic1),
        });

        assert_eq!(
            change.markdown(Locale::EN, "Gnusto", None),
            "Gnusto updated the timerange on to TOPIC to be".to_string()
        );
    }

    #[test]
    fn upsert_link() {
        let topic1 = topic("Climate change");
        let link = link("Reddit", "https://www.reddit.com");

        let change = Change::UpsertLink(UpsertLink {
            actor_id: "2".to_owned(),
            date: chrono::Utc::now(),
            id: Change::new_id(),
            upserted_link: LinkInfo::from(&link),
            add_parent_topic: Some(TopicInfo::from(&topic1)),
        });

        assert_eq!(
            change.markdown(Locale::EN, "Gnusto", None),
            format!(
                "Gnusto added [Reddit](https://www.reddit.com) to [Climate change]({})",
                topic1.metadata.path
            ),
        );
    }

    #[test]
    fn upsert_topic() {
        let topic1 = topic("Climate change");
        let topic2 = topic("Climate");

        let change = Change::UpsertTopic(UpsertTopic {
            actor_id: "2".to_owned(),
            date: chrono::Utc::now(),
            id: Change::new_id(),
            parent_topic: TopicInfo::from(&topic2),
            upserted_topic: TopicInfo::from(&topic1),
        });

        assert_eq!(
            change.markdown(Locale::EN, "Gnusto", None),
            format!(
                "Gnusto added [Climate change]({}) to [Climate]({})",
                topic1.metadata.path, topic2.metadata.path
            ),
        );
    }
}
