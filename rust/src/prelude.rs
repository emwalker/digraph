pub use async_graphql::{Context, Object, SimpleObject, ID};

pub use super::errors::Error;
pub use super::graphql::Viewer;
pub use super::http::RepoUrl;
pub use super::types::{
    Alert, Locale, RepoPath, Result, Timerange, TimerangePrefixFormat, Timestamp,
};

pub const DEFAULT_ORGANIZATION_NAME: &str = "system:default";
pub const DEFAULT_REPOSITORY_NAME: &str = "system:default";
pub const DEFAULT_ROOT_TOPIC_NAME: &str = "Everything";
pub const WIKI_ORGANIZATION_ID: &str = "45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb";
pub const WIKI_REPO_PREFIX: &str = "/wiki";
pub const WIKI_REPOSITORY_ID: &str = "32212616-fc1b-11e8-8eda-b70af6d8d09f";
pub const WIKI_ROOT_TOPIC_PATH: &str = "/wiki/df63295e-ee02-11e8-9e36-17d56b662bc8";
