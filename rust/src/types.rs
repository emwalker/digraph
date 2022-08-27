use base64;
use geotime::Geotime;
use lazy_static::lazy_static;
use rand::{distributions::Alphanumeric, Rng};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use strum_macros::EnumString;

use crate::{
    errors::Error,
    prelude::{ROOT_TOPIC_ID, WIKI_REPO_PREFIX},
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

#[derive(Clone, Debug, Ord, PartialOrd)]
#[allow(dead_code)]
pub struct RepoName {
    inner: String,
}

impl TryFrom<&str> for RepoName {
    type Error = Error;

    fn try_from(prefix: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^/([\w][\w-]*)/$").unwrap();
        }

        if !RE.is_match(prefix) {
            return Err(Error::Path(format!("invalid repo: {}", prefix)));
        }

        Ok(Self {
            inner: prefix.to_owned(),
        })
    }
}

impl TryFrom<&String> for RepoName {
    type Error = Error;

    fn try_from(prefix: &String) -> Result<Self> {
        Self::try_from(prefix.as_str())
    }
}

impl TryFrom<String> for RepoName {
    type Error = Error;

    fn try_from(prefix: String) -> Result<Self> {
        Self::try_from(prefix.as_str())
    }
}

impl std::fmt::Display for RepoName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl std::cmp::PartialEq for RepoName {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl std::cmp::Eq for RepoName {}

impl std::hash::Hash for RepoName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl RepoName {
    pub fn wiki() -> Self {
        Self::try_from(WIKI_REPO_PREFIX).unwrap()
    }

    pub fn from_login(login: &str) -> Result<Self> {
        Self::try_from(&format!("/{}/", login))
    }

    pub fn org_login(&self) -> &str {
        self.inner.trim_start_matches('/').trim_end_matches('/')
    }

    pub fn root_topic_id(&self) -> RepoId {
        ROOT_TOPIC_ID
            .try_into()
            .expect("failed to convert root topic id")
    }

    pub fn path(&self, id: &RepoId) -> Result<RepoPath> {
        RepoPath::try_from(&format!("{}{}", self.inner, id))
    }

    pub fn relative_path(&self) -> &str {
        self.inner.trim_start_matches('/')
    }
}

#[derive(Clone, Debug)]
pub struct RepoNames(Vec<RepoName>);

impl TryFrom<&[String]> for RepoNames {
    type Error = Error;

    fn try_from(prefixes: &[String]) -> Result<Self> {
        Ok(Self(
            prefixes
                .iter()
                .map(RepoName::try_from)
                .collect::<Result<Vec<RepoName>>>()?,
        ))
    }
}

impl TryFrom<&Vec<String>> for RepoNames {
    type Error = Error;

    fn try_from(prefixes: &Vec<String>) -> Result<Self> {
        Self::try_from(prefixes.as_slice())
    }
}

impl From<&[RepoName]> for RepoNames {
    fn from(prefixes: &[RepoName]) -> Self {
        Self(prefixes.to_owned())
    }
}

impl From<&Vec<RepoName>> for RepoNames {
    fn from(prefixes: &Vec<RepoName>) -> Self {
        Self(prefixes.to_vec())
    }
}

impl From<&RepoNames> for Vec<RepoName> {
    fn from(repos: &RepoNames) -> Self {
        repos.0.to_owned()
    }
}

impl RepoNames {
    pub fn include(&self, repo: &RepoName) -> bool {
        self.0.iter().any(|prefix| prefix == repo)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, RepoName> {
        self.0.iter()
    }

    pub fn to_vec(&self) -> Vec<String> {
        let mut prefixes = vec![];
        for prefix in &self.0 {
            prefixes.push(prefix.to_string());
        }
        prefixes
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RepoId(String);

impl std::fmt::Display for RepoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for RepoId {
    type Error = Error;

    fn try_from(id: &str) -> Result<Self> {
        Self::try_from(id.to_owned())
    }
}

impl TryFrom<&String> for RepoId {
    type Error = Error;

    fn try_from(id: &String) -> Result<Self> {
        Self::try_from(id.to_owned())
    }
}

impl TryFrom<String> for RepoId {
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

impl RepoId {
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RepoPath {
    pub id: RepoId,
    pub inner: String,
    pub org_login: String,
    pub repo: RepoName,
}

impl std::fmt::Display for RepoPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl TryFrom<&str> for RepoPath {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(/([\w-]+)/)([\w_-]+)$").unwrap();
        }

        let cap = match RE.captures(input) {
            Some(cap) => cap,
            _ => {
                return Err(Error::Path(format!("invalid path: {}", input)));
            }
        };

        let (prefix, org_login, short_id) = match (cap.get(1), cap.get(2), cap.get(3)) {
            (Some(prefix), Some(org_login), Some(short_id)) => {
                (prefix.as_str(), org_login.as_str(), short_id.as_str())
            }
            _ => {
                return Err(Error::Path(format!("invalid path: {}", input)));
            }
        };

        Ok(RepoPath {
            inner: input.to_string(),
            org_login: org_login.to_string(),
            repo: prefix.try_into()?,
            id: short_id.try_into()?,
        })
    }
}

impl TryFrom<&String> for RepoPath {
    type Error = Error;

    fn try_from(input: &String) -> Result<Self> {
        RepoPath::try_from(input.as_str())
    }
}

impl std::cmp::Ord for RepoPath {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl std::cmp::PartialOrd for RepoPath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::str::FromStr for RepoPath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        RepoPath::try_from(s)
    }
}

pub fn random_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(43)
        .map(char::from)
        .collect()
}

impl RepoPath {
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.inner.starts_with(prefix)
    }
}

pub enum Alert {
    Danger(String),
    Success(String),
    Warning(String),
}

#[derive(Clone, Debug)]
pub struct Timespec;

#[derive(Debug)]
pub struct ReadPath {
    pub commit: git2::Oid,
    pub id: RepoId,
    pub repo: RepoName,
}

pub trait Downset {
    fn intersection(&self, topic_ids: &[ReadPath]) -> Result<HashSet<RepoId>>;

    fn downset(&self, path: &ReadPath) -> HashSet<RepoId>;
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
    base64::encode_config(hash, base64::URL_SAFE_NO_PAD)
}

#[derive(Clone, Debug)]
pub struct Viewer {
    pub read_repos: RepoNames,
    pub session_id: Option<String>,
    pub super_user: bool,
    pub user_id: String,
    pub write_repos: RepoNames,
}

impl Viewer {
    pub fn service_account() -> Self {
        Self {
            read_repos: RepoNames(vec![]),
            session_id: None,
            super_user: true,
            user_id: "".to_owned(),
            write_repos: RepoNames(vec![]),
        }
    }

    pub fn ensure_can_read(&self, repo: &RepoName) -> Result<()> {
        if !self.can_read(repo) {
            return Err(Error::NotFound("not found".into()));
        }

        Ok(())
    }

    pub fn guest() -> Self {
        use crate::prelude::GUEST_ID;

        let user_id = GUEST_ID.to_string();
        Viewer {
            write_repos: RepoNames(vec![]),
            read_repos: RepoNames(vec![RepoName::wiki()]),
            session_id: None,
            super_user: false,
            user_id,
        }
    }

    pub fn can_read(&self, repo: &RepoName) -> bool {
        if self.super_user {
            return true;
        }
        self.read_repos.include(repo)
    }

    pub fn can_update(&self, repo: &RepoName) -> bool {
        if self.super_user {
            return true;
        }
        self.write_repos.include(repo)
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
            let dt = chrono::Utc.ymd(2000, 1, 1).and_hms(0, 0, 0);
            Some(Geotime::from(&dt))
        }

        #[test]
        fn none() {
            let prefix = TimerangePrefix::new(None, None);
            assert_eq!(prefix.format("a"), "a");
        }

        #[test]
        fn prefix_none() {
            let prefix = TimerangePrefix::new(Some("NONE"), valid_date());
            assert_eq!(prefix.format("a"), "a");
        }

        #[test]
        fn start_year() {
            let prefix = TimerangePrefix::new(Some("START_YEAR"), valid_date());
            assert_eq!(prefix.format("a"), "2000 a");
        }

        #[test]
        fn start_year_month() {
            let prefix = TimerangePrefix::new(Some("START_YEAR_MONTH"), valid_date());
            assert_eq!(prefix.format("a"), "2000-01 a");
        }
    }

    mod viewer {
        use super::*;

        #[test]
        fn can_update() {
            let path = RepoPath::try_from("/wiki/00001").unwrap();

            let viewer = Viewer::guest();
            assert!(!viewer.can_update(&path.repo));

            let prefixes = RepoNames(vec![RepoName::wiki()]);
            let viewer = Viewer {
                write_repos: prefixes.to_owned(),
                read_repos: prefixes,
                session_id: Some("1".to_owned()),
                super_user: false,
                user_id: "2".to_owned(),
            };

            assert!(viewer.can_update(&path.repo));

            let path = RepoPath::try_from("/private/00001").unwrap();
            assert!(!viewer.can_update(&path.repo));
        }
    }

    mod repo_path {
        use super::*;

        #[test]
        fn simple_case() {
            let path = RepoPath::try_from("/wiki/00001").unwrap();
            assert_eq!("/wiki/00001", path.inner);
            assert_eq!(RepoName::wiki(), path.repo);
            assert_eq!("wiki", path.org_login);
            assert_eq!("00001", path.id.as_str());
        }
    }

    mod repo_prefix {
        use super::*;

        #[test]
        fn prefix() {
            let path =
                RepoPath::try_from("/wiki/q-ZZmeNzLnZvgk_QGVjqPIpSgkADx71iWZrapMTphpQ").unwrap();
            assert_eq!(path.repo, RepoName::wiki());
        }

        #[test]
        fn equality() {
            assert_eq!(RepoName::wiki(), RepoName::try_from("/wiki/").unwrap());
        }

        #[test]
        fn relative_path() {
            let prefix = RepoName::wiki();
            assert_eq!(prefix.relative_path(), "wiki/");
        }

        #[test]
        fn display() {
            let prefix = RepoName::wiki();
            assert_eq!(format!("{}", prefix), "/wiki/".to_owned());
        }

        #[test]
        fn validation() {
            assert!(matches!(RepoName::try_from("//"), Err(_)));
            assert!(matches!(RepoName::try_from("/"), Err(_)));
            assert!(matches!(RepoName::try_from("a"), Err(_)));
            assert!(matches!(RepoName::try_from("/a"), Err(_)));
            assert!(matches!(RepoName::try_from("a/"), Err(_)));
            assert!(matches!(RepoName::try_from("/-/"), Err(_)));
            assert!(matches!(RepoName::try_from("/./"), Err(_)));
            assert!(matches!(RepoName::try_from("/../"), Err(_)));
            assert!(matches!(RepoName::try_from("/other/../wiki/"), Err(_)));
        }
    }
}
