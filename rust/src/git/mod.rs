use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

mod index;
mod link;
mod loader;
mod repository;
mod topic;

use crate::http::repo_url;
use crate::prelude::*;
pub use index::*;
pub use link::*;
pub use loader::*;
pub use repository::*;
pub use topic::*;

pub const API_VERSION: &str = "objects/v1";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkMetadata {
    pub added: DateTime<Utc>,
    pub path: String,
    pub title: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub api_version: String,
    pub kind: String,
    pub metadata: LinkMetadata,
    pub parent_topics: Vec<ParentTopic>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ParentTopic {
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicChild {
    pub added: DateTime<Utc>,
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Synonym {
    pub added: DateTime<Utc>,
    pub locale: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Timerange {
    pub starts: DateTime<Utc>,
    pub prefix_format: TimerangePrefixFormat,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TimerangePrefixFormat {
    None,
    StartYear,
    StartYearMonth,
}

impl From<&str> for TimerangePrefixFormat {
    fn from(format: &str) -> Self {
        match format {
            "NONE" => Self::None,
            "START_YEAR" => Self::StartYear,
            "START_YEAR_MONTH" => Self::StartYearMonth,
            _ => Self::None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicMetadata {
    pub added: DateTime<Utc>,
    pub path: String,
    pub root: bool,
    pub synonyms: Vec<Synonym>,
    pub timerange: Option<Timerange>,
}

impl TopicMetadata {
    pub fn name(&self) -> String {
        for synonym in &self.synonyms {
            if synonym.locale == "en" {
                return synonym.name.clone();
            }
        }
        "Missing name".into()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Topic {
    pub api_version: String,
    pub kind: String,
    pub metadata: TopicMetadata,
    pub parent_topics: Vec<ParentTopic>,
    pub children: Vec<TopicChild>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Object {
    Topic(Topic),
    Link(Link),
}

impl Object {
    pub fn accept<V>(&self, mut visitor: V) -> Result<()>
    where
        V: Visitor,
    {
        match self {
            Self::Topic(topic) => {
                visitor.visit_topic(topic)?;
            }
            Self::Link(link) => {
                visitor.visit_link(link)?;
            }
        }

        Ok(())
    }
}

pub trait Visitor {
    fn visit_topic(&mut self, topic: &Topic) -> Result<()>;
    fn visit_link(&mut self, link: &Link) -> Result<()>;
}

#[derive(Clone, Debug, Default)]
pub struct DataRoot {
    root: PathBuf,
}

impl std::fmt::Display for DataRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.root)
    }
}

pub fn parse_path(input: &str) -> Result<(DataRoot, String)> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(.+?)/(\w+)/objects/(\w{2})/(\w{2})/([\w-]+)/object.yaml$").unwrap();
    }

    let cap = RE
        .captures(input)
        .ok_or_else(|| Error::Repo(format!("bad path: {}", input)))?;
    if cap.len() != 6 {
        return Err(Error::Command(format!("bad path: {:?}", cap)));
    }

    let root = cap
        .get(1)
        .ok_or_else(|| Error::Repo(format!("bad path: {}", input)))?
        .as_str();
    let org_login = cap
        .get(2)
        .ok_or_else(|| Error::Repo(format!("bad path: {}", input)))?
        .as_str();
    let part1 = cap
        .get(3)
        .ok_or_else(|| Error::Repo(format!("bad path: {}", input)))?
        .as_str();
    let part2 = cap
        .get(4)
        .ok_or_else(|| Error::Repo(format!("bad path: {}", input)))?
        .as_str();
    let part3 = cap
        .get(5)
        .ok_or_else(|| Error::Repo(format!("bad path: {}", input)))?
        .as_str();

    let id = format!("/{}/{}{}{}", org_login, part1, part2, part3);
    let root = PathBuf::from(root);

    Ok((DataRoot::new(root), id))
}

impl DataRoot {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn index_filename(&self, key: &IndexKey) -> Result<PathBuf> {
        let file_path = format!("{}/indexes/tokens/{}.yaml", key.prefix, key.field);
        Ok(self.root.join(file_path))
    }

    pub fn object_filename(&self, path: &str) -> Result<PathBuf> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\w{2})(\w{2})([\w-]+)$").unwrap();
        }

        let path = RepoPath::from(path);
        if !path.valid {
            return Err(Error::Repo(format!("invalid path: {:?}", path)));
        }

        let cap = RE
            .captures(&path.short_id)
            .ok_or_else(|| Error::Repo(format!("bad id: {}", path)))?;

        if cap.len() != 4 {
            return Err(Error::Repo(format!("bad id: {}", path)));
        }

        let part1 = cap
            .get(1)
            .ok_or_else(|| Error::Repo(format!("bad id: {}", path)))?
            .as_str();
        let part2 = cap
            .get(2)
            .ok_or_else(|| Error::Repo(format!("bad id: {}", path)))?
            .as_str();
        let part3 = cap
            .get(3)
            .ok_or_else(|| Error::Repo(format!("bad id: {}", path)))?
            .as_str();

        let file_path = format!(
            "{}/objects/{}/{}/{}/object.yaml",
            path.org_login, part1, part2, part3
        );

        Ok(self.root.join(file_path))
    }
}

#[derive(Clone, Debug, Default)]
pub struct Git {
    root: DataRoot,
}

impl Git {
    pub fn new(root: DataRoot) -> Self {
        Self { root }
    }

    pub fn exists(&self, path: &RepoPath) -> Result<bool> {
        let filename = self.root.object_filename(&path.inner)?;
        Ok(Path::new(&filename).exists())
    }

    pub fn get(&self, path: &str) -> Result<Object> {
        let path = self.root.object_filename(path)?;
        let fh = std::fs::File::open(&path).map_err(|e| Error::Repo(format!("{:?}", e)))?;
        let object: Object = serde_yaml::from_reader(fh)?;
        Ok(object)
    }

    pub fn index_key(&self, path: &RepoPath, token: &str) -> Result<IndexKey> {
        match token.get(0..2) {
            Some(field) => Ok(IndexKey {
                prefix: path.prefix.to_owned(),
                field: field.to_owned(),
            }),
            None => Err(Error::Repo(format!("bad token: {}", token))),
        }
    }

    pub fn index_for(&self, key: &IndexKey) -> Result<PathIndex> {
        let filename = self.root.index_filename(key)?;
        PathIndex::load(&filename)
    }

    pub fn indexed_on(&self, path: &RepoPath, phrase: &Phrase) -> Result<bool> {
        for token in &phrase.tokens {
            let key = self.index_key(path, token)?;
            if !self.index_for(&key)?.indexed_on(path, token)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn save<T: Serialize>(&self, path: &RepoPath, object: &T) -> Result<()> {
        let filename = self.root.object_filename(&path.inner)?;
        let dest = filename.parent().expect("expected a parent directory");
        fs::create_dir_all(&dest).ok();
        let s = serde_yaml::to_string(&object)?;
        log::debug!("saving {:?}", filename);
        fs::write(&filename, s)?;

        Ok(())
    }

    pub fn save_link(&self, path: &RepoPath, link: &Link, indexer: &mut Indexer) -> Result<()> {
        let meta = &link.metadata;
        let url = repo_url::Url::parse(&meta.url)?;
        let phrases = &[
            Phrase::approximate(&meta.title),
            Phrase::lowercase(&url.normalized),
        ];
        indexer.index(path, phrases)?;
        self.save(path, link)
    }

    pub fn save_topic(&self, path: &RepoPath, topic: &Topic, _indexer: &mut Indexer) -> Result<()> {
        // TODO: Indexing
        self.save(path, topic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_path_works() {
        let result = parse_path("../../wiki/12/34/5678/object.yaml");
        assert!(matches!(result, Err(Error::Repo(_))));

        let (root, id) = parse_path("../../wiki/objects/12/34/5678/object.yaml").unwrap();
        assert_eq!(root.root, PathBuf::from("../.."));
        assert_eq!(id, String::from("/wiki/12345678"));
    }

    #[test]
    fn data_root_file_path() {
        let root = DataRoot::new(PathBuf::from("../.."));

        assert!(matches!(root.object_filename("1234"), Err(Error::Repo(_))));
        assert!(matches!(
            root.object_filename("wiki/123456"),
            Err(Error::Repo(_))
        ));
        assert!(matches!(
            root.object_filename("/wiki/1234"),
            Err(Error::Repo(_))
        ));

        assert_eq!(
            root.object_filename("/wiki/123456").unwrap(),
            PathBuf::from("../../wiki/objects/12/34/56/object.yaml")
        );

        assert_eq!(
            root.object_filename("/with-dash/123456").unwrap(),
            PathBuf::from("../../with-dash/objects/12/34/56/object.yaml")
        );
    }
}
