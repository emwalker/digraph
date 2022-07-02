use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

mod index;
mod link;
mod repository;
mod topic;

use crate::http::repo_url;
use crate::prelude::*;
pub use index::*;
pub use link::*;
pub use repository::*;
pub use topic::*;

pub const API_VERSION: &str = "objects/v1";

#[derive(Clone, Debug, Deserialize, Serialize)]
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

impl std::cmp::PartialEq for Kind {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl std::cmp::Eq for Kind {}

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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkMetadata {
    pub added: DateTime<Utc>,
    pub path: String,
    pub title: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub api_version: String,
    pub kind: String,
    pub metadata: LinkMetadata,
    pub parent_topics: BTreeSet<ParentTopic>,
}

impl Link {
    pub fn path(&self) -> RepoPath {
        RepoPath::from(&self.metadata.path)
    }

    pub fn to_topic_child(&self, added: chrono::DateTime<Utc>) -> TopicChild {
        TopicChild {
            added,
            kind: Kind::Link,
            path: self.metadata.path.to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ParentTopic {
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
    pub fn fetch(&self, git: &Git) -> Result<Topic> {
        git.fetch_topic(&self.path)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicChild {
    pub added: DateTime<Utc>,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Synonym {
    pub added: DateTime<Utc>,
    pub locale: String,
    pub name: String,
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
#[serde(rename_all = "camelCase")]
pub struct Timerange {
    pub starts: DateTime<Utc>,
    pub prefix_format: TimerangePrefixFormat,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimerangePrefixFormat {
    #[serde(alias = "none")]
    None,
    #[serde(alias = "startYear")]
    StartYear,
    #[serde(alias = "startYearMonth")]
    StartYearMonth,
}

impl From<&str> for TimerangePrefixFormat {
    fn from(format: &str) -> Self {
        match format {
            "NONE" => Self::None,
            "START_YEAR" => Self::StartYear,
            "START_YEAR_MONTH" => Self::StartYearMonth,
            _ => Self::None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicMetadata {
    pub added: DateTime<Utc>,
    pub path: String,
    pub root: bool,
    pub synonyms: Vec<Synonym>,
    pub timerange: Option<Timerange>,
}

impl TopicMetadata {
    pub fn name(&self, locale: &str) -> String {
        for synonym in &self.synonyms {
            if synonym.locale == locale {
                return synonym.name.clone();
            }
        }
        "Missing name".into()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Topic {
    pub api_version: String,
    pub kind: Kind,
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

impl Topic {
    pub fn has_child(&self, path: &RepoPath) -> bool {
        self.children.iter().any(|child| child.path == path.inner)
    }

    pub fn name(&self, desired_locale: &str) -> String {
        self.metadata.name(desired_locale)
    }

    pub fn path(&self) -> RepoPath {
        RepoPath::from(&self.metadata.path)
    }

    pub fn to_parent_topic(&self) -> ParentTopic {
        ParentTopic {
            path: self.metadata.path.to_owned(),
        }
    }

    pub fn to_topic_child(&self, added: chrono::DateTime<Utc>) -> TopicChild {
        TopicChild {
            added,
            kind: Kind::Topic,
            path: self.metadata.path.to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
}

pub trait Visitor {
    fn visit_topic(&mut self, topic: &Topic) -> Result<()>;
    fn visit_link(&mut self, link: &Link) -> Result<()>;
}

#[derive(Clone, Debug, Default)]
pub struct DataRoot {
    root: PathBuf,
}

impl std::fmt::Display for DataRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.root)
    }
}

pub fn parse_path(input: &str) -> Result<(DataRoot, String)> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(.+?)/(\w+)/objects/(\w{2})/(\w{2})/([\w-]+)/object.yaml$").unwrap();
    }

    let cap = RE
        .captures(input)
        .ok_or_else(|| Error::Repo(format!("bad path: {}", input)))?;
    if cap.len() != 6 {
        return Err(Error::Command(format!("bad path: {:?}", cap)));
    }

    let (root, org_login, part1, part2, part3) =
        match (cap.get(1), cap.get(2), cap.get(3), cap.get(4), cap.get(5)) {
            (Some(root), Some(org_login), Some(part1), Some(part2), Some(part3)) => (
                root.as_str(),
                org_login.as_str(),
                part1.as_str(),
                part2.as_str(),
                part3.as_str(),
            ),
            _ => return Err(Error::Repo(format!("bad path: {}", input))),
        };

    let id = format!("/{}/{}{}{}", org_login, part1, part2, part3);
    let root = PathBuf::from(root);

    Ok((DataRoot::new(root), id))
}

impl DataRoot {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn object_filename(&self, path: &str) -> Result<PathBuf> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\w{2})(\w{2})([\w-]+)$").unwrap();
        }

        let path = RepoPath::from(path);
        if !path.valid {
            return Err(Error::Repo(format!("invalid path: {:?}", path)));
        }

        let cap = RE
            .captures(&path.short_id)
            .ok_or_else(|| Error::Repo(format!("bad id: {}", path)))?;

        if cap.len() != 4 {
            return Err(Error::Repo(format!("bad id: {}", path)));
        }

        let (part1, part2, part3) = match (cap.get(1), cap.get(2), cap.get(3)) {
            (Some(part1), Some(part2), Some(part3)) => {
                (part1.as_str(), part2.as_str(), part3.as_str())
            }
            _ => return Err(Error::Repo(format!("bad id: {}", path))),
        };

        let file_path = format!(
            "{}/objects/{}/{}/{}/object.yaml",
            path.org_login, part1, part2, part3
        );

        Ok(self.root.join(file_path))
    }

    pub fn synonym_index_filename(&self, key: &IndexKey) -> Result<PathBuf> {
        let file_path = format!("{}/indexes/synonyms/{}.yaml", &key.prefix[1..], key.field);
        Ok(self.root.join(file_path))
    }

    pub fn token_index_filename(&self, key: &IndexKey) -> Result<PathBuf> {
        let file_path = format!("{}/indexes/tokens/{}.yaml", &key.prefix[1..], key.field);
        Ok(self.root.join(file_path))
    }
}

#[derive(Clone, Debug, Default)]
pub struct Git {
    root: DataRoot,
}

impl Git {
    pub fn new(root: DataRoot) -> Self {
        Self { root }
    }

    pub fn exists(&self, path: &RepoPath) -> Result<bool> {
        let filename = self.root.object_filename(&path.inner)?;
        Ok(Path::new(&filename).exists())
    }

    pub fn fetch(&self, path: &str) -> Result<Object> {
        let path = self.root.object_filename(path)?;
        let fh = std::fs::File::open(&path)
            .map_err(|e| Error::Repo(format!("problem opening file {:?}: {}", path, e)))?;
        let object: Object = serde_yaml::from_reader(fh)?;
        Ok(object)
    }

    pub fn fetch_topic(&self, path: &str) -> Result<Topic> {
        match &self.fetch(path)? {
            Object::Topic(topic) => Ok(topic.clone()),
            other => return Err(Error::Repo(format!("not a topic: {:?}", other))),
        }
    }

    pub fn fetch_link(&self, path: &str) -> Result<Link> {
        match &self.fetch(path)? {
            Object::Link(link) => Ok(link.clone()),
            other => return Err(Error::Repo(format!("not a link: {:?}", other))),
        }
    }

    pub fn cycle_exists(
        &self,
        descendant_path: &RepoPath,
        ancestor_path: &RepoPath,
    ) -> Result<bool> {
        let mut stack = vec![descendant_path.clone()];
        let mut seen: BTreeSet<RepoPath> = BTreeSet::new();
        let mut iterations = 0;

        while !stack.is_empty() {
            if let Some(descendant_path) = stack.pop() {
                if seen.contains(&descendant_path) {
                    continue;
                }

                if &descendant_path == ancestor_path {
                    log::info!("cycle check completed in {} iterations", iterations);
                    return Ok(true);
                }

                let descendant = self.fetch_topic(&descendant_path.inner)?;

                for child in &descendant.children {
                    if child.kind != Kind::Topic {
                        break;
                    }
                    let child_path = RepoPath::from(&child.path);
                    stack.push(child_path);
                }

                seen.insert(descendant_path.clone());
            }

            iterations += 1;
        }

        log::info!("cycle check completed in {} iterations", iterations);
        Ok(false)
    }

    // The value of `token` will sometimes need to be normalized by the caller in order for lookups
    // to work as expected.  We do not normalize the token here because some searches, e.g.,
    // of urls, are more sensitive to normalization, and so we omit it in those cases.
    pub fn index_key(&self, prefix: &str, token: &str) -> Result<IndexKey> {
        match token.get(0..2) {
            Some(field) => Ok(IndexKey {
                prefix: prefix.to_owned(),
                field: field.replace([' '], "+"),
            }),
            None => Err(Error::Repo(format!("bad token: {}", token))),
        }
    }

    pub fn indexed_on(&self, path: &RepoPath, search: &Search) -> Result<bool> {
        for token in &search.tokens {
            let key = self.index_key(&path.prefix, token)?;
            if !self
                .token_index(&key, IndexMode::Update)?
                .indexed_on(path, token)?
            {
                return Ok(false);
            }
        }

        for url in &search.urls {
            let url = &url.normalized;
            let key = self.index_key(&path.prefix, url)?;
            if !self
                .token_index(&key, IndexMode::Update)?
                .indexed_on(path, url)?
            {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn link(&self, path: &RepoPath) -> Result<Option<Link>> {
        if self.exists(path)? {
            return Ok(Some(self.fetch_link(&path.inner)?));
        }
        Ok(None)
    }

    fn link_searches(&self, link: Option<Link>) -> Result<BTreeSet<Search>> {
        let searches = match link {
            Some(link) => {
                let meta = &link.metadata;
                let url = repo_url::Url::parse(&meta.url)?;
                BTreeSet::from([Search::parse(&meta.title)?, Search::parse(&url.normalized)?])
            }
            None => BTreeSet::new(),
        };
        Ok(searches)
    }

    fn remove(&self, path: &RepoPath) -> Result<()> {
        let filename = self.root.object_filename(&path.inner)?;
        if filename.exists() {
            fs::remove_file(filename)?;
        }

        Ok(())
    }

    fn remove_link(&self, path: &RepoPath, link: &Link, indexer: &mut Indexer) -> Result<()> {
        let searches = self.link_searches(Some(link.to_owned()))?;
        indexer.remove_searches(path, searches.iter())?;
        self.remove(path)?;
        Ok(())
    }

    fn remove_topic(&self, path: &RepoPath, topic: &Topic, indexer: &mut Indexer) -> Result<()> {
        indexer.remove_synonyms(path, topic)?;

        let meta = &topic.metadata;
        let mut searches = vec![];
        for synonym in &meta.synonyms {
            let search = Search::parse(&synonym.name)?;
            if search.is_empty() {
                continue;
            }
            searches.push(search);
        }
        indexer.remove_searches(path, searches.iter())?;
        self.remove(path)?;

        Ok(())
    }

    fn save<T: Serialize>(&self, path: &RepoPath, object: &T) -> Result<()> {
        let filename = self.root.object_filename(&path.inner)?;
        let dest = filename.parent().expect("expected a parent directory");
        fs::create_dir_all(&dest).ok();
        let s = serde_yaml::to_string(&object)?;
        log::debug!("saving {:?}", filename);
        fs::write(&filename, s)?;
        Ok(())
    }

    pub fn save_link(&self, path: &RepoPath, link: &Link, indexer: &mut Indexer) -> Result<()> {
        let before = self.link(path)?;
        let before = self.link_searches(before)?;
        let after = self.link_searches(Some(link.to_owned()))?;
        indexer.update_lookups(path, &before, &after)?;
        self.save(path, link)
    }

    pub fn save_topic(&self, path: &RepoPath, topic: &Topic, indexer: &mut Indexer) -> Result<()> {
        let before = self.topic(path)?;
        indexer.update_synonyms(&before, topic)?;

        let before = self.topic_searches(before)?;
        let after = self.topic_searches(Some(topic.to_owned()))?;
        indexer.update_lookups(path, &before, &after)?;

        self.save(path, topic)
    }

    pub fn synonym_index(&self, key: &IndexKey, mode: IndexMode) -> Result<SynonymIndex> {
        let filename = self.root.synonym_index_filename(key)?;
        match mode {
            IndexMode::Replace => Ok(SynonymIndex::new(&filename)),
            IndexMode::Update => SynonymIndex::load(&filename),
        }
    }

    pub fn synonym_matches(&self, prefix: &str, name: &str) -> Result<BTreeSet<SynonymMatch>> {
        let normalized = normalize(name);
        let key = self.index_key(prefix, &normalized)?;
        let mut matches = BTreeSet::new();

        for entry in &self.synonym_index(&key, IndexMode::Update)?.matches(name)? {
            let topic = self.fetch_topic(&entry.path)?;
            matches.insert(SynonymMatch {
                cycle: false,
                entry: (*entry).clone(),
                name: name.to_string(),
                topic,
            });
        }

        Ok(matches)
    }

    pub fn token_index(&self, key: &IndexKey, mode: IndexMode) -> Result<TokenIndex> {
        let filename = self.root.token_index_filename(key)?;
        match mode {
            IndexMode::Replace => Ok(TokenIndex::new(&filename)),
            IndexMode::Update => TokenIndex::load(&filename),
        }
    }

    pub fn topic(&self, path: &RepoPath) -> Result<Option<Topic>> {
        if self.exists(path)? {
            return Ok(Some(self.fetch_topic(&path.inner)?));
        }
        Ok(None)
    }

    fn topic_searches(&self, topic: Option<Topic>) -> Result<BTreeSet<Search>> {
        let searches = match topic {
            Some(topic) => {
                let meta = &topic.metadata;
                let mut searches = BTreeSet::new();

                for synonym in &meta.synonyms {
                    let search = Search::parse(&synonym.name)?;
                    if search.is_empty() {
                        continue;
                    }
                    searches.insert(search);
                }

                searches
            }
            None => BTreeSet::new(),
        };

        Ok(searches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_path_works() {
        let result = parse_path("../../wiki/12/34/5678/object.yaml");
        assert!(matches!(result, Err(Error::Repo(_))));

        let (root, id) = parse_path("../../wiki/objects/12/34/5678/object.yaml").unwrap();
        assert_eq!(root.root, PathBuf::from("../.."));
        assert_eq!(id, String::from("/wiki/12345678"));
    }

    #[test]
    fn data_root_file_path() {
        let root = DataRoot::new(PathBuf::from("../.."));

        assert!(matches!(root.object_filename("1234"), Err(Error::Repo(_))));
        assert!(matches!(
            root.object_filename("wiki/123456"),
            Err(Error::Repo(_))
        ));
        assert!(matches!(
            root.object_filename("/wiki/1234"),
            Err(Error::Repo(_))
        ));

        assert_eq!(
            root.object_filename("/wiki/123456").unwrap(),
            PathBuf::from("../../wiki/objects/12/34/56/object.yaml")
        );

        assert_eq!(
            root.object_filename("/with-dash/123456").unwrap(),
            PathBuf::from("../../with-dash/objects/12/34/56/object.yaml")
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
}
