use crate::prelude::*;

pub struct Repository {
    pub prefix: RepoName,
}

impl Repository {
    pub fn new(prefix: &RepoName) -> Self {
        Self {
            prefix: prefix.to_owned(),
        }
    }
}
