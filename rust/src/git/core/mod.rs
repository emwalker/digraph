use git2;
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};

use crate::prelude::*;

use super::{activity, DataRoot, GitPaths, Link, Object, Topic};

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
    pub fn ensure(root: &DataRoot, prefix: &RepoPrefix) -> Result<Self> {
        let path = root.repo_path(prefix);

        match git2::Repository::open(&path) {
            Ok(repo) => Ok(Repo { inner: repo, path }),
            Err(err) => match err.code() {
                git2::ErrorCode::NotFound => Self::init(&path),
                _ => Err(Error::Repo(format!("unable to open repo: {:?}", path))),
            },
        }
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

    pub fn blob_exists(&self, filename: &Path) -> Result<bool> {
        let blob = self.find_blob_by_filename(filename)?;
        Ok(blob.is_some())
    }

    pub(crate) fn find_blob_by_filename(&self, filename: &Path) -> Result<Option<git2::Blob>> {
        let mut path = deque_from_path(filename);
        let tree = self.tree("HEAD")?;

        if let Some(oid) = self.path_to_oid(tree, &mut path) {
            let blob = self.inner.find_blob(oid)?;
            return Ok(Some(blob));
        }

        Ok(None)
    }

    pub fn change(&self, path: &RepoPath) -> Result<activity::Change> {
        let result = self.find_blob_by_filename(&path.change_filename()?)?;
        match result {
            Some(blob) => Ok(blob.try_into()?),
            None => Err(Error::NotFound(format!("not found: {}", path))),
        }
    }

    pub fn object_exists(&self, path: &RepoPath) -> Result<bool> {
        let path = path.object_filename()?;
        self.blob_exists(&path)
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

    pub fn topic(&self, path: &RepoPath) -> Result<Option<Topic>> {
        let topic = match self.find_blob(path)? {
            Some(blob) => Some(blob.try_into()?),
            None => None,
        };
        Ok(topic)
    }

    pub fn tree(&self, name: &str) -> Result<git2::Tree> {
        let reference = self.inner.find_reference(name)?;
        Ok(reference.peel_to_tree()?)
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
