use std::collections::{BTreeSet, HashMap};

use super::{
    activity, Git, IndexMode, Indexer, Kind, Link, Object, ParentTopic, SaveChangesForPrefix,
    Synonym, SynonymEntry, SynonymMatch, Timerange, Topic, TopicChild, TopicMetadata, API_VERSION,
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
    pub fn call<S>(&self, git: &Git, store: &S) -> Result<DeleteTopicResult>
    where
        S: SaveChangesForPrefix,
    {
        let topic_path = &self.topic_path;
        let date = chrono::Utc::now();
        let topic = git.fetch_topic(&topic_path.inner)?;

        if topic.metadata.root {
            return Err(Error::Repo("cannot delete root topic".to_owned()));
        }

        git.mark_deleted(topic_path)?;

        let parent_topics = topic
            .parent_topics
            .iter()
            .map(|parent| ParentTopic {
                path: parent.path.to_owned(),
            })
            .collect::<BTreeSet<ParentTopic>>();

        let mut indexer = Indexer::new(git, IndexMode::Update);
        let mut topics = vec![];

        // Remove the topic from the children of the parent topics
        for parent in &topic.parent_topics {
            let mut parent = git.fetch_topic(&parent.path)?;
            parent.children.remove(&TopicChild {
                // The 'added' field is ignored
                added: chrono::Utc::now(),
                kind: Kind::Topic,
                path: topic_path.inner.to_owned(),
            });
            git.save_topic(&parent.path(), &parent, &mut indexer)?;
            topics.push(parent.clone());
        }

        let mut child_links = vec![];
        let mut child_topics = vec![];

        // Remove the topic from its children, moving them onto the parent topics
        for child in &topic.children {
            match git.fetch(&child.path)? {
                Object::Link(child_link) => {
                    let mut link = child_link.to_owned();
                    link.parent_topics.remove(&topic.to_parent_topic());
                    link.parent_topics.append(&mut parent_topics.clone());
                    child_links.push(link.clone());
                    git.save_link(&RepoPath::from(&child.path), &link, &mut indexer)?;
                }

                Object::Topic(child_topic) => {
                    let mut child_topic = child_topic.to_owned();
                    child_topic.parent_topics.remove(&topic.to_parent_topic());
                    child_topic.parent_topics.append(&mut parent_topics.clone());
                    child_topics.push(child_topic.clone());
                    git.save_topic(&RepoPath::from(&child.path), &child_topic, &mut indexer)?;
                }
            }
        }

        let change = self.change(&topic, &topics, &child_links, &child_topics, date);

        git.remove_topic(&self.topic_path, &topic, &mut indexer)?;
        indexer.add_change(&change)?;
        indexer.save(store)?;

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

pub struct RemoveTopicTimerange {
    pub actor: Viewer,
    pub topic_path: RepoPath,
}

pub struct RemoveTopicTimerangeResult {
    pub alerts: Vec<Alert>,
    pub topic: Topic,
}

impl RemoveTopicTimerange {
    pub fn call<S>(&self, git: &Git, store: &S) -> Result<RemoveTopicTimerangeResult>
    where
        S: SaveChangesForPrefix,
    {
        let mut topic = git.fetch_topic(&self.topic_path.inner)?;
        let previous_timerange = topic.metadata.timerange.clone();
        let mut indexer = Indexer::new(git, IndexMode::Update);

        topic.metadata.timerange = None;
        git.save_topic(&self.topic_path, &topic, &mut indexer)?;
        indexer.add_change(&self.change(&topic, previous_timerange))?;
        indexer.save(store)?;

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
            parent_topics,
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
    pub fn call<S>(&self, git: &Git, store: &S) -> Result<UpdateTopicParentTopicsResult>
    where
        S: SaveChangesForPrefix,
    {
        self.validate(git)?;

        let date = chrono::Utc::now();
        let mut indexer = Indexer::new(git, IndexMode::Update);
        let mut child = git.fetch_topic(&self.topic_path.inner)?;
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
            let mut topic = parent.fetch(git)?;
            topic.children.insert(child.to_topic_child(date));
            child.parent_topics.insert(parent.to_owned());
            added_topics.push(topic.clone());
            updates.push(topic);
        }

        let removed = child
            .parent_topics
            .difference(&parent_topics)
            .cloned()
            .collect::<Vec<ParentTopic>>();
        let mut removed_topics = vec![];

        for parent in removed {
            let mut topic = parent.fetch(git)?;
            topic.children.remove(&child.to_topic_child(date));
            child.parent_topics.remove(&parent);
            removed_topics.push(topic.clone());
            updates.push(topic);
        }

        let change = self.change(&child, &added_topics, &removed_topics, date);

        updates.push(child.clone());
        for topic in updates {
            git.save_topic(&topic.path(), &topic, &mut indexer)?;
        }
        indexer.add_change(&change)?;
        indexer.save(store)?;

        Ok(UpdateTopicParentTopicsResult {
            alerts: vec![],
            topic: child,
        })
    }

    fn change(
        &self,
        topic: &Topic,
        added: &Vec<Topic>,
        removed: &Vec<Topic>,
        date: Timestamp,
    ) -> activity::Change {
        activity::Change::UpdateTopicParentTopics(activity::UpdateTopicParentTopics {
            actor_id: self.actor.user_id.to_owned(),
            added_parent_topics: activity::TopicInfoList::from(added),
            id: activity::Change::new_id(),
            date,
            removed_parent_topics: activity::TopicInfoList::from(removed),
            updated_topic: activity::TopicInfo::from(topic),
        })
    }

    fn validate(&self, git: &Git) -> Result<()> {
        if self.parent_topic_paths.is_empty() {
            return Err(Error::Repo(
                "at least one parent topic must be provided".into(),
            ));
        }

        for parent in &self.parent_topic_paths {
            if git.cycle_exists(&self.topic_path, parent)? {
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
    pub fn call<S>(&self, git: &Git, store: &S) -> Result<UpdateTopicSynonymsResult>
    where
        S: SaveChangesForPrefix,
    {
        let mut topic = git.fetch_topic(&self.topic_path.inner)?;
        let mut indexer = Indexer::new(git, IndexMode::Update);
        let lookup = topic
            .metadata
            .synonyms
            .iter()
            .map(|synonym| ((&synonym.name, &synonym.locale), synonym.to_owned()))
            .collect::<HashMap<(&String, &Locale), Synonym>>();

        let mut synonyms = vec![];

        // Preserve the date the synonym was added
        for new in &self.synonyms {
            let key = (&new.name, &new.locale);
            if lookup.contains_key(&key) {
                match lookup.get(&key) {
                    Some(existing) => synonyms.push(existing.to_owned()),
                    None => synonyms.push(new.to_owned()),
                };
            } else {
                synonyms.push(new.to_owned());
            }
        }

        let added = vec![];
        let removed = vec![];

        topic.metadata.synonyms = synonyms;
        git.save_topic(&self.topic_path, &topic, &mut indexer)?;
        indexer.add_change(&self.change(&topic, &added, &removed))?;
        indexer.save(store)?;

        Ok(UpdateTopicSynonymsResult {
            alerts: vec![],
            topic,
        })
    }

    fn change(
        &self,
        topic: &Topic,
        added: &Vec<Synonym>,
        removed: &Vec<Synonym>,
    ) -> activity::Change {
        activity::Change::UpdateTopicSynonyms(activity::UpdateTopicSynonyms {
            actor_id: self.actor.user_id.to_owned(),
            added_synonyms: activity::SynonymList::from(added),
            id: activity::Change::new_id(),
            date: chrono::Utc::now(),
            updated_topic: activity::TopicInfo::from(topic),
            reordered: added.is_empty() && removed.is_empty(),
            removed_synonyms: activity::SynonymList::from(removed),
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
    pub prefix: String,
}

pub struct UpsertTopicResult {
    pub alerts: Vec<Alert>,
    pub matching_synonyms: BTreeSet<SynonymMatch>,
    pub saved: bool,
    pub topic: Option<Topic>,
}

impl UpsertTopic {
    pub fn call<S>(&self, git: &Git, store: &S) -> Result<UpsertTopicResult>
    where
        S: SaveChangesForPrefix,
    {
        let mut parent = self.fetch_parent(git)?;
        let mut matches = git.synonym_phrase_matches(&[&self.prefix], &self.name)?;
        let date = chrono::Utc::now();

        let (path, child) = if matches.is_empty() {
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
                        if git.cycle_exists(path, parent_path)? {
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
                    if git.cycle_exists(path, &parent.path())? {
                        let ancestor = git.fetch_topic(&path.inner)?;
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
                    let mut topic = git.fetch_topic(&path.inner)?;
                    topic.parent_topics.insert(parent.to_parent_topic());

                    topic.metadata.synonyms.push(Synonym {
                        added: date,
                        locale: self.locale,
                        name: self.name.to_owned(),
                    });

                    (path.clone(), topic)
                }
            }
        };

        parent.children.insert(child.to_topic_child(date));

        let change = self.change(&child, &parent, date);
        let mut indexer = Indexer::new(git, IndexMode::Update);
        git.save_topic(&path, &child, &mut indexer)?;
        git.save_topic(&parent.path(), &parent, &mut indexer)?;
        indexer.add_change(&change)?;
        indexer.save(store)?;

        Ok(UpsertTopicResult {
            alerts: vec![],
            matching_synonyms: BTreeSet::new(),
            saved: true,
            topic: Some(child),
        })
    }

    fn change(&self, topic: &Topic, parent: &Topic, date: Timestamp) -> activity::Change {
        activity::Change::UpsertTopic(activity::UpsertTopic {
            actor_id: self.actor.user_id.to_owned(),
            id: activity::Change::new_id(),
            date,
            parent_topic: activity::TopicInfo::from(parent),
            upserted_topic: activity::TopicInfo::from(topic),
        })
    }

    fn make_topic(&self, parent: &Topic) -> (RepoPath, Topic) {
        let added = chrono::Utc::now();
        let path = RepoPath::random(&self.prefix);

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
            parent_topics: BTreeSet::from([parent.to_parent_topic()]),
            children: BTreeSet::new(),
        };

        (path, topic)
    }

    fn fetch_parent(&self, git: &Git) -> Result<Topic> {
        let path = &self.parent_topic.inner;
        git.fetch_topic(path)
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
    pub fn call<S>(&self, git: &Git, store: &S) -> Result<UpsertTopicTimerangeResult>
    where
        S: SaveChangesForPrefix,
    {
        let mut topic = git.fetch_topic(&self.topic_path.inner)?;
        let previous_timerange = topic.metadata.timerange.clone();

        let mut indexer = Indexer::new(git, IndexMode::Update);
        topic.metadata.timerange = Some(self.timerange.clone());
        git.save_topic(&self.topic_path, &topic, &mut indexer)?;
        indexer.add_change(&self.change(&topic, previous_timerange))?;
        indexer.save(store)?;

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
            parent_topics,
            previous_timerange,
            updated_topic: activity::TopicInfo::from(topic),
            updated_timerange: self.timerange.clone(),
        })
    }
}
