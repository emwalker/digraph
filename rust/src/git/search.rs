use lazy_static::lazy_static;
use redis_rs;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};
use strum_macros::EnumString;

use super::{Git, Kind, Locale, Object, Phrase, SynonymEntry, Topic};
use crate::prelude::*;
use crate::redis;
use crate::DownSet;

#[derive(
    Copy,
    Clone,
    Debug,
    Deserialize,
    EnumString,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    strum_macros::Display,
)]
#[serde(rename_all = "lowercase")]
#[strum(ascii_case_insensitive)]
pub enum PathSpecOperation {
    IN,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PathSpec {
    pub op: PathSpecOperation,
    pub path: RepoPath,
}

const PATH_PATTERN: &str = r#"^in:/\w+/[\w-]+$"#;

impl PathSpec {
    fn valid_path_spec(input: &str) -> bool {
        lazy_static! {
            static ref IS_PATH_SPEC: Regex = Regex::new(PATH_PATTERN).unwrap();
        }
        IS_PATH_SPEC.is_match(input)
    }

    fn parse(input: &str) -> Result<Self> {
        use std::str::FromStr;

        let parts: Vec<String> = input.split(':').map(str::to_string).collect();
        if !parts.len() == 2 {
            return Err(Error::Parse(format!("invalid topic spec: {}", input)));
        }

        let op = PathSpecOperation::from_str(&parts[0])?;
        Ok(Self {
            op,
            path: RepoPath::from(&parts[1]),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Search {
    pub normalized: Phrase,
    pub urls: BTreeSet<RepoUrl>,
    pub tokens: BTreeSet<Phrase>,
    pub path_specs: BTreeSet<PathSpec>,
}

impl std::cmp::Ord for Search {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.urls, &self.tokens).cmp(&(&other.urls, &other.tokens))
    }
}

impl std::cmp::PartialOrd for Search {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Search {
    pub fn parse(input: &str) -> Result<Self> {
        let mut tokens = BTreeSet::new();
        let mut urls = BTreeSet::new();
        let mut path_specs = BTreeSet::new();

        for part in input.split_whitespace() {
            if PathSpec::valid_path_spec(part) {
                path_specs.insert(PathSpec::parse(part)?);
                continue;
            }

            if RepoUrl::is_valid_url(part) {
                urls.insert(RepoUrl::parse(part)?);
                continue;
            }

            let phrase = Phrase::parse(part);
            for token in phrase.tokens() {
                // is_valid is called during Phrase::tokens
                tokens.insert(token);
            }
        }

        Ok(Self {
            normalized: Phrase::parse(input),
            path_specs,
            tokens,
            urls,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.urls.is_empty() && self.tokens.is_empty() && self.path_specs.is_empty()
    }
}

pub struct FetchTopicLiveSearch {
    pub limit: usize,
    pub prefixes: Vec<String>,
    pub search: Search,
    pub viewer: Viewer,
}

pub struct FetchTopicLiveSearchResult {
    pub synonym_matches: BTreeSet<SynonymEntry>,
}

impl FetchTopicLiveSearch {
    pub fn call(&self, git: &Git) -> Result<FetchTopicLiveSearchResult> {
        if self.search.tokens.is_empty() {
            log::info!("empty search, returning no results");
            return Ok(FetchTopicLiveSearchResult {
                synonym_matches: BTreeSet::new(),
            });
        }

        log::info!("searching for topics: {:?}", self.search);
        let matches = self.fetch(git);

        Ok(FetchTopicLiveSearchResult {
            synonym_matches: matches.iter().take(self.limit).cloned().collect(),
        })
    }

    fn fetch(&self, git: &Git) -> BTreeSet<SynonymEntry> {
        let mut matches = BTreeSet::new();
        for prefix in &self.prefixes {
            self.fetch_prefix(git, prefix, &mut matches);
        }
        matches
    }

    fn fetch_prefix(&self, git: &Git, prefix: &str, matches: &mut BTreeSet<SynonymEntry>) {
        let tokens = &mut self.search.tokens.iter();
        let start = match tokens.next() {
            Some(token) => git.synonym_token_prefix_matches(prefix, token),
            None => BTreeSet::new(),
        };

        let mut result = tokens.fold(start, |acc, token| {
            let result = git.synonym_token_prefix_matches(prefix, token);
            acc.intersection(&result).cloned().collect()
        });

        matches.append(&mut result);
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SortKey(pub Kind, pub bool, pub String);

#[derive(Clone, Debug)]
pub struct SearchMatch {
    pub sort_key: SortKey,
    pub object: Object,
}

impl std::cmp::Ord for SearchMatch {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sort_key.cmp(&other.sort_key)
    }
}

impl std::cmp::PartialOrd for SearchMatch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for SearchMatch {
    fn eq(&self, other: &Self) -> bool {
        self.object == other.object
    }
}

impl std::cmp::Eq for SearchMatch {}

struct Filter<T>(HashSet<T>);

impl<T: Eq + std::hash::Hash> Filter<T> {
    fn test(&self, value: &T) -> bool {
        if self.0.is_empty() {
            return true;
        }
        self.0.contains(value)
    }
}

pub struct RedisFetchDownSet<T: Clone + redis_rs::IntoConnectionInfo> {
    pub git: Git,
    pub redis: redis::Redis<T>,
}

impl<T: Clone + redis_rs::IntoConnectionInfo> DownSet for RedisFetchDownSet<T> {
    fn transitive_closure(&self, topic_paths: &[&RepoPath]) -> Result<HashSet<String>> {
        self.redis.transitive_closure(self, topic_paths)
    }

    fn down_set(&self, path: &RepoPath) -> HashSet<String> {
        self.git
            .topic_down_set(path)
            .map(|topic| topic.metadata.path)
            .collect::<HashSet<String>>()
    }
}

pub struct SearchWithinTopic {
    pub limit: usize,
    pub locale: Locale,
    pub prefixes: Vec<String>,
    pub recursive: bool,
    pub search: Search,
    pub topic_path: RepoPath,
    pub viewer: Viewer,
}

pub struct SearchWithinTopicResult {
    pub matches: BTreeSet<SearchMatch>,
}

impl SearchWithinTopic {
    pub fn call<F>(&self, git: &Git, fetch: &F) -> Result<SearchWithinTopicResult>
    where
        F: DownSet,
    {
        if self.search.is_empty() {
            log::info!("empty search, returning no results");
            return Ok(SearchWithinTopicResult {
                matches: BTreeSet::new(),
            });
        }

        let mut topics = self.topic_intersection(git, fetch)?;
        let filter = self.token_filter(git);
        let mut matches = BTreeSet::new();

        for topic in topics.drain() {
            for child in &topic.children {
                if child.kind == Kind::Link && filter.test(&child.path) {
                    let object = git.fetch(&child.path)?;
                    matches.insert(object.to_search_match(self.locale, &self.search));
                }
            }

            if filter.test(&topic.metadata.path) {
                matches.insert(Object::Topic(topic).to_search_match(self.locale, &self.search));
            }
        }

        Ok(SearchWithinTopicResult {
            matches: matches.iter().take(self.limit).cloned().collect(),
        })
    }

    fn topic_intersection<F>(&self, git: &Git, fetch: &F) -> Result<HashSet<Topic>>
    where
        F: DownSet,
    {
        let topic_paths = self
            .search
            .path_specs
            .iter()
            .map(|s| &s.path)
            .chain([&self.topic_path])
            .collect::<Vec<&RepoPath>>();

        let mut set = fetch.transitive_closure(&topic_paths)?;
        let mut topics = HashSet::new();

        for path in set.drain() {
            let topic = git.fetch_topic(&path)?;
            topics.insert(topic);
        }

        Ok(topics)
    }

    fn token_filter(&self, git: &Git) -> Filter<String> {
        let mut token_matches = HashSet::new();

        for prefix in &self.prefixes {
            let mut iter = self.search.tokens.iter();

            if let Some(token) = iter.next() {
                let mut prefix_matches = git.search_token_prefix_matches(prefix, token);

                for token in iter {
                    let other = git.search_token_prefix_matches(prefix, token);
                    prefix_matches.retain(|e| other.contains(e));
                }

                for entry in prefix_matches.drain() {
                    token_matches.insert(entry.path);
                }
            }
        }

        Filter(token_matches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn phrases(iter: &[&str]) -> BTreeSet<Phrase> {
        iter.iter()
            .map(|s| Phrase::parse(*s))
            .collect::<BTreeSet<Phrase>>()
    }

    #[test]
    fn valid_path_specs() {
        assert!(PathSpec::valid_path_spec(
            "in:/wiki/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
        ));
        assert!(PathSpec::valid_path_spec(
            "in:/emwalker/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
        ));
        // Well-formed UUIDs are not required
        assert!(PathSpec::valid_path_spec(
            "in:/wiki/e76a690f-2eb2-45a0-9cbc-5e7d76f9285"
        ));
    }

    #[test]
    fn invalid_path_specs() {
        assert!(!PathSpec::valid_path_spec(
            "/wiki/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
        ));
        assert!(!PathSpec::valid_path_spec(
            "In:/wiki/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
        ));
        assert!(!PathSpec::valid_path_spec(
            "up:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
        ));
    }

    #[test]
    fn path_spec_parsing() {
        let s = PathSpec::parse("in:/wiki/e76a690f-2eb2-45a0-9cbc-5e7d76f92851").unwrap();
        assert_eq!(s.op, PathSpecOperation::IN);
        assert_eq!(
            s.path,
            RepoPath::from("/wiki/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"),
        );
    }

    #[test]
    fn empty_query() {
        let s = Search::parse("").unwrap();
        assert_eq!(s.normalized, Phrase::parse(""));
        assert_eq!(s.tokens.len(), 0);
        assert_eq!(s.path_specs.len(), 0);
    }

    #[test]
    fn simple_search() {
        let s = Search::parse("a b aa bb").unwrap();
        assert_eq!(s.normalized, Phrase::parse("a b aa bb"));
        assert_eq!(s.tokens, phrases(&["aa", "bb"]));
        assert_eq!(s.path_specs.len(), 0);
    }

    #[test]
    fn punctuation() {
        let search = Search::parse("one.!?:`#$@*&;+-{}[]()/\\'\",=    <> two").unwrap();
        assert_eq!(search.tokens, phrases(&["one", "two"]));
    }

    #[test]
    fn uppercase_letters() {
        let search = Search::parse("One TWO three").unwrap();
        assert_eq!(search.tokens, phrases(&["one", "two", "three"]));
    }

    #[test]
    fn unicode_characters() {
        let phrase = Search::parse("Æneid étude 北亰 ᔕᓇᓇ げんまい茶").unwrap();
        assert_eq!(
            phrase.tokens,
            phrases(&["aeneid", "etude", "bei", "jing", "shanana", "genmaicha"])
        );
    }

    #[test]
    fn splits_on_hyphens() {
        let phrase = Search::parse("Existing non-root topic").unwrap();
        assert_eq!(
            phrase.tokens,
            phrases(&["existing", "non", "root", "topic"])
        );
    }

    #[test]
    fn token_length() {
        let token = (0..=20).map(|_| "a").collect::<String>();
        assert_eq!(token.len(), 21);

        // Long phrases are allowed for now, to accomodate synonym matches
        let phrase = Search::parse(&format!("a aa aaa aaaa {}", token)).unwrap();
        assert_eq!(
            phrase.tokens,
            phrases(&["aa", "aaa", "aaaa", "aaaaaaaaaaaaaaaaaaaaa"])
        );
    }

    #[test]
    fn url() {
        let phrase = Search::parse("one https://www.google.com").unwrap();
        assert_eq!(
            phrase.urls,
            BTreeSet::from([RepoUrl::parse("https://www.google.com").unwrap()]),
        );

        let phrase = Search::parse("aaas:").unwrap();
        assert_eq!(phrase.urls, BTreeSet::new());
    }

    #[test]
    fn topic_search() {
        let s = Search::parse("in:/wiki/e76a690f-2eb2-45a0-9cbc-5e7d76f92851").unwrap();
        assert_eq!(
            s.normalized,
            Phrase::parse("in:/wiki/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"),
        );
        assert_eq!(s.tokens.len(), 0);
        assert_eq!(s.path_specs.len(), 1);
    }

    #[test]
    fn combined_query() {
        let s = Search::parse("in:/wiki/e76a690f-2eb2-45a0-9cbc-5e7d76f92851 a b").unwrap();
        assert_eq!(
            s.normalized,
            Phrase::parse("in:/wiki/e76a690f-2eb2-45a0-9cbc-5e7d76f92851 a b"),
        );
        assert_eq!(s.tokens.len(), 0);
        assert_eq!(s.path_specs.len(), 1);
        assert_eq!(
            *s.path_specs.iter().next().unwrap(),
            PathSpec::parse("in:/wiki/e76a690f-2eb2-45a0-9cbc-5e7d76f92851").unwrap()
        );
    }
}
