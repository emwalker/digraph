use super::{Git, Phrase};
use crate::prelude::*;

pub struct Repository {
    pub git: Git,
    pub prefix: String,
}

impl Repository {
    pub fn new(prefix: &str, git: Git) -> Self {
        Self {
            prefix: prefix.to_string(),
            git,
        }
    }

    pub fn exists(&self, path: &RepoPath) -> Result<bool> {
        self.git.exists(path)
    }

    pub fn indexed_on(&self, path: &RepoPath, phrase: &Phrase) -> Result<bool> {
        self.git.indexed_on(path, phrase)
    }
}
