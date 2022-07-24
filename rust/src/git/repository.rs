use crate::prelude::*;

pub struct Repository {
    pub prefix: RepoPrefix,
}

impl Repository {
    pub fn new(prefix: &RepoPrefix) -> Self {
        Self {
            prefix: prefix.to_owned(),
        }
    }
}
