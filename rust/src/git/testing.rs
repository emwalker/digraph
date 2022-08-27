use chrono;
use geotime::Geotime;
use std::collections::BTreeSet;

use super::{
    Link, LinkDetails, LinkMetadata, ParentTopic, Synonym, Topic, TopicDetails, TopicMetadata,
};
use crate::{prelude::*, types::sha256_base64};

pub fn timerange_epoch() -> Geotime {
    use chrono::TimeZone;
    let dt = chrono::Utc.ymd(1970, 1, 1).and_hms(0, 0, 0);
    Geotime::from(&dt)
}

pub fn topic(name: &str) -> Topic {
    let added = chrono::Utc::now();
    // Some unique path
    let id = sha256_base64(name);
    let path = RepoPath::try_from(&format!("/wiki/{}", id)).expect("failed to parse path");
    Topic {
        api_version: API_VERSION.to_owned(),
        metadata: TopicMetadata {
            added,
            id: path.id.to_owned(),
            details: Some(TopicDetails {
                root: false,
                timerange: None,
                synonyms: Vec::from([Synonym {
                    name: name.to_owned(),
                    locale: Locale::EN,
                    added,
                }]),
            }),
        },
        parent_topics: BTreeSet::from([ParentTopic { id: path.id }]),
        children: BTreeSet::new(),
    }
}

pub fn link(title: &str, url: &str) -> Link {
    let added = chrono::Utc::now();
    Link {
        api_version: API_VERSION.to_owned(),
        metadata: LinkMetadata {
            id: "00002".try_into().unwrap(),
            added,
            details: Some(LinkDetails {
                title: title.to_owned(),
                url: url.to_owned(),
            }),
        },
        parent_topics: BTreeSet::from([ParentTopic {
            id: "00001".try_into().expect("failed to parse id"),
        }]),
    }
}
