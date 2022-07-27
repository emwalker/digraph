use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use super::checks::LeakedData;
use super::index::{
    ActivityIndex, ActivityIndexMap, ChangeReference, IndexKey, IndexMode, IndexType, Indexer,
    Phrase, SearchEntry, SearchTokenIndex, SynonymEntry, SynonymIndex, SynonymMatch,
};
use super::{activity, DownSetIter, Link, Object, Search, Topic};
use crate::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DataRoot {
    pub inner: PathBuf,
}

impl std::fmt::Display for DataRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

pub fn parse_path(input: &str) -> Result<(DataRoot, RepoPath)> {
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

    let path = format!("/{}/{}{}{}", org_login, part1, part2, part3);
    let root = PathBuf::from(root);

    Ok((DataRoot::new(root), RepoPath::from(&path)))
}

impl DataRoot {
    pub fn new(root: PathBuf) -> Self {
        Self { inner: root }
    }

    pub fn change_filename(&self, path: &RepoPath) -> Result<PathBuf> {
        Ok(self.basename("changes", path)?.join("change.yaml"))
    }

    pub fn change_index_filename(&self, path: &RepoPath) -> Result<PathBuf> {
        Ok(self.basename("objects", path)?.join("changes.yaml"))
    }

    pub fn basename(&self, subdirectory: &str, path: &RepoPath) -> Result<PathBuf> {
        let (part1, part2, part3) = path.parts()?;
        let basename = format!(
            "{}/{}/{}/{}/{}",
            path.org_login, subdirectory, part1, part2, part3
        );

        Ok(self.inner.join(basename))
    }

    pub fn object_filename(&self, path: &RepoPath) -> Result<PathBuf> {
        Ok(self.basename("objects", path)?.join("object.yaml"))
    }

    pub fn index_filename(&self, key: &IndexKey, index_type: IndexType) -> Result<PathBuf> {
        let prefix = &key.prefix.relative_path();

        let file_path = match index_type {
            IndexType::Search => format!("{}indexes/search/{}.yaml", prefix, key.basename),
            IndexType::SynonymPhrase => {
                format!("{}indexes/synonyms/phrases/{}.yaml", prefix, key.basename)
            }
            IndexType::SynonymToken => {
                format!("{}indexes/synonyms/tokens/{}.yaml", prefix, key.basename)
            }
        };
        Ok(self.inner.join(file_path))
    }
}

#[derive(Clone, Debug)]
pub struct Git {
    pub root: DataRoot,
    viewer: Viewer,
}

impl Git {
    pub fn new(viewer: &Viewer, root: &DataRoot) -> Self {
        Self {
            viewer: viewer.to_owned(),
            root: root.to_owned(),
        }
    }

    pub fn appears_in(&self, search: &Search, entry: &SearchEntry) -> Result<bool> {
        let path = entry.path();
        for token in &search.tokens {
            let key = self.index_key(&path.repo, token)?;
            if !self
                .token_index(&key, IndexMode::Update)?
                .indexed_on(entry, token)?
            {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub fn change_index(&self, path: &RepoPath, index_mode: IndexMode) -> Result<ActivityIndex> {
        let filename = self.root.change_index_filename(path)?;
        match index_mode {
            IndexMode::Replace => Ok(ActivityIndex::new(&filename)),
            IndexMode::ReadOnly => ActivityIndex::load(&filename),
            IndexMode::Update => ActivityIndex::load(&filename),
        }
    }

    pub fn change_filename(&self, path: &RepoPath) -> Result<PathBuf> {
        self.root.change_filename(path)
    }

    // How to handle path visibility?
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

    pub fn exists(&self, path: &RepoPath) -> Result<bool> {
        if !self.viewer.can_read(path) {
            return Ok(false);
        }
        let filename = self.root.object_filename(path)?;
        Ok(Path::new(&filename).exists())
    }

    pub fn fetch(&self, path: &RepoPath) -> Option<Object> {
        if !self.viewer.can_read(path) {
            return None;
        }

        let path = match self.root.object_filename(path) {
            Ok(path) => path,
            Err(_) => return None,
        };

        let fh = match std::fs::File::open(&path) {
            Ok(fh) => fh,
            Err(err) => {
                log::error!("problem opening file {:?}: {}", path, err);
                return None;
            }
        };

        let object: Object = match serde_yaml::from_reader(fh) {
            Ok(object) => object,
            Err(err) => {
                log::error!("problem deserializing result: {}", err);
                return None;
            }
        };

        Some(object)
    }

    pub fn fetch_activity(&self, path: &RepoPath, first: usize) -> Result<Vec<activity::Change>> {
        log::info!("fetching first {} change logs from Git for {}", first, path);
        let index = self.change_index(path, IndexMode::ReadOnly)?;
        let mut changes = vec![];

        for reference in index.references().iter().take(first) {
            let path = RepoPath::from(&reference.path);
            let filename = self.change_filename(&path)?;
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

    pub fn fetch_topic(&self, path: &RepoPath) -> Option<Topic> {
        match &self.fetch(path)? {
            Object::Topic(topic) => Some(topic.clone()),
            _ => None,
        }
    }

    pub fn fetch_link(&self, path: &RepoPath) -> Option<Link> {
        match &self.fetch(path)? {
            Object::Link(link) => Some(link.clone()),
            _ => None,
        }
    }

    // The value of `token` will sometimes need to be normalized by the caller in order for lookups
    // to work as expected.  We do not normalize the token here because some searches, e.g.,
    // of urls, are more sensitive to normalization, and so we omit it in those cases.
    pub fn index_key(&self, prefix: &RepoPrefix, token: &Phrase) -> Result<IndexKey> {
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

    pub fn leaked_data(&self) -> Result<Vec<(RepoPrefix, String)>> {
        LeakedData.call(self)
    }

    fn link(&self, path: &RepoPath) -> Result<Option<Link>> {
        if self.exists(path)? {
            return Ok(self.fetch_link(path));
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
        let filename = self.root.object_filename(path)?;
        if filename.exists() {
            fs::remove_file(filename)?;
        }

        Ok(())
    }

    pub fn remove_link(&self, path: &RepoPath, link: &Link, indexer: &mut Indexer) -> Result<()> {
        if !self.viewer.can_update(path) {
            return Err(Error::NotFound(format!("not found: {}", path)));
        }

        let searches = self.link_searches(Some(link.to_owned()))?;
        indexer.remove_searches(&link.to_search_entry(), searches.iter())?;
        self.remove(path)
    }

    pub fn remove_topic(
        &self,
        path: &RepoPath,
        topic: &Topic,
        indexer: &mut Indexer,
    ) -> Result<()> {
        if !self.viewer.can_update(path) {
            return Err(Error::NotFound(format!("not found: {}", path)));
        }

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
        if !self.viewer.can_update(path) {
            return Err(Error::NotFound(format!("not found: {}", path)));
        }

        let activity = self.fetch_activity(path, usize::MAX)?;

        for mut change in activity {
            let paths = change.paths();
            let prefixes = paths
                .iter()
                .map(|path| path.repo.to_owned())
                .collect::<HashSet<RepoPrefix>>();

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
        prefix: &RepoPrefix,
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
        let path = RepoPath::from(&reference.path);
        let filename = self.root.change_filename(&path)?;
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
        if !self.viewer.can_update(path) {
            return Err(Error::NotFound(format!("not found: {}", path)));
        }

        let filename = self.root.object_filename(path)?;
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
        prefix: &RepoPrefix,
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
        prefixes: &[&RepoPrefix],
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
                let path = RepoPath::from(&entry.path);
                if !self.viewer.can_read(&path) {
                    continue;
                }

                if let Some(topic) = self.fetch_topic(&path) {
                    matches.insert(SynonymMatch {
                        cycle: false,
                        entry: (*entry).clone(),
                        name: name.to_string(),
                        topic,
                    });
                }
            }
        }

        Ok(matches)
    }

    // The "prefix" argument tells us which repo to look in.  The "prefix" in the method name
    // alludes to the prefix scan that is done to find matching synonyms.
    pub fn synonym_token_prefix_matches(
        &self,
        prefix: &RepoPrefix,
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
            return Ok(self.fetch_topic(path));
        }
        Ok(None)
    }

    pub fn topic_down_set(&self, topic_path: &RepoPath) -> DownSetIter {
        DownSetIter::new(self.clone(), self.fetch_topic(topic_path))
    }

    fn topic_searches(&self, topic: Option<Topic>) -> Result<BTreeSet<Search>> {
        let searches = match topic {
            Some(topic) => {
                let path = topic.path();
                if !self.viewer.can_read(&path) {
                    return Err(Error::NotFound(format!("not found: {}", path)));
                }

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
