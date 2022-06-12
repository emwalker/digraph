use lazy_static::lazy_static;
use regex::Regex;

use crate::http::repo_url;
use crate::prelude::*;

mod queries;
mod repo;
pub use repo::*;
mod link;
pub use link::*;
mod organization;
pub use organization::*;
mod repository;
pub use repository::*;
mod search;
pub mod shared;
pub use search::*;
mod session;
pub use session::*;
mod synonyms;
pub use synonyms::*;
pub mod topic;
pub use topic::*;
pub mod user;
pub use user::*;

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

pub struct QuerySpec {
    input: String,
    tokens: Vec<String>,
    string_tokens: Vec<String>,
    topics: Vec<TopicSpec>,
}

impl std::fmt::Display for QuerySpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.input)
    }
}

impl QuerySpec {
    fn parse(input: &str) -> Result<Self> {
        let iter: Vec<String> = input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect();

        let mut tokens: Vec<String> = Vec::new();
        let mut string_tokens: Vec<String> = Vec::new();
        let mut topics: Vec<TopicSpec> = Vec::new();

        for token in iter {
            tokens.push(token.clone());
            if TopicSpec::is_topic_spec(&token) {
                let spec = TopicSpec::parse(token)?;
                topics.push(spec);
            } else {
                match repo_url::Url::parse(&token) {
                    Ok(url) => string_tokens.push(url.normalized),
                    Err(_) => string_tokens.push(token),
                }
            }
        }

        Ok(Self {
            input: input.into(),
            tokens,
            string_tokens,
            topics,
        })
    }

    fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    fn wildcard_tokens(&self) -> Vec<String> {
        self.string_tokens
            .iter()
            .map(|s| format!("%{s}%"))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_topic_specs() {
        assert!(TopicSpec::is_topic_spec(
            "in:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
        ));
        assert!(TopicSpec::is_topic_spec(
            "in:/emwalker/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
        ));
    }

    #[test]
    fn test_invalid_topic_specs() {
        assert!(!TopicSpec::is_topic_spec(
            "in:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f9285"
        ));
        assert!(!TopicSpec::is_topic_spec(
            "/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
        ));
        assert!(!TopicSpec::is_topic_spec(
            "In:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
        ));
        assert!(!TopicSpec::is_topic_spec(
            "up:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
        ));
        assert!(!TopicSpec::is_topic_spec(
            "in:/wiki/links/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
        ));
    }

    #[test]
    fn test_topic_spec_parsing() {
        let s = TopicSpec::parse("in:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851".into())
            .unwrap();
        assert_eq!(s.op, "in".to_string());
        assert_eq!(
            s.resource_path,
            "/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851".to_string()
        );
        assert_eq!(s.topic_id(), "e76a690f-2eb2-45a0-9cbc-5e7d76f92851");
    }

    #[test]
    fn test_empty_query() {
        let s = QuerySpec::parse("").unwrap();
        assert_eq!(s.input, "");
        assert_eq!(s.tokens.len(), 0);
        assert_eq!(s.string_tokens.len(), 0);
        assert_eq!(s.topics.len(), 0);
    }

    #[test]
    fn test_simple_query() {
        let s = QuerySpec::parse("a b").unwrap();
        assert_eq!(s.input, "a b");
        assert_eq!(s.tokens, ["a", "b"]);
        assert_eq!(s.string_tokens, ["a", "b"]);
        assert_eq!(s.topics.len(), 0);
    }

    #[test]
    fn test_topic_query() {
        let s = QuerySpec::parse("in:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851").unwrap();
        assert_eq!(
            s.input,
            "in:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
        );
        assert_eq!(s.tokens.len(), 1);
        assert_eq!(s.string_tokens.len(), 0);
        assert_eq!(s.topics.len(), 1);
    }

    #[test]
    fn test_urls_are_normalized() {
        let s = QuerySpec::parse("https://www.google.com/?s=1234").unwrap();
        assert_eq!(s.string_tokens, ["https://www.google.com"]);
    }

    #[test]
    fn test_combined_query() {
        let s =
            QuerySpec::parse("in:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851 a b").unwrap();
        assert_eq!(
            s.input,
            "in:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851 a b"
        );
        assert_eq!(s.tokens.len(), 3);
        assert_eq!(s.string_tokens, ["a", "b"]);
        assert_eq!(s.topics.len(), 1);
        assert_eq!(
            *s.topics.get(0).unwrap(),
            TopicSpec::parse("in:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851".into())
                .unwrap()
        );
    }

    #[test]
    fn test_wildcard_tokens_empty_string() {
        let s = QuerySpec::parse("").unwrap();
        assert_eq!(s.wildcard_tokens().len(), 0);
    }

    #[test]
    fn test_wildcard_tokens_simple_string() {
        let s = QuerySpec::parse("one two").unwrap();
        assert_eq!(s.wildcard_tokens(), ["%one%", "%two%"]);
    }

    #[test]
    fn test_wildcard_tokens_exclude_a_topic_spec() {
        let s = QuerySpec::parse("one two in:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851")
            .unwrap();
        assert_eq!(s.wildcard_tokens(), ["%one%", "%two%"]);
    }
}
