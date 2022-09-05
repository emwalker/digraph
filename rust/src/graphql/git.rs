use async_graphql::dataloader::*;
use itertools::Itertools;
use std::collections::HashMap;

use super::{
    timerange, Link, LinkDetail, Synonym, SynonymInput, SynonymMatch, Synonyms, Topic, TopicChild,
    TopicDetail, ViewStats,
};
use crate::git;
use crate::prelude::*;

pub use git::{Client, DataRoot};

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
            .map(|topic| topic.id.to_owned())
            .collect::<Vec<Oid>>();

        let detail = LinkDetail {
            color: "".to_owned(),
            link_id: link.id().to_owned(),
            parent_topic_ids: parent_topic_ids.to_owned(),
            // FIXME
            repo_id: RepoId::wiki(),
            title: link.title().to_owned(),
            url: link.url().to_owned(),
        };

        Ok(Self {
            display_detail: detail.to_owned(),
            details: vec![detail],
            id: link.id().to_owned(),
            newly_added: false,
            viewer_review: None,
        })
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
        let git::SearchMatch { object, kind, .. } = item;
        let child = match kind {
            git::Kind::Topic => TopicChild::Topic(object.try_into()?),
            git::Kind::Link => TopicChild::Link(object.try_into()?),
        };
        Ok(child)
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

impl TryFrom<(&RepoId, &Option<git::RepoObject>)> for LinkDetail {
    type Error = Error;

    fn try_from((repo_id, object): (&RepoId, &Option<git::RepoObject>)) -> Result<Self> {
        match object {
            Some(git::RepoObject::Link(link)) => {
                let parent_topic_ids = link
                    .parent_topics
                    .iter()
                    .map(|parent| parent.id.to_owned())
                    .collect_vec();

                Ok(Self {
                    color: "".to_owned(),
                    link_id: link.id().to_owned(),
                    parent_topic_ids,
                    repo_id: repo_id.to_owned(),
                    title: link.title().to_owned(),
                    url: link.url().to_owned(),
                })
            }

            _ => Err(Error::NotFound(format!("expected a link: {:?}", object))),
        }
    }
}

impl TryFrom<&git::Object> for Link {
    type Error = Error;

    fn try_from(value: &git::Object) -> Result<Self> {
        let mut details: Vec<LinkDetail> = vec![];

        for (repo_id, repo_obj) in value.iter() {
            if repo_obj.is_none() {
                continue;
            }
            details.push((repo_id, repo_obj).try_into()?);
        }

        if details.is_empty() {
            return Err(Error::NotFound(format!("no link: {}", value.id())));
        }

        Ok(Link {
            display_detail: details.first().unwrap().to_owned(),
            details,
            id: value.id().to_owned(),
            newly_added: false,
            viewer_review: None,
        })
    }
}

impl TryFrom<git::Object> for Link {
    type Error = Error;

    fn try_from(value: git::Object) -> Result<Self> {
        (&value).try_into()
    }
}

impl TryFrom<Option<git::Object>> for Link {
    type Error = Error;

    fn try_from(value: Option<git::Object>) -> Result<Self> {
        value
            .ok_or_else(|| Error::NotFound("no link".into()))?
            .try_into()
    }
}

impl TryFrom<(&RepoId, &Option<git::RepoObject>)> for TopicDetail {
    type Error = Error;

    fn try_from((repo_id, object): (&RepoId, &Option<git::RepoObject>)) -> Result<Self> {
        match object {
            Some(git::RepoObject::Topic(topic)) => {
                let parent_topic_ids = topic
                    .parent_topics
                    .iter()
                    .map(|parent| parent.id.to_owned())
                    .collect_vec();

                let child_ids = topic
                    .children
                    .iter()
                    .map(|child| child.id.to_owned())
                    .collect_vec();

                let timerange = topic.timerange().as_ref();
                let timerange = match timerange {
                    Some(timerange) => Some(timerange::Timerange::try_from(timerange)?),
                    None => None,
                };

                Ok(Self {
                    child_ids,
                    color: "".to_owned(),         // FIXME
                    name: topic.name(Locale::EN), // FIXME
                    parent_topic_ids,
                    repo_id: repo_id.to_owned(),
                    synonyms: topic.synonyms().into(),
                    timerange,
                    topic_id: topic.id().to_owned(),
                })
            }

            _ => Err(Error::NotFound(format!("expected a topic: {:?}", object))),
        }
    }
}

impl TryFrom<Option<git::Object>> for Topic {
    type Error = Error;

    fn try_from(value: Option<git::Object>) -> Result<Self> {
        value
            .ok_or_else(|| Error::NotFound("no topic".into()))?
            .try_into()
    }
}

impl TryFrom<&git::Object> for Topic {
    type Error = Error;

    fn try_from(value: &git::Object) -> Result<Self> {
        let mut details: Vec<TopicDetail> = vec![];

        for (repo_id, repo_obj) in value.iter() {
            if repo_obj.is_none() {
                continue;
            }
            details.push((repo_id, repo_obj).try_into()?);
        }

        if details.is_empty() {
            return Err(Error::NotFound(format!("no link: {}", value.id())));
        }

        Ok(Topic {
            display_detail: details.first().unwrap().to_owned(),
            details,
            id: value.id().to_owned(),
            newly_added: false,
            root: value.id().is_root(),
        })
    }
}

impl TryFrom<git::Object> for Topic {
    type Error = Error;

    fn try_from(value: git::Object) -> Result<Self> {
        (&value).try_into()
    }
}

pub struct ObjectLoader {
    client: git::Client,
}

impl ObjectLoader {
    pub fn new(client: git::Client) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl Loader<Oid> for ObjectLoader {
    type Value = git::Object;
    type Error = Error;

    async fn load(&self, oids: &[Oid]) -> Result<HashMap<Oid, Self::Value>> {
        log::debug!("batch load topics: {:?}", oids);
        Ok(self.client.fetch_all(oids).to_hash())
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
