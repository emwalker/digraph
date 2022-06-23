use async_graphql::dataloader::*;
use std::collections::HashMap;

use crate::git;
use crate::prelude::*;
use crate::schema::{DateTime, Prefix, Synonym, Synonyms, TimeRangePrefixFormat, Timerange, Topic};

impl From<&git::Synonym> for Synonym {
    fn from(synonym: &git::Synonym) -> Self {
        Self {
            name: synonym.name.clone(),
            locale: synonym.locale.clone(),
        }
    }
}

impl From<&Vec<git::Synonym>> for Synonyms {
    fn from(synonyms: &Vec<git::Synonym>) -> Self {
        Self(synonyms.iter().map(Synonym::from).collect())
    }
}

impl From<&git::TimerangePrefixFormat> for TimeRangePrefixFormat {
    fn from(format: &git::TimerangePrefixFormat) -> Self {
        match format {
            git::TimerangePrefixFormat::None => Self::None,
            git::TimerangePrefixFormat::StartYear => Self::StartYear,
            git::TimerangePrefixFormat::StartYearMonth => Self::StartYearMonth,
        }
    }
}

impl From<&git::Timerange> for Timerange {
    fn from(timerange: &git::Timerange) -> Self {
        Self {
            ends_at: None,
            starts_at: DateTime(timerange.starts),
            prefix_format: TimeRangePrefixFormat::from(&timerange.prefix_format),
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

        let synonyms = Synonyms::from(&meta.synonyms);
        let time_range = meta.timerange.clone().map(|r| Timerange::from(&r));
        let prefix = Prefix::from(&time_range);

        Self {
            path: RepoPath::from(&meta.path),
            parent_topic_paths,
            name: meta.name(),
            prefix,
            root: meta.root,
            synonyms,
            time_range,
        }
    }
}

pub struct TopicLoader {
    viewer: Viewer,
    git: git::Git,
}

impl TopicLoader {
    pub fn new(viewer: Viewer, git: git::Git) -> Self {
        Self { viewer, git }
    }
}

#[async_trait::async_trait]
impl Loader<String> for TopicLoader {
    type Value = Topic;
    type Error = Error;

    async fn load(&self, paths: &[String]) -> Result<HashMap<String, Self::Value>> {
        log::debug!("batch load topics: {:?}", paths);
        let mut map: HashMap<_, _> = HashMap::new();

        for path in paths {
            let link = match &self.git.get(path)? {
                git::Object::Topic(topic) => Topic::from(topic),
                other => {
                    return Err(Error::Repo(format!("expected a topic: {:?}", other)));
                }
            };
            map.insert(path.to_owned(), link);
        }

        Ok(map)
    }
}
