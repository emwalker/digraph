use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

pub use super::errors::Error;
pub use super::graphql::Viewer;
pub use super::http::RepoUrl;
pub use super::repo::RepoPath;
pub use async_graphql::{Context, Object, SimpleObject, ID};

pub const DEFAULT_ORGANIZATION_NAME: &str = "system:default";
pub const DEFAULT_REPOSITORY_NAME: &str = "system:default";
pub const DEFAULT_ROOT_TOPIC_NAME: &str = "Everything";
pub const WIKI_ORGANIZATION_ID: &str = "45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb";
pub const WIKI_REPO_PREFIX: &str = "/wiki";
pub const WIKI_REPOSITORY_ID: &str = "32212616-fc1b-11e8-8eda-b70af6d8d09f";
pub const WIKI_ROOT_TOPIC_PATH: &str = "/wiki/df63295e-ee02-11e8-9e36-17d56b662bc8";

pub type Result<T> = std::result::Result<T, Error>;
pub type Timestamp = chrono::DateTime<chrono::Utc>;

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    EnumString,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    strum_macros::Display,
)]
#[serde(rename_all = "lowercase")]
pub enum Locale {
    EN,
    AR,
    DE,
    EL,
    ES,
    FA,
    FI,
    FR,
    HI,
    IT,
    JA,
    JI,
    KO,
    LA,
    NL,
    NO,
    PT,
    RU,
    SV,
    TR,
    UA,
    UK,
    ZH,
}

pub fn random_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect()
}
