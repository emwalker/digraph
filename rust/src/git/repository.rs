use crate::prelude::*;

pub struct Repository {
    pub prefix: RepoId,
}

impl Repository {
    pub fn new(prefix: &RepoId) -> Self {
        Self {
            prefix: prefix.to_owned(),
        }
    }
}
