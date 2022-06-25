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

        let child_paths = topic
            .children
            .iter()
            .map(|p| RepoPath::from(&p.path))
            .collect::<Vec<RepoPath>>();

        let synonyms = Synonyms::from(&meta.synonyms);
        let time_range = meta.timerange.clone().map(|r| Timerange::from(&r));
        let prefix = Prefix::from(&time_range);

        Self {
            child_paths,
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
