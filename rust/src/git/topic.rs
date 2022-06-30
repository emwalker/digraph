use rand::{distributions::Alphanumeric, Rng};
use std::collections::BTreeSet;

use super::{Indexer, ParentTopic, SynonymMatch, TopicMetadata, API_VERSION};
use crate::git::{Git, IndexMode, Kind, Synonym, Topic, TopicChild};
use crate::prelude::*;

pub struct UpsertTopicResult {
    pub alerts: Vec<String>,
    pub matching_synonyms: Vec<SynonymMatch>,
    pub saved: bool,
    pub topic: Option<Topic>,
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

impl UpsertTopic {
    pub fn call(&self, git: &Git) -> Result<UpsertTopicResult> {
        let mut parent = self.fetch_parent(git)?;
        let matches = git.synonym_matches(&self.prefix, &self.name)?;
        let added = chrono::Utc::now();

        let (path, topic) = if matches.is_empty() {
            self.make_topic(&parent)
        } else {
            match &self.on_matching_synonym {
                OnMatchingSynonym::Ask => {
                    return Ok(UpsertTopicResult {
                        alerts: vec![],
                        matching_synonyms: matches,
                        saved: false,
                        topic: None,
                    })
                }

                OnMatchingSynonym::CreateDistinct => {
                    log::info!(
                        "creating new topic even though there are matching synonyms: {:?}",
                        matches
                    );
                    self.make_topic(&parent)
                }

                OnMatchingSynonym::Update(path) => {
                    log::info!("updating existing topic {:?}", path);
                    let mut topic = git.fetch_topic(&path.inner)?;
                    topic.parent_topics.insert(ParentTopic {
                        path: parent.metadata.path.to_owned(),
                    });

                    topic.metadata.synonyms.push(Synonym {
                        added,
                        locale: self.locale.to_owned(),
                        name: self.name.to_owned(),
                    });

                    (path.clone(), topic)
                }
            }
        };

        parent.children.insert(TopicChild {
            added,
            kind: Kind::Topic,
            path: path.inner.to_owned(),
        });
        let parent_path = RepoPath::from(&parent.metadata.path);

        let mut indexer = Indexer::new(git, IndexMode::Update);
        git.save_topic(&path, &topic, &mut indexer)?;
        git.save_topic(&parent_path, &parent, &mut indexer)?;
        indexer.save()?;

        Ok(UpsertTopicResult {
            alerts: vec![],
            matching_synonyms: vec![],
            saved: true,
            topic: Some(topic),
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
            parent_topics: BTreeSet::from([ParentTopic {
                path: parent.metadata.path.to_owned(),
            }]),
            children: BTreeSet::new(),
        };

        (path, topic)
    }

    fn fetch_parent(&self, git: &Git) -> Result<Topic> {
        let path = &self.parent_topic.inner;
        git.fetch_topic(path)
    }
}
