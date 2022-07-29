use base64;
use geotime::Geotime;
use lazy_static::lazy_static;
use rand::{distributions::Alphanumeric, Rng};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use strum_macros::EnumString;
use void::Void;

use crate::{errors::Error, prelude::WIKI_REPO_PREFIX};

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
pub struct RepoPrefix {
    inner: String,
    valid: bool,
}

impl From<&str> for RepoPrefix {
    fn from(prefix: &str) -> Self {
        let valid = prefix.ends_with('/');
        Self {
            inner: prefix.to_owned(),
            valid,
        }
    }
}

impl From<&String> for RepoPrefix {
    fn from(prefix: &String) -> Self {
        Self::from(prefix.as_str())
    }
}

impl std::fmt::Display for RepoPrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl std::cmp::PartialEq for RepoPrefix {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl std::cmp::Eq for RepoPrefix {}

impl std::hash::Hash for RepoPrefix {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl RepoPrefix {
    pub fn wiki() -> Self {
        Self::from(WIKI_REPO_PREFIX)
    }

    pub fn from_name(name: &str) -> Self {
        Self::from(&format!("/{}/", name))
    }

    pub fn relative_path(&self) -> &str {
        self.inner.trim_start_matches('/')
    }

    pub fn test(&self, path: &RepoPath) -> bool {
        path.starts_with(&self.inner)
    }
}

#[derive(Clone, Debug)]
pub struct RepoList(pub Vec<RepoPrefix>);

impl From<&Vec<String>> for RepoList {
    fn from(prefixes: &Vec<String>) -> Self {
        Self(
            prefixes
                .iter()
                .map(RepoPrefix::from)
                .collect::<Vec<RepoPrefix>>(),
        )
    }
}

impl IntoIterator for RepoList {
    type Item = RepoPrefix;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl RepoList {
    pub fn include(&self, path: &RepoPath) -> bool {
        self.0.iter().any(|prefix| prefix.test(path))
    }

    pub fn to_vec(&self) -> Vec<String> {
        let mut prefixes = vec![];
        for prefix in &self.0 {
            prefixes.push(prefix.to_string());
        }
        prefixes
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RepoPath {
    pub inner: String,
    pub org_login: String,
    pub repo: RepoPrefix,
    pub short_id: String,
    pub valid: bool,
}

impl std::fmt::Display for RepoPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl From<&String> for RepoPath {
    fn from(input: &String) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(/([\w-]+)/)([\w_-]+)$").unwrap();
        }

        let cap = match RE.captures(input) {
            Some(cap) => cap,
            _ => return Self::invalid_path(input),
        };

        let (prefix, org_login, short_id) = match (cap.get(1), cap.get(2), cap.get(3)) {
            (Some(prefix), Some(org_login), Some(short_id)) => {
                (prefix.as_str(), org_login.as_str(), short_id.as_str())
            }
            _ => return Self::invalid_path(input),
        };

        RepoPath {
            inner: input.to_string(),
            org_login: org_login.to_string(),
            repo: RepoPrefix::from(prefix),
            short_id: short_id.to_string(),
            valid: true,
        }
    }
}

impl From<&str> for RepoPath {
    fn from(input: &str) -> Self {
        Self::from(&input.to_string())
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
    type Err = Void;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(RepoPath::from(s))
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
    pub fn make(prefix: &String) -> Self {
        let s: String = random_id();
        Self::from(&format!("{}{}", prefix, s))
    }

    fn invalid_path(input: &str) -> Self {
        Self {
            inner: input.to_owned(),
            org_login: "wiki".to_owned(),
            repo: RepoPrefix::wiki(),
            short_id: "invalid-id".to_owned(),
            valid: false,
        }
    }

    pub fn parts(&self) -> Result<(&str, &str, &str)> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([\w_-]{2})([\w_-]{2})([\w_-]+)$").unwrap();
        }

        if !self.valid {
            return Err(Error::Repo(format!("invalid path: {:?}", self)));
        }

        let cap = RE
            .captures(&self.short_id)
            .ok_or_else(|| Error::Repo(format!("bad id: {}", self)))?;

        if cap.len() != 4 {
            return Err(Error::Repo(format!("bad id: {}", self)));
        }

        match (cap.get(1), cap.get(2), cap.get(3)) {
            (Some(part1), Some(part2), Some(part3)) => {
                Ok((part1.as_str(), part2.as_str(), part3.as_str()))
            }
            _ => Err(Error::Repo(format!("bad id: {}", self))),
        }
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        self.inner.starts_with(prefix)
    }
}

pub enum Alert {
    Danger(String),
    Success(String),
    Warning(String),
}

pub trait DownSet {
    fn transitive_closure(&self, topic_paths: &[&RepoPath]) -> Result<HashSet<String>>;

    fn down_set(&self, key: &RepoPath) -> HashSet<String>;
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
    pub read_repos: RepoList,
    pub session_id: Option<String>,
    pub super_user: bool,
    pub user_id: String,
    pub write_repos: RepoList,
}

impl Viewer {
    pub fn super_user() -> Self {
        Self {
            read_repos: RepoList(vec![]),
            session_id: None,
            super_user: true,
            user_id: "".to_owned(),
            write_repos: RepoList(vec![]),
        }
    }

    pub fn ensure_can_read(&self, path: &RepoPath) -> Result<()> {
        if !self.can_read(path) {
            return Err(Error::Repo("not allowed".into()));
        }

        Ok(())
    }

    pub fn guest() -> Self {
        use crate::prelude::GUEST_ID;

        let user_id = GUEST_ID.to_string();
        Viewer {
            write_repos: RepoList(vec![]),
            read_repos: RepoList(vec![RepoPrefix::wiki()]),
            session_id: None,
            super_user: false,
            user_id,
        }
    }

    pub fn can_read(&self, path: &RepoPath) -> bool {
        if self.super_user {
            return true;
        }
        self.read_repos.include(path)
    }

    pub fn can_update(&self, path: &RepoPath) -> bool {
        if self.super_user {
            return true;
        }
        self.write_repos.include(path)
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
            let path = RepoPath::from("/wiki/00001");

            let viewer = Viewer::guest();
            assert!(!viewer.can_update(&path));

            let prefixes = RepoList(vec![RepoPrefix::wiki()]);
            let viewer = Viewer {
                write_repos: prefixes.to_owned(),
                read_repos: prefixes,
                session_id: Some("1".to_owned()),
                super_user: false,
                user_id: "2".to_owned(),
            };

            assert!(viewer.can_update(&path));

            let path = RepoPath::from("/private/00001");
            assert!(!viewer.can_update(&path));
        }
    }

    mod repo_path {
        use super::*;

        #[test]
        fn simple_case() {
            let path = RepoPath::from("/wiki/00001");
            assert!(path.valid);
            assert_eq!("/wiki/00001", path.inner);
            assert_eq!(RepoPrefix::wiki(), path.repo);
            assert_eq!("wiki", path.org_login);
            assert_eq!("00001", path.short_id);
        }

        #[test]
        fn parts() {
            let path = RepoPath::from("/wiki/q-ZZmeNzLnZvgk_QGVjqPIpSgkADx71iWZrapMTphpQ");
            assert_eq!(
                path.parts().unwrap(),
                ("q-", "ZZ", "meNzLnZvgk_QGVjqPIpSgkADx71iWZrapMTphpQ")
            );
        }
    }

    mod repo_prefix {
        use super::*;

        #[test]
        fn prefix() {
            let path = RepoPath::from("/wiki/q-ZZmeNzLnZvgk_QGVjqPIpSgkADx71iWZrapMTphpQ");
            assert_eq!(path.repo, RepoPrefix::wiki());
        }

        #[test]
        fn equality() {
            assert_eq!(RepoPrefix::wiki(), RepoPrefix::from("/wiki/"));
        }

        #[test]
        fn relative_path() {
            let prefix = RepoPrefix::wiki();
            assert_eq!(prefix.relative_path(), "wiki/");
        }

        #[test]
        fn display() {
            let prefix = RepoPrefix::wiki();
            assert_eq!(format!("{}", prefix), "/wiki/".to_owned());
        }
    }
}
