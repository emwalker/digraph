use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};
use std::sync::Arc;
use std::time::Instant;
use strum_macros::EnumString;

use super::{Client, Kind, Object, ObjectBuilders, Phrase, RepoObject, SynonymEntry};
use crate::git::SearchEntry;
use crate::prelude::*;
use crate::redis;
use crate::types::{Downset, Timespec, TopicPath};

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
pub enum TopicSpecOperation {
    IN,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SearchTopicSpec {
    pub op: TopicSpecOperation,
    pub id: ExternalId,
}

const ID_PATTERN: &str = r"^in:[\w-]+$";

impl SearchTopicSpec {
    fn valid_path_spec(input: &str) -> bool {
        lazy_static! {
            static ref IS_ID_SPEC: Regex = Regex::new(ID_PATTERN).unwrap();
        }
        IS_ID_SPEC.is_match(input)
    }

    fn parse(input: &str) -> Result<Self> {
        use std::str::FromStr;

        let parts: Vec<String> = input.trim().split(':').map(str::to_string).collect();
        if !parts.len() == 2 {
            return Err(Error::Parse(format!("invalid topic spec: {input}")));
        }

        let op = TopicSpecOperation::from_str(&parts[0])?;
        Ok(Self {
            op,
            id: ExternalId::try_from(&parts[1])?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Search {
    pub normalized: Phrase,
    pub urls: BTreeSet<RepoUrl>,
    pub tokens: BTreeSet<Phrase>,
    pub topic_specs: BTreeSet<SearchTopicSpec>,
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
            topic_specs: BTreeSet::new(),
        }
    }

    pub fn parse(input: &str) -> Result<Self> {
        let mut tokens = BTreeSet::new();
        let mut urls = BTreeSet::new();
        let mut topic_specs = BTreeSet::new();

        for part in input.split_whitespace() {
            if SearchTopicSpec::valid_path_spec(part) {
                topic_specs.insert(SearchTopicSpec::parse(part)?);
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
            topic_specs,
            tokens,
            urls,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.urls.is_empty() && self.tokens.is_empty() && self.topic_specs.is_empty()
    }

    pub fn topics_only(&self) -> bool {
        self.urls.is_empty() && self.tokens.is_empty() && !self.topic_specs.is_empty()
    }
}

pub struct FetchTopicLiveSearch {
    pub limit: usize,
    pub repos: RepoIds,
    pub search: Search,
    pub viewer: Arc<Viewer>,
}

#[derive(Debug)]
pub struct FetchTopicLiveSearchResult {
    pub synonyms: BTreeSet<SynonymEntry>,
}

impl FetchTopicLiveSearch {
    pub fn call(&self, client: &Client) -> Result<FetchTopicLiveSearchResult> {
        if self.search.tokens.is_empty() {
            log::info!("empty search, returning no results");
            return Ok(FetchTopicLiveSearchResult {
                synonyms: BTreeSet::new(),
            });
        }

        log::info!("searching for topics: {:?}", self.search);
        let matches = self.fetch(client);

        Ok(FetchTopicLiveSearchResult {
            synonyms: matches.iter().take(self.limit).cloned().collect(),
        })
    }

    fn fetch(&self, client: &Client) -> BTreeSet<SynonymEntry> {
        let mut matches = BTreeSet::new();
        for &repo_id in self.repos.iter() {
            self.fetch_prefix(client, repo_id, &mut matches);
        }
        matches
    }

    fn fetch_prefix(&self, client: &Client, repo_id: RepoId, matches: &mut BTreeSet<SynonymEntry>) {
        let tokens = &mut self.search.tokens.iter();
        let start = match tokens.next() {
            Some(token) => client.synonym_token_prefix_matches(repo_id, token),
            None => BTreeSet::new(),
        };

        let mut result = tokens.fold(start, |acc, token| {
            let result = client.synonym_token_prefix_matches(repo_id, token);
            acc.intersection(&result).cloned().collect()
        });

        matches.append(&mut result);
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SortKey(pub Kind, pub bool, pub String, pub ExternalId);

#[derive(Clone, Debug)]
pub struct SearchMatch {
    pub sort_key: SortKey,
    pub kind: Kind,
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
    ids: HashSet<ExternalId>,
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

    fn test(&self, object: &RepoObject) -> bool {
        if self.impossible_result {
            return false;
        }

        if self.ids.is_empty() {
            return true;
        }

        if let RepoObject::Link(link) = object {
            if self.ids.contains(link.id()) {
                return true;
            }
        }

        false
    }
}

struct Filter {
    paths: HashSet<ExternalId>,
    urls: UrlMatches,
}

impl Filter {
    fn test(&self, object: &RepoObject) -> bool {
        if !self.urls.test(object) {
            return false;
        }

        if self.paths.is_empty() {
            return true;
        }

        match object {
            RepoObject::Topic(topic) => self.paths.contains(topic.topic_id()),
            RepoObject::Link(link) => self.paths.contains(link.id()),
        }
    }
}

pub struct RedisFetchDownSet {
    pub client: Arc<Client>,
    pub redis: Arc<redis::Redis>,
}

impl Downset for RedisFetchDownSet {
    fn intersection(&self, topic_paths: &[TopicPath]) -> Result<HashSet<ExternalId>> {
        self.redis.intersection(self, topic_paths)
    }

    fn downset(&self, path: &TopicPath) -> HashSet<ExternalId> {
        self.client.downset(path).collect::<HashSet<ExternalId>>()
    }
}

pub struct FindMatches {
    pub context_repo_id: RepoId,
    pub limit: usize,
    pub locale: Locale,
    pub recursive: bool,
    pub search: Search,
    pub timespec: Timespec,
    pub topic_id: ExternalId,
    pub viewer: Arc<Viewer>,
}

#[derive(Debug)]
pub struct FindMatchesResult {
    pub matches: BTreeSet<SearchMatch>,
}

impl FindMatches {
    pub fn call<F>(&self, client: &Client, fetch: &F) -> Result<FindMatchesResult>
    where
        F: Downset,
    {
        if self.search.is_empty() {
            log::info!("search: empty search, returning no results");
            return Ok(FindMatchesResult {
                matches: BTreeSet::new(),
            });
        }

        log::info!(
            "search: looking within topic {} for {:?}",
            self.topic_id,
            self.search
        );
        let now = Instant::now();

        let matches = if self.search.topics_only() {
            self.fetch_downset(client, fetch)?
        } else {
            self.fetch_matches(client, fetch)?
        };

        let elapsed = now.elapsed();
        log::info!("search: elapsed time {:.2?}", elapsed);

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

        let mut objects = ObjectBuilders::new();
        let mut count: usize = 0;

        for &repo_id in self.viewer.read_repo_ids.iter() {
            let mut iter = self.search.tokens.iter();

            if let Some(token) = iter.next() {
                let mut prefix_matches = client.search_token_prefix_matches(repo_id, token)?;

                for token in iter {
                    let other = client.search_token_prefix_matches(repo_id, token)?;
                    prefix_matches.retain(|e| other.contains(e));
                }

                entries.extend(prefix_matches);
            }

            for entry in entries.iter() {
                if let Some(repo_object) = client.fetch(repo_id, &entry.id) {
                    if !filter.test(&repo_object) {
                        continue;
                    }

                    let key = Okey(entry.id.to_owned(), self.context_repo_id);
                    objects.add(key, repo_id, repo_object);
                    count += 1;

                    if count >= self.limit {
                        break;
                    }
                }
            }
        }

        objects
            .finalize()?
            .into_matches(&self.search, self.locale, self.limit)
    }

    fn fetch_downset<F>(&self, client: &Client, fetch: &F) -> Result<BTreeSet<SearchMatch>>
    where
        F: Downset,
    {
        let topic_ids = self.intersection(client, fetch)?;
        log::info!(
            "search: fetching topic downset ({} paths) in repos {:?}",
            topic_ids.len(),
            self.viewer.read_repo_ids
        );

        let mut objects = ObjectBuilders::new();

        // Ensure that the wiki repo id is a the end of the list so that private items appear at the
        // top of search results;
        let wiki_repo_id = RepoId::wiki();
        let mut repo_ids = self
            .viewer
            .read_repo_ids
            .iter()
            .cloned()
            .collect::<Vec<RepoId>>();
        repo_ids.sort_by_key(|repo_id| repo_id == &wiki_repo_id);

        for &repo_id in repo_ids.iter() {
            log::info!("search: looking within {:?} for {:?}", repo_id, self.search);
            for topic_id in topic_ids.iter().take(self.limit) {
                if let Some(repo_object) = client.fetch(repo_id, topic_id) {
                    let key = Okey(topic_id.to_owned(), self.context_repo_id);
                    objects.add(key, repo_id, repo_object);
                }
            }
        }

        objects
            .finalize()?
            .into_matches(&self.search, self.locale, self.limit)
    }

    fn intersection<F>(&self, client: &Client, fetch: &F) -> Result<HashSet<ExternalId>>
    where
        F: Downset,
    {
        let mut result = HashSet::new();

        for &repo_id in self.viewer.read_repo_ids.iter() {
            let mut topic_paths = vec![];

            // The (wiki) root topic is mostly not needed for now; let's exclude it until we know
            // how to make the downset and related implementation details fast.
            if !self.topic_id.is_root() {
                if let Some(path) = client.topic_path(repo_id, &self.topic_id)? {
                    topic_paths.push(path);
                }
            }

            for spec in &self.search.topic_specs {
                if let Some(path) = client.topic_path(repo_id, &spec.id)? {
                    topic_paths.push(path);
                }
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
                        log::error!("search: problem parsing url: {}", err);
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
            .map(|s| Phrase::parse(s))
            .collect::<BTreeSet<Phrase>>()
    }

    #[test]
    fn valid_path_specs() {
        assert!(SearchTopicSpec::valid_path_spec(
            "in:e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
        ));
    }

    #[test]
    fn invalid_path_specs() {
        assert!(!SearchTopicSpec::valid_path_spec(
            "e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
        ));
        assert!(!SearchTopicSpec::valid_path_spec(
            "In:/wiki/e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
        ));
        assert!(!SearchTopicSpec::valid_path_spec(
            "In:e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
        ));
        assert!(!SearchTopicSpec::valid_path_spec(
            "up:e76a690f-2eb2-45a0-9cbc-5e7d76f92851"
        ));
    }

    #[test]
    fn path_spec_parsing() {
        let s = SearchTopicSpec::parse("in:e76a690f-2eb2-45a0-9cbc-5e7d76f92851").unwrap();
        assert_eq!(s.op, TopicSpecOperation::IN);
        assert_eq!(
            s.id,
            ExternalId::try_from("e76a690f-2eb2-45a0-9cbc-5e7d76f92851").unwrap(),
        );
    }

    #[test]
    fn empty_query() {
        let s = Search::parse("").unwrap();
        assert_eq!(s.normalized, Phrase::parse(""));
        assert_eq!(s.tokens.len(), 0);
        assert_eq!(s.topic_specs.len(), 0);
    }

    #[test]
    fn simple_search() {
        let s = Search::parse("a b aa bb").unwrap();
        assert_eq!(s.normalized, Phrase::parse("a b aa bb"));
        assert_eq!(s.tokens, phrases(&["aa", "bb"]));
        assert_eq!(s.topic_specs.len(), 0);
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
        let phrase = Search::parse(&format!("a aa aaa aaaa {token}")).unwrap();
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
        assert_eq!(s.topic_specs.len(), 1);
    }

    #[test]
    fn combined_query() {
        let s = Search::parse("in:e76a690f-2eb2-45a0-9cbc-5e7d76f92851 a b").unwrap();
        assert_eq!(
            s.normalized,
            Phrase::parse("in:e76a690f-2eb2-45a0-9cbc-5e7d76f92851 a b"),
        );
        assert_eq!(s.tokens.len(), 0);
        assert_eq!(s.topic_specs.len(), 1);
        assert_eq!(
            *s.topic_specs.iter().next().unwrap(),
            SearchTopicSpec::parse("in:e76a690f-2eb2-45a0-9cbc-5e7d76f92851").unwrap()
        );
    }
}
