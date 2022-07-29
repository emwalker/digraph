use std::collections::{BTreeSet, HashMap, HashSet};

use super::{
    activity, BatchUpdate, Client, Kind, Link, Object, ParentTopic, SaveChangesForPrefix, Synonym,
    SynonymEntry, SynonymMatch, Timerange, Topic, TopicChild, TopicMetadata, API_VERSION,
};
use crate::prelude::*;

pub struct DeleteTopic {
    pub actor: Viewer,
    pub topic_path: RepoPath,
}

pub struct DeleteTopicResult {
    pub alerts: Vec<Alert>,
    pub deleted_topic_path: RepoPath,
}

impl DeleteTopic {
    pub fn call<S>(&self, mut builder: BatchUpdate, store: &S) -> Result<DeleteTopicResult>
    where
        S: SaveChangesForPrefix,
    {
        let topic_path = &self.topic_path;
        let date = chrono::Utc::now();

        let topic = builder.fetch_topic(topic_path);
        if topic.is_none() {
            return Err(Error::NotFound(format!("not found: {}", topic_path)));
        }
        let topic = topic.unwrap();

        if topic.metadata.root {
            return Err(Error::Repo("cannot delete root topic".to_owned()));
        }

        builder.mark_deleted(topic_path)?;

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
            let path = RepoPath::from(&parent.path);
            if let Some(mut topic) = builder.fetch_topic(&path) {
                topic.children.remove(&TopicChild {
                    // The 'added' field is ignored
                    added: chrono::Utc::now(),
                    kind: Kind::Topic,
                    path: topic_path.inner.to_owned(),
                });
                builder.save_topic(&path, &topic)?;
                topics.push(topic.clone());
            }
        }

        let mut child_links = vec![];
        let mut child_topics = vec![];

        // Remove the topic from its children, moving them onto the parent topics
        for child in &topic.children {
            let path = RepoPath::from(&child.path);
            match builder.fetch(&path) {
                Some(Object::Link(child_link)) => {
                    let mut link = child_link.to_owned();
                    link.parent_topics.remove(&topic.to_parent_topic());
                    link.parent_topics.append(&mut parent_topics.clone());
                    child_links.push(link.clone());
                    builder.save_link(&path, &link)?;
                }

                Some(Object::Topic(child_topic)) => {
                    let mut child_topic = child_topic.to_owned();
                    child_topic.parent_topics.remove(&topic.to_parent_topic());
                    child_topic.parent_topics.append(&mut parent_topics.clone());
                    child_topics.push(child_topic.clone());
                    builder.save_topic(&path, &child_topic)?;
                }

                None => {}
            }
        }

        let change = self.change(&topic, &topics, &child_links, &child_topics, date);

        builder.remove_topic(&self.topic_path, &topic)?;
        builder.add_change(&change)?;
        builder.write(store)?;

        Ok(DeleteTopicResult {
            alerts: vec![],
            deleted_topic_path: self.topic_path.clone(),
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

pub struct FetchTopicCount {
    pub actor: Viewer,
}

pub struct FetchTopicCountResult {
    pub count: usize,
}

impl FetchTopicCount {
    pub fn call(&self, _git: &Client) -> Result<FetchTopicCountResult> {
        Ok(FetchTopicCountResult { count: 100 })
    }
}

pub struct RemoveTopicTimerange {
    pub actor: Viewer,
    pub topic_path: RepoPath,
}

pub struct RemoveTopicTimerangeResult {
    pub alerts: Vec<Alert>,
    pub topic: Topic,
}

impl RemoveTopicTimerange {
    pub fn call<S>(&self, mut builder: BatchUpdate, store: &S) -> Result<RemoveTopicTimerangeResult>
    where
        S: SaveChangesForPrefix,
    {
        let topic = builder.fetch_topic(&self.topic_path);
        if topic.is_none() {
            return Err(Error::NotFound(format!("not found: {}", self.topic_path)));
        }
        let mut topic = topic.unwrap();

        let previous_timerange = topic.metadata.timerange.clone();

        topic.metadata.timerange = None;
        builder.save_topic(&self.topic_path, &topic)?;
        builder.add_change(&self.change(&topic, previous_timerange))?;
        builder.write(store)?;

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
    pub parent_topic_paths: BTreeSet<RepoPath>,
    pub topic_path: RepoPath,
}

pub struct UpdateTopicParentTopicsResult {
    pub alerts: Vec<Alert>,
    pub topic: Topic,
}

impl UpdateTopicParentTopics {
    pub fn call<S>(
        &self,
        mut builder: BatchUpdate,
        store: &S,
    ) -> Result<UpdateTopicParentTopicsResult>
    where
        S: SaveChangesForPrefix,
    {
        self.validate(&builder)?;

        let date = chrono::Utc::now();
        let child = builder.fetch_topic(&self.topic_path);
        if child.is_none() {
            return Err(Error::NotFound(format!("not found: {}", self.topic_path)));
        }
        let mut child = child.unwrap();

        let mut updates: Vec<Topic> = vec![];

        let parent_topics = self
            .parent_topic_paths
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
            if let Some(mut topic) = parent.fetch(&builder) {
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
            if let Some(mut topic) = parent.fetch(&builder) {
                topic.children.remove(&child.to_topic_child(date));
                child.parent_topics.remove(&parent);
                removed_topics.push(topic.clone());
                updates.push(topic);
            }
        }

        let change = self.change(&child, &parent_topics, &added_topics, &removed_topics, date);

        updates.push(child.clone());
        for topic in updates {
            builder.save_topic(&topic.path(), &topic)?;
        }
        builder.add_change(&change)?;
        builder.write(store)?;

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

    fn validate(&self, builder: &BatchUpdate) -> Result<()> {
        if self.parent_topic_paths.is_empty() {
            return Err(Error::Repo(
                "at least one parent topic must be provided".into(),
            ));
        }

        for parent in &self.parent_topic_paths {
            if builder.cycle_exists(&self.topic_path, parent)? {
                return Err(Error::Repo(format!(
                    "{} is a parent topic of {}",
                    self.topic_path, parent
                )));
            }
        }

        Ok(())
    }
}

pub struct UpdateTopicSynonyms {
    pub actor: Viewer,
    pub synonyms: Vec<Synonym>,
    pub topic_path: RepoPath,
}

pub struct UpdateTopicSynonymsResult {
    pub alerts: Vec<Alert>,
    pub topic: Topic,
}

impl UpdateTopicSynonyms {
    pub fn call<S>(&self, mut builder: BatchUpdate, store: &S) -> Result<UpdateTopicSynonymsResult>
    where
        S: SaveChangesForPrefix,
    {
        let topic = builder.fetch_topic(&self.topic_path);
        if topic.is_none() {
            return Err(Error::NotFound(format!("not found: {}", self.topic_path)));
        }
        let mut topic = topic.unwrap();

        let lookup = topic
            .metadata
            .synonyms
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

        topic.metadata.synonyms = synonyms;
        builder.save_topic(&self.topic_path, &topic)?;
        builder.add_change(&self.change(&topic, &added, &removed))?;
        builder.write(store)?;

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
    pub repo: RepoPrefix,
}

pub struct UpsertTopicResult {
    pub alerts: Vec<Alert>,
    pub matching_synonyms: BTreeSet<SynonymMatch>,
    pub saved: bool,
    pub topic: Option<Topic>,
}

impl UpsertTopic {
    pub fn call<S>(&self, mut builder: BatchUpdate, store: &S) -> Result<UpsertTopicResult>
    where
        S: SaveChangesForPrefix,
    {
        let parent = self.fetch_parent(&builder);
        if parent.is_none() {
            return Err(Error::NotFound(format!("not found: {}", self.parent_topic)));
        }
        let mut parent = parent.unwrap();

        let mut matches = builder.synonym_phrase_matches(&[&self.repo], &self.name)?;
        let date = chrono::Utc::now();

        let (path, child, parent_topics) = if matches.is_empty() {
            self.make_topic(&parent)
        } else {
            match &self.on_matching_synonym {
                OnMatchingSynonym::Ask => {
                    log::info!("topic '{}' exists and OnMatchingSynonym::Ask", self.name);
                    let alert = Alert::Warning(format!(
                        r#"A topic with the name '{}' already exists. Would you like to update it
                        or create a new topic?"#,
                        self.name
                    ));

                    let parent_path = &parent.path();
                    let mut matching_synonyms: BTreeSet<SynonymMatch> = BTreeSet::new();

                    for synonym_match in matches {
                        let path = &synonym_match.topic.path();
                        if builder.cycle_exists(path, parent_path)? {
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
                    self.make_topic(&parent)
                }

                OnMatchingSynonym::Update(path) => {
                    if builder.cycle_exists(path, &parent.path())? {
                        let ancestor = builder.fetch_topic(path);
                        if ancestor.is_none() {
                            return Err(Error::NotFound(format!("not found: {}", path)));
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
                                path: path.inner.to_owned(),
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

                    log::info!("updating existing topic {:?}", path);
                    let topic = builder.fetch_topic(path);
                    if topic.is_none() {
                        return Err(Error::NotFound(format!("not found: {}", path)));
                    }
                    let mut topic = topic.unwrap();
                    let parent_topics = topic.parent_topics.clone();

                    topic.parent_topics.insert(parent.to_parent_topic());

                    topic.metadata.synonyms.push(Synonym {
                        added: date,
                        locale: self.locale,
                        name: self.name.to_owned(),
                    });

                    (path.clone(), topic, parent_topics)
                }
            }
        };

        parent.children.insert(child.to_topic_child(date));

        let change = self.change(&child, &parent_topics, &parent, date);
        builder.save_topic(&path, &child)?;
        builder.save_topic(&parent.path(), &parent)?;
        builder.add_change(&change)?;
        builder.write(store)?;

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

    fn make_topic(&self, parent: &Topic) -> (RepoPath, Topic, BTreeSet<ParentTopic>) {
        let added = chrono::Utc::now();
        let path = RepoPath::make(&self.repo.to_string());
        let parent_topics = BTreeSet::from([parent.to_parent_topic()]);

        let topic = Topic {
            api_version: API_VERSION.into(),
            metadata: TopicMetadata {
                added,
                path: path.to_string(),
                root: false,
                synonyms: vec![Synonym {
                    added,
                    locale: self.locale.to_owned(),
                    name: self.name.to_owned(),
                }],
                timerange: None,
            },
            parent_topics: parent_topics.clone(),
            children: BTreeSet::new(),
        };

        (path, topic, parent_topics)
    }

    fn fetch_parent(&self, builder: &BatchUpdate) -> Option<Topic> {
        builder.fetch_topic(&self.parent_topic)
    }
}

pub struct UpsertTopicTimerange {
    pub actor: Viewer,
    pub timerange: Timerange,
    pub topic_path: RepoPath,
}

pub struct UpsertTopicTimerangeResult {
    pub alerts: Vec<Alert>,
    pub timerange: Timerange,
    pub topic: Topic,
}

impl UpsertTopicTimerange {
    pub fn call<S>(&self, mut builder: BatchUpdate, store: &S) -> Result<UpsertTopicTimerangeResult>
    where
        S: SaveChangesForPrefix,
    {
        let topic = builder.fetch_topic(&self.topic_path);
        if topic.is_none() {
            return Err(Error::NotFound(format!("not found: {}", self.topic_path)));
        }
        let mut topic = topic.unwrap();

        let previous_timerange = topic.metadata.timerange.clone();

        topic.metadata.timerange = Some(self.timerange.clone());
        builder.save_topic(&self.topic_path, &topic)?;
        builder.add_change(&self.change(&topic, previous_timerange))?;
        builder.write(store)?;

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
