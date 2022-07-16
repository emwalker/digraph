use async_graphql::dataloader::*;
use std::collections::HashMap;

use super::{Link, Synonym, SynonymInput, SynonymMatch, Synonyms, Topic, TopicChild};
use super::timerange;
use crate::git;
use crate::types;
use crate::prelude::*;

impl From<&git::Link> for Link {
    fn from(link: &git::Link) -> Self {
        let meta = &link.metadata;
        let parent_topic_paths = link
            .parent_topics
            .iter()
            .map(|topic| RepoPath::from(&topic.path))
            .collect::<Vec<RepoPath>>();

        Self {
            path: RepoPath::from(&meta.path),
            newly_added: false,
            parent_topic_paths,
            repository_id: WIKI_REPOSITORY_ID.into(),
            viewer_review: None,
            title: meta.title.clone(),
            url: meta.url.clone(),
        }
    }
}

impl From<&git::Synonym> for Synonym {
    fn from(synonym: &git::Synonym) -> Self {
        Self {
            name: synonym.name.clone(),
            locale: synonym.locale.to_string(),
        }
    }
}

impl From<&Vec<git::Synonym>> for Synonyms {
    fn from(synonyms: &Vec<git::Synonym>) -> Self {
        Self(synonyms.iter().map(Synonym::from).collect())
    }
}

impl From<&SynonymInput> for git::Synonym {
    fn from(synonym: &SynonymInput) -> Self {
        use std::str::FromStr;

        Self {
            added: chrono::Utc::now(),
            name: synonym.name.clone(),
            locale: Locale::from_str(&synonym.locale).unwrap_or(Locale::EN),
        }
    }
}

impl From<&git::Topic> for Topic {
    fn from(topic: &git::Topic) -> Self {
        let meta = &topic.metadata;
        let parent_topic_paths = topic
            .parent_topics
            .iter()
            .map(|p| RepoPath::from(&p.path))
            .collect::<Vec<RepoPath>>();

        let child_paths = topic
            .children
            .iter()
            .map(|p| RepoPath::from(&p.path))
            .collect::<Vec<RepoPath>>();

        let synonyms = Synonyms::from(&meta.synonyms);
        let prefix = types::Prefix::from(&meta.timerange);

        Self {
            child_paths,
            path: RepoPath::from(&meta.path),
            parent_topic_paths,
            name: topic.name(Locale::EN),
            prefix,
            root: meta.root,
            synonyms,
            timerange: meta.timerange.as_ref().map(timerange::Timerange::from),
        }
    }
}

#[allow(dead_code)]
pub struct LinkLoader {
    viewer: Viewer,
    git: git::Git,
}

impl LinkLoader {
    pub fn new(viewer: Viewer, git: git::Git) -> Self {
        Self { viewer, git }
    }
}

#[async_trait::async_trait]
impl Loader<String> for LinkLoader {
    type Value = git::Link;
    type Error = Error;

    async fn load(&self, paths: &[String]) -> Result<HashMap<String, Self::Value>> {
        log::debug!("batch links: {:?}", paths);
        let mut map: HashMap<_, _> = HashMap::new();

        for path in paths {
            let link = &self.git.fetch_link(path)?;
            map.insert(path.to_owned(), link.to_owned());
        }

        Ok(map)
    }
}

#[allow(dead_code)]
pub struct ObjectLoader {
    viewer: Viewer,
    git: git::Git,
}

impl ObjectLoader {
    pub fn new(viewer: Viewer, git: git::Git) -> Self {
        Self { viewer, git }
    }
}

#[async_trait::async_trait]
impl Loader<String> for ObjectLoader {
    type Value = git::Object;
    type Error = Error;

    async fn load(&self, paths: &[String]) -> Result<HashMap<String, Self::Value>> {
        log::debug!("batch load topics: {:?}", paths);
        let mut map: HashMap<_, _> = HashMap::new();

        for path in paths {
            let object = &self.git.fetch(path)?;
            map.insert(path.to_owned(), object.clone());
        }

        Ok(map)
    }
}

impl From<&git::SynonymEntry> for SynonymMatch {
    fn from(git::SynonymEntry { name, path }: &git::SynonymEntry) -> Self {
        Self {
            display_name: name.to_owned(),
            path: path.to_owned(),
        }
    }
}

impl From<&git::Object> for TopicChild {
    fn from(object: &git::Object) -> Self {
        match object {
            git::Object::Link(link) => TopicChild::Link(Link::from(link)),
            git::Object::Topic(topic) => TopicChild::Topic(Topic::from(topic)),
        }
    }
}
