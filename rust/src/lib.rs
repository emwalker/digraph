extern crate redis as redis_rs;
extern crate strum;
extern crate strum_macros;

#[macro_use]
extern crate quick_error;

#[macro_use]
extern crate derivative;

use std::collections::HashSet;

pub mod config;
pub mod db;
pub mod errors;
pub mod git;
pub mod graphql;
pub mod http;
pub mod prelude;
use prelude::{RepoPath, Result};
mod psql;
pub mod redis;
pub mod repo;

pub enum Alert {
    Danger(String),
    Success(String),
    Warning(String),
}

pub trait DownSet {
    fn transitive_closure(&self, topic_paths: &[&RepoPath]) -> Result<HashSet<String>>;

    fn down_set(&self, key: &RepoPath) -> HashSet<String>;
}
