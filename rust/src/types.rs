use lazy_static::lazy_static;
use rand::{distributions::Alphanumeric, Rng};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use strum_macros::EnumString;

use crate::errors::Error;

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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RepoPath {
    pub inner: String,
    pub org_login: String,
    pub prefix: String,
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
            static ref RE: Regex = Regex::new(r"^(/([\w-]+))/([\w-]+)$").unwrap();
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
            prefix: prefix.to_string(),
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

pub fn random_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect()
}

impl RepoPath {
    pub fn random(prefix: &String) -> Self {
        let s: String = random_id();
        Self::from(&format!("{}/{}", prefix, s))
    }

    fn invalid_path(input: &String) -> Self {
        Self {
            inner: input.clone(),
            org_login: "wiki".into(),
            prefix: "wiki".into(),
            short_id: input.into(),
            valid: false,
        }
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
    pub starts: chrono::DateTime<chrono::Utc>,
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
pub enum Prefix {
    None,
    StartYear(chrono::DateTime<chrono::Utc>),
    StartYearMonth(chrono::DateTime<chrono::Utc>),
}

impl From<&Option<Timerange>> for Prefix {
    fn from(timerange: &Option<Timerange>) -> Self {
        match &timerange {
            Some(Timerange {
                starts,
                prefix_format,
                ..
            }) => match prefix_format {
                TimerangePrefixFormat::None => Self::None,
                TimerangePrefixFormat::StartYear => Self::StartYear(*starts),
                TimerangePrefixFormat::StartYearMonth => Self::StartYearMonth(*starts),
            },
            None => Self::None,
        }
    }
}

impl From<&Timerange> for Prefix {
    fn from(timerange: &Timerange) -> Self {
        let Timerange {
            starts,
            prefix_format,
        } = timerange;
        match prefix_format {
            TimerangePrefixFormat::None => Self::None,
            TimerangePrefixFormat::StartYear => Self::StartYear(*starts),
            TimerangePrefixFormat::StartYearMonth => Self::StartYearMonth(*starts),
        }
    }
}

impl Prefix {
    pub fn new(prefix_format: Option<&str>, starts: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        match prefix_format {
            Some(format) => match starts {
                Some(starts_at) => match format {
                    "START_YEAR" => Self::StartYear(starts_at),
                    "START_YEAR_MONTH" => Self::StartYearMonth(starts_at),
                    _ => Self::None,
                },
                None => Self::None,
            },
            None => Self::None,
        }
    }

    pub fn prefix(&self) -> Option<String> {
        match self {
            Self::None => None,
            Self::StartYear(starts) => Some(format!("{}", starts.format("%Y"))),
            Self::StartYearMonth(starts) => Some(format!("{}", starts.format("%Y-%m"))),
        }
    }

    pub fn format(&self, name: &str) -> String {
        match self.prefix() {
            Some(prefix) => format!("{} {}", prefix, name),
            None => name.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_date() -> Option<chrono::DateTime<chrono::Utc>> {
        chrono::DateTime::parse_from_rfc2822("Sat, 1 Jan 2000 00:00:00 +0000")
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc))
    }

    #[test]
    fn none() {
        let prefix = Prefix::new(None, None);
        assert_eq!(prefix.format("a"), "a");
    }

    #[test]
    fn prefix_none() {
        let prefix = Prefix::new(Some("NONE"), valid_date());
        assert_eq!(prefix.format("a"), "a");
    }

    #[test]
    fn start_year() {
        let prefix = Prefix::new(Some("START_YEAR"), valid_date());
        assert_eq!(prefix.format("a"), "2000 a");
    }

    #[test]
    fn start_year_month() {
        let prefix = Prefix::new(Some("START_YEAR_MONTH"), valid_date());
        assert_eq!(prefix.format("a"), "2000-01 a");
    }
}
