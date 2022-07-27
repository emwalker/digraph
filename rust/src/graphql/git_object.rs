use async_graphql::dataloader::*;
use std::collections::HashMap;

use super::timerange;
use super::{Link, Synonym, SynonymInput, SynonymMatch, Synonyms, Topic, TopicChild};
use crate::git;
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

impl TryFrom<&git::Topic> for Topic {
    type Error = Error;

    fn try_from(topic: &git::Topic) -> Result<Self> {
        let meta = &topic.metadata;
        let parent_topic_paths = topic
            .parent_topics
            .iter()
            .map(|parent| RepoPath::from(&parent.path))
            .collect::<Vec<RepoPath>>();

        let child_paths = topic
            .children
            .iter()
            .map(|p| RepoPath::from(&p.path))
            .collect::<Vec<RepoPath>>();
        let synonyms = Synonyms::from(&meta.synonyms);

        let timerange = meta.timerange.as_ref();
        let timerange = match timerange {
            Some(timerange) => Some(timerange::Timerange::try_from(timerange)?),
            None => None,
        };

        Ok(Self {
            child_paths,
            path: RepoPath::from(&meta.path),
            parent_topic_paths,
            name: topic.name(Locale::EN),
            root: meta.root,
            synonyms,
            timerange,
        })
    }
}

#[allow(dead_code)]
pub struct LinkLoader {
    viewer: Viewer,
    git: git::Client,
}

impl LinkLoader {
    pub fn new(viewer: Viewer, git: git::Client) -> Self {
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
            if let Some(link) = &self.git.fetch_link(&RepoPath::from(path)) {
                map.insert(path.to_owned(), link.to_owned());
            }
        }

        Ok(map)
    }
}

#[allow(dead_code)]
pub struct ObjectLoader {
    viewer: Viewer,
    git: git::Client,
}

impl ObjectLoader {
    pub fn new(viewer: Viewer, git: git::Client) -> Self {
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
            if let Some(object) = &self.git.fetch(&RepoPath::from(path)) {
                map.insert(path.to_owned(), object.clone());
            }
        }

        Ok(map)
    }
}

impl TryFrom<&git::Object> for TopicChild {
    type Error = Error;

    fn try_from(object: &git::Object) -> Result<Self> {
        let object = match object {
            git::Object::Link(link) => TopicChild::Link(Link::from(link)),
            git::Object::Topic(topic) => TopicChild::Topic(Topic::try_from(topic)?),
        };
        Ok(object)
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

impl From<&git::Synonym> for Synonym {
    fn from(synonym: &git::Synonym) -> Self {
        Self {
            name: synonym.name.clone(),
            locale: synonym.locale.to_string().to_lowercase(),
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

impl TryFrom<&git::SearchMatch> for TopicChild {
    type Error = Error;

    fn try_from(item: &git::SearchMatch) -> Result<Self> {
        let git::SearchMatch { object, .. } = item;
        let object = match object {
            git::Object::Topic(topic) => TopicChild::Topic(Topic::try_from(topic)?),
            git::Object::Link(link) => TopicChild::Link(Link::from(link)),
        };
        Ok(object)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod synonyms {
        use super::*;

        #[test]
        fn lowercase_serialization() {
            let from = git::Synonym {
                added: chrono::Utc::now(),
                name: "Name".to_owned(),
                locale: Locale::EN,
            };

            let to = Synonym::from(&from);
            assert_eq!(to.locale, "en");
        }
    }
}
