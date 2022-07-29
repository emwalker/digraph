use git2;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::path::PathBuf;

use super::checks::LeakedData;
use super::index::{
    ActivityIndex, ChangeReference, GitIndexKey, IndexMode, IndexType, Indexer, Phrase,
    SaveChangesForPrefix, SearchEntry, SynonymEntry, SynonymMatch,
};
use super::{
    activity, core, DownSetIter, Link, Object, Search, SearchTokenIndex, SynonymIndex, Topic,
};
use crate::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DataRoot {
    pub path: PathBuf,
}

impl std::fmt::Display for DataRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.path)
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
        Self { path: root }
    }

    pub fn repo_path(&self, prefix: &RepoPrefix) -> PathBuf {
        self.path.join(prefix.relative_path())
    }
}

pub trait GitPaths {
    fn change_filename(&self) -> Result<PathBuf> {
        Ok(self.relative_path("changes")?.join("change.yaml"))
    }

    fn activity_log_filename(&self) -> Result<PathBuf> {
        Ok(self.relative_path("objects")?.join("changes.yaml"))
    }

    fn relative_path(&self, subdirectory: &str) -> Result<PathBuf> {
        let (part1, part2, part3) = self.parts()?;
        let relative_path = PathBuf::from([subdirectory, part1, part2, part3].join("/"));
        Ok(relative_path)
    }

    fn object_filename(&self) -> Result<PathBuf> {
        Ok(self.relative_path("objects")?.join("object.yaml"))
    }

    fn parts(&self) -> Result<(&str, &str, &str)>;
}

impl GitPaths for RepoPath {
    fn parts(&self) -> Result<(&str, &str, &str)> {
        self.parts()
    }
}

#[derive(Clone, Debug)]
pub struct Client {
    pub root: DataRoot,
    viewer: Viewer,
}

impl Client {
    pub fn new(viewer: &Viewer, root: &DataRoot) -> Self {
        Self {
            root: root.to_owned(),
            viewer: viewer.to_owned(),
        }
    }

    pub fn appears_in(&self, search: &Search, entry: &SearchEntry) -> Result<bool> {
        let path = entry.path();
        for token in &search.tokens {
            let key = path.repo.index_key(token)?;
            if !key
                .token_index(self, IndexMode::Update)?
                .indexed_on(entry, token)?
            {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub fn fetch_activity_log(
        &self,
        path: &RepoPath,
        index_mode: IndexMode,
    ) -> Result<ActivityIndex> {
        let filename = path.activity_log_filename()?;
        let repo = self.repo(&path.repo)?;
        match index_mode {
            IndexMode::Replace => Ok(ActivityIndex::new(&filename)),
            IndexMode::ReadOnly => ActivityIndex::load(&filename, &repo),
            IndexMode::Update => ActivityIndex::load(&filename, &repo),
        }
    }

    // How to handle path visibility?
    fn cycle_exists(&self, descendant_path: &RepoPath, ancestor_path: &RepoPath) -> Result<bool> {
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
        let repo = self.repo(&path.repo)?;
        repo.object_exists(path)
    }

    pub fn fetch(&self, path: &RepoPath) -> Option<Object> {
        if !self.viewer.can_read(path) {
            log::warn!("viewer cannot read path: {}", path);
            return None;
        }

        match self.repo(&path.repo) {
            Ok(repo) => match repo.object(path) {
                Ok(object) => object,
                Err(err) => {
                    println!("failed to fetch object: {:?}", err);
                    None
                }
            },
            Err(err) => {
                println!("failed to open repo: {:?}", err);
                None
            }
        }
    }

    pub fn fetch_synonym_index(
        &self,
        prefix: &RepoPrefix,
        filename: &PathBuf,
    ) -> Result<SynonymIndex> {
        let repo = self.repo(prefix)?;
        let result = repo.find_blob_by_filename(filename)?;
        match result {
            Some(blob) => {
                let index = blob.try_into()?;
                Ok(SynonymIndex::make(filename.to_owned(), index))
            }
            None => Ok(SynonymIndex::new(filename)),
        }
    }

    pub fn fetch_token_index(
        &self,
        prefix: &RepoPrefix,
        filename: &PathBuf,
    ) -> Result<SearchTokenIndex> {
        let repo = self.repo(prefix)?;
        let result = repo.find_blob_by_filename(filename)?;
        match result {
            Some(blob) => Ok(SearchTokenIndex::make(
                filename.to_owned(),
                blob.try_into()?,
            )),
            None => Ok(SearchTokenIndex::new(filename)),
        }
    }

    pub fn fetch_activity(&self, path: &RepoPath, first: usize) -> Result<Vec<activity::Change>> {
        log::info!("fetching first {} change logs from Git for {}", first, path);
        let index = self.fetch_activity_log(path, IndexMode::ReadOnly)?;
        let mut changes = vec![];

        for reference in index.references().iter().take(first) {
            let path = RepoPath::from(&reference.path);
            let repo = self.repo(&path.repo)?;
            let result = repo.change(&path);
            match result {
                Ok(change) => changes.push(change),
                Err(err) => log::warn!("failed to load change: {}", err),
            }
        }

        Ok(changes)
    }

    pub fn fetch_topic(&self, path: &RepoPath) -> Option<Topic> {
        match &self.fetch(path)? {
            Object::Topic(topic) => Some(topic.to_owned()),
            _ => None,
        }
    }

    pub fn fetch_link(&self, path: &RepoPath) -> Option<Link> {
        match &self.fetch(path)? {
            Object::Link(link) => Some(link.to_owned()),
            other => {
                println!("expected a link, found: {:?}", other);
                None
            }
        }
    }

    pub fn leaked_data(&self) -> Result<Vec<(RepoPrefix, String)>> {
        LeakedData.call(self)
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

    fn repo(&self, prefix: &RepoPrefix) -> Result<core::Repo> {
        core::Repo::ensure(&self.root, prefix)
    }

    // The "prefix" argument tells us which repo to look in.  The "prefix" in the method name
    // alludes to the prefix scan that is done to find matching synonyms.
    pub fn search_token_prefix_matches(
        &self,
        prefix: &RepoPrefix,
        token: &Phrase,
    ) -> Result<HashSet<SearchEntry>> {
        let key = prefix.index_key(token)?;
        let index = key.search_index(self, IndexType::Search, IndexMode::ReadOnly)?;
        Ok(index.prefix_matches(token))
    }

    pub fn synonym_phrase_matches(
        &self,
        prefixes: &[&RepoPrefix],
        name: &str,
    ) -> Result<BTreeSet<SynonymMatch>> {
        let phrase = Phrase::parse(name);
        let mut matches = BTreeSet::new();

        for prefix in prefixes {
            let key = prefix.index_key(&phrase)?;
            for entry in &key
                .synonym_index(self, IndexType::SynonymPhrase, IndexMode::Update)?
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
        match prefix.index_key(token) {
            Ok(key) => {
                match key.synonym_index(self, IndexType::SynonymToken, IndexMode::ReadOnly) {
                    Ok(index) => index.prefix_matches(token),
                    Err(err) => {
                        log::error!("problem fetching index: {}", err);
                        BTreeSet::new()
                    }
                }
            }
            Err(err) => {
                log::error!("problem fetching index key: {}", err);
                BTreeSet::new()
            }
        }
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

    pub fn update(&self, mode: IndexMode) -> Result<BatchUpdate> {
        Ok(BatchUpdate {
            client: self.to_owned(),
            files: BTreeMap::new(),
            indexer: Indexer::new(mode),
        })
    }
}

pub struct BatchUpdate {
    client: Client,
    indexer: Indexer,
    files: BTreeMap<(RepoPrefix, PathBuf), Option<git2::Oid>>,
}

impl std::fmt::Debug for BatchUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreeBuilder")
            .field("client", &self.client)
            .field("indexer", &self.indexer)
            .finish()
    }
}

impl BatchUpdate {
    pub fn add_change(&mut self, change: &activity::Change) -> Result<()> {
        let repos = self.indexer.add_change(&self.client, change)?;

        for repo in repos {
            // Write the individual change to a /prefix/changes/ directory
            let reference = ChangeReference::new(&repo, change);
            self.save_change(&reference, change)?;
        }

        Ok(())
    }

    fn check_can_update(&self, path: &RepoPath) -> Result<()> {
        if !self.client.viewer.can_update(path) {
            return Err(Error::NotFound(format!("not found: {}", path)));
        }
        Ok(())
    }

    pub fn activity_log(&self, path: &RepoPath, index_mode: IndexMode) -> Result<ActivityIndex> {
        self.client.fetch_activity_log(path, index_mode)
    }

    pub fn cycle_exists(
        &self,
        descendant_path: &RepoPath,
        ancestor_path: &RepoPath,
    ) -> Result<bool> {
        self.client.cycle_exists(descendant_path, ancestor_path)
    }

    pub fn exists(&self, path: &RepoPath) -> Result<bool> {
        self.client.exists(path)
    }

    pub fn fetch(&self, path: &RepoPath) -> Option<Object> {
        self.client.fetch(path)
    }

    pub fn fetch_link(&self, path: &RepoPath) -> Option<Link> {
        self.client.fetch_link(path)
    }

    pub fn fetch_topic(&self, path: &RepoPath) -> Option<Topic> {
        self.client.fetch_topic(path)
    }

    pub fn mark_deleted(&mut self, path: &RepoPath) -> Result<()> {
        self.check_can_update(path)?;

        let activity = self.client.fetch_activity(path, usize::MAX)?;

        for mut change in activity {
            let paths = change.paths();
            let repos = paths
                .iter()
                .map(|path| path.repo.to_owned())
                .collect::<HashSet<RepoPrefix>>();

            change.mark_deleted(path);

            for repo in repos {
                let reference = ChangeReference::new(&repo, &change);
                self.save_change(&reference, &change)?;
            }
        }

        Ok(())
    }

    pub fn remove(&mut self, path: &RepoPath) -> Result<()> {
        let filename = path.object_filename()?;
        self.files.insert((path.repo.to_owned(), filename), None);
        Ok(())
    }

    pub fn remove_link(&mut self, path: &RepoPath, link: &Link) -> Result<()> {
        self.check_can_update(path)?;

        let searches = self.client.link_searches(Some(link.to_owned()))?;
        self.indexer
            .remove_searches(&self.client, &link.to_search_entry(), searches.iter())?;
        self.remove(path)
    }

    pub fn remove_topic(&mut self, path: &RepoPath, topic: &Topic) -> Result<()> {
        self.check_can_update(path)?;

        self.indexer.remove_synonyms(&self.client, path, topic)?;

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
        self.indexer
            .remove_searches(&self.client, &entry, searches.iter())?;
        self.remove(path)?;

        Ok(())
    }

    fn repo(&self, prefix: &RepoPrefix) -> Result<core::Repo> {
        self.client.repo(prefix)
    }

    pub fn write<S>(&self, store: &S) -> Result<()>
    where
        S: SaveChangesForPrefix,
    {
        self.indexer.save(store)?;

        #[derive(Debug, Default)]
        struct TreeNode {
            files: HashMap<String, Option<git2::Oid>>,
            subtrees: HashMap<String, TreeNode>,
        }

        impl TreeNode {
            fn new() -> Self {
                Self {
                    ..Default::default()
                }
            }
        }

        fn add_blob(tree: &mut TreeNode, path: &mut VecDeque<String>, oid: &Option<git2::Oid>) {
            if let Some(name) = path.pop_front() {
                if path.is_empty() {
                    tree.files.insert(name, oid.to_owned());
                } else {
                    let subtree = tree.subtrees.entry(name).or_insert_with(TreeNode::new);
                    add_blob(subtree, path, oid);
                }
            }
        }

        fn write(
            repo: &git2::Repository,
            before: Option<git2::Tree>,
            tree: &TreeNode,
        ) -> Result<git2::Oid> {
            let mut builder = repo.treebuilder(before.as_ref())?;

            for (filename, subtree) in &tree.subtrees {
                let before = match &before {
                    Some(before) => match before.get_name(filename) {
                        Some(entry) => {
                            let tree_id = entry.id();
                            Some(repo.find_tree(tree_id)?)
                        }
                        None => None,
                    },
                    None => None,
                };

                let oid = write(repo, before, subtree)?;
                builder.insert(filename, oid, 0o040000)?;
            }

            for (filename, oid) in &tree.files {
                if let Some(oid) = oid {
                    builder.insert(filename, oid.to_owned(), 0o100644)?;
                } else {
                    builder.remove(filename)?;
                }
            }

            let oid = builder.write()?;
            Ok(oid)
        }

        let sig = git2::Signature::now("digraph-bot", "digraph-bot@digraph.app")?;

        let mut repos = HashMap::new();

        // Write topics and links
        for ((repo, filename), oid) in self.files.iter() {
            let mut deque = core::deque_from_path(filename);
            let root = repos.entry(repo).or_insert_with(TreeNode::new);
            add_blob(root, &mut deque, oid);
        }

        let index_files = self.indexer.files()?;

        // Write activity logs
        for (prefix, filename, ser) in index_files {
            let repo = self.repo(prefix)?;
            let oid = repo.add_blob(ser.as_bytes())?;
            let mut deque = core::deque_from_path(&filename);
            let root = repos.entry(prefix).or_insert_with(TreeNode::new);
            add_blob(root, &mut deque, &Some(oid));
        }

        for (repo, root) in repos {
            let repo = self.client.repo(repo)?;
            let head = repo.inner.find_reference("HEAD")?;
            let before = head.peel_to_tree()?;
            let oid = write(&repo.inner, Some(before), &root)?;
            let tree = repo.inner.find_tree(oid)?;
            let parent = head.peel_to_commit()?;
            repo.inner
                .commit(Some("HEAD"), &sig, &sig, "Updated repo", &tree, &[&parent])?;
        }

        Ok(())
    }

    pub fn save_change(
        &mut self,
        reference: &ChangeReference,
        change: &activity::Change,
    ) -> Result<()> {
        self.indexer.add_change(&self.client, change)?;

        let path = RepoPath::from(&reference.path);
        let s = serde_yaml::to_string(&change)?;
        let oid = self.repo(&path.repo)?.add_blob(s.as_bytes())?;
        self.files
            .insert((path.repo.to_owned(), path.change_filename()?), Some(oid));

        Ok(())
    }

    pub fn save_link(&mut self, path: &RepoPath, link: &Link) -> Result<()> {
        self.check_can_update(path)?;

        let repo = self.client.repo(&path.repo)?;
        let before = repo.link(path)?;
        let before = self.client.link_searches(before)?;
        let after = self.client.link_searches(Some(link.to_owned()))?;
        self.indexer
            .update(&self.client, &link.to_search_entry(), &before, &after)?;
        let s = serde_yaml::to_string(&link)?;
        let oid = repo.add_blob(s.as_bytes())?;
        self.save_object(path, oid)
    }

    fn save_object(&mut self, path: &RepoPath, oid: git2::Oid) -> Result<()> {
        let filename = path.object_filename()?;
        self.files
            .insert((path.repo.to_owned(), filename), Some(oid));
        Ok(())
    }

    pub fn save_topic(&mut self, path: &RepoPath, topic: &Topic) -> Result<()> {
        self.check_can_update(path)?;

        let repo = self.client.repo(&path.repo)?;
        let before = repo.topic(path)?;
        self.indexer.update_synonyms(&self.client, &before, topic)?;

        let before = self.client.topic_searches(before)?;
        let after = self.client.topic_searches(Some(topic.to_owned()))?;
        self.indexer
            .update(&self.client, &topic.to_search_entry(), &before, &after)?;
        let s = serde_yaml::to_string(&topic)?;
        let oid = repo.add_blob(s.as_bytes())?;

        self.save_object(path, oid)
    }

    pub fn synonym_phrase_matches(
        &self,
        prefixes: &[&RepoPrefix],
        name: &str,
    ) -> Result<BTreeSet<SynonymMatch>> {
        self.client.synonym_phrase_matches(prefixes, name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod git_paths {
        use super::*;

        #[test]
        fn object_filename() {
            assert!(matches!(
                RepoPath::from("1234").object_filename(),
                Err(Error::Repo(_))
            ));
            assert!(matches!(
                RepoPath::from("wiki/123456").object_filename(),
                Err(Error::Repo(_))
            ));
            assert!(matches!(
                RepoPath::from("/wiki/1234").object_filename(),
                Err(Error::Repo(_))
            ));

            assert_eq!(
                RepoPath::from("/wiki/123456").object_filename().unwrap(),
                PathBuf::from("objects/12/34/56/object.yaml")
            );

            assert_eq!(
                RepoPath::from("/with-dash/123456")
                    .object_filename()
                    .unwrap(),
                PathBuf::from("objects/12/34/56/object.yaml")
            );
        }
    }
}
