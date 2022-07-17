use super::{Git, Search, SearchEntry};
use crate::prelude::*;

pub struct Repository {
    pub git: Git,
    pub prefix: RepoPrefix,
}

impl Repository {
    pub fn new(prefix: &RepoPrefix, git: Git) -> Self {
        Self {
            prefix: prefix.to_owned(),
            git,
        }
    }

    pub fn exists(&self, path: &RepoPath) -> Result<bool> {
        self.git.exists(path)
    }

    pub fn appears_in(&self, search: &Search, entry: &SearchEntry) -> Result<bool> {
        self.git.indexed_on(entry, search)
    }
}
