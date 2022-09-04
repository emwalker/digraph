use async_graphql::dataloader::*;
use std::collections::{HashMap};

use super::{
    timerange, Link, LinkDetail, Synonym, SynonymInput, SynonymMatch, Synonyms, Topic, TopicChild,
    ViewStats,
};
use crate::git;
use crate::prelude::*;

impl From<&[git::Synonym]> for Synonyms {
    fn from(slice: &[git::Synonym]) -> Self {
        let vec = slice.iter().map(Synonym::from).collect::<Vec<Synonym>>();
        Self(vec)
    }
}

impl TryFrom<&git::RepoLink> for Link {
    type Error = Error;

    fn try_from(link: &git::RepoLink) -> Result<Self> {
        let parent_topic_ids = link
            .parent_topics
            .iter()
            .map(|topic| topic.id.to_string())
            .collect::<Vec<String>>();

        let detail = LinkDetail {
            color: "".to_owned(),
            link_id: link.id().to_owned(),
            parent_topic_ids: parent_topic_ids.to_owned(),
            // FIXME
            repo_id: RepoId::wiki(),
            title: link.title().to_owned(),
            url: link.url().to_owned(),
        };

        let display_detail = LinkDetail {
            color: "".to_owned(),
            link_id: link.id().to_owned(),
            parent_topic_ids,
            // FIXME
            repo_id: RepoId::wiki(),
            title: link.title().to_owned(),
            url: link.url().to_owned(),
        };

        Ok(Self {
            details: vec![detail],
            display_detail,
            id: link.id().to_string(),
            newly_added: false,
            viewer_review: None,
        })
    }
}

impl TryFrom<&git::RepoTopic> for Topic {
    type Error = Error;

    fn try_from(topic: &git::RepoTopic) -> Result<Self> {
        let parent_topic_ids = topic
            .parent_topics
            .iter()
            .map(|parent| parent.id.to_string())
            .collect::<Vec<String>>();

        let child_ids = topic
            .children
            .iter()
            .map(|child| child.id.clone())
            .collect::<Vec<Oid>>();
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
impl Loader<(RepoId, Oid)> for ObjectLoader {
    type Value = git::RepoObject;
    type Error = Error;

    async fn load(&self, paths: &[(RepoId, Oid)]) -> Result<HashMap<(RepoId, Oid), Self::Value>> {
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

impl TryFrom<&git::RepoObject> for TopicChild {
    type Error = Error;

    fn try_from(object: &git::RepoObject) -> Result<Self> {
        let object = match object {
            git::RepoObject::Link(link) => TopicChild::Link(Link::try_from(link)?),
            git::RepoObject::Topic(topic) => TopicChild::Topic(Topic::try_from(topic)?),
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
            git::RepoObject::Topic(topic) => TopicChild::Topic(Topic::try_from(topic)?),
            git::RepoObject::Link(link) => TopicChild::Link(Link::try_from(link)?),
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
