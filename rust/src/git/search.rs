use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};
use strum_macros::EnumString;

use super::{Git, Kind, Locale, Object, Phrase, Row, SynonymEntry};
use crate::prelude::*;

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
    op: PathSpecOperation,
    path: RepoPath,
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
pub struct SortKey(pub bool, pub Kind, pub String);

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
    pub fn call(&self, git: &Git) -> Result<SearchWithinTopicResult> {
        if self.search.is_empty() {
            log::info!("empty search, returning no results");
            return Ok(SearchWithinTopicResult {
                matches: BTreeSet::new(),
            });
        }

        // Initial working set
        // https://stackoverflow.com/a/65175186/61048
        let mut sets = self.topic_down_sets(git);
        let (start, remaining) = sets.split_at_mut(1);
        let rows = &mut start[0];
        for set in remaining {
            rows.retain(|o| set.contains(o));
        }

        // Drop anything that doesn't match the other search terms if they're present
        if !self.search.tokens.is_empty() {
            for prefix in &self.prefixes {
                for token in &self.search.tokens {
                    let other = git.search_token_prefix_matches(prefix, token);
                    rows.retain(|o| other.contains(o));
                }
            }
        }

        let normalized = &self.search.normalized;
        let mut matches = BTreeSet::new();

        for row in rows.iter() {
            let object = match row {
                Row::SearchEntry(entry) => git.fetch(&entry.path)?,
                Row::Topic(topic) => Object::Topic(topic.to_owned()),
                Row::TopicChild(child) => git.fetch(&child.path)?,
            };
            matches.insert(object.to_search_match(self.locale, normalized));
        }

        Ok(SearchWithinTopicResult {
            matches: matches.iter().take(self.limit).cloned().collect(),
        })
    }

    fn topic_down_sets(&self, git: &Git) -> Vec<HashSet<Row>> {
        let set = git
            .topic_down_set(&self.topic_path)
            .collect::<HashSet<Row>>();
        let mut sets = vec![set];
        for spec in &self.search.path_specs {
            let set = git.topic_down_set(&spec.path).collect::<HashSet<Row>>();
            sets.push(set);
        }
        // Use the smallest set ast the starting point
        sets.sort_by_key(HashSet::len);
        sets
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
