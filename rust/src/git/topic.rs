use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;

use super::{
    activity, Kind, Mutation, ParentTopic, RepoLink, RepoObject, RepoTopic, RepoTopicDetails,
    RepoTopicMetadata, RepoTopicWrapper, SaveChangesForPrefix, Synonym, SynonymEntry, SynonymMatch,
    TopicChild,
};
use crate::prelude::*;

fn normalize_name(name: &str) -> String {
    name.split_whitespace().join(" ")
}

pub struct DeleteTopic {
    pub actor: Viewer,
    pub repo: RepoId,
    pub topic_id: Oid,
}

pub struct DeleteTopicResult {
    pub alerts: Vec<Alert>,
    pub deleted_topic_id: Oid,
}

impl DeleteTopic {
    pub fn call<S>(&self, mut mutation: Mutation, store: &S) -> Result<DeleteTopicResult>
    where
        S: SaveChangesForPrefix,
    {
        let topic_id = &self.topic_id;
        let added = chrono::Utc::now();

        let topic = mutation.fetch_topic(&self.repo, topic_id);
        if topic.is_none() {
            return Err(Error::NotFound(format!("not found: {}", topic_id)));
        }
        let topic = topic.unwrap();

        if topic.root() {
            return Err(Error::Repo("cannot delete root topic".to_owned()));
        }

        mutation.mark_deleted(&self.repo, topic_id)?;

        let parent_topics = topic
            .parent_topics
            .iter()
            .map(|parent| ParentTopic {
                id: parent.id.to_owned(),
            })
            .collect::<BTreeSet<ParentTopic>>();

        let mut topics = vec![];

        let mut child_links = vec![];
        let mut child_topics = vec![];

        // Remove the topic from its children
        for child in &topic.children {
            match mutation.fetch(&self.repo, &child.id) {
                Some(RepoObject::Link(child_link)) => {
                    let mut child_link = child_link.to_owned();
                    child_link.parent_topics.remove(&topic.to_parent_topic());
                    child_link.parent_topics.append(&mut parent_topics.clone());
                    child_links.push(child_link.clone());
                    mutation.save_link(&self.repo, &child_link)?;
                }

                Some(RepoObject::Topic(child_topic)) => {
                    let mut child_topic = child_topic.to_owned();
                    child_topic.parent_topics.remove(&topic.to_parent_topic());
                    child_topic.parent_topics.append(&mut parent_topics.clone());
                    child_topics.push(child_topic.clone());
                    mutation.save_topic(&self.repo, &child_topic)?;
                }

                None => {}
            }
        }

        for parent in &topic.parent_topics {
            if let Some(mut parent) = mutation.fetch_topic(&self.repo, &parent.id) {
                // Remove the topic from the children of the parent topics
                parent.children.remove(&TopicChild {
                    // The 'added' field is ignored
                    added,
                    kind: Kind::Topic,
                    id: topic_id.to_owned(),
                });

                // Move the child topics to the parent topics
                for child in &child_topics {
                    parent.children.insert(child.to_topic_child(added));
                }

                // Move the child links to the parent topics
                for child in &child_links {
                    parent.children.insert(child.to_topic_child(added));
                }

                mutation.save_topic(&self.repo, &parent)?;
                topics.push(parent.clone());
            }
        }

        let change = self.change(&topic, &topics, &child_links, &child_topics, added);

        mutation.remove_topic(&self.repo, &self.topic_id, &topic)?;
        mutation.add_change(&self.repo, &change)?;
        mutation.write(store)?;

        Ok(DeleteTopicResult {
            alerts: vec![],
            deleted_topic_id: self.topic_id.to_owned(),
        })
    }

    fn change(
        &self,
        topic: &RepoTopic,
        parent_topics: &Vec<RepoTopic>,
        child_links: &Vec<RepoLink>,
        child_topics: &Vec<RepoTopic>,
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
    pub repo_id: RepoId,
    pub topic_id: Oid,
}

pub struct RemoveTopicTimerangeResult {
    pub alerts: Vec<Alert>,
    pub repo_topic: RepoTopicWrapper,
}

impl RemoveTopicTimerange {
    pub fn call<S>(&self, mut mutation: Mutation, store: &S) -> Result<RemoveTopicTimerangeResult>
    where
        S: SaveChangesForPrefix,
    {
        let topic = mutation.fetch_topic(&self.repo_id, &self.topic_id);
        if topic.is_none() {
            return Err(Error::NotFound(format!("not found: {}", self.topic_id)));
        }
        let mut repo_topic = topic.unwrap();

        let previous_timerange = repo_topic.timerange().to_owned();

        match &mut repo_topic.metadata.details {
            Some(details) => {
                details.timerange = None;
            }

            None => {}
        }

        mutation.save_topic(&self.repo_id, &repo_topic)?;
        mutation.add_change(&self.repo_id, &self.change(&repo_topic, previous_timerange))?;
        mutation.write(store)?;

        Ok(RemoveTopicTimerangeResult {
            alerts: vec![],
            repo_topic: RepoTopicWrapper {
                repo_topic,
                repo_id: self.repo_id.to_owned(),
            },
        })
    }

    pub fn change(
        &self,
        topic: &RepoTopic,
        previous_timerange: Option<Timerange>,
    ) -> activity::Change {
        let mut parent_topics = BTreeSet::new();
        for parent in &topic.parent_topics {
            parent_topics.insert(parent.id.to_owned());
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
    pub parent_topic_ids: BTreeSet<Oid>,
    pub repo_id: RepoId,
    pub topic_id: Oid,
}

pub struct UpdateTopicParentTopicsResult {
    pub alerts: Vec<Alert>,
    pub repo_topic: RepoTopic,
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
        let child = mutation.fetch_topic(&self.repo_id, &self.topic_id);
        if child.is_none() {
            return Err(Error::NotFound(format!("not found: {}", self.topic_id)));
        }
        let mut child = child.unwrap();

        let mut updates: Vec<RepoTopic> = vec![];

        let parent_topics = self
            .parent_topic_ids
            .iter()
            .map(|id| ParentTopic { id: id.to_owned() })
            .collect::<BTreeSet<ParentTopic>>();

        let added = parent_topics
            .difference(&child.parent_topics)
            .cloned()
            .collect::<Vec<ParentTopic>>();
        let mut added_topics = vec![];

        for parent in added {
            if let Some(mut topic) = parent.fetch(&self.repo_id, &mutation)? {
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
            if let Some(mut topic) = parent.fetch(&self.repo_id, &mutation)? {
                topic.children.remove(&child.to_topic_child(date));
                child.parent_topics.remove(&parent);
                removed_topics.push(topic.clone());
                updates.push(topic);
            }
        }

        let change = self.change(&child, &parent_topics, &added_topics, &removed_topics, date);

        updates.push(child.clone());
        for topic in updates {
            mutation.save_topic(&self.repo_id, &topic)?;
        }
        mutation.add_change(&self.repo_id, &change)?;
        mutation.write(store)?;

        Ok(UpdateTopicParentTopicsResult {
            alerts: vec![],
            repo_topic: child,
        })
    }

    fn change(
        &self,
        topic: &RepoTopic,
        parent_topics: &BTreeSet<ParentTopic>,
        added: &Vec<RepoTopic>,
        removed: &Vec<RepoTopic>,
        date: Timestamp,
    ) -> activity::Change {
        activity::Change::UpdateTopicParentTopics(activity::UpdateTopicParentTopics {
            actor_id: self.actor.user_id.to_owned(),
            added_parent_topics: activity::TopicInfoList::from(added),
            date,
            id: activity::Change::new_id(),
            parent_topic_ids: parent_topics
                .iter()
                .map(|parent| parent.id.to_owned())
                .collect::<BTreeSet<Oid>>(),
            removed_parent_topics: activity::TopicInfoList::from(removed),
            updated_topic: activity::TopicInfo::from(topic),
        })
    }

    fn validate(&self, mutation: &Mutation) -> Result<()> {
        if self.parent_topic_ids.is_empty() {
            return Err(Error::Repo(
                "at least one parent topic must be provided".into(),
            ));
        }

        for parent in &self.parent_topic_ids {
            if mutation.cycle_exists(&self.repo_id, &self.topic_id, parent)? {
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
    pub repo_id: RepoId,
    pub synonyms: Vec<Synonym>,
    pub topic_id: Oid,
}

pub struct UpdateTopicSynonymsResult {
    pub alerts: Vec<Alert>,
    pub repo_topic: RepoTopicWrapper,
}

impl UpdateTopicSynonyms {
    pub fn call<S>(&self, mut mutation: Mutation, store: &S) -> Result<UpdateTopicSynonymsResult>
    where
        S: SaveChangesForPrefix,
    {
        log::info!(
            "updating synonyms for {} within {}",
            self.topic_id,
            self.repo_id
        );

        let topic = mutation.fetch_topic(&self.repo_id, &self.topic_id);
        if topic.is_none() {
            return Err(Error::NotFound(format!("not found: {}", self.topic_id)));
        }
        let mut repo_topic = topic.unwrap();

        let lookup = repo_topic
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
        let mut alerts = vec![];

        // Preserve the date the synonym was added
        for new in &self.synonyms {
            let name = normalize_name(&new.name);
            if name.is_empty() {
                continue;
            }

            let new = Synonym {
                added: new.added,
                locale: new.locale,
                name: name.to_owned(),
            };

            let key = (new.name.to_owned(), new.locale.to_owned());

            // Remove duplicates
            if after.contains(&key) {
                alerts.push(Alert::Success(format!(
                    "Synonym already exists: {}",
                    new.name
                )));
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

        if synonyms.is_empty() {
            return Err(Error::Repo("at least one synonym is required".to_owned()));
        }

        let added = after
            .difference(&before)
            .cloned()
            .collect::<HashSet<(String, Locale)>>();
        let removed = before
            .difference(&after)
            .cloned()
            .collect::<HashSet<(String, Locale)>>();

        match &mut repo_topic.metadata.details {
            Some(details) => {
                details.synonyms = synonyms;
            }

            None => {
                repo_topic.metadata.details = Some(RepoTopicDetails {
                    root: false,
                    synonyms,
                    timerange: None,
                })
            }
        }

        mutation.save_topic(&self.repo_id, &repo_topic)?;
        mutation.add_change(&self.repo_id, &self.change(&repo_topic, &added, &removed))?;
        mutation.write(store)?;

        Ok(UpdateTopicSynonymsResult {
            alerts,
            repo_topic: RepoTopicWrapper {
                repo_topic,
                repo_id: self.repo_id.to_owned(),
            },
        })
    }

    fn change(
        &self,
        topic: &RepoTopic,
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
                .map(|parent| parent.id.to_owned())
                .collect::<BTreeSet<Oid>>(),
            reordered: added.is_empty() && removed.is_empty(),
            removed_synonyms: activity::SynonymList::from(removed),
            updated_topic: activity::TopicInfo::from(topic),
        })
    }
}

pub enum OnMatchingSynonym {
    Ask,
    CreateDistinct,
    Update(Oid),
}

pub struct UpsertTopic {
    pub actor: Viewer,
    pub locale: Locale,
    pub name: String,
    pub on_matching_synonym: OnMatchingSynonym,
    pub parent_topic_id: Oid,
    pub repo_id: RepoId,
}

pub struct UpsertTopicResult {
    pub alerts: Vec<Alert>,
    pub matching_repo_topics: BTreeSet<SynonymMatch>,
    pub repo_topic: Option<RepoTopic>,
    pub saved: bool,
}

impl UpsertTopic {
    pub fn call<S>(&self, mutation: Mutation, store: &S) -> Result<UpsertTopicResult>
    where
        S: SaveChangesForPrefix,
    {
        let name = normalize_name(&self.name);
        let parent = self.ensure_topic(&mutation, &self.parent_topic_id);
        let matches = self.find_matches(&mutation, &name)?;

        if matches.is_empty() {
            return self.add_repo_topic(mutation, store, name, parent, matches);
        }

        match &self.on_matching_synonym {
            OnMatchingSynonym::Ask => self.request_decision(mutation, parent, matches),

            OnMatchingSynonym::CreateDistinct => {
                self.add_repo_topic(mutation, store, name, parent, matches)
            }

            OnMatchingSynonym::Update(topic_id) => {
                self.update_repo_topic(mutation, store, name, topic_id, parent, matches)
            }
        }
    }

    fn find_matches(&self, mutation: &Mutation, name: &String) -> Result<BTreeSet<SynonymMatch>> {
        mutation.synonym_phrase_matches(&self.actor.read_repo_ids, name)
    }

    fn request_decision(
        &self,
        mutation: Mutation,
        parent: RepoTopic,
        matches: BTreeSet<SynonymMatch>,
    ) -> Result<UpsertTopicResult> {
        log::info!("topic '{}' exists and OnMatchingSynonym::Ask", self.name);

        let alert = Alert::Success(format!(
            r#"One or more existing topics with a synonym of "{}" were found. Would you like to
            add one of the matching topics to the parent topic we're looking at, or would you like
            to create a new subtopic with the same name?"#,
            self.name
        ));

        let parent_id = &parent.topic_id();
        let mut matching_repo_topics: BTreeSet<SynonymMatch> = BTreeSet::new();

        for synonym_match in matches {
            let topic_id = &synonym_match.repo_topic.topic_id();
            if mutation.cycle_exists(&self.repo_id, topic_id, parent_id)? {
                matching_repo_topics.insert(synonym_match.with_cycle(true));
            } else {
                matching_repo_topics.insert(synonym_match);
            }
        }

        Ok(UpsertTopicResult {
            alerts: vec![alert],
            matching_repo_topics,
            repo_topic: None,
            saved: false,
        })
    }

    fn add_repo_topic<S>(
        &self,
        mutation: Mutation,
        store: &S,
        name: String,
        parent: RepoTopic,
        matches: BTreeSet<SynonymMatch>,
    ) -> Result<UpsertTopicResult>
    where
        S: SaveChangesForPrefix,
    {
        if !matches.is_empty() {
            log::info!(
                "creating new repo topic even though there are matching synonyms: {:?}",
                matches
            );
        }

        let (child, parent_topics) = self.make_topic(&parent, name)?;
        self.persist_repo_topic(mutation, store, child, parent, parent_topics)
    }

    fn persist_repo_topic<S>(
        &self,
        mut mutation: Mutation,
        store: &S,
        child: RepoTopic,
        mut parent: RepoTopic,
        parent_topics: BTreeSet<ParentTopic>,
    ) -> Result<UpsertTopicResult>
    where
        S: SaveChangesForPrefix,
    {
        let date = chrono::Utc::now();
        parent.children.insert(child.to_topic_child(date));

        let change = self.change(&child, &parent_topics, &parent, date);
        mutation.save_topic(&self.repo_id, &child)?;
        mutation.save_topic(&self.repo_id, &parent)?;
        mutation.add_change(&self.repo_id, &change)?;
        mutation.write(store)?;

        Ok(UpsertTopicResult {
            alerts: vec![],
            matching_repo_topics: BTreeSet::new(),
            repo_topic: Some(child),
            saved: true,
        })
    }

    fn update_repo_topic<S>(
        &self,
        mutation: Mutation,
        store: &S,
        name: String,
        topic_id: &Oid,
        parent: RepoTopic,
        matches: BTreeSet<SynonymMatch>,
    ) -> Result<UpsertTopicResult>
    where
        S: SaveChangesForPrefix,
    {
        let date = chrono::Utc::now();

        if mutation.cycle_exists(&self.repo_id, topic_id, parent.topic_id())? {
            return self.handle_cycle(mutation, topic_id, parent, matches);
        }

        log::info!("updating existing topic {:?}", topic_id);
        let mut topic = self.ensure_topic(&mutation, topic_id);
        let parent_topics = topic.parent_topics.clone();

        topic.parent_topics.insert(parent.to_parent_topic());

        match &mut topic.metadata.details {
            Some(details) => {
                details.synonyms.push(Synonym {
                    added: date,
                    locale: self.locale,
                    name,
                });
            }

            None => {}
        }

        self.persist_repo_topic(mutation, store, topic, parent, parent_topics)
    }

    fn handle_cycle(
        &self,
        mutation: Mutation,
        topic_id: &Oid,
        parent: RepoTopic,
        mut matches: BTreeSet<SynonymMatch>,
    ) -> Result<UpsertTopicResult> {
        let ancestor = mutation.fetch_topic(&self.repo_id, topic_id);
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
                id: topic_id.to_owned(),
            },
            name: self.name.to_owned(),
            repo_id: self.repo_id.to_owned(),
            repo_topic: ancestor,
        });

        Ok(UpsertTopicResult {
            alerts,
            matching_repo_topics: matches,
            repo_topic: None,
            saved: false,
        })
    }

    fn change(
        &self,
        topic: &RepoTopic,
        parent_topics: &BTreeSet<ParentTopic>,
        parent: &RepoTopic,
        date: Timestamp,
    ) -> activity::Change {
        activity::Change::UpsertTopic(activity::UpsertTopic {
            actor_id: self.actor.user_id.to_owned(),
            id: activity::Change::new_id(),
            date,
            parent_topic: activity::TopicInfo::from(parent),
            parent_topic_ids: parent_topics
                .iter()
                .map(|parent| parent.id.to_owned())
                .collect::<BTreeSet<Oid>>(),
            upserted_topic: activity::TopicInfo::from(topic),
        })
    }

    fn make_topic(
        &self,
        parent: &RepoTopic,
        name: String,
    ) -> Result<(RepoTopic, BTreeSet<ParentTopic>)> {
        let added = chrono::Utc::now();
        let id = Oid::make();
        let parent_topics = BTreeSet::from([parent.to_parent_topic()]);

        let topic = RepoTopic {
            api_version: API_VERSION.into(),
            metadata: RepoTopicMetadata {
                added,
                id,
                details: Some(RepoTopicDetails {
                    root: false,
                    synonyms: vec![Synonym {
                        added,
                        locale: self.locale.to_owned(),
                        name,
                    }],
                    timerange: None,
                }),
            },
            parent_topics: parent_topics.clone(),
            children: BTreeSet::new(),
        };

        Ok((topic, parent_topics))
    }

    fn ensure_topic(&self, mutation: &Mutation, topic_id: &Oid) -> RepoTopic {
        match mutation.fetch_topic(&self.repo_id, topic_id) {
            Some(topic) => topic,

            // If the topic is being upserted into another repo, we create a reference to the
            // parent topic in the current repo
            None => {
                log::info!(
                    "no topic found in selected repo, creating reference: {}",
                    topic_id
                );
                RepoTopic::make_reference(topic_id.to_owned())
            }
        }
    }
}

pub struct UpsertTopicTimerange {
    pub actor: Viewer,
    pub repo_id: RepoId,
    pub timerange: Timerange,
    pub topic_id: Oid,
}

pub struct UpsertTopicTimerangeResult {
    pub alerts: Vec<Alert>,
    pub timerange: Timerange,
    pub updated_repo_topic: RepoTopicWrapper,
}

impl UpsertTopicTimerange {
    pub fn call<S>(&self, mut mutation: Mutation, store: &S) -> Result<UpsertTopicTimerangeResult>
    where
        S: SaveChangesForPrefix,
    {
        let repo_topic = mutation.fetch_topic(&self.repo_id, &self.topic_id);
        if repo_topic.is_none() {
            return Err(Error::NotFound(format!("not found: {}", self.topic_id)));
        }
        let mut repo_topic = repo_topic.unwrap();

        let previous_timerange = repo_topic.timerange().clone();

        match &mut repo_topic.metadata.details {
            Some(details) => {
                details.timerange = Some(self.timerange.clone());
            }

            None => {}
        }

        mutation.save_topic(&self.repo_id, &repo_topic)?;
        mutation.add_change(&self.repo_id, &self.change(&repo_topic, previous_timerange))?;
        mutation.write(store)?;

        Ok(UpsertTopicTimerangeResult {
            alerts: vec![],
            timerange: self.timerange.clone(),
            updated_repo_topic: RepoTopicWrapper {
                repo_topic,
                repo_id: self.repo_id.to_owned(),
            },
        })
    }

    fn change(&self, topic: &RepoTopic, previous_timerange: Option<Timerange>) -> activity::Change {
        let mut parent_topics = BTreeSet::new();
        for parent in &topic.parent_topics {
            parent_topics.insert(parent.id.to_owned());
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
