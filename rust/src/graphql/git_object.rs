use async_graphql::dataloader::*;
use std::collections::HashMap;

use super::{timerange, ViewStats};
use super::{Link, Synonym, SynonymInput, SynonymMatch, Synonyms, Topic, TopicChild};
use crate::git;
use crate::prelude::*;

impl From<&[git::Synonym]> for Synonyms {
    fn from(slice: &[git::Synonym]) -> Self {
        let vec = slice.iter().map(Synonym::from).collect::<Vec<Synonym>>();
        Self(vec)
    }
}

impl TryFrom<&git::Link> for Link {
    type Error = Error;

    fn try_from(link: &git::Link) -> Result<Self> {
        let parent_topic_ids = link
            .parent_topics
            .iter()
            .map(|topic| topic.id.to_string())
            .collect::<Vec<String>>();

        Ok(Self {
            id: link.id().to_string(),
            newly_added: false,
            parent_topic_ids,
            viewer_review: None,
            title: link.title().to_owned(),
            url: link.url().to_owned(),
        })
    }
}

impl TryFrom<&git::Topic> for Topic {
    type Error = Error;

    fn try_from(topic: &git::Topic) -> Result<Self> {
        let parent_topic_ids = topic
            .parent_topics
            .iter()
            .map(|parent| parent.id.to_string())
            .collect::<Vec<String>>();

        let child_ids = topic
            .children
            .iter()
            .map(|child| child.id.clone())
            .collect::<Vec<RepoId>>();
        let synonyms = Synonyms::from(topic.synonyms());

        let timerange = topic.timerange().as_ref();
        let timerange = match timerange {
            Some(timerange) => Some(timerange::Timerange::try_from(timerange)?),
            None => None,
        };

        Ok(Self {
            child_ids,
            id: topic.id().to_string(),
            parent_topic_ids,
            name: topic.name(Locale::EN),
            root: topic.root(),
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
impl Loader<(RepoName, RepoId)> for LinkLoader {
    type Value = git::Link;
    type Error = Error;

    async fn load(
        &self,
        paths: &[(RepoName, RepoId)],
    ) -> Result<HashMap<(RepoName, RepoId), Self::Value>> {
        log::debug!("batch links: {:?}", paths);
        let mut map: HashMap<_, _> = HashMap::new();

        for (repo, link_id) in paths {
            if let Some(link) = &self.git.fetch_link(repo, link_id) {
                map.insert((repo.to_owned(), link_id.to_owned()), link.to_owned());
            }
        }

        Ok(map)
    }
}

#[allow(dead_code)]
pub struct ObjectLoader {
    client: git::Client,
}

impl ObjectLoader {
    pub fn new(client: git::Client) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl Loader<(RepoName, RepoId)> for ObjectLoader {
    type Value = git::Object;
    type Error = Error;

    async fn load(
        &self,
        paths: &[(RepoName, RepoId)],
    ) -> Result<HashMap<(RepoName, RepoId), Self::Value>> {
        log::debug!("batch load topics: {:?}", paths);
        let mut map: HashMap<_, _> = HashMap::new();

        for (repo, id) in paths {
            if let Some(object) = &self.client.fetch(repo, id) {
                map.insert((repo.to_owned(), id.to_owned()), object.clone());
            }
        }

        Ok(map)
    }
}

impl TryFrom<&git::Object> for TopicChild {
    type Error = Error;

    fn try_from(object: &git::Object) -> Result<Self> {
        let object = match object {
            git::Object::Link(link) => TopicChild::Link(Link::try_from(link)?),
            git::Object::Topic(topic) => TopicChild::Topic(Topic::try_from(topic)?),
        };
        Ok(object)
    }
}

impl From<&git::SynonymEntry> for SynonymMatch {
    fn from(git::SynonymEntry { name, id }: &git::SynonymEntry) -> Self {
        Self {
            display_name: name.to_owned(),
            id: id.to_string(),
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
            git::Object::Link(link) => TopicChild::Link(Link::try_from(link)?),
        };
        Ok(object)
    }
}

impl From<git::Stats> for ViewStats {
    fn from(stats: git::Stats) -> Self {
        let link_count = match stats.link_count().try_into() {
            Ok(count) => count,

            Err(err) => {
                log::error!("failed to convert link count: {}", err);
                0
            }
        };

        let topic_count = match stats.topic_count().try_into() {
            Ok(count) => count,

            Err(err) => {
                log::error!("failed to convert topic count: {}", err);
                0
            }
        };

        Self {
            link_count: Some(link_count),
            topic_count: Some(topic_count),
        }
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
