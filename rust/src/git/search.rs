use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};
use std::time::Instant;
use strum_macros::EnumString;

use super::{Client, Kind, Locale, Object, Phrase, SynonymEntry};
use crate::git::SearchEntry;
use crate::prelude::*;
use crate::redis;
use crate::types::{Downset, ReadPath, Timespec};

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
pub struct SearchPathSpec {
    pub op: PathSpecOperation,
    pub id: RepoId,
}

const ID_PATTERN: &str = r#"^in:[\w-]+$"#;

impl SearchPathSpec {
    fn valid_path_spec(input: &str) -> bool {
        lazy_static! {
            static ref IS_ID_SPEC: Regex = Regex::new(ID_PATTERN).unwrap();
        }
        IS_ID_SPEC.is_match(input)
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
            id: RepoId::try_from(&parts[1])?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Search {
    pub normalized: Phrase,
    pub urls: BTreeSet<RepoUrl>,
    pub tokens: BTreeSet<Phrase>,
    pub path_specs: BTreeSet<SearchPathSpec>,
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
    pub fn empty() -> Self {
        Self {
            normalized: Phrase::parse(""),
            urls: BTreeSet::new(),
            tokens: BTreeSet::new(),
            path_specs: BTreeSet::new(),
        }
    }

    pub fn parse(input: &str) -> Result<Self> {
        let mut tokens = BTreeSet::new();
        let mut urls = BTreeSet::new();
        let mut path_specs = BTreeSet::new();

        for part in input.split_whitespace() {
            if SearchPathSpec::valid_path_spec(part) {
                path_specs.insert(SearchPathSpec::parse(part)?);
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

    pub fn topics_only(&self) -> bool {
        self.urls.is_empty() && self.tokens.is_empty() && !self.path_specs.is_empty()
    }
}

pub struct FetchTopicLiveSearch {
    pub limit: usize,
    pub repos: Vec<RepoName>,
    pub search: Search,
    pub viewer: Viewer,
}

pub struct FetchTopicLiveSearchResult {
    pub synonym_matches: BTreeSet<SynonymEntry>,
}

impl FetchTopicLiveSearch {
    pub fn call(&self, client: &Client) -> Result<FetchTopicLiveSearchResult> {
        if self.search.tokens.is_empty() {
            log::info!("empty search, returning no results");
            return Ok(FetchTopicLiveSearchResult {
                synonym_matches: BTreeSet::new(),
            });
        }

        log::info!("searching for topics: {:?}", self.search);
        let matches = self.fetch(client);

        Ok(FetchTopicLiveSearchResult {
            synonym_matches: matches.iter().take(self.limit).cloned().collect(),
        })
    }

    fn fetch(&self, client: &Client) -> BTreeSet<SynonymEntry> {
        let mut matches = BTreeSet::new();
        for prefix in &self.repos {
            self.fetch_prefix(client, prefix, &mut matches);
        }
        matches
    }

    fn fetch_prefix(
        &self,
        client: &Client,
        prefix: &RepoName,
        matches: &mut BTreeSet<SynonymEntry>,
    ) {
        let tokens = &mut self.search.tokens.iter();
        let start = match tokens.next() {
            Some(token) => client.synonym_token_prefix_matches(prefix, token),
            None => BTreeSet::new(),
        };

        let mut result = tokens.fold(start, |acc, token| {
            let result = client.synonym_token_prefix_matches(prefix, token);
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

struct UrlMatches {
    ids: HashSet<RepoId>,
    impossible_result: bool,
}

impl UrlMatches {
    fn impossible_result() -> Self {
        Self {
            ids: HashSet::new(),
            impossible_result: true,
        }
    }

    fn allow_everything() -> Self {
        Self {
            ids: HashSet::new(),
            impossible_result: false,
        }
    }

    fn test(&self, object: &Object) -> bool {
        if self.impossible_result {
            return false;
        }

        if self.ids.is_empty() {
            return true;
        }

        if let Object::Link(link) = object {
            if self.ids.contains(link.id()) {
                return true;
            }
        }

        false
    }
}

struct Filter {
    paths: HashSet<RepoId>,
    urls: UrlMatches,
}

impl Filter {
    fn test(&self, object: &Object) -> bool {
        if !self.urls.test(object) {
            return false;
        }

        if self.paths.is_empty() {
            return true;
        }

        match object {
            Object::Topic(topic) => self.paths.contains(topic.id()),
            Object::Link(link) => self.paths.contains(link.id()),
        }
    }
}

pub struct RedisFetchDownSet {
    pub client: Client,
    pub redis: redis::Redis,
}

impl Downset for RedisFetchDownSet {
    fn intersection(&self, topic_paths: &[ReadPath]) -> Result<HashSet<RepoId>> {
        self.redis.intersection(self, topic_paths)
    }

    fn downset(&self, path: &ReadPath) -> HashSet<RepoId> {
        self.client.downset(path).collect::<HashSet<RepoId>>()
    }
}

pub struct FindMatches {
    pub limit: usize,
    pub locale: Locale,
    pub repos: RepoNames,
    pub recursive: bool,
    pub search: Search,
    pub timespec: Timespec,
    pub topic_id: RepoId,
    pub viewer: Viewer,
}

pub struct FindMatchesResult {
    pub matches: BTreeSet<SearchMatch>,
}

impl FindMatches {
    pub fn call<F>(&self, client: &Client, fetch: &F) -> Result<FindMatchesResult>
    where
        F: Downset,
    {
        if self.search.is_empty() {
            log::info!("empty search, returning no results");
            return Ok(FindMatchesResult {
                matches: BTreeSet::new(),
            });
        }

        log::info!("searching within topic {}", self.topic_id);
        let now = Instant::now();

        let matches = if self.search.topics_only() {
            self.fetch_downset(client, fetch)?
        } else {
            self.fetch_matches(client, fetch)?
        };

        let elapsed = now.elapsed();
        log::info!("search took {:.2?}", elapsed);

        Ok(FindMatchesResult { matches })
    }

    fn fetch_matches<F>(&self, client: &Client, fetch: &F) -> Result<BTreeSet<SearchMatch>>
    where
        F: Downset,
    {
        log::info!("fetching matches");
        let paths = self.intersection(client, fetch)?;

        let filter = Filter {
            urls: self.url_paths()?,
            paths,
        };

        let mut entries = BTreeSet::new();

        for path in &filter.urls.ids {
            let entry = SearchEntry {
                id: path.to_owned(),
                kind: Kind::Link,
            };
            entries.insert(entry);
        }

        let mut matches = BTreeSet::new();
        let mut count: usize = 0;

        for repo in self.repos.iter() {
            let mut iter = self.search.tokens.iter();

            if let Some(token) = iter.next() {
                let mut prefix_matches = client.search_token_prefix_matches(repo, token)?;

                for token in iter {
                    let other = client.search_token_prefix_matches(repo, token)?;
                    prefix_matches.retain(|e| other.contains(e));
                }

                entries.extend(prefix_matches);
            }

            for entry in entries.iter() {
                if let Some(object) = client.fetch(repo, &entry.id) {
                    if !filter.test(&object) {
                        continue;
                    }

                    let object = object.to_search_match(self.locale, &self.search);
                    matches.insert(object);
                    count += 1;

                    if count >= self.limit {
                        break;
                    }
                }
            }
        }

        Ok(matches)
    }

    fn fetch_downset<F>(&self, client: &Client, fetch: &F) -> Result<BTreeSet<SearchMatch>>
    where
        F: Downset,
    {
        let topic_ids = self.intersection(client, fetch)?;
        log::info!("fetching topic downset ({} paths)", topic_ids.len());

        let mut matches = BTreeSet::new();
        let mut count: usize = 0;

        for repo in self.repos.iter() {
            for topic_id in topic_ids.iter().take(self.limit) {
                if let Some(object) = client.fetch(repo, topic_id) {
                    matches.insert(object.to_search_match(Locale::EN, &self.search));
                    count += 1;

                    if count >= self.limit {
                        break;
                    }
                }
            }
        }

        Ok(matches)
    }

    fn intersection<F>(&self, client: &Client, fetch: &F) -> Result<HashSet<RepoId>>
    where
        F: Downset,
    {
        let mut result = HashSet::new();

        for repo in self.repos.iter() {
            let mut topic_paths = vec![];

            // The (wiki) root topic is mostly not needed for now; let's exclude it until we know how to
            // make the downset and related implementation details fast.
            if !self.topic_id.is_root() {
                let path = client.read_path(repo, &self.topic_id)?;
                topic_paths.push(path);
            }

            for spec in &self.search.path_specs {
                let path = client.read_path(repo, &spec.id)?;
                topic_paths.push(path);
            }

            let set = fetch.intersection(&topic_paths)?;
            result.extend(set);
        }

        Ok(result)
    }

    fn url_paths(&self) -> Result<UrlMatches> {
        let mut urls = HashSet::new();
        for url in &self.search.urls {
            urls.insert(url.normalized.to_owned());
        }

        if urls.is_empty() {
            return Ok(UrlMatches::allow_everything());
        }

        if urls.len() != 1 {
            return Ok(UrlMatches::impossible_result());
        }

        match urls.iter().next() {
            Some(url) => {
                let mut ids = HashSet::new();
                let url = match RepoUrl::parse(url) {
                    Ok(url) => url,
                    Err(err) => {
                        log::error!("problem parsing url: {}", err);
                        return Ok(UrlMatches::impossible_result());
                    }
                };

                let id = url.id()?;
                ids.insert(id);

                Ok(UrlMatches {
                    ids,
                    impossible_result: false,
                })
            }

            None => Ok(UrlMatches::impossible_result()),
        }
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
        assert!(SearchPathSpec::valid_path_spec(
            "in:e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
        ));
    }

    #[test]
    fn invalid_path_specs() {
        assert!(!SearchPathSpec::valid_path_spec(
            "/wiki/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
        ));
        assert!(!SearchPathSpec::valid_path_spec(
            "In:/wiki/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
        ));
        assert!(!SearchPathSpec::valid_path_spec(
            "up:/wiki/topics/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
        ));
    }

    #[test]
    fn path_spec_parsing() {
        let s = SearchPathSpec::parse("in:e76a690f-2eb2-45a0-9cbc-5e7d76f92851").unwrap();
        assert_eq!(s.op, PathSpecOperation::IN);
        assert_eq!(
            s.id,
            RepoId::try_from("e76a690f-2eb2-45a0-9cbc-5e7d76f92851").unwrap(),
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
        let s = Search::parse("in:e76a690f-2eb2-45a0-9cbc-5e7d76f92851").unwrap();
        assert_eq!(
            s.normalized,
            Phrase::parse("in:e76a690f-2eb2-45a0-9cbc-5e7d76f92851"),
        );
        assert_eq!(s.tokens.len(), 0);
        assert_eq!(s.path_specs.len(), 1);
    }

    #[test]
    fn combined_query() {
        let s = Search::parse("in:e76a690f-2eb2-45a0-9cbc-5e7d76f92851 a b").unwrap();
        assert_eq!(
            s.normalized,
            Phrase::parse("in:e76a690f-2eb2-45a0-9cbc-5e7d76f92851 a b"),
        );
        assert_eq!(s.tokens.len(), 0);
        assert_eq!(s.path_specs.len(), 1);
        assert_eq!(
            *s.path_specs.iter().next().unwrap(),
            SearchPathSpec::parse("in:e76a690f-2eb2-45a0-9cbc-5e7d76f92851").unwrap()
        );
    }
}
