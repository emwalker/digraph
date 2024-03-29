use git2;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::PathBuf;
use std::sync::Arc;

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
use crate::types::{Timespec, TopicPath};

#[derive(Clone, Debug, Default)]
pub struct DataRoot {
    pub path: PathBuf,
}

impl std::fmt::Display for DataRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.path)
    }
}

pub fn parse_path(input: &str) -> Result<(DataRoot, RepoId, ExternalId)> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(.+?)/([\w-]+)/objects/([\w_-]{2})/([\w_-]{2})/([\w_-]+)/object.yaml$")
                .unwrap();
    }

    let cap = RE
        .captures(input)
        .ok_or_else(|| Error::Repo(format!("bad path: {input}")))?;
    if cap.len() != 6 {
        return Err(Error::Command(format!("bad path: {cap:?}")));
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
            _ => return Err(Error::Repo(format!("bad path: {input}"))),
        };

    let oid: ExternalId = format!("{part1}{part2}{part3}").try_into()?;
    let root = PathBuf::from(root);

    Ok((DataRoot::new(root), repo_id, oid))
}

impl DataRoot {
    pub fn new(root: PathBuf) -> Self {
        Self { path: root }
    }

    pub fn repo_path(&self, repo_id: RepoId) -> PathBuf {
        self.path.join(repo_id.relative_path())
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

impl GitPaths for ExternalId {
    fn parts(&self) -> Result<(&str, &str, &str)> {
        self.parts()
    }
}

#[derive(Clone, Debug)]
pub struct Client {
    pub root: DataRoot,
    pub timespec: Timespec,
    pub viewer: Arc<Viewer>,
}

impl Client {
    pub fn new(viewer: Arc<Viewer>, root: &DataRoot, timespec: Timespec) -> Self {
        Self {
            root: root.to_owned(),
            timespec,
            viewer,
        }
    }

    pub fn appears_in(
        &self,
        repo_id: RepoId,
        search: &Search,
        entry: &SearchEntry,
    ) -> Result<bool> {
        for token in &search.tokens {
            let key = repo_id.index_key(token)?;
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
    fn cycle_exists(
        &self,
        repo_id: RepoId,
        descendant_id: &ExternalId,
        ancestor_id: &ExternalId,
    ) -> Result<bool> {
        let mut i = 0;

        if let Some(descendant_path) = self.topic_path(repo_id, descendant_id)? {
            for topic in self.topic_downset(&descendant_path) {
                i += 1;
                if topic.topic_id() == ancestor_id {
                    log::info!("cycle found after {} iterations", i);
                    return Ok(true);
                }
            }
        }

        log::info!("no cycle found after {} iterations", i);
        Ok(false)
    }

    pub fn downset(&self, topic_path: &TopicPath) -> DownsetIter {
        DownsetIter::new(
            self,
            topic_path.repo_id,
            self.fetch_topic(topic_path.repo_id, &topic_path.topic_id),
        )
    }

    pub fn exists(&self, repo_id: RepoId, id: &ExternalId) -> Result<bool> {
        if !self.viewer.can_read(repo_id) {
            return Ok(false);
        }
        let repo = self.view(repo_id)?;
        repo.object_exists(id)
    }

    pub fn fetch(&self, repo_id: RepoId, id: &ExternalId) -> Option<RepoObject> {
        if !self.viewer.can_read(repo_id) {
            log::warn!("viewer cannot read path: {}", id);
            return None;
        }

        match self.view(repo_id) {
            Ok(repo) => match repo.object(id) {
                Ok(object) => object,
                Err(err) => {
                    println!("failed to fetch object: {err:?}");
                    None
                }
            },
            Err(err) => {
                println!("failed to open repo: {err:?}");
                None
            }
        }
    }

    pub fn fetch_activity(
        &self,
        repo_id: RepoId,
        id: &ExternalId,
        first: usize,
    ) -> Result<Vec<activity::Change>> {
        log::info!("fetching first {} change logs from Git for {}", first, id);
        let index = self.fetch_activity_log(repo_id, id, IndexMode::ReadOnly)?;
        let mut changes = vec![];

        for reference in index.references().iter().take(first) {
            let result = self.view(repo_id)?.change(&reference.id);
            match result {
                Ok(change) => changes.push(change),
                Err(err) => log::warn!("failed to load change: {}", err),
            }
        }

        Ok(changes)
    }

    pub fn fetch_activity_log(
        &self,
        repo_id: RepoId,
        id: &ExternalId,
        index_mode: IndexMode,
    ) -> Result<ActivityIndex> {
        let filename = id.activity_log_filename()?;
        let view = self.view(repo_id)?;
        match index_mode {
            IndexMode::Replace => Ok(ActivityIndex::new(&filename)),
            IndexMode::ReadOnly => ActivityIndex::load(&filename, &view),
            IndexMode::Update => ActivityIndex::load(&filename, &view),
        }
    }

    pub fn fetch_link(&self, repo_id: RepoId, link_id: &ExternalId) -> Option<RepoLink> {
        match &self.fetch(repo_id, link_id)? {
            RepoObject::Link(link) => Some(link.to_owned()),
            other => {
                println!("expected a link, found: {other:?}");
                None
            }
        }
    }

    pub fn fetch_all(&self, keys: &[Okey]) -> ObjectBuilders {
        let mut objects = ObjectBuilders::new();

        for &repo_id in self.viewer.read_repo_ids.iter() {
            for key in keys {
                let object = self.fetch(repo_id, &key.0);
                if object.is_none() {
                    continue;
                }
                objects.add(key.to_owned(), repo_id, object.unwrap());
            }
        }

        objects
    }

    pub fn fetch_synonym_index(&self, repo_id: RepoId, filename: &PathBuf) -> Result<SynonymIndex> {
        let view = self.view(repo_id)?;
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
        repo_id: RepoId,
        filename: &PathBuf,
    ) -> Result<SearchTokenIndex> {
        let view = self.view(repo_id)?;
        let result = view.find_blob_by_filename(filename)?;
        match result {
            Some(blob) => Ok(SearchTokenIndex::make(
                filename.to_owned(),
                blob.try_into()?,
            )),
            None => Ok(SearchTokenIndex::new(filename)),
        }
    }

    pub fn fetch_topic(&self, repo_id: RepoId, topic_id: &ExternalId) -> Option<RepoTopic> {
        match &self.fetch(repo_id, topic_id)? {
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

    pub fn topic_path(&self, repo_id: RepoId, topic_id: &ExternalId) -> Result<Option<TopicPath>> {
        let topic_oid = self.repo(repo_id)?.topic_oid(&self.timespec, topic_id)?;

        if let Some(commit) = topic_oid {
            return Ok(Some(TopicPath {
                topic_oid: commit,
                topic_id: topic_id.to_owned(),
                repo_id,
            }));
        }

        Ok(None)
    }

    fn repo(&self, repo_id: RepoId) -> Result<core::Repo> {
        core::Repo::ensure(&self.root, repo_id)
    }

    // The "prefix" argument tells us which repo to look in.  The "prefix" in the method name
    // alludes to the prefix scan that is done to find matching synonyms.
    pub fn search_token_prefix_matches(
        &self,
        repo_id: RepoId,
        token: &Phrase,
    ) -> Result<HashSet<SearchEntry>> {
        let key = repo_id.index_key(token)?;
        let index = key.search_index(self, IndexType::Search, IndexMode::ReadOnly)?;
        Ok(index.prefix_matches(token))
    }

    pub fn view_stats(&self, repo_id: RepoId) -> Result<RepoStats> {
        self.view(repo_id)?.stats()
    }

    pub fn synonym_phrase_matches(
        &self,
        repo_ids: &RepoIds,
        name: &str,
    ) -> Result<BTreeSet<SynonymMatch>> {
        let phrase = Phrase::parse(name);
        let mut matches = BTreeSet::new();

        for &repo_id in repo_ids.iter() {
            let key = repo_id.index_key(&phrase)?;
            for entry in &key
                .synonym_index(self, IndexType::SynonymPhrase, IndexMode::Update)?
                .full_matches(&phrase)?
            {
                if !self.viewer.can_read(repo_id) {
                    continue;
                }

                if let Some(topic) = self.fetch_topic(repo_id, &entry.id) {
                    matches.insert(SynonymMatch {
                        cycle: false,
                        entry: (*entry).clone(),
                        name: name.to_string(),
                        repo_id,
                        repo_topic: topic,
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
        repo_id: RepoId,
        token: &Phrase,
    ) -> BTreeSet<SynonymEntry> {
        match repo_id.index_key(token) {
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

    pub fn topic_downset(&self, topic_path: &TopicPath) -> TopicDownsetIter {
        TopicDownsetIter::new(
            self,
            topic_path.repo_id,
            self.fetch_topic(topic_path.repo_id, &topic_path.topic_id),
        )
    }

    fn topic_searches(
        &self,
        repo_id: RepoId,
        topic: Option<RepoTopic>,
    ) -> Result<BTreeSet<Search>> {
        if !self.viewer.can_read(repo_id) {
            return Err(Error::NotFound(format!("not found: {topic:?}")));
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

    pub fn view(&self, repo_id: RepoId) -> Result<core::View> {
        core::View::ensure(&self.root, repo_id, &self.timespec)
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
        repo_id: RepoId,
        id: &ExternalId,
        index_mode: IndexMode,
    ) -> Result<ActivityIndex> {
        self.client.fetch_activity_log(repo_id, id, index_mode)
    }

    pub fn add_change(&mut self, repo_id: RepoId, change: &activity::Change) -> Result<()> {
        self.indexer.add_change(repo_id, &self.client, change)?;
        self.changes.push(change.to_owned());
        self.save_change(repo_id, change)?;
        Ok(())
    }

    fn check_can_update(&self, repo_id: RepoId) -> Result<()> {
        if !self.client.viewer.can_update(repo_id) {
            return Err(Error::NotFound(format!("not found: {repo_id}")));
        }
        Ok(())
    }

    fn commit_message(&self) -> String {
        "Add change".to_owned()
    }

    pub fn cycle_exists(
        &self,
        repo_id: RepoId,
        descendant_id: &ExternalId,
        ancestor_id: &ExternalId,
    ) -> Result<bool> {
        self.client
            .cycle_exists(repo_id, descendant_id, ancestor_id)
    }

    pub fn delete_repo(&self, repo_id: RepoId) -> Result<()> {
        core::Repo::delete(&self.client.root, repo_id)
    }

    pub fn exists(&self, repo_id: RepoId, id: &ExternalId) -> Result<bool> {
        self.client.exists(repo_id, id)
    }

    pub fn fetch(&self, repo_id: RepoId, id: &ExternalId) -> Option<RepoObject> {
        self.client.fetch(repo_id, id)
    }

    pub fn fetch_link(&self, repo_id: RepoId, link_id: &ExternalId) -> Option<RepoLink> {
        self.client.fetch_link(repo_id, link_id)
    }

    pub fn fetch_topic(&self, repo_id: RepoId, topic_id: &ExternalId) -> Option<RepoTopic> {
        self.client.fetch_topic(repo_id, topic_id)
    }

    pub fn mark_deleted(&mut self, repo_id: RepoId, id: &ExternalId) -> Result<()> {
        self.check_can_update(repo_id)?;

        let activity = self.client.fetch_activity(repo_id, id, usize::MAX)?;

        for mut change in activity {
            change.mark_deleted(id);
            self.save_change(repo_id, &change)?;
        }

        Ok(())
    }

    pub fn remove(&mut self, repo_id: RepoId, id: &ExternalId) -> Result<()> {
        let filename = id.object_filename()?;
        self.files.insert((repo_id, filename), None);
        Ok(())
    }

    pub fn remove_link(
        &mut self,
        repo_id: RepoId,
        link_id: &ExternalId,
        link: &RepoLink,
    ) -> Result<()> {
        self.check_can_update(repo_id)?;

        let searches = self.client.link_searches(Some(link.to_owned()))?;
        self.indexer.remove_searches(
            &self.client,
            repo_id,
            &link.to_search_entry(),
            searches.iter(),
        )?;
        self.remove(repo_id, link_id)
    }

    pub fn remove_topic(
        &mut self,
        repo_id: RepoId,
        topic_id: &ExternalId,
        topic: &RepoTopic,
    ) -> Result<()> {
        self.check_can_update(repo_id)?;

        self.indexer
            .remove_synonyms(&self.client, repo_id, topic_id, topic)?;

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
            .remove_searches(&self.client, repo_id, &entry, searches.iter())?;
        self.remove(repo_id, topic_id)?;

        Ok(())
    }

    pub fn repo(&self, repo_id: RepoId) -> Result<core::Repo> {
        self.client.repo(repo_id)
    }

    pub fn write<S>(&self, store: &S) -> Result<()>
    where
        S: SaveChangesForPrefix,
    {
        self.indexer.write_repo_changes(store)?;

        let mut update = core::Update::new();

        // Write topics and links
        for ((repo_id, filename), oid) in self.files.iter() {
            update.add(*repo_id, filename, oid)?;
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

    pub fn save_change(&mut self, repo_id: RepoId, change: &activity::Change) -> Result<()> {
        self.indexer.add_change(repo_id, &self.client, change)?;

        let s = serde_yaml::to_string(&change)?;
        let oid = self.repo(repo_id)?.add_blob(s.as_bytes())?;
        let reference = change.to_reference();
        self.files
            .insert((repo_id, reference.id.change_filename()?), Some(oid));

        Ok(())
    }

    pub fn save_link(&mut self, repo_id: RepoId, link: &RepoLink) -> Result<()> {
        self.check_can_update(repo_id)?;

        let view = self.client.view(repo_id)?;
        let link_id = link.id();
        let before = view.link(link_id)?;
        let before = self.client.link_searches(before)?;
        let after = self.client.link_searches(Some(link.to_owned()))?;
        self.indexer.update(
            &self.client,
            repo_id,
            &link.to_search_entry(),
            &before,
            &after,
        )?;
        let s = serde_yaml::to_string(&link)?;
        let oid = self.client.repo(repo_id)?.add_blob(s.as_bytes())?;

        self.save_object(repo_id, link_id, oid)
    }

    fn save_object(&mut self, repo_id: RepoId, id: &ExternalId, oid: git2::Oid) -> Result<()> {
        let filename = id.object_filename()?;
        self.files.insert((repo_id, filename), Some(oid));
        Ok(())
    }

    pub fn save_topic(&mut self, repo_id: RepoId, topic: &RepoTopic) -> Result<()> {
        self.check_can_update(repo_id)?;

        let topic_id = topic.topic_id();
        let view = self.client.view(repo_id)?;
        let before = view.topic(topic_id)?;
        self.indexer
            .update_synonyms(&self.client, repo_id, &before, topic)?;

        let before = self.client.topic_searches(repo_id, before)?;
        let after = self
            .client
            .topic_searches(repo_id, Some(topic.to_owned()))?;
        self.indexer.update(
            &self.client,
            repo_id,
            &topic.to_search_entry(),
            &before,
            &after,
        )?;
        let s = serde_yaml::to_string(&topic)?;
        let oid = self.client.repo(repo_id)?.add_blob(s.as_bytes())?;

        self.save_object(repo_id, topic_id, oid)
    }

    pub fn synonym_phrase_matches(
        &self,
        repo_ids: &RepoIds,
        name: &str,
    ) -> Result<BTreeSet<SynonymMatch>> {
        self.client.synonym_phrase_matches(repo_ids, name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_path_works() {
        let result = parse_path("../../12345/12/34/5678/object.yaml");
        assert!(matches!(result, Err(Error::Repo(_))));

        let (root, repo_id, oid) =
            parse_path("../../32212616-fc1b-11e8-8eda-b70af6d8d09f/objects/12/34/5678/object.yaml")
                .unwrap();
        assert_eq!(root.path, PathBuf::from("../.."));
        assert_eq!(
            repo_id.to_string(),
            "32212616-fc1b-11e8-8eda-b70af6d8d09f".to_owned()
        );
        assert_eq!(oid.to_string(), "12345678".to_owned());

        let (root, repo_id, oid) = parse_path(
            "../../32212616-fc1b-11e8-8eda-b70af6d8d09f/objects/q-/ZZ/meNzLnZvgk_QGVjqPIpSgkADx71iWZrapMTphpQ/object.yaml",
        )
        .unwrap();
        assert_eq!(root.path, PathBuf::from("../.."));
        assert_eq!(
            repo_id.to_string(),
            "32212616-fc1b-11e8-8eda-b70af6d8d09f".to_owned()
        );
        assert_eq!(
            oid.to_string(),
            "q-ZZmeNzLnZvgk_QGVjqPIpSgkADx71iWZrapMTphpQ".to_owned(),
        );
    }

    mod git_paths {
        use super::*;

        #[test]
        fn object_filename() {
            assert_eq!(
                ExternalId::try_from("123456")
                    .unwrap()
                    .object_filename()
                    .unwrap(),
                PathBuf::from("objects/12/34/56/object.yaml")
            );
        }

        #[test]
        fn parts() {
            let id = ExternalId::try_from("q-ZZmeNzLnZvgk_QGVjqPIpSgkADx71iWZrapMTphpQ").unwrap();
            assert_eq!(
                id.parts().unwrap(),
                ("q-", "ZZ", "meNzLnZvgk_QGVjqPIpSgkADx71iWZrapMTphpQ")
            );
        }
    }
}
