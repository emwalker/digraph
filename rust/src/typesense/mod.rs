use chrono::{Duration, Utc};
use itertools::Itertools;
use queues::{queue, IsQueue, Queue};
use serde::Serialize;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use unidecode::unidecode;

use crate::git::Kind;
use crate::git::{self, OuterRepoObject, RepoObject};
use crate::prelude::*;

fn normalize(input: &str) -> String {
    unidecode(input).split_whitespace().join(" ")
}

#[derive(Serialize)]
pub struct Record {
    pub display_string: String,
    pub external_id: ExternalId,
    pub id: String,
    pub kind: Kind,
    pub locale: Locale,
    pub repo_id: String,
    pub sort_key: String,
    pub synonyms: Vec<String>,
    pub upset_topic_ids: BTreeSet<ExternalId>,
}

pub struct DownsetIter<'v> {
    queue: Queue<ExternalId>,
    seen: HashSet<ExternalId>,
    view: &'v git::core::View,
}

impl<'v> Iterator for DownsetIter<'v> {
    type Item = git::OuterRepoObject;

    fn next(&mut self) -> Option<Self::Item> {
        while self.queue.size() > 0 {
            let id = self.queue.remove().expect("failed to remove from queue");

            if self.seen.contains(&id) {
                continue;
            }

            match self.view.object(&id) {
                Ok(Some(object)) => {
                    if let OuterRepoObject {
                        inner: RepoObject::Topic(topic),
                        ..
                    } = &object
                    {
                        for child in &topic.children {
                            if self.seen.contains(&child.id) {
                                continue;
                            }

                            self.queue
                                .add(child.id.to_owned())
                                .expect("failed to add to queue");
                        }
                    }

                    self.seen.insert(id.to_owned());
                    return Some(object);
                }

                Ok(None) => {
                    log::warn!("no object found for id: {id}");
                }

                Err(err) => {
                    log::error!("error fetching id: {err}");
                }
            }
        }

        None
    }
}

impl<'v> DownsetIter<'v> {
    fn from(view: &'v git::core::View, topic_id: ExternalId) -> Self {
        let queue = queue![topic_id];
        Self {
            queue,
            view,
            seen: HashSet::new(),
        }
    }
}

struct Upsets<'v> {
    view: &'v git::core::View,
    map: HashMap<ExternalId, BTreeSet<ExternalId>>,
}

impl<'v> Upsets<'v> {
    fn new(view: &'v git::core::View) -> Self {
        Self {
            view,
            map: HashMap::new(),
        }
    }
}

impl<'v> Upsets<'v> {
    fn build(
        &mut self,
        topic_id: Option<&ExternalId>,
        parent_topics: &BTreeSet<git::ParentTopic>,
    ) -> BTreeSet<ExternalId> {
        if let Some(topic_id) = topic_id {
            if let Some(upset) = self.map.get(topic_id) {
                return upset.iter().cloned().collect();
            }
        }

        let mut upset: BTreeSet<ExternalId> = BTreeSet::new();

        if let Some(topic_id) = topic_id {
            upset.insert(topic_id.to_owned());
        }

        for git::ParentTopic { id, .. } in parent_topics.iter() {
            let parent = self.view.topic(id).unwrap().unwrap();
            let parent_upset = self.build(Some(parent.topic_id()), &parent.parent_topics);
            for parent_topic_id in parent_upset.iter() {
                upset.insert(parent_topic_id.to_owned());
            }
        }

        if let Some(topic_id) = topic_id {
            self.map
                .insert(topic_id.to_owned(), upset.iter().cloned().collect());
        }

        upset
    }
}

pub struct GenerateRecords<'v> {
    inner: DownsetIter<'v>,
    future: Timestamp,
    locale: Locale,
    upsets: Upsets<'v>,
}

impl<'v> GenerateRecords<'v> {
    pub fn new(view: &'v git::core::View, topic_id: ExternalId, locale: Locale) -> Self {
        let inner = DownsetIter::from(view, topic_id);
        let future = Utc::now() + Duration::weeks(1_000_000);

        Self {
            inner,
            locale,
            future,
            upsets: Upsets::new(view),
        }
    }
}

impl<'v> Iterator for GenerateRecords<'v> {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        let object = self.inner.next();
        object.as_ref()?;

        match object.unwrap() {
            OuterRepoObject {
                inner: RepoObject::Topic(topic),
                oid,
                repo_id,
            } => {
                let upset_topic_ids = self
                    .upsets
                    .build(Some(topic.topic_id()), &topic.parent_topics);
                let name = normalize(&topic.name(self.locale));
                let synonyms = topic
                    .synonyms()
                    .iter()
                    .map(git::Synonym::to_string)
                    .collect();
                let id = format!("{}@{}", topic.topic_id(), oid);

                Some(Record {
                    display_string: name.to_owned(),
                    external_id: topic.topic_id().to_owned(),
                    id,
                    kind: Kind::Topic,
                    locale: self.locale,
                    repo_id: repo_id.to_string(),
                    sort_key: format!("0|{name}"),
                    synonyms,
                    upset_topic_ids,
                })
            }

            OuterRepoObject {
                inner: RepoObject::Link(link),
                oid,
                repo_id,
            } => {
                let upset_topic_ids = self.upsets.build(None, &link.parent_topics);
                let added_desc = (self.future - link.added()).num_seconds().to_string();
                let id = format!("{}@{}", link.id(), oid);

                Some(Record {
                    display_string: link.title().to_owned(),
                    external_id: link.id().to_owned(),
                    id,
                    kind: Kind::Link,
                    locale: self.locale,
                    repo_id: repo_id.to_string(),
                    sort_key: format!("1|{added_desc}"),
                    synonyms: vec![],
                    upset_topic_ids,
                })
            }
        }
    }
}
