use chrono;
use std::collections::BTreeSet;

use super::{Link, LinkMetadata, ParentTopic, Synonym, Topic, TopicMetadata};
use crate::prelude::*;

pub fn unix_epoch() -> Timestamp {
    use chrono::TimeZone;
    chrono::Utc.ymd(1970, 1, 1).and_hms_milli(0, 0, 0, 0)
}

pub fn topic(name: &str) -> Topic {
    let added = chrono::Utc::now();
    // Some unique path
    let path = format!("/wiki/{}", name);
    Topic {
        api_version: API_VERSION.to_owned(),
        metadata: TopicMetadata {
            added,
            path: path.to_owned(),
            root: false,
            timerange: None,
            synonyms: Vec::from([Synonym {
                name: name.to_owned(),
                locale: Locale::EN,
                added,
            }]),
        },
        parent_topics: BTreeSet::from([ParentTopic { path }]),
        children: BTreeSet::new(),
    }
}

pub fn link(title: &str, url: &str) -> Link {
    let added = chrono::Utc::now();
    Link {
        api_version: API_VERSION.to_owned(),
        metadata: LinkMetadata {
            title: title.to_owned(),
            url: url.to_owned(),
            path: "/wiki/00002".to_owned(),
            added,
        },
        parent_topics: BTreeSet::from([ParentTopic {
            path: "/wiki/0001".to_owned(),
        }]),
    }
}
