use std::collections::{BTreeSet, HashMap};

use super::{Kind, Phrase, RepoObject, Search, SearchMatch, SortKey};
use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Map(HashMap<RepoId, Option<RepoObject>>);

impl Map {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, repo_id: RepoId, repo_obj: Option<RepoObject>) {
        self.0.insert(repo_id, repo_obj);
    }

    pub fn values(&self) -> std::collections::hash_map::Values<'_, RepoId, Option<RepoObject>> {
        self.0.values()
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, RepoId, Option<RepoObject>> {
        self.0.iter()
    }
}

#[derive(Clone, Debug)]
pub enum Object {
    Topic { id: Oid, map: Map },
    Link { id: Oid, map: Map },
    Unknown { id: Oid, map: Map },
}

impl std::cmp::PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl std::cmp::Eq for Object {}

impl Object {
    pub fn id(&self) -> &Oid {
        match self {
            Self::Topic { id, .. } => id,
            Self::Link { id, .. } => id,
            Self::Unknown { id, .. } => id,
        }
    }

    fn insert(&mut self, repo_id: RepoId, repo_obj: Option<RepoObject>) {
        match self {
            Self::Topic { map, .. } => map.insert(repo_id, repo_obj),
            Self::Link { map, .. } => map.insert(repo_id, repo_obj),
            Self::Unknown { map, .. } => map.insert(repo_id, repo_obj),
        }
    }

    pub fn display_string(&self, locale: Locale) -> String {
        for repo_obj in self.map().values() {
            match repo_obj {
                Some(RepoObject::Link(link)) => return link.title().to_owned(),
                Some(RepoObject::Topic(topic)) => return topic.name(locale),
                _ => continue,
            }
        }
        "".into()
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, RepoId, Option<RepoObject>> {
        match self {
            Self::Link { map, .. } => map.iter(),
            Self::Topic { map, .. } => map.iter(),
            Self::Unknown { map, .. } => map.iter(),
        }
    }

    fn map(&self) -> &Map {
        match self {
            Self::Topic { map, .. } => map,
            Self::Link { map, .. } => map,
            Self::Unknown { map, .. } => map,
        }
    }

    pub fn to_search_match(self, locale: Locale, search: &Search) -> Result<SearchMatch> {
        let normalized = &search.normalized;
        let display_string = self.display_string(locale);
        let search_string = Phrase::parse(&display_string);

        match self {
            Self::Link { .. } => Ok(SearchMatch {
                sort_key: SortKey(Kind::Link, &search_string != normalized, display_string),
                kind: Kind::Link,
                object: self,
            }),

            Self::Topic { .. } => {
                let topic_id = self.id();
                let explicit_in_search = search.topic_specs.iter().any(|s| &s.id == topic_id);
                Ok(SearchMatch {
                    sort_key: SortKey(
                        Kind::Topic,
                        !explicit_in_search && &search_string != normalized,
                        display_string,
                    ),
                    kind: Kind::Topic,
                    object: self,
                })
            }

            _ => Err(Error::Repo("failed to convert to search match".into())),
        }
    }
}

#[derive(Clone, Default)]
pub struct Objects(HashMap<Oid, Object>);

impl Objects {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, id: Oid, repo_id: RepoId, repo_obj: Option<RepoObject>) {
        let obj = self
            .0
            .entry(id.to_owned())
            .or_insert_with(|| match repo_obj {
                Some(RepoObject::Link(_)) => Object::Link {
                    id: id.to_owned(),
                    map: Map::new(),
                },

                Some(RepoObject::Topic(_)) => Object::Topic {
                    id: id.to_owned(),
                    map: Map::new(),
                },

                None => Object::Unknown {
                    id: id.to_owned(),
                    map: Map::new(),
                },
            });

        obj.insert(repo_id, repo_obj);
    }

    pub fn to_hash(self) -> HashMap<Oid, Object> {
        self.0
    }

    pub fn to_matches(self, search: &Search, locale: Locale) -> Result<BTreeSet<SearchMatch>> {
        let mut matches = BTreeSet::new();

        for (_oid, object) in self.0.into_iter() {
            matches.insert(object.to_search_match(locale, search)?);
        }

        Ok(matches)
    }
}
