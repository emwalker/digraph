use base64::engine::{self, Engine};
use geotime::Geotime;
use lazy_static::lazy_static;
use rand::{distributions::Alphanumeric, Rng};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx;
use std::collections::HashSet;
use strum_macros::EnumString;
use uuid::Uuid;

use crate::{
    errors::Error,
    prelude::{OTHER_REPOSITORY_ID, ROOT_TOPIC_ID, WIKI_REPOSITORY_ID},
};

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
#[strum(serialize_all = "lowercase")]
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

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[allow(dead_code)]
pub struct RepoId(Uuid);

impl TryFrom<&str> for RepoId {
    type Error = Error;

    fn try_from(id: &str) -> Result<Self> {
        Ok(Self(id.try_into()?))
    }
}

impl TryFrom<&String> for RepoId {
    type Error = Error;

    fn try_from(id: &String) -> Result<Self> {
        Self::try_from(id.as_str())
    }
}

impl TryFrom<String> for RepoId {
    type Error = Error;

    fn try_from(id: String) -> Result<Self> {
        Self::try_from(id.as_str())
    }
}

impl TryFrom<&sqlx::types::Uuid> for RepoId {
    type Error = Error;

    fn try_from(id: &sqlx::types::Uuid) -> Result<Self> {
        Self::try_from(&id.to_string())
    }
}

impl std::fmt::Display for RepoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<sqlx::types::Uuid> for RepoId {
    fn from(value: sqlx::types::Uuid) -> Self {
        Self(value)
    }
}

impl RepoId {
    pub fn make() -> Self {
        Self(Uuid::new_v4())
    }

    // For testing
    pub fn other() -> Self {
        Self::try_from(OTHER_REPOSITORY_ID).unwrap()
    }

    pub fn is_wiki(&self) -> bool {
        lazy_static! {
            static ref WIKI: Uuid = WIKI_REPOSITORY_ID.try_into().unwrap();
        }
        self.0 == *WIKI
    }

    pub fn relative_path(&self) -> String {
        format!("{}/", self.0)
    }

    pub fn root_topic_id(&self) -> ExternalId {
        ROOT_TOPIC_ID
            .try_into()
            .expect("failed to convert root topic id")
    }

    pub fn wiki() -> Self {
        Self::try_from(WIKI_REPOSITORY_ID).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct RepoIds(Vec<RepoId>);

impl TryFrom<&[String]> for RepoIds {
    type Error = Error;

    fn try_from(repo_ids: &[String]) -> Result<Self> {
        Ok(Self(
            repo_ids
                .iter()
                .map(RepoId::try_from)
                .collect::<Result<Vec<RepoId>>>()?,
        ))
    }
}

impl From<Vec<RepoId>> for RepoIds {
    fn from(ids: Vec<RepoId>) -> Self {
        Self(ids)
    }
}

impl TryFrom<&Vec<String>> for RepoIds {
    type Error = Error;

    fn try_from(repo_ids: &Vec<String>) -> Result<Self> {
        Self::try_from(repo_ids.as_slice())
    }
}

impl TryFrom<&Vec<sqlx::types::Uuid>> for RepoIds {
    type Error = Error;

    fn try_from(repo_ids: &Vec<sqlx::types::Uuid>) -> Result<Self> {
        Ok(Self(
            repo_ids
                .iter()
                .map(RepoId::try_from)
                .collect::<Result<Vec<RepoId>>>()?,
        ))
    }
}

impl From<&[RepoId]> for RepoIds {
    fn from(repo_ids: &[RepoId]) -> Self {
        Self(repo_ids.to_owned())
    }
}

impl From<&Vec<RepoId>> for RepoIds {
    fn from(repo_ids: &Vec<RepoId>) -> Self {
        Self(repo_ids.to_vec())
    }
}

impl From<&RepoIds> for Vec<RepoId> {
    fn from(repos: &RepoIds) -> Self {
        repos.0.to_owned()
    }
}

impl RepoIds {
    pub fn include(&self, repo: &RepoId) -> bool {
        self.0.iter().any(|id| id == repo)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, RepoId> {
        self.0.iter()
    }

    pub fn to_vec(&self) -> Vec<String> {
        let mut repo_ids = vec![];
        for id in &self.0 {
            repo_ids.push(id.to_string());
        }
        repo_ids
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ExternalId(String);

impl std::fmt::Display for ExternalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for ExternalId {
    type Error = Error;

    fn try_from(id: &str) -> Result<Self> {
        Self::try_from(id.to_owned())
    }
}

impl TryFrom<&String> for ExternalId {
    type Error = Error;

    fn try_from(id: &String) -> Result<Self> {
        Self::try_from(id.to_owned())
    }
}

impl TryFrom<String> for ExternalId {
    type Error = Error;

    fn try_from(id: String) -> Result<Self> {
        if id.len() < 5 {
            return Err(Error::Path(format!("bad id: {}", id)));
        }

        for c in id.chars() {
            if c.is_alphanumeric() {
                continue;
            }

            if c == '-' || c == '_' {
                continue;
            }

            return Err(Error::Path(format!("bad id: {}", id)));
        }

        Ok(Self(id))
    }
}

impl ExternalId {
    pub fn make() -> Self {
        let s: String = random_id();
        Self::try_from(&s).unwrap()
    }

    pub fn root_topic() -> Self {
        Self::try_from(ROOT_TOPIC_ID).unwrap()
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn is_root(&self) -> bool {
        self.0 == ROOT_TOPIC_ID
    }

    pub fn parts(&self) -> Result<(&str, &str, &str)> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([\w_-]{2})([\w_-]{2})([\w_-]+)$").unwrap();
        }

        let cap = RE
            .captures(self.0.as_str())
            .ok_or_else(|| Error::Path(format!("bad id: {}", self)))?;

        if cap.len() != 4 {
            return Err(Error::Path(format!("bad id: {}", self)));
        }

        match (cap.get(1), cap.get(2), cap.get(3)) {
            (Some(part1), Some(part2), Some(part3)) => {
                Ok((part1.as_str(), part2.as_str(), part3.as_str()))
            }
            _ => Err(Error::Path(format!("bad id: {}", self))),
        }
    }
}

// Lookup key for a topic or link from the standpoint of a specific repo.
//
// Links and topics are shown differently depending on what repo is currently selected by the
// user.  It is convenient to handle objects viewed from the standpoint of different context repos
// as different objects with different lookup keys, because knowing the context allows us to select
// a "display" repo topic or repo link to be used for display purposes. If we select the display
// repo topic or repo link up front on the basis of the context repo, we don't have to calculate it
// dynamically over and over.  We need a distinct lookup key for each context repo so that we can
// cache these different objects in the same maps.  We might need to have different versions of the
// same link or topic if we're in the process of changing the selected repo as a result of a
// mutation, for example.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Okey(pub ExternalId, pub RepoId);

pub fn random_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(43)
        .map(char::from)
        .collect()
}

pub enum Alert {
    Danger(String),
    Success(String),
    Warning(String),
}

#[derive(Clone, Debug)]
pub struct Timespec;

#[derive(Debug)]
pub struct TopicPath {
    pub repo_id: RepoId,
    pub topic_id: ExternalId,
    pub topic_oid: git2::Oid,
}

pub trait Downset {
    fn intersection(&self, topic_ids: &[TopicPath]) -> Result<HashSet<ExternalId>>;

    fn downset(&self, path: &TopicPath) -> HashSet<ExternalId>;
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Timerange {
    pub starts: geotime::LexicalGeohash,
    pub prefix_format: TimerangePrefixFormat,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimerangePrefixFormat {
    #[serde(alias = "none")]
    None,
    #[serde(alias = "startYear")]
    StartYear,
    #[serde(alias = "startYearMonth")]
    StartYearMonth,
}

impl From<&str> for TimerangePrefixFormat {
    fn from(format: &str) -> Self {
        match format {
            "NONE" => Self::None,
            "START_YEAR" => Self::StartYear,
            "START_YEAR_MONTH" => Self::StartYearMonth,
            _ => Self::None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TimerangePrefix {
    None(Geotime),
    StartYear(Geotime),
    StartYearMonth(Geotime),
}

impl From<&Option<Timerange>> for TimerangePrefix {
    fn from(timerange: &Option<Timerange>) -> Self {
        match &timerange {
            Some(Timerange {
                starts,
                prefix_format,
                ..
            }) => match prefix_format {
                TimerangePrefixFormat::None => Self::None((*starts).into()),
                TimerangePrefixFormat::StartYear => Self::StartYear((*starts).into()),
                TimerangePrefixFormat::StartYearMonth => Self::StartYearMonth((*starts).into()),
            },
            None => Self::None(Geotime::now()),
        }
    }
}

impl From<&Timerange> for TimerangePrefix {
    fn from(timerange: &Timerange) -> Self {
        let Timerange {
            starts,
            prefix_format,
        } = timerange;
        match prefix_format {
            TimerangePrefixFormat::None => Self::None((*starts).into()),
            TimerangePrefixFormat::StartYear => Self::StartYear((*starts).into()),
            TimerangePrefixFormat::StartYearMonth => Self::StartYearMonth((*starts).into()),
        }
    }
}

impl TimerangePrefix {
    pub fn new(prefix_format: Option<&str>, starts: Option<Geotime>) -> Self {
        match prefix_format {
            Some(format) => match starts {
                Some(ts) => match format {
                    "START_YEAR" => Self::StartYear(ts),
                    "START_YEAR_MONTH" => Self::StartYearMonth(ts),
                    _ => Self::None(ts),
                },
                None => Self::None(Geotime::now()),
            },
            None => Self::None(Geotime::now()),
        }
    }

    pub fn date_string(&self) -> String {
        let ts = match self {
            Self::None(ts) => ts,
            Self::StartYear(ts) => ts,
            Self::StartYearMonth(ts) => ts,
        };
        ts.display_string("%Y-%m-%d")
    }

    pub fn prefix(&self) -> Option<String> {
        match self {
            Self::None(_) => None,
            Self::StartYear(ts) => Some(ts.display_string("%Y")),
            Self::StartYearMonth(ts) => Some(ts.display_string("%Y-%m")),
        }
    }

    pub fn format(&self, name: &str) -> String {
        match self.prefix() {
            Some(prefix) => format!("{} {}", prefix, name),
            None => name.to_owned(),
        }
    }
}

pub fn sha256_base64(normalized: &str) -> String {
    let bytes = normalized.as_bytes();
    let hash = Sha256::digest(bytes);
    engine::general_purpose::URL_SAFE_NO_PAD.encode(hash)
}

#[derive(Clone, Debug)]
pub struct Viewer {
    pub read_repo_ids: RepoIds,
    pub session_id: Option<String>,
    pub super_user: bool,
    pub user_id: String,
    pub write_repo_ids: RepoIds,
    pub context_repo_id: RepoId,
}

impl Viewer {
    pub fn service_account() -> Self {
        Self {
            read_repo_ids: RepoIds(vec![]),
            context_repo_id: RepoId::wiki(),
            session_id: None,
            super_user: true,
            user_id: "".to_owned(),
            write_repo_ids: RepoIds(vec![]),
        }
    }

    pub fn ensure_can_read(&self, repo: &RepoId) -> Result<()> {
        if !self.can_read(repo) {
            return Err(Error::NotFound("not found".into()));
        }

        Ok(())
    }

    pub fn guest() -> Self {
        use crate::prelude::GUEST_ID;

        let user_id = GUEST_ID.to_string();
        Viewer {
            read_repo_ids: RepoIds(vec![RepoId::wiki()]),
            session_id: None,
            context_repo_id: RepoId::wiki(),
            super_user: false,
            user_id,
            write_repo_ids: RepoIds(vec![]),
        }
    }

    pub fn can_read(&self, repo: &RepoId) -> bool {
        if self.super_user {
            return true;
        }
        self.read_repo_ids.include(repo)
    }

    pub fn can_update(&self, repo: &RepoId) -> bool {
        if self.super_user {
            return true;
        }
        self.write_repo_ids.include(repo)
    }

    pub fn is_guest(&self) -> bool {
        self.session_id.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod locale {
        use std::str::FromStr;

        use super::*;

        #[test]
        fn from_str() {
            assert_eq!(Locale::from_str("en").unwrap(), Locale::EN);
        }

        #[test]
        fn to_string() {
            assert_eq!(Locale::EN.to_string(), "en");
        }
    }

    mod timerange {
        use chrono::TimeZone;

        use super::*;

        fn valid_date() -> Option<Geotime> {
            let dt = chrono::Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
            Some(Geotime::from(&dt))
        }

        #[test]
        fn none() {
            let id = TimerangePrefix::new(None, None);
            assert_eq!(id.format("a"), "a");
        }

        #[test]
        fn prefix_none() {
            let id = TimerangePrefix::new(Some("NONE"), valid_date());
            assert_eq!(id.format("a"), "a");
        }

        #[test]
        fn start_year() {
            let id = TimerangePrefix::new(Some("START_YEAR"), valid_date());
            assert_eq!(id.format("a"), "2000 a");
        }

        #[test]
        fn start_year_month() {
            let id = TimerangePrefix::new(Some("START_YEAR_MONTH"), valid_date());
            assert_eq!(id.format("a"), "2000-01 a");
        }
    }

    mod repo_id {
        use super::*;

        #[test]
        fn equality() {
            assert_eq!(
                RepoId::wiki(),
                RepoId::try_from("32212616-fc1b-11e8-8eda-b70af6d8d09f").unwrap()
            );
        }

        #[test]
        fn relative_path() {
            let id = RepoId::wiki();
            assert_eq!(id.relative_path(), "32212616-fc1b-11e8-8eda-b70af6d8d09f/");
        }

        #[test]
        fn display() {
            let id = RepoId::wiki();
            assert_eq!(
                format!("{}", id),
                "32212616-fc1b-11e8-8eda-b70af6d8d09f".to_owned()
            );
        }

        #[test]
        fn validation() {
            assert!(matches!(
                RepoId::try_from("32212616-fc1b-11e8-8eda-b70af6d8d09"),
                Err(_)
            ));
            assert!(matches!(RepoId::try_from("random"), Err(_)));
        }
    }
}
