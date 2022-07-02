use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::PathBuf;
use unidecode::unidecode;

use super::{Git, Synonym, Topic, API_VERSION};
use crate::http::repo_url;
use crate::prelude::*;

const SPECIAL_CHARS: &[char] = &[
    '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=',
    '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
];

pub fn normalize(input: &str) -> String {
    unidecode(input).to_lowercase().replace(SPECIAL_CHARS, "")
}

#[derive(Debug, Eq, PartialEq)]
pub struct Search {
    pub urls: BTreeSet<repo_url::Url>,
    pub tokens: BTreeSet<String>,
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
        let input = unidecode(input);
        let mut tokens = BTreeSet::new();
        let mut urls = BTreeSet::new();

        for token in input.split_whitespace() {
            if repo_url::Url::is_valid_url(token) {
                urls.insert(repo_url::Url::parse(token)?);
                continue;
            }

            let token = token.to_lowercase().replace(SPECIAL_CHARS, "");
            if token.len() >= 3 && token.len() <= 20 {
                tokens.insert(token);
            }
        }

        Ok(Self { tokens, urls })
    }

    pub fn is_empty(&self) -> bool {
        self.urls.is_empty() && self.tokens.is_empty()
    }
}

#[derive(Copy, Clone)]
pub enum IndexMode {
    Update,
    Replace,
}

trait Index {
    fn filename(&self) -> &PathBuf;

    fn serialize(&self) -> Result<String>;

    fn write(&self) -> Result<()> {
        let filename = self.filename();
        let dest = filename.parent().expect("expected a parent directory");
        fs::create_dir_all(&dest).ok();
        let s = self.serialize()?;
        log::debug!("saving {:?}", filename);
        fs::write(&filename, s).map_err(Error::from)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SynonymEntry {
    pub name: String,
    pub path: String,
}

impl std::cmp::PartialOrd for SynonymEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.name.partial_cmp(&other.name) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.path.partial_cmp(&other.path)
    }
}

impl std::cmp::Ord for SynonymEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct SynonymMatch {
    pub cycle: bool,
    pub entry: SynonymEntry,
    pub name: String,
    pub topic: Topic,
}

impl std::cmp::Ord for SynonymMatch {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.entry.cmp(&other.entry)
    }
}

impl std::cmp::PartialOrd for SynonymMatch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl SynonymMatch {
    pub fn with_cycle(&self, cycle: bool) -> Self {
        Self {
            cycle,
            entry: self.entry.to_owned(),
            name: self.name.to_owned(),
            topic: self.topic.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct SynonymIndexMap {
    api_version: String,
    synonyms: BTreeMap<String, BTreeSet<SynonymEntry>>,
}

impl SynonymIndexMap {
    fn get(&self, string: &str) -> Option<&BTreeSet<SynonymEntry>> {
        self.synonyms.get(string)
    }
}

#[derive(Debug)]
pub struct SynonymIndex {
    filename: PathBuf,
    index: SynonymIndexMap,
}

impl Index for SynonymIndex {
    fn filename(&self) -> &PathBuf {
        &self.filename
    }

    fn serialize(&self) -> Result<String> {
        Ok(serde_yaml::to_string(&self.index)?)
    }
}

impl SynonymIndex {
    pub fn new(filename: &PathBuf) -> Self {
        Self {
            filename: filename.to_owned(),
            index: SynonymIndexMap {
                api_version: API_VERSION.to_owned(),
                synonyms: BTreeMap::new(),
            },
        }
    }

    pub fn load(filename: &PathBuf) -> Result<Self> {
        let index = if filename.as_path().exists() {
            let fh = std::fs::File::open(&filename).map_err(|e| Error::Repo(format!("{:?}", e)))?;
            serde_yaml::from_reader(fh)?
        } else {
            SynonymIndexMap {
                api_version: API_VERSION.to_owned(),
                synonyms: BTreeMap::new(),
            }
        };

        Ok(Self {
            filename: filename.to_owned(),
            index,
        })
    }

    pub fn add(&mut self, path: &RepoPath, normalized: &str, name: &str) -> Result<()> {
        let paths = self
            .index
            .synonyms
            .entry(normalized.to_owned())
            .or_insert(BTreeSet::new());

        paths.insert(SynonymEntry {
            name: name.to_owned(),
            path: path.to_string(),
        });

        Ok(())
    }

    pub fn remove(&mut self, path: &RepoPath, normalized: &str, name: &str) -> Result<()> {
        let paths = self
            .index
            .synonyms
            .entry(normalized.to_owned())
            .or_insert(BTreeSet::new());

        paths.remove(&SynonymEntry {
            name: name.to_owned(),
            path: path.to_string(),
        });

        Ok(())
    }

    pub fn matches(&self, name: &str) -> Result<Vec<&SynonymEntry>> {
        let normalized = normalize(name);
        match self.index.get(&normalized) {
            Some(entries) => Ok(entries.iter().collect_vec()),
            None => Ok(vec![]),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TokenIndexMap {
    api_version: String,
    tokens: BTreeMap<String, BTreeSet<String>>,
}

impl TokenIndexMap {
    fn get(&self, string: &str) -> Option<&BTreeSet<String>> {
        self.tokens.get(string)
    }
}

#[derive(Debug)]
pub struct TokenIndex {
    filename: PathBuf,
    index: TokenIndexMap,
}

impl Index for TokenIndex {
    fn filename(&self) -> &PathBuf {
        &self.filename
    }

    fn serialize(&self) -> Result<String> {
        Ok(serde_yaml::to_string(&self.index)?)
    }
}

impl TokenIndex {
    pub fn new(filename: &PathBuf) -> Self {
        Self {
            filename: filename.to_owned(),
            index: TokenIndexMap {
                api_version: API_VERSION.to_owned(),
                tokens: BTreeMap::new(),
            },
        }
    }

    pub fn load(filename: &PathBuf) -> Result<Self> {
        let index = if filename.as_path().exists() {
            let fh = std::fs::File::open(&filename).map_err(|e| Error::Repo(format!("{:?}", e)))?;
            serde_yaml::from_reader(fh)?
        } else {
            TokenIndexMap {
                api_version: API_VERSION.to_owned(),
                tokens: BTreeMap::new(),
            }
        };

        Ok(Self {
            filename: filename.to_owned(),
            index,
        })
    }

    fn add(&mut self, path: &RepoPath, normalized: &str) -> Result<()> {
        let paths = self
            .index
            .tokens
            .entry(normalized.to_owned())
            .or_insert(BTreeSet::new());
        paths.insert(path.to_string());
        Ok(())
    }

    pub fn indexed_on(&self, path: &RepoPath, token: &str) -> Result<bool> {
        Ok(match &self.index.get(token) {
            Some(paths) => paths.contains(&path.inner),
            None => false,
        })
    }

    fn remove(&mut self, path: &RepoPath, normalized: &str) -> Result<()> {
        let paths = self
            .index
            .tokens
            .entry(normalized.to_owned())
            .or_insert(BTreeSet::new());
        paths.remove(&path.inner);
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IndexKey {
    pub prefix: String,
    pub field: String,
}

pub struct Indexer {
    git: Git,
    mode: IndexMode,
    synonym_indexes: HashMap<IndexKey, SynonymIndex>,
    token_indexes: HashMap<IndexKey, TokenIndex>,
}

impl Indexer {
    pub fn new(git: &Git, mode: IndexMode) -> Self {
        Self {
            git: git.clone(),
            mode,
            synonym_indexes: HashMap::new(),
            token_indexes: HashMap::new(),
        }
    }

    fn synonym_indexes<'s, S, F>(&mut self, prefix: &str, synonyms: S, f: F) -> Result<()>
    where
        S: Iterator<Item = &'s Synonym>,
        F: Fn(&mut SynonymIndex, &String, &String) -> Result<()>,
    {
        for synonym in synonyms {
            let name = &synonym.name;

            let normalized = normalize(name);
            if normalized.len() < 3 {
                continue;
            }

            let key = self.git.index_key(prefix, &normalized)?;
            let index = self.synonym_index_for(&key)?;
            f(index, &normalized, name)?;
        }

        Ok(())
    }

    pub fn remove_searches(&mut self, path: &RepoPath, searches: &Vec<Search>) -> Result<()> {
        for search in searches {
            for token in &search.tokens {
                let key = self.git.index_key(&path.prefix, token)?;
                self.token_index_for(&key)?.remove(path, token)?;
            }

            for url in &search.urls {
                let normalized = &url.normalized;
                let key = self.git.index_key(&path.prefix, normalized)?;
                self.token_index_for(&key)?.remove(path, normalized)?;
            }
        }

        Ok(())
    }

    pub fn remove_synonyms(&mut self, path: &RepoPath, topic: &Topic) -> Result<()> {
        self.synonym_indexes(
            &path.prefix,
            topic.metadata.synonyms.iter(),
            |index, normalized, name| {
                index.remove(path, normalized, name)?;
                Ok(())
            },
        )?;

        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        for index in self.token_indexes.values() {
            index.write()?;
        }

        for index in self.synonym_indexes.values() {
            index.write()?;
        }

        Ok(())
    }

    fn synonym_index_for(&mut self, key: &IndexKey) -> Result<&mut SynonymIndex> {
        Ok(self
            .synonym_indexes
            .entry(key.to_owned())
            .or_insert(self.git.synonym_index(key, self.mode)?))
    }

    fn token_index_for(&mut self, key: &IndexKey) -> Result<&mut TokenIndex> {
        Ok(self
            .token_indexes
            .entry(key.to_owned())
            .or_insert(self.git.token_index(key, self.mode)?))
    }

    pub fn update_lookups(
        &mut self,
        path: &RepoPath,
        before: &BTreeSet<Search>,
        after: &BTreeSet<Search>,
    ) -> Result<()> {
        let removed = before.difference(after);
        for search in removed {
            for token in &search.tokens {
                let key = self.git.index_key(&path.prefix, token)?;
                self.token_index_for(&key)?.remove(path, token)?;
            }

            for url in &search.urls {
                let normalized = &url.normalized;
                let key = self.git.index_key(&path.prefix, normalized)?;
                self.token_index_for(&key)?.remove(path, normalized)?;
            }
        }

        let added = after.difference(before);
        for search in added {
            for token in &search.tokens {
                let key = self.git.index_key(&path.prefix, token)?;
                self.token_index_for(&key)?.add(path, token)?;
            }

            for url in &search.urls {
                let normalized = &url.normalized;
                let key = self.git.index_key(&path.prefix, normalized)?;
                self.token_index_for(&key)?.add(path, normalized)?;
            }
        }

        Ok(())
    }

    pub fn update_synonyms(&mut self, before: &Option<Topic>, after: &Topic) -> Result<()> {
        let path = after.path();

        let before = match before {
            Some(before) => before
                .metadata
                .synonyms
                .iter()
                .map(|s| s.to_owned())
                .collect::<BTreeSet<Synonym>>(),
            None => BTreeSet::new(),
        };

        let after = after
            .metadata
            .synonyms
            .iter()
            .map(|s| s.to_owned())
            .collect::<BTreeSet<Synonym>>();

        self.synonym_indexes(
            &path.prefix,
            after.difference(&before),
            |index, normalized, name| {
                index.add(&path, normalized, name)?;
                Ok(())
            },
        )?;

        self.synonym_indexes(
            &path.prefix,
            before.difference(&after),
            |index, normalized, name| {
                index.remove(&path, normalized, name)?;
                Ok(())
            },
        )?;

        Ok(())
    }
}

impl std::fmt::Debug for Indexer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Indexer")
            .field("indexes", &self.token_indexes.keys())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn punctuation() {
        let phrase = Search::parse("one.!?:`#$@*&;+-{}[]()/\\'\",=    <> two").unwrap();
        assert_eq!(phrase.tokens, BTreeSet::from(["one".into(), "two".into()]));
    }

    #[test]
    fn uppercase_letters() {
        let phrase = Search::parse("One TWO three").unwrap();
        assert_eq!(
            phrase.tokens,
            BTreeSet::from(["one".into(), "two".into(), "three".into()])
        );
    }

    #[test]
    fn unicode_characters() {
        let phrase = Search::parse("Æneid étude 北亰 ᔕᓇᓇ げんまい茶").unwrap();
        assert_eq!(
            phrase.tokens,
            BTreeSet::from([
                "aeneid".into(),
                "etude".into(),
                "bei".into(),
                "jing".into(),
                "shanana".into(),
                "genmaicha".into()
            ])
        );
    }

    #[test]
    fn token_length() {
        let token = (0..=20).map(|_| "a").collect::<String>();
        assert_eq!(token.len(), 21);

        let phrase = Search::parse(&format!("a aa aaa aaaa {}", token)).unwrap();
        assert_eq!(phrase.tokens, BTreeSet::from(["aaa".into(), "aaaa".into()]));
    }

    #[test]
    fn url() {
        let phrase = Search::parse("one https://www.google.com").unwrap();
        assert_eq!(
            phrase.urls,
            BTreeSet::from([repo_url::Url::parse("https://www.google.com").unwrap()]),
        );

        let phrase = Search::parse("aaas:").unwrap();
        assert_eq!(phrase.urls, BTreeSet::new());
    }
}
