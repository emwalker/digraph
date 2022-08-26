use git2;
use std::{
    collections::{HashMap, VecDeque},
    path::{Path, PathBuf},
};

use crate::prelude::*;
use crate::types::Timespec;

use super::{activity, DataRoot, GitPaths, Link, Object, RepoStats, Topic};

pub fn deque_from_path(path: &Path) -> VecDeque<String> {
    let mut deque = VecDeque::new();

    for component in path.components() {
        if let Some(part) = component.as_os_str().to_str() {
            deque.push_back(part.to_owned());
        }
    }

    deque
}

pub struct Repo {
    path: PathBuf,
    pub inner: git2::Repository,
}

impl std::fmt::Debug for Repo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Repo").field("path", &self.path).finish()
    }
}

impl Repo {
    pub fn ensure(root: &DataRoot, prefix: &RepoName) -> Result<Self> {
        let path = root.repo_path(prefix);
        Self::open(path)
    }

    pub fn open(path: PathBuf) -> Result<Self> {
        match git2::Repository::open(&path) {
            Ok(repo) => Ok(Repo { inner: repo, path }),
            Err(err) => match err.code() {
                git2::ErrorCode::NotFound => Self::init(&path),
                _ => Err(Error::Repo(format!("unable to open repo: {:?}", path))),
            },
        }
    }

    pub fn delete(root: &DataRoot, prefix: &RepoName) -> Result<()> {
        let path = root.repo_path(prefix);
        log::warn!("deleting repo {} at {:?}", prefix, path);

        match std::fs::remove_dir_all(&path) {
            Ok(()) => log::warn!("deleted {:?}", path),
            Err(err) => log::warn!("failed to delete {:?}: {}", path, err),
        }

        Ok(())
    }

    // https://github.com/rust-lang/git2-rs/blob/master/examples/init.rs#L94
    fn init(path: &PathBuf) -> Result<Self> {
        let repo = git2::Repository::init(&path)?;

        log::info!("repo not found, initializing: {:?}", path);
        let sig = git2::Signature::now("digraph-bot", "noreply@digraph.app")?;

        let tree_id = {
            let mut index = repo.index()?;
            index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
            index.write_tree()?
        };

        let tree = repo.find_tree(tree_id)?;
        repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;
        drop(tree);

        Ok(Repo {
            inner: repo,
            path: path.to_owned(),
        })
    }

    pub fn add_blob(&self, ser: &[u8]) -> Result<git2::Oid> {
        Ok(self.inner.odb()?.write(git2::ObjectType::Blob, ser)?)
    }

    pub fn commit(&self, oid: git2::Oid) -> Result<git2::Commit<'_>> {
        Ok(self.inner.find_commit(oid)?)
    }

    pub fn commit_oid(&self, _timespec: &Timespec) -> Result<git2::Oid> {
        let reference = self.inner.find_reference("HEAD")?;
        let commit = reference.peel_to_commit()?;
        Ok(commit.id())
    }

    pub fn duplicate(&self) -> Result<Self> {
        Self::open(self.path.to_owned())
    }

    fn path_to_oid(&self, tree: git2::Tree, path: &mut VecDeque<String>) -> Option<git2::Oid> {
        let name = path.pop_front()?;

        if path.is_empty() {
            let entry = tree.get_name(&name)?;
            return Some(entry.id());
        }

        let oid = tree.get_name(&name)?.id();

        let subtree = match self.inner.find_tree(oid) {
            Ok(subtree) => subtree,
            Err(err) => match err.code() {
                git2::ErrorCode::NotFound => return None,
                _ => {
                    log::error!("failed to fetch subtree: {:?}", err);
                    return None;
                }
            },
        };

        self.path_to_oid(subtree, path)
    }
}

impl<'repo> TryInto<Link> for git2::Blob<'repo> {
    type Error = Error;

    fn try_into(self) -> Result<Link> {
        let bytes = self.content();
        let link: Link = serde_yaml::from_slice(bytes)?;
        Ok(link)
    }
}

impl<'repo> TryInto<Object> for git2::Blob<'repo> {
    type Error = Error;

    fn try_into(self) -> Result<Object> {
        let bytes = self.content();
        let object: Object = serde_yaml::from_slice(bytes)?;
        Ok(object)
    }
}

impl<'repo> TryInto<Topic> for git2::Blob<'repo> {
    type Error = Error;

    fn try_into(self) -> Result<Topic> {
        let bytes = self.content();
        let topic: Topic = serde_yaml::from_slice(bytes)?;
        Ok(topic)
    }
}

#[derive(Debug)]
pub struct View {
    pub repo: Repo,
    pub commit: git2::Oid,
}

impl View {
    pub fn ensure(root: &DataRoot, prefix: &RepoName, timespec: &Timespec) -> Result<Self> {
        let repo = Repo::ensure(root, prefix)?;
        let commit = repo.commit_oid(timespec)?;
        Ok(Self { repo, commit })
    }

    pub fn blob_exists(&self, filename: &Path) -> Result<bool> {
        let blob = self.find_blob_by_filename(filename)?;
        Ok(blob.is_some())
    }

    pub fn change(&self, path: &RepoPath) -> Result<activity::Change> {
        let result = self.find_blob_by_filename(&path.change_filename()?)?;
        match result {
            Some(blob) => Ok(blob.try_into()?),
            None => Err(Error::NotFound(format!("not found: {}", path))),
        }
    }

    pub fn duplicate(&self) -> Result<Self> {
        Ok(Self {
            repo: self.repo.duplicate()?,
            commit: self.commit,
        })
    }

    pub(crate) fn find_blob_by_filename(&self, filename: &Path) -> Result<Option<git2::Blob>> {
        let mut path = deque_from_path(filename);
        let tree = self.repo.commit(self.commit)?.tree()?;

        if let Some(oid) = self.repo.path_to_oid(tree, &mut path) {
            let blob = self.repo.inner.find_blob(oid)?;
            return Ok(Some(blob));
        }

        Ok(None)
    }

    fn find_blob(&self, path: &RepoPath) -> Result<Option<git2::Blob>> {
        self.find_blob_by_filename(&path.object_filename()?)
    }

    pub fn link(&self, path: &RepoPath) -> Result<Option<Link>> {
        let link = match self.find_blob(path)? {
            Some(blob) => Some(blob.try_into()?),
            None => None,
        };
        Ok(link)
    }

    pub fn object(&self, path: &RepoPath) -> Result<Option<Object>> {
        let object = match self.find_blob(path)? {
            Some(blob) => Some(blob.try_into()?),
            None => None,
        };
        Ok(object)
    }

    pub fn object_exists(&self, path: &RepoPath) -> Result<bool> {
        let path = path.object_filename()?;
        self.blob_exists(&path)
    }

    pub fn stats(&self) -> Result<RepoStats> {
        log::info!("computing stats for view {:?}", self);
        let tree = self.repo.commit(self.commit)?.tree()?;

        let mut topic_count = 0;
        let mut link_count = 0;

        fn has_subsequence(haystack: &[u8], needle: &[u8]) -> bool {
            haystack
                .windows(needle.len())
                .any(|window| window == needle)
        }

        tree.walk(git2::TreeWalkMode::PreOrder, |_root, entry| {
            if let Some(name) = entry.name() {
                if name != "object.yaml" {
                    return git2::TreeWalkResult::Ok;
                }

                let oid = entry.id();

                match self.repo.inner.find_blob(oid) {
                    Ok(blob) => {
                        let haystack = blob.content();
                        if has_subsequence(haystack, b"\nkind: Topic\n") {
                            topic_count += 1;
                        } else if has_subsequence(haystack, b"\nkind: Link\n") {
                            link_count += 1;
                        }
                    }

                    Err(err) => {
                        log::error!("failed to convert to string: {}", err);
                    }
                }
            }

            git2::TreeWalkResult::Ok
        })?;

        Ok(RepoStats {
            computing: false,
            link_count: Some(link_count),
            topic_count: Some(topic_count),
        })
    }

    pub fn topic(&self, path: &RepoPath) -> Result<Option<Topic>> {
        let topic = match self.find_blob(path)? {
            Some(blob) => Some(blob.try_into()?),
            None => None,
        };
        Ok(topic)
    }
}

#[derive(Debug, Default)]
pub struct Tree {
    files: HashMap<String, Option<git2::Oid>>,
    subtrees: HashMap<String, Tree>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_blob(&mut self, path: &mut VecDeque<String>, oid: &Option<git2::Oid>) {
        if let Some(name) = path.pop_front() {
            if path.is_empty() {
                self.files.insert(name, oid.to_owned());
            } else {
                let subtree = self.subtrees.entry(name).or_insert_with(Tree::new);
                subtree.add_blob(path, oid);
            }
        }
    }

    pub fn write(&self, repo: &git2::Repository, before: Option<git2::Tree>) -> Result<git2::Oid> {
        let mut builder = repo.treebuilder(before.as_ref())?;

        for (filename, subtree) in &self.subtrees {
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

            let oid = subtree.write(repo, before)?;
            builder.insert(filename, oid, 0o040000)?;
        }

        for (filename, oid) in &self.files {
            if let Some(oid) = oid {
                builder.insert(filename, oid.to_owned(), 0o100644)?;
            } else {
                builder.remove(filename)?;
            }
        }

        let oid = builder.write()?;
        Ok(oid)
    }
}

#[derive(Debug, Default)]
pub struct Update(HashMap<RepoName, Tree>);

impl Update {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, repo: &RepoName, filename: &Path, oid: &Option<git2::Oid>) -> Result<()> {
        let mut deque = deque_from_path(filename);
        let tree = self.0.entry(repo.to_owned()).or_insert_with(Tree::new);
        tree.add_blob(&mut deque, oid);
        Ok(())
    }

    // Writes should only target HEAD
    pub fn write(&self, root: &DataRoot, sig: &git2::Signature, message: &str) -> Result<()> {
        for (prefix, tree) in &self.0 {
            let repo = Repo::ensure(root, prefix)?;
            let head = repo.inner.find_reference("HEAD")?;
            let before = head.peel_to_tree()?;
            let oid = tree.write(&repo.inner, Some(before))?;
            let tree = repo.inner.find_tree(oid)?;
            let parent = head.peel_to_commit()?;
            repo.inner
                .commit(Some("HEAD"), sig, sig, message, &tree, &[&parent])?;
        }

        Ok(())
    }
}
