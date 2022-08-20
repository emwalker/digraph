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
    let path = format!("/wiki/{}", id);
    Topic {
        api_version: API_VERSION.to_owned(),
        metadata: TopicMetadata {
            added,
            path: path.to_owned(),
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
        parent_topics: BTreeSet::from([ParentTopic { path }]),
        children: BTreeSet::new(),
    }
}

pub fn link(title: &str, url: &str) -> Link {
    let added = chrono::Utc::now();
    Link {
        api_version: API_VERSION.to_owned(),
        metadata: LinkMetadata {
            path: "/wiki/00002".to_owned(),
            added,
            details: Some(LinkDetails {
                title: title.to_owned(),
                url: url.to_owned(),
            }),
        },
        parent_topics: BTreeSet::from([ParentTopic {
            path: "/wiki/0001".to_owned(),
        }]),
    }
}
