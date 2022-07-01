use rand::{distributions::Alphanumeric, Rng};
use std::collections::BTreeSet;

use super::{Indexer, ParentTopic, SynonymEntry, SynonymMatch, TopicMetadata, API_VERSION};
use crate::git::{Git, IndexMode, Kind, Synonym, Topic};
use crate::prelude::*;
use crate::Alert;

pub struct UpdateTopicParentTopics {
    pub actor: Viewer,
    pub parent_topics: BTreeSet<RepoPath>,
    pub topic: RepoPath,
}

pub struct UpdateTopicParentTopicsResult {
    pub alerts: Vec<Alert>,
    pub topic: Topic,
}

impl UpdateTopicParentTopics {
    pub fn call(&self, git: &Git) -> Result<UpdateTopicParentTopicsResult> {
        self.validate(git)?;

        let now = chrono::Utc::now();
        let mut indexer = Indexer::new(git, IndexMode::Update);
        let mut child = git.fetch_topic(&self.topic.inner)?;
        let mut updates: Vec<Topic> = vec![];
        let parent_topics = self
            .parent_topics
            .iter()
            .map(|path| ParentTopic {
                path: path.to_string(),
            })
            .collect::<BTreeSet<ParentTopic>>();

        let added = parent_topics.difference(&child.parent_topics);
        for parent in &added.cloned().collect::<Vec<ParentTopic>>() {
            let mut topic = parent.fetch(git)?;
            topic.children.insert(child.to_topic_child(now));
            child.parent_topics.insert(parent.to_owned());
            updates.push(topic);
        }

        let deleted = child.parent_topics.difference(&parent_topics);
        for parent in &deleted.cloned().collect::<Vec<ParentTopic>>() {
            let mut topic = parent.fetch(git)?;
            topic.children.remove(&child.to_topic_child(now));
            child.parent_topics.remove(parent);
            updates.push(topic);
        }

        updates.push(child.clone());

        for topic in updates {
            git.save_topic(&topic.path(), &topic, &mut indexer)?;
        }

        indexer.save()?;

        Ok(UpdateTopicParentTopicsResult {
            alerts: vec![],
            topic: child,
        })
    }

    fn validate(&self, git: &Git) -> Result<()> {
        if self.parent_topics.is_empty() {
            return Err(Error::Repo(
                "at least one parent topic must be provided".into(),
            ));
        }

        for parent in &self.parent_topics {
            if git.cycle_exists(&self.topic, parent)? {
                return Err(Error::Repo(format!(
                    "{} is a parent topic of {}",
                    self.topic, parent
                )));
            }
        }

        Ok(())
    }
}

pub enum OnMatchingSynonym {
    Ask,
    CreateDistinct,
    Update(RepoPath),
}

pub struct UpsertTopic {
    pub actor: Viewer,
    pub locale: String,
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
    pub fn call(&self, git: &Git) -> Result<UpsertTopicResult> {
        let mut parent = self.fetch_parent(git)?;
        let mut matches = git.synonym_matches(&self.prefix, &self.name)?;
        let added = chrono::Utc::now();

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
                            &parent.name("en"),
                            &ancestor.name("en"),
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
                        added,
                        locale: self.locale.to_owned(),
                        name: self.name.to_owned(),
                    });

                    (path.clone(), topic)
                }
            }
        };

        parent.children.insert(child.to_topic_child(added));

        let mut indexer = Indexer::new(git, IndexMode::Update);
        git.save_topic(&path, &child, &mut indexer)?;
        git.save_topic(&parent.path(), &parent, &mut indexer)?;
        indexer.save()?;

        Ok(UpsertTopicResult {
            alerts: vec![],
            matching_synonyms: BTreeSet::new(),
            saved: true,
            topic: Some(child),
        })
    }

    fn make_path(&self) -> RepoPath {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();
        RepoPath::from(&format!("{}/{}", self.prefix, s))
    }

    fn make_topic(&self, parent: &Topic) -> (RepoPath, Topic) {
        let added = chrono::Utc::now();
        let path = self.make_path();

        let topic = Topic {
            api_version: API_VERSION.into(),
            kind: Kind::Topic,
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
