use std::collections::{BTreeSet, HashMap, HashSet};

use super::{
    activity, Kind, Link, Mutation, Object, ParentTopic, SaveChangesForPrefix, Synonym,
    SynonymEntry, SynonymMatch, Timerange, Topic, TopicChild, TopicDetails, TopicMetadata,
    API_VERSION,
};
use crate::prelude::*;

pub struct DeleteTopic {
    pub actor: Viewer,
    pub topic_id: RepoPath,
}

pub struct DeleteTopicResult {
    pub alerts: Vec<Alert>,
    pub deleted_topic_path: RepoPath,
}

impl DeleteTopic {
    pub fn call<S>(&self, mut mutation: Mutation, store: &S) -> Result<DeleteTopicResult>
    where
        S: SaveChangesForPrefix,
    {
        let topic_id = &self.topic_id;
        let date = chrono::Utc::now();

        let topic = mutation.fetch_topic(&topic_id.repo, topic_id);
        if topic.is_none() {
            return Err(Error::NotFound(format!("not found: {}", topic_id)));
        }
        let topic = topic.unwrap();

        if topic.root() {
            return Err(Error::Repo("cannot delete root topic".to_owned()));
        }

        mutation.mark_deleted(topic_id)?;

        let parent_topics = topic
            .parent_topics
            .iter()
            .map(|parent| ParentTopic {
                path: parent.path.to_owned(),
            })
            .collect::<BTreeSet<ParentTopic>>();

        let mut topics = vec![];

        // Remove the topic from the children of the parent topics
        for parent in &topic.parent_topics {
            let parent_id = RepoPath::try_from(&parent.path)?;
            if let Some(mut topic) = mutation.fetch_topic(&parent_id.repo, &parent_id) {
                topic.children.remove(&TopicChild {
                    // The 'added' field is ignored
                    added: chrono::Utc::now(),
                    kind: Kind::Topic,
                    path: topic_id.inner.to_owned(),
                });
                mutation.save_topic(&parent_id.repo, &parent_id, &topic)?;
                topics.push(topic.clone());
            }
        }

        let mut child_links = vec![];
        let mut child_topics = vec![];

        // Remove the topic from its children, moving them onto the parent topics
        for child in &topic.children {
            let link_id = RepoPath::try_from(&child.path)?;
            match mutation.fetch(&link_id.repo, &link_id) {
                Some(Object::Link(child_link)) => {
                    let mut link = child_link.to_owned();
                    link.parent_topics.remove(&topic.to_parent_topic());
                    link.parent_topics.append(&mut parent_topics.clone());
                    child_links.push(link.clone());
                    mutation.save_link(&link_id.repo, &link_id, &link)?;
                }

                Some(Object::Topic(child_topic)) => {
                    let mut child_topic = child_topic.to_owned();
                    child_topic.parent_topics.remove(&topic.to_parent_topic());
                    child_topic.parent_topics.append(&mut parent_topics.clone());
                    child_topics.push(child_topic.clone());
                    mutation.save_topic(&link_id.repo, &link_id, &child_topic)?;
                }

                None => {}
            }
        }

        let change = self.change(&topic, &topics, &child_links, &child_topics, date);

        mutation.remove_topic(&self.topic_id, &topic)?;
        mutation.add_change(&change)?;
        mutation.write(store)?;

        Ok(DeleteTopicResult {
            alerts: vec![],
            deleted_topic_path: self.topic_id.clone(),
        })
    }

    fn change(
        &self,
        topic: &Topic,
        parent_topics: &Vec<Topic>,
        child_links: &Vec<Link>,
        child_topics: &Vec<Topic>,
        date: Timestamp,
    ) -> activity::Change {
        let mut deleted_topic = activity::TopicInfo::from(topic);
        deleted_topic.deleted = true;

        activity::Change::DeleteTopic(activity::DeleteTopic {
            actor_id: self.actor.user_id.to_owned(),
            child_links: activity::LinkInfoList::from(child_links),
            child_topics: activity::TopicInfoList::from(child_topics),
            date,
            deleted_topic,
            id: activity::Change::new_id(),
            parent_topics: activity::TopicInfoList::from(parent_topics),
        })
    }
}

pub struct RemoveTopicTimerange {
    pub actor: Viewer,
    pub topic_id: RepoPath,
}

pub struct RemoveTopicTimerangeResult {
    pub alerts: Vec<Alert>,
    pub topic: Topic,
}

impl RemoveTopicTimerange {
    pub fn call<S>(&self, mut mutation: Mutation, store: &S) -> Result<RemoveTopicTimerangeResult>
    where
        S: SaveChangesForPrefix,
    {
        let topic = mutation.fetch_topic(&self.topic_id.repo, &self.topic_id);
        if topic.is_none() {
            return Err(Error::NotFound(format!("not found: {}", self.topic_id)));
        }
        let mut topic = topic.unwrap();

        let previous_timerange = topic.timerange().to_owned();

        match &mut topic.metadata.details {
            Some(details) => {
                details.timerange = None;
            }

            None => {}
        }

        mutation.save_topic(&self.topic_id.repo, &self.topic_id, &topic)?;
        mutation.add_change(&self.change(&topic, previous_timerange))?;
        mutation.write(store)?;

        Ok(RemoveTopicTimerangeResult {
            alerts: vec![],
            topic,
        })
    }

    pub fn change(&self, topic: &Topic, previous_timerange: Option<Timerange>) -> activity::Change {
        let mut parent_topics = BTreeSet::new();
        for parent in &topic.parent_topics {
            parent_topics.insert(parent.path.to_owned());
        }

        activity::Change::RemoveTopicTimerange(activity::RemoveTopicTimerange {
            actor_id: self.actor.user_id.to_owned(),
            date: chrono::Utc::now(),
            id: activity::Change::new_id(),
            parent_topics: parent_topics.iter().map(|path| path.to_owned()).collect(),
            previous_timerange,
            updated_topic: activity::TopicInfo::from(topic),
        })
    }
}

pub struct UpdateTopicParentTopics {
    pub actor: Viewer,
    pub parent_topic_ids: BTreeSet<RepoPath>,
    pub topic_id: RepoPath,
}

pub struct UpdateTopicParentTopicsResult {
    pub alerts: Vec<Alert>,
    pub topic: Topic,
}

impl UpdateTopicParentTopics {
    pub fn call<S>(
        &self,
        mut mutation: Mutation,
        store: &S,
    ) -> Result<UpdateTopicParentTopicsResult>
    where
        S: SaveChangesForPrefix,
    {
        self.validate(&mutation)?;

        let date = chrono::Utc::now();
        let child = mutation.fetch_topic(&self.topic_id.repo, &self.topic_id);
        if child.is_none() {
            return Err(Error::NotFound(format!("not found: {}", self.topic_id)));
        }
        let mut child = child.unwrap();

        let mut updates: Vec<Topic> = vec![];

        let parent_topics = self
            .parent_topic_ids
            .iter()
            .map(|path| ParentTopic {
                path: path.to_string(),
            })
            .collect::<BTreeSet<ParentTopic>>();

        let added = parent_topics
            .difference(&child.parent_topics)
            .cloned()
            .collect::<Vec<ParentTopic>>();
        let mut added_topics = vec![];

        for parent in added {
            if let Some(mut topic) = parent.fetch(&mutation)? {
                topic.children.insert(child.to_topic_child(date));
                child.parent_topics.insert(parent.to_owned());
                added_topics.push(topic.clone());
                updates.push(topic);
            }
        }

        let removed = child
            .parent_topics
            .difference(&parent_topics)
            .cloned()
            .collect::<Vec<ParentTopic>>();
        let mut removed_topics = vec![];

        for parent in removed {
            if let Some(mut topic) = parent.fetch(&mutation)? {
                topic.children.remove(&child.to_topic_child(date));
                child.parent_topics.remove(&parent);
                removed_topics.push(topic.clone());
                updates.push(topic);
            }
        }

        let change = self.change(&child, &parent_topics, &added_topics, &removed_topics, date);

        updates.push(child.clone());
        for topic in updates {
            let topic_id = &topic.path()?;
            mutation.save_topic(&topic_id.repo, &topic_id, &topic)?;
        }
        mutation.add_change(&change)?;
        mutation.write(store)?;

        Ok(UpdateTopicParentTopicsResult {
            alerts: vec![],
            topic: child,
        })
    }

    fn change(
        &self,
        topic: &Topic,
        parent_topics: &BTreeSet<ParentTopic>,
        added: &Vec<Topic>,
        removed: &Vec<Topic>,
        date: Timestamp,
    ) -> activity::Change {
        activity::Change::UpdateTopicParentTopics(activity::UpdateTopicParentTopics {
            actor_id: self.actor.user_id.to_owned(),
            added_parent_topics: activity::TopicInfoList::from(added),
            date,
            id: activity::Change::new_id(),
            parent_topic_paths: parent_topics
                .iter()
                .map(|parent| parent.path.to_owned())
                .collect::<BTreeSet<String>>(),
            removed_parent_topics: activity::TopicInfoList::from(removed),
            updated_topic: activity::TopicInfo::from(topic),
        })
    }

    fn validate(&self, builder: &Mutation) -> Result<()> {
        if self.parent_topic_ids.is_empty() {
            return Err(Error::Repo(
                "at least one parent topic must be provided".into(),
            ));
        }

        for parent in &self.parent_topic_ids {
            if builder.cycle_exists(&self.topic_id.repo, &self.topic_id, parent)? {
                return Err(Error::Repo(format!(
                    "{} is a parent topic of {}",
                    self.topic_id, parent
                )));
            }
        }

        Ok(())
    }
}

pub struct UpdateTopicSynonyms {
    pub actor: Viewer,
    pub synonyms: Vec<Synonym>,
    pub topic_id: RepoPath,
}

pub struct UpdateTopicSynonymsResult {
    pub alerts: Vec<Alert>,
    pub topic: Topic,
}

impl UpdateTopicSynonyms {
    pub fn call<S>(&self, mut mutation: Mutation, store: &S) -> Result<UpdateTopicSynonymsResult>
    where
        S: SaveChangesForPrefix,
    {
        let topic = mutation.fetch_topic(&self.topic_id.repo, &self.topic_id);
        if topic.is_none() {
            return Err(Error::NotFound(format!("not found: {}", self.topic_id)));
        }
        let mut topic = topic.unwrap();

        let lookup = topic
            .synonyms()
            .iter()
            .map(|synonym| {
                (
                    (synonym.name.to_owned(), synonym.locale.to_owned()),
                    synonym.to_owned(),
                )
            })
            .collect::<HashMap<(String, Locale), Synonym>>();

        let before = lookup
            .keys()
            .map(|(name, locale)| (name.to_owned(), locale.to_owned()))
            .collect::<HashSet<(String, Locale)>>();

        let mut synonyms = vec![];
        let mut after = HashSet::new();

        // Preserve the date the synonym was added
        for new in &self.synonyms {
            let key = (new.name.to_owned(), new.locale.to_owned());

            // Remove duplicates
            if after.contains(&key) {
                continue;
            }
            after.insert(key.to_owned());

            if lookup.contains_key(&key) {
                match lookup.get(&key) {
                    Some(existing) => synonyms.push(existing.to_owned()),
                    None => synonyms.push(new.to_owned()),
                };
            } else {
                synonyms.push(new.to_owned());
            }
        }

        let added = after
            .difference(&before)
            .cloned()
            .collect::<HashSet<(String, Locale)>>();
        let removed = before
            .difference(&after)
            .cloned()
            .collect::<HashSet<(String, Locale)>>();

        match &mut topic.metadata.details {
            Some(details) => {
                details.synonyms = synonyms;
            }

            None => {}
        }

        mutation.save_topic(&self.topic_id.repo, &self.topic_id, &topic)?;
        mutation.add_change(&self.change(&topic, &added, &removed))?;
        mutation.write(store)?;

        Ok(UpdateTopicSynonymsResult {
            alerts: vec![],
            topic,
        })
    }

    fn change(
        &self,
        topic: &Topic,
        added: &HashSet<(String, Locale)>,
        removed: &HashSet<(String, Locale)>,
    ) -> activity::Change {
        activity::Change::UpdateTopicSynonyms(activity::UpdateTopicSynonyms {
            actor_id: self.actor.user_id.to_owned(),
            added_synonyms: activity::SynonymList::from(added),
            id: activity::Change::new_id(),
            date: chrono::Utc::now(),
            parent_topics: topic
                .parent_topics
                .iter()
                .map(|parent| parent.path.to_owned())
                .collect::<BTreeSet<String>>(),
            reordered: added.is_empty() && removed.is_empty(),
            removed_synonyms: activity::SynonymList::from(removed),
            updated_topic: activity::TopicInfo::from(topic),
        })
    }
}

pub enum OnMatchingSynonym {
    Ask,
    CreateDistinct,
    Update(RepoPath),
}

pub struct UpsertTopic {
    pub actor: Viewer,
    pub locale: Locale,
    pub name: String,
    pub on_matching_synonym: OnMatchingSynonym,
    pub parent_topic: RepoPath,
    pub repo: RepoName,
}

pub struct UpsertTopicResult {
    pub alerts: Vec<Alert>,
    pub matching_synonyms: BTreeSet<SynonymMatch>,
    pub saved: bool,
    pub topic: Option<Topic>,
}

impl UpsertTopic {
    pub fn call<S>(&self, mut mutation: Mutation, store: &S) -> Result<UpsertTopicResult>
    where
        S: SaveChangesForPrefix,
    {
        let parent = self.fetch_parent(&mutation);
        if parent.is_none() {
            return Err(Error::NotFound(format!("not found: {}", self.parent_topic)));
        }
        let mut parent = parent.unwrap();

        let mut matches = mutation.synonym_phrase_matches(&[&self.repo], &self.name)?;
        let date = chrono::Utc::now();

        let (path, child, parent_topics) = if matches.is_empty() {
            self.make_topic(&parent)?
        } else {
            match &self.on_matching_synonym {
                OnMatchingSynonym::Ask => {
                    log::info!("topic '{}' exists and OnMatchingSynonym::Ask", self.name);
                    let alert = Alert::Warning(format!(
                        r#"A topic with the name '{}' already exists. Would you like to update it
                        or create a new topic?"#,
                        self.name
                    ));

                    let parent_path = &parent.path()?;
                    let mut matching_synonyms: BTreeSet<SynonymMatch> = BTreeSet::new();

                    for synonym_match in matches {
                        let topic_id = &synonym_match.topic.path()?;
                        if mutation.cycle_exists(&topic_id.repo, topic_id, parent_path)? {
                            matching_synonyms.insert(synonym_match.with_cycle(true));
                        } else {
                            matching_synonyms.insert(synonym_match);
                        }
                    }

                    return Ok(UpsertTopicResult {
                        alerts: vec![alert],
                        matching_synonyms,
                        saved: false,
                        topic: None,
                    });
                }

                OnMatchingSynonym::CreateDistinct => {
                    log::info!(
                        "creating new topic even though there are matching synonyms: {:?}",
                        matches
                    );
                    self.make_topic(&parent)?
                }

                OnMatchingSynonym::Update(topic_id) => {
                    if mutation.cycle_exists(&topic_id.repo, topic_id, &parent.path()?)? {
                        let ancestor = mutation.fetch_topic(&topic_id.repo, topic_id);
                        if ancestor.is_none() {
                            return Err(Error::NotFound(format!("not found: {}", topic_id)));
                        }
                        let ancestor = ancestor.unwrap();

                        log::info!("a cycle was found");
                        let alerts = vec![Alert::Warning(format!(
                            "{} is a subtopic of {}",
                            &parent.name(Locale::EN),
                            &ancestor.name(Locale::EN),
                        ))];

                        matches.replace(SynonymMatch {
                            cycle: true,
                            entry: SynonymEntry {
                                name: self.name.to_owned(),
                                path: topic_id.inner.to_owned(),
                            },
                            name: self.name.to_owned(),
                            topic: ancestor,
                        });

                        return Ok(UpsertTopicResult {
                            alerts,
                            matching_synonyms: matches,
                            saved: false,
                            topic: None,
                        });
                    }

                    log::info!("updating existing topic {:?}", topic_id);
                    let topic = mutation.fetch_topic(&topic_id.repo, topic_id);
                    if topic.is_none() {
                        return Err(Error::NotFound(format!("not found: {}", topic_id)));
                    }
                    let mut topic = topic.unwrap();
                    let parent_topics = topic.parent_topics.clone();

                    topic.parent_topics.insert(parent.to_parent_topic());

                    match &mut topic.metadata.details {
                        Some(details) => {
                            details.synonyms.push(Synonym {
                                added: date,
                                locale: self.locale,
                                name: self.name.to_owned(),
                            });
                        }

                        None => {}
                    }

                    (topic_id.clone(), topic, parent_topics)
                }
            }
        };

        parent.children.insert(child.to_topic_child(date));

        let change = self.change(&child, &parent_topics, &parent, date);
        mutation.save_topic(&path.repo, &path, &child)?;
        let parent_id = parent.path()?;
        mutation.save_topic(&parent_id.repo, &parent_id, &parent)?;
        mutation.add_change(&change)?;
        mutation.write(store)?;

        Ok(UpsertTopicResult {
            alerts: vec![],
            matching_synonyms: BTreeSet::new(),
            saved: true,
            topic: Some(child),
        })
    }

    fn change(
        &self,
        topic: &Topic,
        parent_topics: &BTreeSet<ParentTopic>,
        parent: &Topic,
        date: Timestamp,
    ) -> activity::Change {
        activity::Change::UpsertTopic(activity::UpsertTopic {
            actor_id: self.actor.user_id.to_owned(),
            id: activity::Change::new_id(),
            date,
            parent_topic: activity::TopicInfo::from(parent),
            parent_topic_paths: parent_topics
                .iter()
                .map(|parent| parent.path.to_owned())
                .collect::<BTreeSet<String>>(),
            upserted_topic: activity::TopicInfo::from(topic),
        })
    }

    fn make_topic(&self, parent: &Topic) -> Result<(RepoPath, Topic, BTreeSet<ParentTopic>)> {
        let added = chrono::Utc::now();
        let path = RepoPath::make(&self.repo.to_string())?;
        let parent_topics = BTreeSet::from([parent.to_parent_topic()]);

        let topic = Topic {
            api_version: API_VERSION.into(),
            metadata: TopicMetadata {
                added,
                path: path.to_string(),
                details: Some(TopicDetails {
                    root: false,
                    synonyms: vec![Synonym {
                        added,
                        locale: self.locale.to_owned(),
                        name: self.name.to_owned(),
                    }],
                    timerange: None,
                }),
            },
            parent_topics: parent_topics.clone(),
            children: BTreeSet::new(),
        };

        Ok((path, topic, parent_topics))
    }

    fn fetch_parent(&self, mutation: &Mutation) -> Option<Topic> {
        mutation.fetch_topic(&self.parent_topic.repo, &self.parent_topic)
    }
}

pub struct UpsertTopicTimerange {
    pub actor: Viewer,
    pub timerange: Timerange,
    pub topic_id: RepoPath,
}

pub struct UpsertTopicTimerangeResult {
    pub alerts: Vec<Alert>,
    pub timerange: Timerange,
    pub topic: Topic,
}

impl UpsertTopicTimerange {
    pub fn call<S>(&self, mut mutation: Mutation, store: &S) -> Result<UpsertTopicTimerangeResult>
    where
        S: SaveChangesForPrefix,
    {
        let topic = mutation.fetch_topic(&self.topic_id.repo, &self.topic_id);
        if topic.is_none() {
            return Err(Error::NotFound(format!("not found: {}", self.topic_id)));
        }
        let mut topic = topic.unwrap();

        let previous_timerange = topic.timerange().clone();

        match &mut topic.metadata.details {
            Some(details) => {
                details.timerange = Some(self.timerange.clone());
            }

            None => {}
        }

        mutation.save_topic(&self.topic_id.repo, &self.topic_id, &topic)?;
        mutation.add_change(&self.change(&topic, previous_timerange))?;
        mutation.write(store)?;

        Ok(UpsertTopicTimerangeResult {
            alerts: vec![],
            topic,
            timerange: self.timerange.clone(),
        })
    }

    fn change(&self, topic: &Topic, previous_timerange: Option<Timerange>) -> activity::Change {
        let mut parent_topics = BTreeSet::new();
        for parent in &topic.parent_topics {
            parent_topics.insert(parent.path.to_owned());
        }

        activity::Change::UpsertTopicTimerange(activity::UpsertTopicTimerange {
            actor_id: self.actor.user_id.to_owned(),
            date: chrono::Utc::now(),
            id: activity::Change::new_id(),
            parent_topics: parent_topics.iter().map(|path| path.to_owned()).collect(),
            previous_timerange,
            updated_topic: activity::TopicInfo::from(topic),
            updated_timerange: self.timerange.clone(),
        })
    }
}
