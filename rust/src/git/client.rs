use git2;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::PathBuf;

use super::checks::LeakedData;
use super::index::{
    ActivityIndex, GitIndexKey, IndexMode, IndexType, Indexer, Phrase, SaveChangesForPrefix,
    SearchEntry, SynonymEntry, SynonymMatch,
};
use super::{
    activity, core, DownsetIter, ObjectBuilders, RepoLink, RepoObject, RepoStats, RepoTopic,
    Search, SearchTokenIndex, SynonymIndex, TopicDownsetIter,
};
use crate::prelude::*;
use crate::types::{ReadPath, Timespec};

#[derive(Clone, Debug, Default)]
pub struct DataRoot {
    pub path: PathBuf,
}

impl std::fmt::Display for DataRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.path)
    }
}

pub fn parse_path(input: &str) -> Result<(DataRoot, RepoId, Oid)> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(.+?)/([\w-]+)/objects/([\w_-]{2})/([\w_-]{2})/([\w_-]+)/object.yaml$")
                .unwrap();
    }

    let cap = RE
        .captures(input)
        .ok_or_else(|| Error::Repo(format!("bad path: {}", input)))?;
    if cap.len() != 6 {
        return Err(Error::Command(format!("bad path: {:?}", cap)));
    }

    let (root, repo_id, part1, part2, part3) =
        match (cap.get(1), cap.get(2), cap.get(3), cap.get(4), cap.get(5)) {
            (Some(root), Some(repo_id), Some(part1), Some(part2), Some(part3)) => (
                root.as_str(),
                RepoId::try_from(repo_id.as_str())?,
                part1.as_str(),
                part2.as_str(),
                part3.as_str(),
            ),
            _ => return Err(Error::Repo(format!("bad path: {}", input))),
        };

    let oid: Oid = format!("{}{}{}", part1, part2, part3).try_into()?;
    let root = PathBuf::from(root);

    Ok((DataRoot::new(root), repo_id, oid))
}

impl DataRoot {
    pub fn new(root: PathBuf) -> Self {
        Self { path: root }
    }

    pub fn repo_path(&self, id: &RepoId) -> PathBuf {
        self.path.join(id.relative_path())
    }
}

pub trait GitPaths {
    fn activity_log_filename(&self) -> Result<PathBuf> {
        Ok(self.relative_path("objects")?.join("changes.yaml"))
    }

    fn change_filename(&self) -> Result<PathBuf> {
        Ok(self.relative_path("changes")?.join("change.yaml"))
    }

    fn relative_path(&self, subdirectory: &str) -> Result<PathBuf> {
        let (part1, part2, part3) = self.parts()?;
        let relative_path = PathBuf::from([subdirectory, part1, part2, part3].join("/"));
        Ok(relative_path)
    }

    fn object_filename(&self) -> Result<PathBuf> {
        Ok(self.relative_path("objects")?.join("object.yaml"))
    }

    fn parts(&self) -> Result<(&str, &str, &str)>;
}

impl GitPaths for Oid {
    fn parts(&self) -> Result<(&str, &str, &str)> {
        self.parts()
    }
}

#[derive(Clone, Debug)]
pub struct Client {
    pub root: DataRoot,
    pub timespec: Timespec,
    pub viewer: Viewer,
}

impl Client {
    pub fn new(viewer: &Viewer, root: &DataRoot, timespec: Timespec) -> Self {
        Self {
            root: root.to_owned(),
            timespec,
            viewer: viewer.to_owned(),
        }
    }

    pub fn appears_in(&self, repo: &RepoId, search: &Search, entry: &SearchEntry) -> Result<bool> {
        for token in &search.tokens {
            let key = repo.index_key(token)?;
            if !key
                .token_index(self, IndexMode::Update)?
                .indexed_on(entry, token)?
            {
                return Ok(false);
            }
        }

        Ok(true)
    }

    // How to handle path visibility?
    fn cycle_exists(&self, repo: &RepoId, descendant_id: &Oid, ancestor_id: &Oid) -> Result<bool> {
        let mut i = 0;

        let descendant_path = self.read_path(repo, descendant_id)?;

        for topic in self.topic_downset(&descendant_path) {
            i += 1;
            if topic.id() == ancestor_id {
                log::info!("cycle found after {} iterations", i);
                return Ok(true);
            }
        }

        log::info!("no cycle found after {} iterations", i);
        Ok(false)
    }

    pub fn downset(&self, topic_path: &ReadPath) -> DownsetIter {
        DownsetIter::new(
            self.clone(),
            topic_path.repo.to_owned(),
            self.fetch_topic(&topic_path.repo, &topic_path.id),
        )
    }

    pub fn exists(&self, repo: &RepoId, id: &Oid) -> Result<bool> {
        if !self.viewer.can_read(repo) {
            return Ok(false);
        }
        let repo = self.view(repo)?;
        repo.object_exists(id)
    }

    pub fn fetch(&self, repo: &RepoId, id: &Oid) -> Option<RepoObject> {
        if !self.viewer.can_read(repo) {
            log::warn!("viewer cannot read path: {}", id);
            return None;
        }

        match self.view(repo) {
            Ok(repo) => match repo.object(id) {
                Ok(object) => object,
                Err(err) => {
                    println!("failed to fetch object: {:?}", err);
                    None
                }
            },
            Err(err) => {
                println!("failed to open repo: {:?}", err);
                None
            }
        }
    }

    pub fn fetch_activity(
        &self,
        repo: &RepoId,
        id: &Oid,
        first: usize,
    ) -> Result<Vec<activity::Change>> {
        log::info!("fetching first {} change logs from Git for {}", first, id);
        let index = self.fetch_activity_log(repo, id, IndexMode::ReadOnly)?;
        let mut changes = vec![];

        for reference in index.references().iter().take(first) {
            let result = self.view(repo)?.change(&reference.id);
            match result {
                Ok(change) => changes.push(change),
                Err(err) => log::warn!("failed to load change: {}", err),
            }
        }

        Ok(changes)
    }

    pub fn fetch_activity_log(
        &self,
        repo: &RepoId,
        id: &Oid,
        index_mode: IndexMode,
    ) -> Result<ActivityIndex> {
        let filename = id.activity_log_filename()?;
        let view = self.view(repo)?;
        match index_mode {
            IndexMode::Replace => Ok(ActivityIndex::new(&filename)),
            IndexMode::ReadOnly => ActivityIndex::load(&filename, &view),
            IndexMode::Update => ActivityIndex::load(&filename, &view),
        }
    }

    pub fn fetch_link(&self, repo_id: &RepoId, link_id: &Oid) -> Option<RepoLink> {
        match &self.fetch(repo_id, link_id)? {
            RepoObject::Link(link) => Some(link.to_owned()),
            other => {
                println!("expected a link, found: {:?}", other);
                None
            }
        }
    }

    pub fn fetch_all(&self, oids: &[Oid]) -> ObjectBuilders {
        let mut objects = ObjectBuilders::new();

        for repo_id in self.viewer.read_repo_ids.iter() {
            for id in oids {
                let object = self.fetch(repo_id, id);
                if object.is_none() {
                    continue;
                }
                objects.add(id.to_owned(), repo_id.to_owned(), object.unwrap());
            }
        }

        objects
    }

    pub fn fetch_synonym_index(&self, prefix: &RepoId, filename: &PathBuf) -> Result<SynonymIndex> {
        let view = self.view(prefix)?;
        let result = view.find_blob_by_filename(filename)?;
        match result {
            Some(blob) => {
                let index = blob.try_into()?;
                Ok(SynonymIndex::make(filename.to_owned(), index))
            }
            None => Ok(SynonymIndex::new(filename)),
        }
    }

    pub fn fetch_token_index(
        &self,
        prefix: &RepoId,
        filename: &PathBuf,
    ) -> Result<SearchTokenIndex> {
        let view = self.view(prefix)?;
        let result = view.find_blob_by_filename(filename)?;
        match result {
            Some(blob) => Ok(SearchTokenIndex::make(
                filename.to_owned(),
                blob.try_into()?,
            )),
            None => Ok(SearchTokenIndex::new(filename)),
        }
    }

    pub fn fetch_topic(&self, repo: &RepoId, topic_id: &Oid) -> Option<RepoTopic> {
        match &self.fetch(repo, topic_id)? {
            RepoObject::Topic(topic) => Some(topic.to_owned()),
            _ => None,
        }
    }

    pub fn leaked_data(&self) -> Result<Vec<(RepoId, String)>> {
        LeakedData.call(self)
    }

    fn link_searches(&self, link: Option<RepoLink>) -> Result<BTreeSet<Search>> {
        let searches = match link {
            Some(link) => {
                if link.is_reference() {
                    return Ok(BTreeSet::new());
                }

                let url = RepoUrl::parse(link.url())?;
                BTreeSet::from([
                    Search::parse(link.title())?,
                    Search::parse(&url.normalized)?,
                ])
            }
            None => BTreeSet::new(),
        };
        Ok(searches)
    }

    pub fn read_path(&self, repo: &RepoId, id: &Oid) -> Result<ReadPath> {
        let commit = self.repo(repo)?.commit_oid(&self.timespec)?;
        Ok(ReadPath {
            commit,
            id: id.to_owned(),
            repo: repo.to_owned(),
        })
    }

    fn repo(&self, prefix: &RepoId) -> Result<core::Repo> {
        core::Repo::ensure(&self.root, prefix)
    }

    // The "prefix" argument tells us which repo to look in.  The "prefix" in the method name
    // alludes to the prefix scan that is done to find matching synonyms.
    pub fn search_token_prefix_matches(
        &self,
        prefix: &RepoId,
        token: &Phrase,
    ) -> Result<HashSet<SearchEntry>> {
        let key = prefix.index_key(token)?;
        let index = key.search_index(self, IndexType::Search, IndexMode::ReadOnly)?;
        Ok(index.prefix_matches(token))
    }

    pub fn view_stats(&self, repo: &RepoId) -> Result<RepoStats> {
        self.view(repo)?.stats()
    }

    pub fn synonym_phrase_matches(
        &self,
        repos: &[&RepoId],
        name: &str,
    ) -> Result<BTreeSet<SynonymMatch>> {
        let phrase = Phrase::parse(name);
        let mut matches = BTreeSet::new();

        for repo in repos {
            let key = repo.index_key(&phrase)?;
            for entry in &key
                .synonym_index(self, IndexType::SynonymPhrase, IndexMode::Update)?
                .full_matches(&phrase)?
            {
                if !self.viewer.can_read(repo) {
                    continue;
                }

                if let Some(topic) = self.fetch_topic(repo, &entry.id) {
                    matches.insert(SynonymMatch {
                        cycle: false,
                        entry: (*entry).clone(),
                        name: name.to_string(),
                        topic,
                    });
                }
            }
        }

        Ok(matches)
    }

    // The "prefix" argument tells us which repo to look in.  The "prefix" in the method name
    // alludes to the prefix scan that is done to find matching synonyms.
    pub fn synonym_token_prefix_matches(
        &self,
        prefix: &RepoId,
        token: &Phrase,
    ) -> BTreeSet<SynonymEntry> {
        match prefix.index_key(token) {
            Ok(key) => {
                match key.synonym_index(self, IndexType::SynonymToken, IndexMode::ReadOnly) {
                    Ok(index) => index.prefix_matches(token),
                    Err(err) => {
                        log::error!("problem fetching index: {}", err);
                        BTreeSet::new()
                    }
                }
            }
            Err(err) => {
                log::error!("problem fetching index key: {}", err);
                BTreeSet::new()
            }
        }
    }

    pub fn topic_downset(&self, topic_path: &ReadPath) -> TopicDownsetIter {
        TopicDownsetIter::new(
            self.clone(),
            topic_path.repo.to_owned(),
            self.fetch_topic(&topic_path.repo, &topic_path.id),
        )
    }

    fn topic_searches(&self, repo: &RepoId, topic: Option<RepoTopic>) -> Result<BTreeSet<Search>> {
        if !self.viewer.can_read(repo) {
            return Err(Error::NotFound(format!("not found: {:?}", topic)));
        }

        let searches = match topic {
            Some(topic) => {
                let mut searches = BTreeSet::new();

                for synonym in topic.synonyms() {
                    let search = Search::parse(&synonym.name)?;
                    if search.is_empty() {
                        continue;
                    }
                    searches.insert(search);
                }

                searches
            }
            None => BTreeSet::new(),
        };

        Ok(searches)
    }

    pub fn mutation(&self, mode: IndexMode) -> Result<Mutation> {
        Ok(Mutation {
            changes: vec![],
            client: self.to_owned(),
            files: BTreeMap::new(),
            indexer: Indexer::new(mode),
        })
    }

    pub fn view(&self, prefix: &RepoId) -> Result<core::View> {
        core::View::ensure(&self.root, prefix, &self.timespec)
    }
}

pub struct Mutation {
    client: Client,
    indexer: Indexer,
    files: BTreeMap<(RepoId, PathBuf), Option<git2::Oid>>,
    changes: Vec<activity::Change>,
}

impl std::fmt::Debug for Mutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreeBuilder")
            .field("client", &self.client)
            .field("indexer", &self.indexer)
            .finish()
    }
}

impl Mutation {
    pub fn activity_log(
        &self,
        repo: &RepoId,
        id: &Oid,
        index_mode: IndexMode,
    ) -> Result<ActivityIndex> {
        self.client.fetch_activity_log(repo, id, index_mode)
    }

    pub fn add_change(&mut self, repo: &RepoId, change: &activity::Change) -> Result<()> {
        self.indexer.add_change(repo, &self.client, change)?;
        self.changes.push(change.to_owned());
        self.save_change(repo, change)?;
        Ok(())
    }

    fn check_can_update(&self, repo: &RepoId) -> Result<()> {
        if !self.client.viewer.can_update(repo) {
            return Err(Error::NotFound(format!("not found: {}", repo)));
        }
        Ok(())
    }

    fn commit_message(&self) -> String {
        "Add change".to_owned()
    }

    pub fn cycle_exists(
        &self,
        repo: &RepoId,
        descendant_id: &Oid,
        ancestor_id: &Oid,
    ) -> Result<bool> {
        self.client.cycle_exists(repo, descendant_id, ancestor_id)
    }

    pub fn delete_repo(&self, repo: &RepoId) -> Result<()> {
        core::Repo::delete(&self.client.root, repo)
    }

    pub fn exists(&self, repo: &RepoId, id: &Oid) -> Result<bool> {
        self.client.exists(repo, id)
    }

    pub fn fetch(&self, repo: &RepoId, id: &Oid) -> Option<RepoObject> {
        self.client.fetch(repo, id)
    }

    pub fn fetch_link(&self, repo: &RepoId, link_id: &Oid) -> Option<RepoLink> {
        self.client.fetch_link(repo, link_id)
    }

    pub fn fetch_topic(&self, repo: &RepoId, topic_id: &Oid) -> Option<RepoTopic> {
        self.client.fetch_topic(repo, topic_id)
    }

    pub fn mark_deleted(&mut self, repo: &RepoId, id: &Oid) -> Result<()> {
        self.check_can_update(repo)?;

        let activity = self.client.fetch_activity(repo, id, usize::MAX)?;

        for mut change in activity {
            change.mark_deleted(id);
            self.save_change(repo, &change)?;
        }

        Ok(())
    }

    pub fn remove(&mut self, repo: &RepoId, id: &Oid) -> Result<()> {
        let filename = id.object_filename()?;
        self.files.insert((repo.to_owned(), filename), None);
        Ok(())
    }

    pub fn remove_link(&mut self, repo: &RepoId, link_id: &Oid, link: &RepoLink) -> Result<()> {
        self.check_can_update(repo)?;

        let searches = self.client.link_searches(Some(link.to_owned()))?;
        self.indexer.remove_searches(
            &self.client,
            repo,
            &link.to_search_entry(),
            searches.iter(),
        )?;
        self.remove(repo, link_id)
    }

    pub fn remove_topic(&mut self, repo: &RepoId, topic_id: &Oid, topic: &RepoTopic) -> Result<()> {
        self.check_can_update(repo)?;

        self.indexer
            .remove_synonyms(&self.client, repo, topic_id, topic)?;

        let meta = &topic.metadata;
        let mut searches = vec![];
        for synonym in meta.synonyms() {
            let search = Search::parse(&synonym.name)?;
            if search.is_empty() {
                continue;
            }
            searches.push(search);
        }

        let entry = topic.to_search_entry();
        self.indexer
            .remove_searches(&self.client, repo, &entry, searches.iter())?;
        self.remove(repo, topic_id)?;

        Ok(())
    }

    pub fn repo(&self, prefix: &RepoId) -> Result<core::Repo> {
        self.client.repo(prefix)
    }

    pub fn write<S>(&self, store: &S) -> Result<()>
    where
        S: SaveChangesForPrefix,
    {
        self.indexer.write_repo_changes(store)?;

        let mut update = core::Update::new();

        // Write topics and links
        for ((repo, filename), oid) in self.files.iter() {
            update.add(repo, filename, oid)?;
        }

        let index_files = self.indexer.files()?;

        // Write activity logs
        for (prefix, filename, ser) in index_files {
            let repo = self.repo(prefix)?;
            let oid = repo.add_blob(ser.as_bytes())?;
            update.add(prefix, &filename, &Some(oid))?;
        }

        let sig = git2::Signature::now("digraph-bot", "digraph-bot@digraph.app")?;
        update.write(&self.client.root, &sig, &self.commit_message())
    }

    pub fn save_change(&mut self, repo: &RepoId, change: &activity::Change) -> Result<()> {
        self.indexer.add_change(repo, &self.client, change)?;

        let s = serde_yaml::to_string(&change)?;
        let oid = self.repo(repo)?.add_blob(s.as_bytes())?;
        let reference = change.to_reference();
        self.files.insert(
            (repo.to_owned(), reference.id.change_filename()?),
            Some(oid),
        );

        Ok(())
    }

    pub fn save_link(&mut self, repo: &RepoId, link: &RepoLink) -> Result<()> {
        self.check_can_update(repo)?;

        let view = self.client.view(repo)?;
        let link_id = link.id();
        let before = view.link(link_id)?;
        let before = self.client.link_searches(before)?;
        let after = self.client.link_searches(Some(link.to_owned()))?;
        self.indexer
            .update(&self.client, repo, &link.to_search_entry(), &before, &after)?;
        let s = serde_yaml::to_string(&link)?;
        let oid = self.client.repo(repo)?.add_blob(s.as_bytes())?;

        self.save_object(repo, link_id, oid)
    }

    fn save_object(&mut self, repo: &RepoId, id: &Oid, oid: git2::Oid) -> Result<()> {
        let filename = id.object_filename()?;
        self.files.insert((repo.to_owned(), filename), Some(oid));
        Ok(())
    }

    pub fn save_topic(&mut self, repo: &RepoId, topic: &RepoTopic) -> Result<()> {
        self.check_can_update(repo)?;

        let topic_id = topic.id();
        let view = self.client.view(repo)?;
        let before = view.topic(topic_id)?;
        self.indexer
            .update_synonyms(&self.client, repo, &before, topic)?;

        let before = self.client.topic_searches(repo, before)?;
        let after = self.client.topic_searches(repo, Some(topic.to_owned()))?;
        self.indexer.update(
            &self.client,
            repo,
            &topic.to_search_entry(),
            &before,
            &after,
        )?;
        let s = serde_yaml::to_string(&topic)?;
        let oid = self.client.repo(repo)?.add_blob(s.as_bytes())?;

        self.save_object(repo, topic_id, oid)
    }

    pub fn synonym_phrase_matches(
        &self,
        prefixes: &[&RepoId],
        name: &str,
    ) -> Result<BTreeSet<SynonymMatch>> {
        self.client.synonym_phrase_matches(prefixes, name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod git_paths {
        use super::*;

        #[test]
        fn object_filename() {
            assert_eq!(
                Oid::try_from("123456").unwrap().object_filename().unwrap(),
                PathBuf::from("objects/12/34/56/object.yaml")
            );
        }

        #[test]
        fn parts() {
            let id = Oid::try_from("q-ZZmeNzLnZvgk_QGVjqPIpSgkADx71iWZrapMTphpQ").unwrap();
            assert_eq!(
                id.parts().unwrap(),
                ("q-", "ZZ", "meNzLnZvgk_QGVjqPIpSgkADx71iWZrapMTphpQ")
            );
        }
    }
}
