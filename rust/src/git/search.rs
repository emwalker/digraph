use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeSet;

use super::{Git, Kind, Locale, Object, Phrase, Search, SynonymEntry};
use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub struct TopicSpec {
    op: String,
    resource_path: String,
}

const TOPIC_PATTERN: &str =
    r#"^in:/\w+/topics/[0-9a-f]{8}\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\b[0-9a-f]{12}$"#;
const NO_TOPIC: &str = "00000000-0000-0000-0000-000000000000";

impl TopicSpec {
    fn is_topic_spec(input: &str) -> bool {
        lazy_static! {
            static ref IS_TOPIC_SPEC: Regex = Regex::new(TOPIC_PATTERN).unwrap();
        }
        IS_TOPIC_SPEC.is_match(input)
    }

    fn parse(input: String) -> Result<Self> {
        let parts: Vec<String> = input.split(':').map(str::to_string).collect();
        if parts.len() == 2 {
            Ok(Self {
                op: parts[0].clone(),
                resource_path: parts[1].clone(),
            })
        } else {
            Err(Error::Parse(format!("invalid topic spec: {}", input)))
        }
    }

    // FIXME: Account for the org and repo in the spec
    fn topic_id(&self) -> String {
        self.resource_path
            .split('/')
            .last()
            .unwrap_or(NO_TOPIC)
            .to_string()
    }
}

// let iter: Vec<String> = input
// .split(' ')
// .filter(|s| !s.is_empty())
// .map(str::to_string)
// .collect();

// let mut tokens: Vec<String> = Vec::new();
// let mut string_tokens: Vec<String> = Vec::new();
// let mut topics: Vec<TopicSpec> = Vec::new();

// for token in iter {
// tokens.push(token.clone());
// if TopicSpec::is_topic_spec(&token) {
//     let spec = TopicSpec::parse(token)?;
//     topics.push(spec);
// } else {
//     match repo_url::Url::parse(&token) {
//         Ok(url) => string_tokens.push(url.normalized),
//         Err(_) => string_tokens.push(token),
//     }
// }
// }

// Ok(Self {
// input: input.into(),
// tokens,
// string_tokens,
// topics,
// })

trait TokenSearch {
    type Item: Clone + Ord;

    fn search(&self) -> &Search;

    fn limit(&self) -> usize;

    fn prefixes(&self) -> &[String];

    fn token_matches(&self, git: &Git, prefix: &str, token: &Phrase) -> BTreeSet<Self::Item>;

    fn fetch(&self, git: &Git) -> BTreeSet<Self::Item> {
        let mut matches: BTreeSet<Self::Item> = BTreeSet::new();
        let mut limit = self.limit();

        for prefix in self.prefixes() {
            let search = self.fetch_prefix(git, prefix);
            let mut result: BTreeSet<Self::Item> = search.iter().take(limit).cloned().collect();
            limit = limit.saturating_sub(result.len());
            matches.append(&mut result);
            if limit == 0 {
                break;
            }
        }

        matches
    }

    fn fetch_prefix(&self, git: &Git, prefix: &str) -> BTreeSet<Self::Item> {
        let tokens = &mut self.search().tokens.iter();
        match tokens.next() {
            Some(token) => {
                let start = self.token_matches(git, prefix, token);
                tokens.fold(start, |acc, token| {
                    let result = self.token_matches(git, prefix, token);
                    acc.intersection(&result).cloned().collect()
                })
            }
            None => BTreeSet::new(),
        }
    }
}

pub struct FetchTopicLiveSearch {
    pub prefixes: Vec<String>,
    pub search: Search,
    pub viewer: Viewer,
}

pub struct FetchTopicLiveSearchResult {
    pub synonym_matches: BTreeSet<SynonymEntry>,
}

impl TokenSearch for FetchTopicLiveSearch {
    type Item = SynonymEntry;

    fn search(&self) -> &Search {
        &self.search
    }

    fn prefixes(&self) -> &[String] {
        &self.prefixes
    }

    fn limit(&self) -> usize {
        10
    }

    fn token_matches(&self, git: &Git, prefix: &str, token: &Phrase) -> BTreeSet<Self::Item> {
        git.synonym_token_prefix_matches(prefix, token)
    }
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
            synonym_matches: matches,
        })
    }
}

#[derive(Clone, Debug)]
pub struct SearchMatch {
    pub sort_key: (bool, Kind, String),
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
    pub locale: Locale,
    pub prefixes: Vec<String>,
    pub search: Search,
    pub topic_path: RepoPath,
    pub viewer: Viewer,
}

pub struct SearchWithinTopicResult {
    pub matches: BTreeSet<SearchMatch>,
}

impl TokenSearch for SearchWithinTopic {
    type Item = SearchMatch;

    fn search(&self) -> &Search {
        &self.search
    }

    fn prefixes(&self) -> &[String] {
        &self.prefixes
    }

    fn limit(&self) -> usize {
        100
    }

    fn token_matches(&self, git: &Git, prefix: &str, token: &Phrase) -> BTreeSet<Self::Item> {
        let normalized = &self.search.normalized;
        git.search_token_prefix_matches(prefix, token)
            .iter()
            .filter_map(|entry| match git.fetch(&entry.path) {
                Ok(object) => Some(SearchMatch {
                    sort_key: (
                        normalized != &object.search_string(self.locale),
                        entry.kind,
                        object.display_string(self.locale),
                    ),
                    object,
                }),
                Err(err) => {
                    log::error!("problem fetching entry {:?}: {}", entry, err);
                    None
                }
            })
            .collect()
    }
}

impl SearchWithinTopic {
    pub fn call(&self, git: &Git) -> Result<SearchWithinTopicResult> {
        if self.search.tokens.is_empty() {
            log::info!("empty search, returning no results");
            return Ok(SearchWithinTopicResult {
                matches: BTreeSet::new(),
            });
        }

        log::info!("searching within topic: {:?}", self.search);
        let matches = self.fetch(git);

        Ok(SearchWithinTopicResult { matches })
    }
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn valid_topic_specs() {
    //     assert!(TopicSpec::is_topic_spec(
    //         "in:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
    //     ));
    //     assert!(TopicSpec::is_topic_spec(
    //         "in:/emwalker/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
    //     ));
    // }

    // #[test]
    // fn invalid_topic_specs() {
    //     assert!(!TopicSpec::is_topic_spec(
    //         "in:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f9285"
    //     ));
    //     assert!(!TopicSpec::is_topic_spec(
    //         "/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
    //     ));
    //     assert!(!TopicSpec::is_topic_spec(
    //         "In:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
    //     ));
    //     assert!(!TopicSpec::is_topic_spec(
    //         "up:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
    //     ));
    //     assert!(!TopicSpec::is_topic_spec(
    //         "in:/wiki/links/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
    //     ));
    // }

    // #[test]
    // fn topic_spec_parsing() {
    //     let s = TopicSpec::parse("in:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851".into())
    //         .unwrap();
    //     assert_eq!(s.op, "in".to_string());
    //     assert_eq!(
    //         s.resource_path,
    //         "/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851".to_string()
    //     );
    //     assert_eq!(s.topic_id(), "e76a690f-2eb2-45a0-9cbc-5e7d76f92851");
    // }

    // #[test]
    // fn empty_query() {
    //     let s = QuerySpec::parse("").unwrap();
    //     assert_eq!(s.input, "");
    //     assert_eq!(s.tokens.len(), 0);
    //     assert_eq!(s.string_tokens.len(), 0);
    //     assert_eq!(s.topics.len(), 0);
    // }

    // #[test]
    // fn simple_query() {
    //     let s = QuerySpec::parse("a b").unwrap();
    //     assert_eq!(s.input, "a b");
    //     assert_eq!(s.tokens, ["a", "b"]);
    //     assert_eq!(s.string_tokens, ["a", "b"]);
    //     assert_eq!(s.topics.len(), 0);
    // }

    // #[test]
    // fn topic_query() {
    //     let s = QuerySpec::parse("in:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851").unwrap();
    //     assert_eq!(
    //         s.input,
    //         "in:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
    //     );
    //     assert_eq!(s.tokens.len(), 1);
    //     assert_eq!(s.string_tokens.len(), 0);
    //     assert_eq!(s.topics.len(), 1);
    // }

    // #[test]
    // fn urls_are_normalized() {
    //     let s = QuerySpec::parse("https://www.google.com/?s=1234").unwrap();
    //     assert_eq!(s.string_tokens, ["https://www.google.com/"]);
    // }

    // #[test]
    // fn combined_query() {
    //     let s =
    //         QuerySpec::parse("in:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851 a b").unwrap();
    //     assert_eq!(
    //         s.input,
    //         "in:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851 a b"
    //     );
    //     assert_eq!(s.tokens.len(), 3);
    //     assert_eq!(s.string_tokens, ["a", "b"]);
    //     assert_eq!(s.topics.len(), 1);
    //     assert_eq!(
    //         *s.topics.get(0).unwrap(),
    //         TopicSpec::parse("in:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851".into())
    //             .unwrap()
    //     );
    // }
}
