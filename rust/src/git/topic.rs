use rand::{distributions::Alphanumeric, Rng};
use std::collections::BTreeSet;

use super::{Indexer, ParentTopic, SynonymMatch, TopicMetadata, API_VERSION};
use crate::git::{
    Git, IndexMode, Kind, Synonym, Timerange, TimerangePrefixFormat, Topic, TopicChild,
};
use crate::prelude::*;
use crate::schema;

impl From<&Synonym> for schema::Synonym {
    fn from(synonym: &Synonym) -> Self {
        Self {
            name: synonym.name.clone(),
            locale: synonym.locale.clone(),
        }
    }
}

impl From<&Vec<Synonym>> for schema::Synonyms {
    fn from(synonyms: &Vec<Synonym>) -> Self {
        Self(synonyms.iter().map(schema::Synonym::from).collect())
    }
}

impl From<&TimerangePrefixFormat> for schema::TimeRangePrefixFormat {
    fn from(format: &TimerangePrefixFormat) -> Self {
        match format {
            TimerangePrefixFormat::None => Self::None,
            TimerangePrefixFormat::StartYear => Self::StartYear,
            TimerangePrefixFormat::StartYearMonth => Self::StartYearMonth,
        }
    }
}

impl From<&Timerange> for schema::Timerange {
    fn from(timerange: &Timerange) -> Self {
        Self {
            ends_at: None,
            starts_at: schema::DateTime(timerange.starts),
            prefix_format: schema::TimeRangePrefixFormat::from(&timerange.prefix_format),
        }
    }
}

impl From<&Topic> for schema::Topic {
    fn from(topic: &Topic) -> Self {
        let meta = &topic.metadata;
        let parent_topic_paths = topic
            .parent_topics
            .iter()
            .map(|p| RepoPath::from(&p.path))
            .collect::<Vec<RepoPath>>();

        let child_paths = topic
            .children
            .iter()
            .map(|p| RepoPath::from(&p.path))
            .collect::<Vec<RepoPath>>();

        let synonyms = schema::Synonyms::from(&meta.synonyms);
        let time_range = meta.timerange.clone().map(|r| schema::Timerange::from(&r));
        let prefix = schema::Prefix::from(&time_range);

        Self {
            child_paths,
            path: RepoPath::from(&meta.path),
            parent_topic_paths,
            name: meta.name(),
            prefix,
            root: meta.root,
            synonyms,
            time_range,
        }
    }
}

pub struct UpsertTopicResult {
    pub alerts: Vec<schema::Alert>,
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
