use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkMetadata {
    pub added_timestamp: DateTime<Utc>,
    pub id: String,
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
    pub id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicChild {
    pub added_timestamp: DateTime<Utc>,
    pub id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Synonym {
    pub added_timestamp: DateTime<Utc>,
    pub locale: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicMetadata {
    pub added_timestamp: DateTime<Utc>,
    pub id: String,
    pub synonyms: Vec<Synonym>,
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

#[derive(Clone, Debug, Deserialize)]
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
pub struct RepoPath {
    pub id: Option<String>,
    root: PathBuf,
}

impl std::fmt::Display for RepoPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.root)
    }
}

impl RepoPath {
    pub fn new(root: PathBuf, id: Option<String>) -> Self {
        Self { id, root }
    }

    pub fn parse(input: PathBuf) -> Result<Self> {
        use itertools::Itertools;

        let path = input.into_os_string().into_string().unwrap();
        let re = Regex::new(r"^(.+?/wiki)/objects/(.+)/object.yaml$").unwrap();
        let cap = re.captures(&path).unwrap();
        if cap.len() != 3 {
            return Err(Error::Command(format!("unexpected path: {:?}", cap)));
        }

        let root = cap.get(1).map_or("".into(), |m| m.as_str().to_string());
        let id = cap
            .get(2)
            .map_or("".into(), |m| m.as_str().to_string())
            .split('/')
            .join("");
        let id = format!("/wiki/{}", id);
        let root = PathBuf::from(root);

        Ok(Self::new(root, Some(id)))
    }

    pub fn path(&self, id: &str) -> Result<PathBuf> {
        let id = id
            .split('/')
            .last()
            .ok_or_else(|| Error::Repo("failed to split path".to_string()))?;
        let first = id
            .get(0..2)
            .ok_or_else(|| Error::Repo("expected at least two chars".into()))?;
        let second = id
            .get(2..4)
            .ok_or_else(|| Error::Repo("expected at least four chars".into()))?;
        let rest = id
            .get(4..)
            .ok_or_else(|| Error::Repo("expectd at least six chars".into()))?;
        let path = format!("objects/{}/{}/{}/object.yaml", first, second, rest);

        Ok(self.root.join(path))
    }
}

#[derive(Default)]
pub struct GitRepo {
    cache: HashMap<String, Object>,
    cache_hits: u32,
    entrypoint: RepoPath,
}

impl std::fmt::Debug for GitRepo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "GitRepo({} cached objects, {} cache hits)",
            self.cache.len(),
            self.cache_hits
        )
    }
}

impl GitRepo {
    pub fn new(entrypoint: RepoPath) -> Self {
        Self {
            entrypoint,
            ..Self::default()
        }
    }

    pub fn get(&mut self, id: &String) -> Result<Option<Object>> {
        match self.cache.get(id) {
            Some(object) => {
                self.cache_hits += 1;
                Ok(Some(object.to_owned()))
            }
            None => {
                let filename = self.entrypoint.path(id)?;
                let fh =
                    std::fs::File::open(&filename).map_err(|e| Error::Repo(format!("{:?}", e)))?;
                let object: Object = serde_yaml::from_reader(fh)?;
                self.cache.insert(id.clone(), object.clone());
                Ok(Some(object))
            }
        }
    }
}
