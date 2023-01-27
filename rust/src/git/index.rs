use git2;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::PathBuf;
use unidecode::unidecode;

use super::{activity, core, Client, GitPaths, Kind, RepoTopic, Search, Synonym, TopicChild};
use crate::prelude::*;

// Omit dashes so that we can split on them
const SPECIAL_CHARS: &[char] = &[
    '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '.', '/', ':', ';', '<', '=', '>',
    '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
];

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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SynonymEntry {
    pub name: String,
    pub id: ExternalId,
}

impl std::cmp::PartialOrd for SynonymEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for SynonymEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.name.len(), &self.name, &self.id).cmp(&(other.name.len(), &other.name, &other.id))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct SynonymMatch {
    pub cycle: bool,
    pub entry: SynonymEntry,
    pub name: String,
    pub repo_id: RepoId,
    pub repo_topic: RepoTopic,
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
            repo_id: self.repo_id,
            repo_topic: self.repo_topic.to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SynonymIndexMap {
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

#[derive(Clone, Debug)]
pub struct SynonymIndex {
    pub filename: PathBuf,
    pub index: SynonymIndexMap,
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
        Self::make(
            filename.to_owned(),
            SynonymIndexMap {
                api_version: API_VERSION.to_owned(),
                kind: "SynonymIndexMap".to_owned(),
                synonyms: BTreeMap::new(),
            },
        )
    }

    pub fn load(filename: &PathBuf, view: &core::View) -> Result<Self> {
        let index = if view.blob_exists(filename)? {
            match view.find_blob_by_filename(filename)? {
                Some(blob) => Self {
                    filename: filename.to_owned(),
                    index: blob.try_into()?,
                },
                None => Self::new(filename),
            }
        } else {
            Self::new(filename)
        };
        Ok(index)
    }

    pub fn make(filename: PathBuf, index: SynonymIndexMap) -> Self {
        Self { filename, index }
    }

    pub fn add(&mut self, topic_id: &ExternalId, phrase: Phrase, name: &str) -> Result<()> {
        let paths = self.index.synonyms.entry(phrase).or_default();

        paths.insert(SynonymEntry {
            name: name.to_owned(),
            id: topic_id.to_owned(),
        });

        Ok(())
    }

    pub fn full_matches(&self, phrase: &Phrase) -> Result<BTreeSet<SynonymEntry>> {
        Ok(self.index.full_matches(phrase))
    }

    pub fn remove(&mut self, id: &ExternalId, phrase: Phrase, name: &str) -> Result<()> {
        if let Some(paths) = self.index.synonyms.get_mut(&phrase) {
            paths.remove(&SynonymEntry {
                name: name.to_owned(),
                id: id.to_owned(),
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
    pub id: ExternalId,
}

impl From<&TopicChild> for SearchEntry {
    fn from(child: &TopicChild) -> Self {
        Self {
            kind: child.kind.to_owned(),
            id: child.id.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchTokenIndexMap {
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
        Self::make(
            filename.to_owned(),
            SearchTokenIndexMap {
                api_version: API_VERSION.to_owned(),
                kind: "SearchTokenIndexMap".to_owned(),
                tokens: BTreeMap::new(),
            },
        )
    }

    pub fn make(filename: PathBuf, index: SearchTokenIndexMap) -> Self {
        Self { filename, index }
    }

    fn add(&mut self, entry: &SearchEntry, token: Phrase) -> Result<()> {
        let entries = self.index.tokens.entry(token).or_default();
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
        if let Some(entries) = self.index.tokens.get_mut(&token) {
            entries.remove(entry);
            if entries.is_empty() {
                self.index.tokens.remove(&token);
            }
        }

        Ok(())
    }
}

impl TryInto<activity::Change> for git2::Blob<'_> {
    type Error = Error;

    fn try_into(self) -> Result<activity::Change> {
        Ok(serde_yaml::from_slice(self.content())?)
    }
}

impl TryInto<ActivityIndexMap> for git2::Blob<'_> {
    type Error = Error;

    fn try_into(self) -> Result<ActivityIndexMap> {
        Ok(serde_yaml::from_slice(self.content())?)
    }
}

impl TryInto<SearchTokenIndexMap> for git2::Blob<'_> {
    type Error = Error;

    fn try_into(self) -> Result<SearchTokenIndexMap> {
        Ok(serde_yaml::from_slice(self.content())?)
    }
}

impl TryInto<SynonymIndexMap> for git2::Blob<'_> {
    type Error = Error;

    fn try_into(self) -> Result<SynonymIndexMap> {
        Ok(serde_yaml::from_slice(self.content())?)
    }
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct ChangeReference {
    pub id: ExternalId,
    pub date: Timestamp,
}

impl std::fmt::Display for ChangeReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl ChangeReference {
    pub fn new(change: &activity::Change) -> Self {
        Self {
            date: change.date(),
            id: change.id(),
        }
    }
}

impl std::cmp::Ord for ChangeReference {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Sort descending by date
        (&other.date, &self.id).cmp(&(&self.date, &other.id))
    }
}

impl std::cmp::PartialOrd for ChangeReference {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub struct ActivityIndexMap {
    pub api_version: String,
    pub changes: BTreeSet<ChangeReference>,
}

impl ActivityIndexMap {
    pub fn new() -> Self {
        Self {
            api_version: API_VERSION.to_owned(),
            changes: BTreeSet::new(),
        }
    }
}

pub struct ActivityIndex {
    filename: PathBuf,
    index: ActivityIndexMap,
}

impl ActivityIndex {
    pub fn new(filename: &PathBuf) -> Self {
        Self {
            filename: filename.to_owned(),
            index: ActivityIndexMap::new(),
        }
    }

    pub fn load(filename: &PathBuf, view: &core::View) -> Result<Self> {
        let index = if view.blob_exists(filename)? {
            match view.find_blob_by_filename(filename)? {
                Some(blob) => Self {
                    filename: filename.to_owned(),
                    index: blob.try_into()?,
                },
                None => Self::new(filename),
            }
        } else {
            Self::new(filename)
        };
        Ok(index)
    }

    pub fn add(&mut self, reference: ChangeReference) {
        self.index.changes.insert(reference);
    }

    pub fn references(&self) -> &BTreeSet<ChangeReference> {
        &self.index.changes
    }
}

impl Index for ActivityIndex {
    fn filename(&self) -> &PathBuf {
        &self.filename
    }

    fn serialize(&self) -> Result<String> {
        Ok(serde_yaml::to_string(&self.index)?)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IndexKey {
    pub repo_id: RepoId,
    pub basename: String,
}

pub trait GitIndexKey {
    // The value of `token` will sometimes need to be normalized by the caller in order for lookups
    // to work as expected.  We do not normalize the token here because some searches, e.g.,
    // of urls, are more sensitive to normalization, and so we omit it in those cases.
    fn index_key(&self, token: &Phrase) -> Result<IndexKey> {
        if !token.is_valid() {
            return Err(Error::Repo(format!("a valid token is required: {token}")));
        }

        match token.basename() {
            Some(basename) => Ok(IndexKey {
                repo_id: self.prefix(),
                basename,
            }),
            None => Err(Error::Repo(format!("bad token: {token}"))),
        }
    }

    fn prefix(&self) -> RepoId;
}

impl GitIndexKey for RepoId {
    fn prefix(&self) -> Self {
        self.to_owned()
    }
}

impl IndexKey {
    fn index_filename(&self, index_type: IndexType) -> Result<PathBuf> {
        let file_path = match index_type {
            IndexType::Search => format!("indexes/search/{}.yaml", self.basename),
            IndexType::SynonymPhrase => {
                format!("indexes/synonyms/phrases/{}.yaml", self.basename)
            }
            IndexType::SynonymToken => {
                format!("indexes/synonyms/tokens/{}.yaml", self.basename)
            }
        };
        Ok(PathBuf::from(file_path))
    }

    pub fn search_index(
        &self,
        client: &Client,
        index_type: IndexType,
        mode: IndexMode,
    ) -> Result<SearchTokenIndex> {
        let filename = self.index_filename(index_type)?;
        match mode {
            IndexMode::Replace => Ok(SearchTokenIndex::new(&filename)),
            IndexMode::ReadOnly => client.fetch_token_index(self.repo_id, &filename),
            IndexMode::Update => client.fetch_token_index(self.repo_id, &filename),
        }
    }

    pub fn synonym_index(
        &self,
        client: &Client,
        index_type: IndexType,
        mode: IndexMode,
    ) -> Result<SynonymIndex> {
        let filename = self.index_filename(index_type)?;
        match mode {
            IndexMode::Replace => Ok(SynonymIndex::new(&filename)),
            IndexMode::ReadOnly => client.fetch_synonym_index(self.repo_id, &filename),
            IndexMode::Update => client.fetch_synonym_index(self.repo_id, &filename),
        }
    }

    pub fn token_index(&self, client: &Client, mode: IndexMode) -> Result<SearchTokenIndex> {
        let filename = self.index_filename(IndexType::Search)?;
        match mode {
            IndexMode::Replace => Ok(SearchTokenIndex::new(&filename)),
            IndexMode::ReadOnly => client.fetch_token_index(self.repo_id, &filename),
            IndexMode::Update => client.fetch_token_index(self.repo_id, &filename),
        }
    }
}

pub trait SaveChangesForPrefix {
    fn save(
        &self,
        repo_id: RepoId,
        changes: &HashMap<RepoId, BTreeSet<activity::Change>>,
    ) -> Result<()>;
}

pub struct Indexer {
    path_activity: HashMap<(RepoId, ExternalId), ActivityIndex>,
    pub mode: IndexMode,
    repo_changes: HashMap<RepoId, BTreeSet<activity::Change>>,
    search_tokens: HashMap<IndexKey, SearchTokenIndex>,
    synonym_phrases: HashMap<IndexKey, SynonymIndex>,
    synonym_tokens: HashMap<IndexKey, SynonymIndex>,
}

impl Indexer {
    pub fn new(mode: IndexMode) -> Self {
        Self {
            mode,
            path_activity: HashMap::new(),
            repo_changes: HashMap::new(),
            search_tokens: HashMap::new(),
            synonym_phrases: HashMap::new(),
            synonym_tokens: HashMap::new(),
        }
    }

    pub fn add_change(
        &mut self,
        repo_id: RepoId,
        client: &Client,
        change: &activity::Change,
    ) -> Result<()> {
        for id in change.ids() {
            let activity = self.id_activity(client, repo_id, id)?;
            activity.add(change.to_reference());
        }

        let set = self.repo_changes.entry(repo_id).or_default();
        set.insert(change.to_owned());

        Ok(())
    }

    pub fn id_activity(
        &mut self,
        client: &Client,
        repo_id: RepoId,
        id: &ExternalId,
    ) -> Result<&mut ActivityIndex> {
        let key = (repo_id, id.to_owned());
        let index = self
            .path_activity
            .entry(key)
            .or_insert_with(|| client.fetch_activity_log(repo_id, id, self.mode).unwrap());
        Ok(index)
    }

    fn synonym_phrase_index(
        &mut self,
        client: &Client,
        key: &IndexKey,
        index_type: IndexType,
    ) -> Result<&mut SynonymIndex> {
        let index = self
            .synonym_phrases
            .entry(key.to_owned())
            .or_insert_with(|| {
                let filename = key
                    .index_filename(index_type)
                    .unwrap_or_else(|_| panic!("no index filename: {key:?}"));
                client
                    .fetch_synonym_index(key.repo_id, &filename)
                    .unwrap_or_else(|_| panic!("no index: {filename:?}"))
            });

        Ok(index)
    }

    fn synonym_token_index(
        &mut self,
        client: &Client,
        key: &IndexKey,
        index_type: IndexType,
    ) -> Result<&mut SynonymIndex> {
        let index = self
            .synonym_tokens
            .entry(key.to_owned())
            .or_insert_with(|| {
                let filename = key
                    .index_filename(index_type)
                    .unwrap_or_else(|_| panic!("no index filename: {key:?}"));
                client
                    .fetch_synonym_index(key.repo_id, &filename)
                    .unwrap_or_else(|_| panic!("no index: {filename:?}"))
            });

        Ok(index)
    }

    fn synonym_indexes<'s, S, F>(
        &mut self,
        client: &Client,
        repo_id: RepoId,
        synonyms: S,
        f: F,
    ) -> Result<()>
    where
        S: Iterator<Item = &'s Synonym>,
        F: Fn(&mut SynonymIndex, &Phrase, &String) -> Result<()>,
    {
        for synonym in synonyms {
            let phrase = Phrase::parse(&synonym.name);
            if !phrase.is_valid() {
                continue;
            }

            let key = repo_id.index_key(&phrase)?;
            let index = self.synonym_phrase_index(client, &key, IndexType::SynonymPhrase)?;
            f(index, &phrase, &synonym.name)?;

            for token in phrase.tokens() {
                if !token.is_valid() {
                    continue;
                }

                let key = repo_id.index_key(&token)?;
                let index = self.synonym_token_index(client, &key, IndexType::SynonymToken)?;
                f(index, &token, &synonym.name)?;
            }
        }

        Ok(())
    }

    pub fn remove_searches<'s, S>(
        &mut self,
        client: &Client,
        repo_id: RepoId,
        entry: &SearchEntry,
        searches: S,
    ) -> Result<()>
    where
        S: Iterator<Item = &'s Search>,
    {
        for search in searches {
            for token in &search.tokens {
                let key = repo_id.index_key(token)?;
                self.search_token_index(client, &key)?
                    .remove(entry, token.to_owned())?;
            }
        }

        Ok(())
    }

    pub fn remove_synonyms(
        &mut self,
        client: &Client,
        repo_id: RepoId,
        id: &ExternalId,
        topic: &RepoTopic,
    ) -> Result<()> {
        self.synonym_indexes(
            client,
            repo_id,
            topic.metadata.synonyms().iter(),
            |index, token, name| {
                index.remove(id, token.to_owned(), name)?;
                Ok(())
            },
        )?;

        Ok(())
    }

    pub fn files(&self) -> Result<Vec<(RepoId, PathBuf, String)>> {
        let mut files = vec![];

        for (key, index) in &self.search_tokens {
            files.push((key.repo_id, index.filename().to_owned(), index.serialize()?));
        }

        for (key, index) in &self.synonym_phrases {
            files.push((key.repo_id, index.filename().to_owned(), index.serialize()?));
        }

        for (key, index) in &self.synonym_tokens {
            files.push((key.repo_id, index.filename().to_owned(), index.serialize()?));
        }

        for ((repo_id, _id), activity_log) in &self.path_activity {
            files.push((
                *repo_id,
                activity_log.filename().to_owned(),
                activity_log.serialize()?,
            ));
        }

        for (&repo_id, changes) in &self.repo_changes {
            for change in changes {
                let reference = change.to_reference();
                files.push((
                    repo_id,
                    reference.id.change_filename()?,
                    serde_yaml::to_string(&change)?,
                ));
            }
        }

        Ok(files)
    }

    pub fn write_repo_changes<S>(&self, store: &S) -> Result<()>
    where
        S: SaveChangesForPrefix,
    {
        match store.save(RepoId::wiki(), &self.repo_changes) {
            Ok(_) => log::info!("changes saved to {}", WIKI_REPOSITORY_ID),
            Err(err) => log::error!("problem saving changes to prefix key: {}", err),
        }

        Ok(())
    }

    fn search_token_index(
        &mut self,
        client: &Client,
        key: &IndexKey,
    ) -> Result<&mut SearchTokenIndex> {
        Ok(self
            .search_tokens
            .entry(key.to_owned())
            .or_insert(key.token_index(client, self.mode)?))
    }

    pub fn update(
        &mut self,
        client: &Client,
        repo_id: RepoId,
        entry: &SearchEntry,
        before: &BTreeSet<Search>,
        after: &BTreeSet<Search>,
    ) -> Result<()> {
        let removed = before.difference(after);
        for search in removed {
            for token in &search.tokens {
                let key = repo_id.index_key(token)?;
                self.search_token_index(client, &key)?
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
                let key = repo_id.index_key(token)?;
                self.search_token_index(client, &key)?
                    .add(entry, token.to_owned())?;
            }
        }

        Ok(())
    }

    pub fn update_synonyms(
        &mut self,
        client: &Client,
        repo_id: RepoId,
        before: &Option<RepoTopic>,
        after: &RepoTopic,
    ) -> Result<()> {
        let topic_id = after.topic_id();

        let before = match before {
            Some(before) => before
                .prefixed_synonyms()
                .iter()
                .cloned()
                .collect::<HashSet<Synonym>>(),
            None => HashSet::new(),
        };

        let after = after
            .prefixed_synonyms()
            .iter()
            .cloned()
            .collect::<HashSet<Synonym>>();

        self.synonym_indexes(
            client,
            repo_id,
            after.difference(&before),
            |index, token, name| {
                index.add(topic_id, token.to_owned(), name)?;
                Ok(())
            },
        )?;

        self.synonym_indexes(
            client,
            repo_id,
            before.difference(&after),
            |index, token, name| {
                index.remove(topic_id, token.to_owned(), name)?;
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
