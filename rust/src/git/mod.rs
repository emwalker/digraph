use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

pub mod activity;
mod index;
mod link;
mod repository;
mod search;
pub mod testing;
mod topic;

use crate::prelude::*;
use crate::types;
pub use index::*;
pub use link::*;
pub use repository::*;
pub use search::*;
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkMetadata {
    pub added: DateTime<Utc>,
    pub path: String,
    pub title: String,
    pub url: String,
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
        (&self.metadata.title, &self.metadata.path)
            .cmp(&(&other.metadata.title, &other.metadata.path))
    }
}

impl std::cmp::PartialOrd for Link {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Link {
    pub fn path(&self) -> RepoPath {
        RepoPath::from(&self.metadata.path)
    }

    pub fn title(&self) -> &String {
        &self.metadata.title
    }

    pub fn to_search_entry(&self) -> SearchEntry {
        SearchEntry {
            path: self.metadata.path.to_owned(),
            kind: Kind::Link,
        }
    }

    pub fn to_topic_child(&self, added: chrono::DateTime<Utc>) -> TopicChild {
        TopicChild {
            added,
            kind: Kind::Link,
            path: self.metadata.path.to_owned(),
        }
    }

    pub fn url(&self) -> &String {
        &self.metadata.url
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

impl std::hash::Hash for TopicChild {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.path.hash(state);
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Synonym {
    pub added: DateTime<Utc>,
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
#[serde(rename_all = "camelCase")]
pub struct TopicMetadata {
    pub added: DateTime<Utc>,
    pub path: String,
    pub root: bool,
    pub synonyms: Vec<Synonym>,
    pub timerange: Option<Timerange>,
}

impl TopicMetadata {
    pub fn name(&self, locale: Locale) -> String {
        for synonym in &self.synonyms {
            if synonym.locale == locale {
                return synonym.name.clone();
            }
        }
        "Missing name".into()
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
    pub fn has_child(&self, path: &RepoPath) -> bool {
        self.children.iter().any(|child| child.path == path.inner)
    }

    pub fn name(&self, locale: Locale) -> String {
        self.prefix().format(&self.metadata.name(locale))
    }

    pub fn path(&self) -> RepoPath {
        RepoPath::from(&self.metadata.path)
    }

    fn prefix(&self) -> types::Prefix {
        types::Prefix::from(&self.metadata.timerange)
    }

    pub fn prefixed_synonyms(&self) -> Vec<Synonym> {
        let mut synonyms = vec![];
        let prefix = self.prefix();
        for Synonym {
            added,
            locale,
            name,
        } in &self.metadata.synonyms
        {
            synonyms.push(Synonym {
                added: *added,
                locale: *locale,
                name: prefix.format(name),
            })
        }
        synonyms
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

    pub fn to_topic_child(&self, added: chrono::DateTime<Utc>) -> TopicChild {
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
            Self::Link(link) => link.metadata.title.to_owned(),
            Self::Topic(topic) => topic.name(locale),
        }
    }

    pub fn kind(&self) -> Kind {
        match self {
            Object::Topic(_) => Kind::Topic,
            Object::Link(_) => Kind::Link,
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

#[derive(Clone, Debug)]
pub enum Row {
    SearchEntry(SearchEntry),
    Topic(Topic),
    TopicChild(TopicChild),
}

impl std::cmp::PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        self.path() == other.path()
    }
}

impl std::cmp::Eq for Row {}

impl std::hash::Hash for Row {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.path().hash(state);
    }
}

impl Row {
    fn path(&self) -> &String {
        match self {
            Self::SearchEntry(SearchEntry { path, .. }) => path,
            Self::Topic(topic) => &topic.metadata.path,
            Self::TopicChild(TopicChild { path, .. }) => path,
        }
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
            Regex::new(r"^(.+?)/(\w+)/objects/([\w_-]{2})/([\w_-]{2})/([\w_-]+)/object.yaml$")
                .unwrap();
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

    pub fn change_filename(&self, path: &str) -> Result<PathBuf> {
        Ok(self.basename("changes", path)?.join("change.yaml"))
    }

    pub fn change_index_filename(&self, path: &str) -> Result<PathBuf> {
        Ok(self.basename("objects", path)?.join("changes.yaml"))
    }

    pub fn basename(&self, subdirectory: &str, path: &str) -> Result<PathBuf> {
        let path = RepoPath::from(path);
        let (part1, part2, part3) = path.parts()?;
        let basename = format!(
            "{}/{}/{}/{}/{}",
            path.org_login, subdirectory, part1, part2, part3
        );

        Ok(self.root.join(basename))
    }

    pub fn object_filename(&self, path: &str) -> Result<PathBuf> {
        Ok(self.basename("objects", path)?.join("object.yaml"))
    }

    pub fn index_filename(&self, key: &IndexKey, index_type: IndexType) -> Result<PathBuf> {
        let prefix = &key.prefix[1..];
        let file_path = match index_type {
            IndexType::Search => format!("{}/indexes/search/{}.yaml", prefix, key.basename),
            IndexType::SynonymPhrase => {
                format!("{}/indexes/synonyms/phrases/{}.yaml", prefix, key.basename)
            }
            IndexType::SynonymToken => {
                format!("{}/indexes/synonyms/tokens/{}.yaml", prefix, key.basename)
            }
        };
        Ok(self.root.join(file_path))
    }
}

#[derive(Debug)]
pub struct DownSetIter {
    git: Git,
    seen: HashSet<TopicChild>,
    stack: Vec<TopicChild>,
}

impl Iterator for DownSetIter {
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

                    match self.git.fetch_topic(&topic_child.path) {
                        Ok(topic) => {
                            for child in &topic.children {
                                if child.kind != Kind::Topic {
                                    break;
                                }
                                self.stack.push(child.clone());
                            }
                            log::debug!("yielding topic {}", topic_child.path);
                            return Some(topic);
                        }

                        Err(err) => {
                            log::error!("failed to fetch topic: {}", err)
                        }
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

impl DownSetIter {
    fn new(git: Git, topic: Option<Topic>) -> Self {
        let mut stack = vec![];
        if let Some(topic) = &topic {
            stack.push(topic.to_topic_child(chrono::Utc::now()));
        }

        Self {
            git,
            seen: HashSet::new(),
            stack,
        }
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

    pub fn change_index(&self, path: &RepoPath, index_mode: IndexMode) -> Result<ActivityIndex> {
        let filename = self.root.change_index_filename(&path.inner)?;
        match index_mode {
            IndexMode::Replace => Ok(ActivityIndex::new(&filename)),
            IndexMode::ReadOnly => ActivityIndex::load(&filename),
            IndexMode::Update => ActivityIndex::load(&filename),
        }
    }

    pub fn change_filename(&self, path: &str) -> Result<PathBuf> {
        self.root.change_filename(path)
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

    pub fn fetch_activity(&self, path: &RepoPath, first: usize) -> Result<Vec<activity::Change>> {
        log::info!("fetching first {} change logs from Git for {}", first, path);
        let index = self.change_index(path, IndexMode::ReadOnly)?;
        let mut changes = vec![];

        for reference in index.references().iter().take(first) {
            let filename = self.change_filename(&reference.path)?;
            let fh = std::fs::File::open(&filename)
                .map_err(|e| Error::Repo(format!("problem opening file {:?}: {}", filename, e)))?;
            let result: std::result::Result<activity::Change, serde_yaml::Error> =
                serde_yaml::from_reader(fh);
            match result {
                Ok(change) => changes.push(change),
                Err(err) => log::warn!("failed to load change: {}", err),
            }
        }

        Ok(changes)
    }

    pub fn fetch_topic(&self, path: &str) -> Result<Topic> {
        match &self.fetch(path)? {
            Object::Topic(topic) => Ok(topic.clone()),
            other => return Err(Error::Repo(format!("{} not a topic: {:?}", path, other))),
        }
    }

    pub fn fetch_link(&self, path: &str) -> Result<Link> {
        match &self.fetch(path)? {
            Object::Link(link) => Ok(link.clone()),
            other => return Err(Error::Repo(format!("{} not a link: {:?}", path, other))),
        }
    }

    pub fn cycle_exists(
        &self,
        descendant_path: &RepoPath,
        ancestor_path: &RepoPath,
    ) -> Result<bool> {
        let mut i = 0;

        for topic in self.topic_down_set(descendant_path) {
            i += 1;
            if topic.metadata.path == ancestor_path.inner {
                log::info!("cycle found after {} iterations", i);
                return Ok(true);
            }
        }

        log::info!("no cycle found after {} iterations", i);
        Ok(false)
    }

    // The value of `token` will sometimes need to be normalized by the caller in order for lookups
    // to work as expected.  We do not normalize the token here because some searches, e.g.,
    // of urls, are more sensitive to normalization, and so we omit it in those cases.
    pub fn index_key(&self, prefix: &str, token: &Phrase) -> Result<IndexKey> {
        if !token.is_valid() {
            return Err(Error::Repo(format!("a valid token is required: {}", token)));
        }

        match token.basename() {
            Some(basename) => Ok(IndexKey {
                prefix: prefix.to_owned(),
                basename,
            }),
            None => Err(Error::Repo(format!("bad token: {}", token))),
        }
    }

    pub fn indexed_on(&self, entry: &SearchEntry, search: &Search) -> Result<bool> {
        let path = entry.path();
        for token in &search.tokens {
            let key = self.index_key(&path.prefix, token)?;
            if !self
                .token_index(&key, IndexMode::Update)?
                .indexed_on(entry, token)?
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
                let url = RepoUrl::parse(&meta.url)?;
                BTreeSet::from([Search::parse(&meta.title)?, Search::parse(&url.normalized)?])
            }
            None => BTreeSet::new(),
        };
        Ok(searches)
    }

    pub fn load_activity(
        &self,
        filename: &PathBuf,
        index_mode: IndexMode,
    ) -> Result<ActivityIndexMap> {
        let load = index_mode == IndexMode::Update || index_mode == IndexMode::ReadOnly;
        let activity = if load && filename.exists() {
            let fh = std::fs::File::open(&filename)?;
            serde_yaml::from_reader::<_, ActivityIndexMap>(fh)?
        } else {
            ActivityIndexMap {
                api_version: API_VERSION.to_owned(),
                ..Default::default()
            }
        };

        Ok(activity)
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
        indexer.remove_searches(&link.to_search_entry(), searches.iter())?;
        self.remove(path)
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

        let entry = topic.to_search_entry();
        indexer.remove_searches(&entry, searches.iter())?;
        self.remove(path)?;

        Ok(())
    }

    pub fn mark_deleted(&self, path: &RepoPath) -> Result<()> {
        let activity = self.fetch_activity(path, usize::MAX)?;

        for mut change in activity {
            let paths = change.paths();
            let prefixes = paths
                .iter()
                .map(|path| path.prefix.to_owned())
                .collect::<HashSet<String>>();

            change.mark_deleted(path);

            for prefix in prefixes {
                let reference = ChangeReference::new(&prefix, &change);
                self.save_change(&reference, &change)?;
            }
        }

        Ok(())
    }

    pub fn save_changes_index(
        &self,
        prefix: &str,
        change: &activity::Change,
        indexer: &Indexer,
        filename: &PathBuf,
    ) -> Result<()> {
        let mut activity = self.load_activity(filename, indexer.mode)?;
        let reference = ChangeReference::new(prefix, change);
        activity.changes.insert(reference);

        let dest = filename.parent().expect("expected a parent directory");
        fs::create_dir_all(&dest).ok();
        let s = serde_yaml::to_string(&activity)?;
        log::debug!("saving {:?}", filename);
        fs::write(&filename, s)?;
        Ok(())
    }

    pub fn save_change(
        &self,
        reference: &ChangeReference,
        change: &activity::Change,
    ) -> Result<()> {
        let filename = self.root.change_filename(&reference.path)?;
        let dest = filename.parent().expect("expected a parent directory");
        fs::create_dir_all(&dest).ok();
        let s = serde_yaml::to_string(&change)?;
        log::debug!("saving {:?}", filename);
        fs::write(&filename, s)?;
        Ok(())
    }

    pub fn save_link(&self, path: &RepoPath, link: &Link, indexer: &mut Indexer) -> Result<()> {
        let before = self.link(path)?;
        let before = self.link_searches(before)?;
        let after = self.link_searches(Some(link.to_owned()))?;
        indexer.update(&link.to_search_entry(), &before, &after)?;
        self.save_object(path, link)
    }

    pub fn save_topic(&self, path: &RepoPath, topic: &Topic, indexer: &mut Indexer) -> Result<()> {
        let before = self.topic(path)?;
        indexer.update_synonyms(&before, topic)?;

        let before = self.topic_searches(before)?;
        let after = self.topic_searches(Some(topic.to_owned()))?;
        indexer.update(&topic.to_search_entry(), &before, &after)?;

        self.save_object(path, topic)
    }

    pub fn search_index(
        &self,
        key: &IndexKey,
        index_type: IndexType,
        mode: IndexMode,
    ) -> Result<SearchTokenIndex> {
        let filename = self.root.index_filename(key, index_type)?;
        match mode {
            IndexMode::Replace => Ok(SearchTokenIndex::new(&filename)),
            IndexMode::ReadOnly => SearchTokenIndex::load(&filename),
            IndexMode::Update => SearchTokenIndex::load(&filename),
        }
    }

    fn save_object<T: Serialize>(&self, path: &RepoPath, object: &T) -> Result<()> {
        let filename = self.root.object_filename(&path.inner)?;
        let dest = filename.parent().expect("expected a parent directory");
        fs::create_dir_all(&dest).ok();
        let s = serde_yaml::to_string(&object)?;
        log::debug!("saving {:?}", filename);
        fs::write(&filename, s)?;
        Ok(())
    }

    // The "prefix" argument tells us which repo to look in.  The "prefix" in the method name
    // alludes to the prefix scan that is done to find matching synonyms.
    pub fn search_token_prefix_matches(
        &self,
        prefix: &str,
        token: &Phrase,
    ) -> HashSet<SearchEntry> {
        match self.index_key(prefix, token) {
            Ok(key) => match self.search_index(&key, IndexType::Search, IndexMode::ReadOnly) {
                Ok(index) => index.prefix_matches(token),
                Err(err) => {
                    log::error!("problem fetching index: {}", err);
                    HashSet::new()
                }
            },
            Err(err) => {
                log::error!("problem fetching index key: {}", err);
                HashSet::new()
            }
        }
    }

    pub fn synonym_index(
        &self,
        key: &IndexKey,
        index_type: IndexType,
        mode: IndexMode,
    ) -> Result<SynonymIndex> {
        let filename = self.root.index_filename(key, index_type)?;
        match mode {
            IndexMode::Replace => Ok(SynonymIndex::new(&filename)),
            IndexMode::ReadOnly => SynonymIndex::load(&filename),
            IndexMode::Update => SynonymIndex::load(&filename),
        }
    }

    pub fn synonym_phrase_matches(
        &self,
        prefixes: &[&str],
        name: &str,
    ) -> Result<BTreeSet<SynonymMatch>> {
        let phrase = Phrase::parse(name);
        let mut matches = BTreeSet::new();

        for prefix in prefixes {
            let key = self.index_key(prefix, &phrase)?;
            for entry in &self
                .synonym_index(&key, IndexType::SynonymPhrase, IndexMode::Update)?
                .full_matches(&phrase)?
            {
                let topic = self.fetch_topic(&entry.path)?;
                matches.insert(SynonymMatch {
                    cycle: false,
                    entry: (*entry).clone(),
                    name: name.to_string(),
                    topic,
                });
            }
        }

        Ok(matches)
    }

    // The "prefix" argument tells us which repo to look in.  The "prefix" in the method name
    // alludes to the prefix scan that is done to find matching synonyms.
    pub fn synonym_token_prefix_matches(
        &self,
        prefix: &str,
        token: &Phrase,
    ) -> BTreeSet<SynonymEntry> {
        match self.index_key(prefix, token) {
            Ok(key) => match self.synonym_index(&key, IndexType::SynonymToken, IndexMode::ReadOnly)
            {
                Ok(index) => index.prefix_matches(token),
                Err(err) => {
                    log::error!("problem fetching index: {}", err);
                    BTreeSet::new()
                }
            },
            Err(err) => {
                log::error!("problem fetching index key: {}", err);
                BTreeSet::new()
            }
        }
    }

    pub fn token_index(&self, key: &IndexKey, mode: IndexMode) -> Result<SearchTokenIndex> {
        let filename = self.root.index_filename(key, IndexType::Search)?;
        match mode {
            IndexMode::ReadOnly => SearchTokenIndex::load(&filename),
            IndexMode::Replace => Ok(SearchTokenIndex::new(&filename)),
            IndexMode::Update => SearchTokenIndex::load(&filename),
        }
    }

    pub fn topic(&self, path: &RepoPath) -> Result<Option<Topic>> {
        if self.exists(path)? {
            return Ok(Some(self.fetch_topic(&path.inner)?));
        }
        Ok(None)
    }

    pub fn topic_down_set(&self, topic_path: &RepoPath) -> DownSetIter {
        match self.fetch_topic(&topic_path.inner) {
            Ok(topic) => DownSetIter::new(self.clone(), Some(topic)),
            Err(err) => {
                log::error!("problem fetching topic: {}", err);
                DownSetIter::new(self.clone(), None)
            }
        }
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
    use super::testing::*;
    use super::*;

    #[test]
    fn parse_path_works() {
        let result = parse_path("../../wiki/12/34/5678/object.yaml");
        assert!(matches!(result, Err(Error::Repo(_))));

        let (root, id) = parse_path("../../wiki/objects/12/34/5678/object.yaml").unwrap();
        assert_eq!(root.root, PathBuf::from("../.."));
        assert_eq!(id, String::from("/wiki/12345678"));

        let (root, id) = parse_path(
            "../../wiki/objects/q-/ZZ/meNzLnZvgk_QGVjqPIpSgkADx71iWZrapMTphpQ/object.yaml",
        )
        .unwrap();
        assert_eq!(root.root, PathBuf::from("../.."));
        assert_eq!(
            id,
            String::from("/wiki/q-ZZmeNzLnZvgk_QGVjqPIpSgkADx71iWZrapMTphpQ")
        );
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

    #[test]
    fn topic_display_name() {
        let date = unix_epoch();

        let mut topic = topic("Climate change");
        topic.metadata.timerange = Some(Timerange {
            starts: date,
            prefix_format: TimerangePrefixFormat::StartYear,
        });

        assert_eq!(topic.name(Locale::EN), "1970 Climate change");
    }
}
