use super::SynonymInput;
use super::{DateTime, Link, Prefix, Synonym, Synonyms, TimeRangePrefixFormat, Timerange, Topic};
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

impl From<&SynonymInput> for git::Synonym {
    fn from(synonym: &SynonymInput) -> Self {
        Self {
            added: chrono::Utc::now(),
            name: synonym.name.clone(),
            locale: synonym.locale.clone(),
        }
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
            name: topic.name("en"),
            prefix,
            root: meta.root,
            synonyms,
            time_range,
        }
    }
}
