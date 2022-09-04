use chrono;
use geotime::Geotime;
use std::collections::BTreeSet;

use super::{
    RepoLink, RepoLinkDetails, RepoLinkMetadata, ParentTopic, Synonym, RepoTopic, RepoTopicDetails, RepoTopicMetadata,
};
use crate::{prelude::*, types::sha256_base64};

pub fn timerange_epoch() -> Geotime {
    use chrono::TimeZone;
    let dt = chrono::Utc.ymd(1970, 1, 1).and_hms(0, 0, 0);
    Geotime::from(&dt)
}

pub fn topic(name: &str) -> RepoTopic {
    let added = chrono::Utc::now();
    // Some unique path
    let id: Oid = sha256_base64(name).try_into().unwrap();
    RepoTopic {
        api_version: API_VERSION.to_owned(),
        metadata: RepoTopicMetadata {
            added,
            id: id.to_owned(),
            details: Some(RepoTopicDetails {
                root: false,
                timerange: None,
                synonyms: Vec::from([Synonym {
                    name: name.to_owned(),
                    locale: Locale::EN,
                    added,
                }]),
            }),
        },
        parent_topics: BTreeSet::from([ParentTopic { id }]),
        children: BTreeSet::new(),
    }
}

pub fn link(title: &str, url: &str) -> RepoLink {
    let added = chrono::Utc::now();
    RepoLink {
        api_version: API_VERSION.to_owned(),
        metadata: RepoLinkMetadata {
            id: "00002".try_into().unwrap(),
            added,
            details: Some(RepoLinkDetails {
                title: title.to_owned(),
                url: url.to_owned(),
            }),
        },
        parent_topics: BTreeSet::from([ParentTopic {
            id: "00001".try_into().expect("failed to parse id"),
        }]),
    }
}
