use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use unidecode::unidecode;

use super::{activity, Git, Kind, Search, Synonym, Topic, TopicChild, API_VERSION};
use crate::prelude::*;

// Omit dashes so that we can split on them
const SPECIAL_CHARS: &[char] = &[
    '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '.', '/', ':', ';', '<', '=', '>',
    '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
];

pub fn normalize(input: &str) -> String {
    unidecode(input).to_lowercase().replace(SPECIAL_CHARS, "")
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Phrase(String);

impl std::fmt::Display for Phrase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Phrase {
    pub fn parse(input: &str) -> Self {
        use itertools::Itertools;

        let normalized = unidecode(input)
            .to_lowercase()
            .replace(SPECIAL_CHARS, "")
            .replace('-', " ")
            .split_whitespace()
            .join(" ");

        Self(normalized)
    }

    // Used to derive the filename for an index
    pub fn basename(&self) -> Option<String> {
        self.0.get(0..2).map(|s| s.replace([' '], "+"))
    }

    pub fn starts_with(&self, pat: &str) -> bool {
        self.0.starts_with(pat)
    }

    pub fn tokens(&self) -> Vec<Self> {
        let mut tokens = vec![];
        for part in self.0.split_whitespace() {
            for part in part.split('-') {
                let token = Self(part.to_owned());
                if token.is_valid() {
                    tokens.push(token)
                }
            }
        }
        tokens
    }

    pub fn is_valid(&self) -> bool {
        self.0.len() >= 2
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.0.contains(&other.0)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IndexMode {
    Update,
    ReadOnly,
    Replace,
}

#[derive(Copy, Clone)]
pub enum IndexType {
    Search,
    SynonymPhrase,
    SynonymToken,
}

pub trait Index {
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
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for SynonymEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.name.len(), &self.name, &self.path).cmp(&(other.name.len(), &other.name, &other.path))
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
#[serde(rename_all = "camelCase")]
struct SynonymIndexMap {
    api_version: String,
    kind: String,
    synonyms: BTreeMap<Phrase, BTreeSet<SynonymEntry>>,
}

impl SynonymIndexMap {
    fn full_matches(&self, term: &Phrase) -> BTreeSet<SynonymEntry> {
        self.synonyms
            .get(term)
            .map(|set| set.to_owned())
            .unwrap_or_else(BTreeSet::new)
    }

    fn prefix_matches(&self, token: &Phrase) -> BTreeSet<SynonymEntry> {
        let iter = self
            .synonyms
            .range::<Phrase, _>(token..)
            .take_while(|(k, _)| k.starts_with(&token.0));

        iter.fold(BTreeSet::new(), |acc, (_token, set)| {
            acc.union(set).cloned().collect()
        })
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
                kind: "SynonymIndexMap".to_owned(),
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
                kind: "SynonymIndexMap".to_owned(),
                synonyms: BTreeMap::new(),
            }
        };

        Ok(Self {
            filename: filename.to_owned(),
            index,
        })
    }

    pub fn add(&mut self, path: &RepoPath, phrase: Phrase, name: &str) -> Result<()> {
        let paths = self.index.synonyms.entry(phrase).or_insert(BTreeSet::new());

        paths.insert(SynonymEntry {
            name: name.to_owned(),
            path: path.to_string(),
        });

        Ok(())
    }

    pub fn full_matches(&self, phrase: &Phrase) -> Result<BTreeSet<SynonymEntry>> {
        Ok(self.index.full_matches(phrase))
    }

    pub fn remove(&mut self, path: &RepoPath, phrase: Phrase, name: &str) -> Result<()> {
        if let Some(paths) = self.index.synonyms.get_mut(&phrase) {
            paths.remove(&SynonymEntry {
                name: name.to_owned(),
                path: path.to_string(),
            });
            if paths.is_empty() {
                self.index.synonyms.remove(&phrase);
            }
        }
        Ok(())
    }

    pub fn prefix_matches(&self, token: &Phrase) -> BTreeSet<SynonymEntry> {
        self.index.prefix_matches(token)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SearchEntry {
    pub kind: Kind,
    pub path: String,
}

impl From<&TopicChild> for SearchEntry {
    fn from(child: &TopicChild) -> Self {
        Self {
            kind: child.kind.to_owned(),
            path: child.path.to_owned(),
        }
    }
}

impl SearchEntry {
    pub fn path(&self) -> RepoPath {
        RepoPath::from(&self.path)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchTokenIndexMap {
    api_version: String,
    kind: String,
    tokens: BTreeMap<Phrase, BTreeSet<SearchEntry>>,
}

impl SearchTokenIndexMap {
    fn get(&self, string: &Phrase) -> Option<&BTreeSet<SearchEntry>> {
        self.tokens.get(string)
    }

    fn prefix_matches(&self, token: &Phrase) -> HashSet<SearchEntry> {
        let iter = self
            .tokens
            .range::<Phrase, _>(token..)
            .take_while(|(k, _)| k.starts_with(&token.0));

        let mut rows = HashSet::new();
        for (_token, set) in iter {
            for entry in set {
                rows.insert(entry.to_owned());
            }
        }
        rows
    }
}

#[derive(Debug)]
pub struct SearchTokenIndex {
    filename: PathBuf,
    index: SearchTokenIndexMap,
}

impl Index for SearchTokenIndex {
    fn filename(&self) -> &PathBuf {
        &self.filename
    }

    fn serialize(&self) -> Result<String> {
        Ok(serde_yaml::to_string(&self.index)?)
    }
}

impl SearchTokenIndex {
    pub fn new(filename: &PathBuf) -> Self {
        Self {
            filename: filename.to_owned(),
            index: SearchTokenIndexMap {
                api_version: API_VERSION.to_owned(),
                kind: "SearchTokenIndexMap".to_owned(),
                tokens: BTreeMap::new(),
            },
        }
    }

    pub fn load(filename: &PathBuf) -> Result<Self> {
        let index = if filename.as_path().exists() {
            let fh = std::fs::File::open(&filename).map_err(|e| Error::Repo(format!("{:?}", e)))?;
            serde_yaml::from_reader(fh)?
        } else {
            SearchTokenIndexMap {
                api_version: API_VERSION.to_owned(),
                kind: "SearchTokenIndexMap".to_owned(),
                tokens: BTreeMap::new(),
            }
        };

        Ok(Self {
            filename: filename.to_owned(),
            index,
        })
    }

    fn add(&mut self, entry: &SearchEntry, token: Phrase) -> Result<()> {
        let entries = self.index.tokens.entry(token).or_insert(BTreeSet::new());
        entries.insert(entry.to_owned());
        Ok(())
    }

    pub fn indexed_on(&self, entry: &SearchEntry, token: &Phrase) -> Result<bool> {
        Ok(match &self.index.get(token) {
            Some(matches) => matches.contains(entry),
            None => false,
        })
    }

    pub fn prefix_matches(&self, token: &Phrase) -> HashSet<SearchEntry> {
        self.index.prefix_matches(token)
    }

    fn remove(&mut self, entry: &SearchEntry, token: Phrase) -> Result<()> {
        match self.index.tokens.get_mut(&token) {
            Some(entries) => {
                entries.remove(entry);
                if entries.is_empty() {
                    self.index.tokens.remove(&token);
                }
            }
            None => {}
        }
        Ok(())
    }
}

pub struct ChangeIndex {
    filename: PathBuf,
    index: activity::ChangeIndexMap,
}

impl ChangeIndex {
    pub fn new(filename: &PathBuf) -> Self {
        Self {
            filename: filename.to_owned(),
            index: activity::ChangeIndexMap {
                api_version: API_VERSION.to_owned(),
                changes: BTreeSet::new(),
            },
        }
    }

    pub fn load(filename: &PathBuf) -> Result<Self> {
        let index = if filename.as_path().exists() {
            let fh = std::fs::File::open(&filename).map_err(|e| Error::Repo(format!("{:?}", e)))?;
            serde_yaml::from_reader(fh)?
        } else {
            activity::ChangeIndexMap {
                api_version: API_VERSION.to_owned(),
                changes: BTreeSet::new(),
            }
        };

        Ok(Self {
            filename: filename.to_owned(),
            index,
        })
    }

    pub fn add(&mut self, change: activity::Change) {
        self.index.changes.insert(change);
    }

    pub fn changes(&self) -> &BTreeSet<activity::Change> {
        &self.index.changes
    }
}

impl Index for ChangeIndex {
    fn filename(&self) -> &PathBuf {
        &self.filename
    }

    fn serialize(&self) -> Result<String> {
        Ok(serde_yaml::to_string(&self.index)?)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IndexKey {
    pub prefix: String,
    pub basename: String,
}

pub trait SaveChangesForPrefix {
    fn save(&self, prefix: &str, changes: &HashMap<String, Vec<activity::Change>>) -> Result<()>;
}

pub struct Indexer {
    git: Git,
    pub mode: IndexMode,
    search_tokens: HashMap<IndexKey, SearchTokenIndex>,
    synonym_phrases: HashMap<IndexKey, SynonymIndex>,
    synonym_tokens: HashMap<IndexKey, SynonymIndex>,
    path_changes: HashMap<String, ChangeIndex>,
    prefix_changes: HashMap<String, Vec<activity::Change>>,
}

impl Indexer {
    pub fn new(git: &Git, mode: IndexMode) -> Self {
        Self {
            git: git.clone(),
            mode,
            path_changes: HashMap::new(),
            prefix_changes: HashMap::new(),
            search_tokens: HashMap::new(),
            synonym_phrases: HashMap::new(),
            synonym_tokens: HashMap::new(),
        }
    }

    pub fn activity(&mut self, path: &str) -> Result<&mut ChangeIndex> {
        Ok(self
            .path_changes
            .entry(path.to_owned())
            .or_insert(self.git.change_index(&RepoPath::from(path), self.mode)?))
    }

    pub fn add_change(&mut self, change: &activity::Change) -> Result<()> {
        let mut prefixes = HashSet::new();

        for path in change.paths().keys() {
            let index = self.activity(path)?;
            index.add(change.to_owned());

            let path = RepoPath::from(path);
            prefixes.insert(path.prefix.to_owned());
        }

        for prefix in &prefixes {
            let list = self
                .prefix_changes
                .entry(prefix.to_owned())
                .or_insert(Vec::new());
            list.push(change.to_owned());
        }

        Ok(())
    }

    fn synonym_indexes<'s, S, F>(&mut self, prefix: &str, synonyms: S, f: F) -> Result<()>
    where
        S: Iterator<Item = &'s Synonym>,
        F: Fn(&mut SynonymIndex, &Phrase, &String) -> Result<()>,
    {
        for synonym in synonyms {
            let phrase = Phrase::parse(&synonym.name);
            if !phrase.is_valid() {
                continue;
            }

            let key = self.git.index_key(prefix, &phrase)?;
            let index = self.synonym_index(&key, IndexType::SynonymPhrase)?;
            f(index, &phrase, &synonym.name)?;

            for token in phrase.tokens() {
                if !token.is_valid() {
                    continue;
                }

                let key = self.git.index_key(prefix, &token)?;
                let index = self.synonym_index(&key, IndexType::SynonymToken)?;
                f(index, &token, &synonym.name)?;
            }
        }

        Ok(())
    }

    pub fn remove_searches<'s, S>(&mut self, entry: &SearchEntry, searches: S) -> Result<()>
    where
        S: Iterator<Item = &'s Search>,
    {
        let path = entry.path();
        for search in searches {
            for token in &search.tokens {
                let key = self.git.index_key(&path.prefix, token)?;
                self.search_token_index(&key)?
                    .remove(entry, token.to_owned())?;
            }
        }

        Ok(())
    }

    pub fn remove_synonyms(&mut self, path: &RepoPath, topic: &Topic) -> Result<()> {
        self.synonym_indexes(
            &path.prefix,
            topic.metadata.synonyms.iter(),
            |index, token, name| {
                index.remove(path, token.to_owned(), name)?;
                Ok(())
            },
        )?;

        Ok(())
    }

    pub fn save<S>(&self, store: &S) -> Result<()>
    where
        S: SaveChangesForPrefix,
    {
        for index in self.search_tokens.values() {
            index.write()?;
        }

        for index in self.synonym_phrases.values() {
            index.write()?;
        }

        for index in self.synonym_tokens.values() {
            index.write()?;
        }

        for index in self.path_changes.values() {
            index.write()?;
        }

        match store.save(WIKI_REPO_PREFIX, &self.prefix_changes) {
            Ok(_) => log::info!("changes saved to prefix key"),
            Err(err) => log::error!("problem saving changes to prefix key: {}", err),
        }

        Ok(())
    }

    fn synonym_index(
        &mut self,
        key: &IndexKey,
        index_type: IndexType,
    ) -> Result<&mut SynonymIndex> {
        let collection = match index_type {
            IndexType::SynonymPhrase => &mut self.synonym_phrases,
            IndexType::SynonymToken => &mut self.synonym_tokens,
            IndexType::Search => return Err(Error::Repo("expected a synonym index type".into())),
        };

        Ok(collection
            .entry(key.to_owned())
            .or_insert(self.git.synonym_index(key, index_type, self.mode)?))
    }

    fn search_token_index(&mut self, key: &IndexKey) -> Result<&mut SearchTokenIndex> {
        Ok(self
            .search_tokens
            .entry(key.to_owned())
            .or_insert(self.git.token_index(key, self.mode)?))
    }

    pub fn update(
        &mut self,
        entry: &SearchEntry,
        before: &BTreeSet<Search>,
        after: &BTreeSet<Search>,
    ) -> Result<()> {
        let removed = before.difference(after);
        let path = entry.path();
        for search in removed {
            for token in &search.tokens {
                let key = self.git.index_key(&path.prefix, token)?;
                self.search_token_index(&key)?
                    .remove(entry, token.to_owned())?;
            }
        }

        let empty = BTreeSet::new();
        let before = match self.mode {
            IndexMode::Replace => &empty,
            _ => before,
        };

        let added = after.difference(before);
        for search in added {
            for token in &search.tokens {
                let key = self.git.index_key(&path.prefix, token)?;
                self.search_token_index(&key)?
                    .add(entry, token.to_owned())?;
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
            |index, token, name| {
                index.add(&path, token.to_owned(), name)?;
                Ok(())
            },
        )?;

        self.synonym_indexes(
            &path.prefix,
            before.difference(&after),
            |index, token, name| {
                index.remove(&path, token.to_owned(), name)?;
                Ok(())
            },
        )?;

        Ok(())
    }
}

impl std::fmt::Debug for Indexer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Indexer")
            .field("indexes", &self.search_tokens.keys())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn phrase_tokens() {
        let phrase = Phrase::parse(
            r#"2019-07 Partnership announced between HHS, DoD and VenatoRx to develop VNRX-5133, a
            new antibiotic"#,
        );
        assert_eq!(
            phrase.tokens().iter().map(Phrase::to_string).collect_vec(),
            &[
                "2019",
                "07",
                "partnership",
                "announced",
                "between",
                "hhs",
                "dod",
                "and",
                "venatorx",
                "to",
                "develop",
                "vnrx",
                "5133",
                "new",
                "antibiotic"
            ],
        )
    }

    #[test]
    fn handling_of_hyphens() {
        let phrase = Phrase::parse("one-two-three");
        assert_eq!(phrase.to_string(), "one two three");

        let phrase = Phrase::parse(
            r#"flexible hierarchical wraps repel drug-resistant gram-negative and positive
            bacteria"#,
        );
        assert_eq!(
            phrase.to_string(),
            "flexible hierarchical wraps repel drug resistant gram negative and positive bacteria"
        );
    }
}
