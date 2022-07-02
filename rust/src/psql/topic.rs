use crate::graphql::{
    DateTime, Prefix, Synonyms, Timerange, TimerangePrefixFormat, Topic, TopicChild,
};
use crate::prelude::*;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Row {
    pub path: String,
    pub name: String,
    pub parent_topic_paths: Vec<String>,
    pub root: bool,
    pub synonyms: serde_json::Value,
    pub timerange_prefix_format: Option<String>,
    pub timerange_starts_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Row {
    pub fn to_topic(&self) -> Topic {
        let synonyms = Synonyms::from_json(&self.synonyms);
        let timerange = self.timerange();
        let prefix = Prefix::from(&timerange);

        Topic {
            child_paths: vec![],
            path: RepoPath::from(&self.path),
            name: self.name.to_owned(),
            parent_topic_paths: self.parent_topic_paths.iter().map(RepoPath::from).collect(),
            prefix,
            root: self.root,
            synonyms,
            timerange,
        }
    }

    pub fn to_search_result_item(&self) -> TopicChild {
        TopicChild::Topic(self.to_topic())
    }

    pub fn timerange(&self) -> Option<Timerange> {
        match (
            self.timerange_starts_at,
            self.timerange_prefix_format.clone(),
        ) {
            (Some(starts_at), Some(prefix_format)) => {
                let prefix_format = match prefix_format.as_str() {
                    "NONE" => TimerangePrefixFormat::None,
                    "START_YEAR" => TimerangePrefixFormat::StartYear,
                    "START_YEAR_MONTH" => TimerangePrefixFormat::StartYearMonth,
                    _ => TimerangePrefixFormat::None,
                };

                Some(Timerange {
                    starts_at: DateTime(starts_at),
                    ends_at: None,
                    prefix_format,
                })
            }
            _ => None,
        }
    }
}
