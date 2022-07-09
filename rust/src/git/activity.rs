use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use super::{Git, Link, Locale, Synonym, Topic};
use crate::prelude::*;

pub mod desc {
    use super::*;

    #[derive(Clone, Debug, Deserialize, Serialize, Eq, Ord, PartialEq, PartialOrd)]
    pub enum Role {
        AddedParentTopic,
        AddedChildLink,
        AddedChildTopic,
        Link,
        DeletedLink { title: String, url: String },
        RemovedParentTopic,
        RemovedChildLink,
        RemovedChildTopic,
        Topic,
    }

    pub type Paths = BTreeMap<String, Role>;

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Body {
        pub date: Timestamp,
        pub paths: Paths,
        pub user_id: String,
    }

    #[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
    #[serde(tag = "kind")]
    pub enum Change {
        DeleteLink(DeleteLink),
        DeleteTopic(DeleteTopic),
        DeleteTopicTimerange(DeleteTopicTimerange),
        ImportLink(UpsertLink),
        ImportTopic(UpsertTopic),
        UpdateLinkParentTopics(UpdateLinkParentTopics),
        UpdateTopicSynonyms(UpdateTopicSynonyms),
        UpdateTopicParentTopics(UpdateTopicParentTopics),
        UpsertLink(UpsertLink),
        UpsertTopic(UpsertTopic),
        UpsertTopicTimerange(UpsertTopicTimerange),
    }

    impl Change {
        fn date(&self) -> &Timestamp {
            match self {
                Self::DeleteLink(desc::DeleteLink(Body { date, .. })) => date,
                Self::DeleteTopic(desc::DeleteTopic(Body { date, .. })) => date,
                Self::DeleteTopicTimerange(desc::DeleteTopicTimerange(Body { date, .. })) => date,
                Self::ImportLink(desc::UpsertLink(Body { date, .. })) => date,
                Self::ImportTopic(desc::UpsertTopic(Body { date, .. })) => date,
                Self::UpdateLinkParentTopics(desc::UpdateLinkParentTopics(Body {
                    date, ..
                })) => date,
                Self::UpdateTopicParentTopics(desc::UpdateTopicParentTopics(Body {
                    date, ..
                })) => date,
                Self::UpdateTopicSynonyms(desc::UpdateTopicSynonyms(Body { date, .. })) => date,
                Self::UpsertLink(desc::UpsertLink(Body { date, .. })) => date,
                Self::UpsertTopic(desc::UpsertTopic(Body { date, .. })) => date,
                Self::UpsertTopicTimerange(desc::UpsertTopicTimerange(Body { date, .. })) => date,
            }
        }

        pub fn paths(&self) -> &BTreeMap<String, Role> {
            match self {
                Self::DeleteLink(desc::DeleteLink(Body { paths, .. })) => paths,
                Self::DeleteTopic(desc::DeleteTopic(Body { paths, .. })) => paths,
                Self::DeleteTopicTimerange(desc::DeleteTopicTimerange(Body { paths, .. })) => paths,
                Self::ImportLink(desc::UpsertLink(Body { paths, .. })) => paths,
                Self::ImportTopic(desc::UpsertTopic(Body { paths, .. })) => paths,
                Self::UpdateLinkParentTopics(desc::UpdateLinkParentTopics(Body {
                    paths, ..
                })) => paths,
                Self::UpdateTopicParentTopics(desc::UpdateTopicParentTopics(Body {
                    paths,
                    ..
                })) => paths,
                Self::UpdateTopicSynonyms(desc::UpdateTopicSynonyms(Body { paths, .. })) => paths,
                Self::UpsertLink(desc::UpsertLink(Body { paths, .. })) => paths,
                Self::UpsertTopic(desc::UpsertTopic(Body { paths, .. })) => paths,
                Self::UpsertTopicTimerange(desc::UpsertTopicTimerange(Body { paths, .. })) => paths,
            }
        }

        pub fn fetch(&self, git: &Git) -> Result<fetched::Change> {
            let change = match self {
                Self::DeleteLink(desc::DeleteLink(body)) => {
                    fetched::Change::DeleteLink(fetched::DeleteLink(self.fetch_body(git, body)?))
                }
                Self::DeleteTopic(desc::DeleteTopic(body)) => {
                    fetched::Change::DeleteTopic(fetched::DeleteTopic(self.fetch_body(git, body)?))
                }
                Self::DeleteTopicTimerange(desc::DeleteTopicTimerange(body)) => {
                    fetched::Change::DeleteTopicTimerange(fetched::DeleteTopicTimerange(
                        self.fetch_body(git, body)?,
                    ))
                }
                Self::ImportLink(desc::UpsertLink(body)) => {
                    fetched::Change::ImportLink(fetched::UpsertLink(self.fetch_body(git, body)?))
                }
                Self::ImportTopic(desc::UpsertTopic(body)) => {
                    fetched::Change::ImportTopic(fetched::UpsertTopic(self.fetch_body(git, body)?))
                }
                Self::UpdateLinkParentTopics(desc::UpdateLinkParentTopics(body)) => {
                    fetched::Change::UpdateLinkParentTopics(fetched::UpdateLinkParentTopics(
                        self.fetch_body(git, body)?,
                    ))
                }
                Self::UpdateTopicParentTopics(desc::UpdateTopicParentTopics(body)) => {
                    fetched::Change::UpdateTopicParentTopics(fetched::UpdateTopicParentTopics(
                        self.fetch_body(git, body)?,
                    ))
                }
                Self::UpdateTopicSynonyms(desc::UpdateTopicSynonyms(body)) => {
                    fetched::Change::UpdateTopicSynonyms(fetched::UpdateTopicSynonyms(
                        self.fetch_body(git, body)?,
                    ))
                }
                Self::UpsertLink(desc::UpsertLink(body)) => {
                    fetched::Change::UpsertLink(fetched::UpsertLink(self.fetch_body(git, body)?))
                }
                Self::UpsertTopic(desc::UpsertTopic(body)) => {
                    fetched::Change::UpsertTopic(fetched::UpsertTopic(self.fetch_body(git, body)?))
                }
                Self::UpsertTopicTimerange(desc::UpsertTopicTimerange(body)) => {
                    fetched::Change::UpsertTopicTimerange(fetched::UpsertTopicTimerange(
                        self.fetch_body(git, body)?,
                    ))
                }
            };

            Ok(change)
        }

        fn fetch_body(&self, git: &Git, body: &Body) -> Result<fetched::Body> {
            let Body {
                date,
                paths,
                user_id,
            } = body;
            let mut next_paths = BTreeMap::new();

            for (path, role) in paths.iter() {
                let repo_path = RepoPath::from(path);

                match role {
                    Role::AddedChildLink => match git.fetch_link(path) {
                        Ok(link) => {
                            next_paths.insert(repo_path, fetched::Role::AddedChildLink(Some(link)));
                        }
                        Err(_) => {
                            next_paths.insert(repo_path, fetched::Role::AddedChildLink(None));
                        }
                    },

                    Role::AddedChildTopic => match git.fetch_topic(path) {
                        Ok(topic) => {
                            next_paths
                                .insert(repo_path, fetched::Role::AddedChildTopic(Some(topic)));
                        }
                        Err(_) => {
                            next_paths.insert(repo_path, fetched::Role::AddedChildTopic(None));
                        }
                    },

                    Role::AddedParentTopic => match git.fetch_topic(path) {
                        Ok(topic) => {
                            next_paths
                                .insert(repo_path, fetched::Role::AddedParentTopic(Some(topic)));
                        }
                        Err(_) => {
                            next_paths.insert(repo_path, fetched::Role::AddedParentTopic(None));
                        }
                    },

                    Role::DeletedLink { title, url } => {
                        next_paths.insert(
                            repo_path,
                            fetched::Role::DeletedLink {
                                url: url.to_owned(),
                                title: title.to_owned(),
                            },
                        );
                    }

                    Role::Link => match git.fetch_link(path) {
                        Ok(link) => {
                            next_paths.insert(repo_path, fetched::Role::Link(Some(link)));
                        }
                        Err(_) => {
                            next_paths.insert(repo_path, fetched::Role::Link(None));
                        }
                    },

                    Role::RemovedChildLink => match git.fetch_link(path) {
                        Ok(link) => {
                            next_paths
                                .insert(repo_path, fetched::Role::RemovedChildLink(Some(link)));
                        }
                        Err(_) => {
                            next_paths.insert(repo_path, fetched::Role::RemovedChildLink(None));
                        }
                    },

                    Role::RemovedChildTopic => match git.fetch_topic(path) {
                        Ok(topic) => {
                            next_paths
                                .insert(repo_path, fetched::Role::RemovedChildTopic(Some(topic)));
                        }
                        Err(_) => {
                            next_paths.insert(repo_path, fetched::Role::RemovedChildTopic(None));
                        }
                    },

                    Role::RemovedParentTopic => match git.fetch_topic(path) {
                        Ok(topic) => {
                            next_paths
                                .insert(repo_path, fetched::Role::RemovedParentTopic(Some(topic)));
                        }
                        Err(_) => {
                            next_paths.insert(repo_path, fetched::Role::RemovedParentTopic(None));
                        }
                    },

                    Role::Topic => match git.fetch_topic(path) {
                        Ok(topic) => {
                            next_paths.insert(repo_path, fetched::Role::Topic(Some(topic)));
                        }
                        Err(_) => {
                            next_paths.insert(repo_path, fetched::Role::Topic(None));
                        }
                    },
                }
            }

            Ok(fetched::Body {
                date: date.to_owned(),
                paths: next_paths,
                user_id: user_id.to_owned(),
            })
        }
    }

    impl std::cmp::Ord for Change {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            // Reverse chronological order
            other.date().cmp(self.date())
        }
    }

    impl std::cmp::PartialOrd for Change {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
    pub struct DeleteLink(pub Body);

    #[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
    pub struct DeleteTopic(pub Body);

    #[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
    pub struct DeleteTopicTimerange(pub Body);

    #[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
    pub struct UpdateLinkParentTopics(pub Body);

    #[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
    pub struct UpdateTopicParentTopics(pub Body);

    #[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
    pub struct UpdateTopicSynonyms(pub Body);

    #[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
    pub struct UpsertLink(pub Body);

    #[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
    pub struct UpsertTopic(pub Body);

    #[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
    pub struct UpsertTopicTimerange(pub Body);
}

#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeIndexMap {
    pub api_version: String,
    pub kind: String,
    pub changes: BTreeSet<desc::Change>,
}

pub mod fetched {
    use super::*;

    pub enum Role {
        AddedChildLink(Option<Link>),
        AddedChildTopic(Option<Topic>),
        AddedParentTopic(Option<Topic>),
        DeletedLink { url: String, title: String },
        DeletedTopic { synonyms: BTreeSet<Synonym> },
        Link(Option<Link>),
        RemovedChildLink(Option<Link>),
        RemovedChildTopic(Option<Topic>),
        RemovedParentTopic(Option<Topic>),
        Topic(Option<Topic>),
    }

    pub type Paths = BTreeMap<RepoPath, Role>;

    fn find_topic(paths: &Paths) -> Option<&Topic> {
        paths.values().find_map(|value| match value {
            Role::Topic(topic) => topic.as_ref(),
            _ => None,
        })
    }

    fn find_link(paths: &Paths) -> Option<&Link> {
        paths.values().find_map(|value| match value {
            Role::Link(link) => link.as_ref(),
            _ => None,
        })
    }

    pub struct Body {
        pub date: Timestamp,
        pub paths: Paths,
        pub user_id: String,
    }

    pub enum Change {
        DeleteLink(DeleteLink),
        DeleteTopic(DeleteTopic),
        DeleteTopicTimerange(DeleteTopicTimerange),
        ImportLink(UpsertLink),
        ImportTopic(UpsertTopic),
        UpdateLinkParentTopics(UpdateLinkParentTopics),
        UpdateTopicSynonyms(UpdateTopicSynonyms),
        UpdateTopicParentTopics(UpdateTopicParentTopics),
        UpsertLink(UpsertLink),
        UpsertTopic(UpsertTopic),
        UpsertTopicTimerange(UpsertTopicTimerange),
    }

    pub struct DeleteLink(pub Body);

    impl DeleteLink {
        pub fn deleted_link(&self) -> Option<(&String, &String)> {
            self.0.paths.values().find_map(|value| match value {
                Role::DeletedLink { title, url } => Some((title, url)),
                _ => None,
            })
        }
    }

    pub struct DeleteTopic(pub Body);

    impl DeleteTopic {
        pub fn deleted_topic_name(&self, locale: Locale) -> String {
            let role = self
                .0
                .paths
                .values()
                .find(|value| matches!(value, Role::DeletedTopic { .. }));

            if let Some(Role::DeletedTopic { synonyms }) = role {
                let synonym = synonyms.iter().find(|s| s.locale == locale);
                if let Some(Synonym { name, .. }) = synonym {
                    name
                } else {
                    "Missing name"
                }
            } else {
                "Missing name"
            }
            .to_owned()
        }
    }

    pub struct DeleteTopicTimerange(pub Body);

    impl DeleteTopicTimerange {
        pub fn topic(&self) -> Option<&Topic> {
            find_topic(&self.0.paths)
        }
    }

    pub struct UpdateLinkParentTopics(pub Body);

    impl UpdateLinkParentTopics {
        pub fn added_topics(&self) -> Vec<&Topic> {
            self.0
                .paths
                .values()
                .filter_map(|value| match value {
                    Role::AddedParentTopic(topic) => topic.as_ref(),
                    _ => None,
                })
                .collect::<Vec<&Topic>>()
        }

        pub fn removed_topics(&self) -> Vec<&Topic> {
            self.0
                .paths
                .values()
                .filter_map(|value| match value {
                    Role::RemovedParentTopic(topic) => topic.as_ref(),
                    _ => None,
                })
                .collect::<Vec<&Topic>>()
        }

        pub fn link(&self) -> Option<&Link> {
            find_link(&self.0.paths)
        }
    }

    pub struct UpdateTopicParentTopics(pub Body);

    pub struct UpdateTopicSynonyms(pub Body);

    pub struct UpsertLink(pub Body);

    pub struct UpsertTopic(pub Body);

    pub struct UpsertTopicTimerange(pub Body);

    impl Change {
        pub fn markdown(&self, locale: Locale, context: Option<&RepoPath>) -> Result<String> {
            use super::markdown::Markdown;
            match self {
                Self::DeleteLink(inner) => inner.markdown(locale, context),
                Self::DeleteTopic(inner) => inner.markdown(locale, context),
                Self::DeleteTopicTimerange(inner) => inner.markdown(locale, context),
                Self::ImportLink(inner) => inner.markdown(locale, context),
                Self::ImportTopic(inner) => inner.markdown(locale, context),
                Self::UpdateLinkParentTopics(inner) => inner.markdown(locale, context),
                Self::UpdateTopicParentTopics(inner) => inner.markdown(locale, context),
                Self::UpdateTopicSynonyms(inner) => inner.markdown(locale, context),
                Self::UpsertLink(inner) => inner.markdown(locale, context),
                Self::UpsertTopic(inner) => inner.markdown(locale, context),
                Self::UpsertTopicTimerange(inner) => inner.markdown(locale, context),
            }
        }

        fn body(&self) -> &Body {
            match self {
                Self::DeleteLink(DeleteLink(body)) => body,
                Self::DeleteTopic(DeleteTopic(body)) => body,
                Self::DeleteTopicTimerange(DeleteTopicTimerange(body)) => body,
                Self::ImportLink(UpsertLink(body)) => body,
                Self::ImportTopic(UpsertTopic(body)) => body,
                Self::UpdateLinkParentTopics(UpdateLinkParentTopics(body)) => body,
                Self::UpdateTopicParentTopics(UpdateTopicParentTopics(body)) => body,
                Self::UpdateTopicSynonyms(UpdateTopicSynonyms(body)) => body,
                Self::UpsertLink(UpsertLink(body)) => body,
                Self::UpsertTopic(UpsertTopic(body)) => body,
                Self::UpsertTopicTimerange(UpsertTopicTimerange(body)) => body,
            }
        }

        pub fn date(&self) -> Timestamp {
            self.body().date
        }
    }
}

mod markdown {
    use super::*;

    pub trait Markdown {
        fn markdown(&self, locale: Locale, context: Option<&RepoPath>) -> Result<String>;
    }

    fn reference(text: &str, path: &str) -> String {
        format!("[{}]({})", text, path)
    }

    fn topic_reference(locale: Locale, topic: &Topic) -> String {
        reference(&topic.name(locale), &topic.metadata.path)
    }

    fn topic_desc(locale: Locale, topics: &[&Topic]) -> Result<String> {
        use itertools::Itertools;
        let topics = topics.to_vec();

        let markdown = match topics.len() {
            0 => {
                return Err(Error::Repo("expected at least one topic".to_owned()));
            }
            1 => topics.iter().map(|t| topic_reference(locale, t)).join(""),
            2 => topics
                .iter()
                .map(|t| topic_reference(locale, t))
                .join(" and "),
            _ => {
                let mut markdown = topics
                    .get(0..topics.len() - 1)
                    .unwrap_or_default()
                    .iter()
                    .map(|t| topic_reference(locale, t))
                    .join(", ");

                match topics.last() {
                    Some(topic) => {
                        markdown.push_str(&format!(" and {}", topic_reference(locale, topic)));
                        markdown
                    }
                    None => return Err(Error::Repo("expected a topic".to_owned())),
                }
            }
        };

        Ok(markdown)
    }

    impl Markdown for fetched::DeleteLink {
        fn markdown(&self, _locale: Locale, _context: Option<&RepoPath>) -> Result<String> {
            let (title, url) = match self.deleted_link() {
                Some((title, url)) => (title, url),
                None => return Err(Error::Repo("expected deleted link info".to_owned())),
            };

            let markdown = format!(
                "<user>{}</user> deleted {}, removing it from 1, 2 and 3",
                self.0.user_id.to_owned(),
                reference(title, url)
            );

            Ok(markdown)
        }
    }

    impl Markdown for fetched::DeleteTopic {
        fn markdown(&self, locale: Locale, _context: Option<&RepoPath>) -> Result<String> {
            let name = self.deleted_topic_name(locale);

            let markdown = format!(
                r#"<user>{}</user> deleted topic "{}", removing it from TOPIC, TOPIC and TOPIC"#,
                self.0.user_id.to_owned(),
                name,
            );

            Ok(markdown)
        }
    }

    impl Markdown for fetched::DeleteTopicTimerange {
        fn markdown(&self, locale: Locale, _context: Option<&RepoPath>) -> Result<String> {
            let topic = match self.topic() {
                Some(topic) => topic,
                None => return Err(Error::Repo("expected a topic".to_owned())),
            };

            let markdown = format!(
                r#"<user>{}</user> deleted topic "{}", removing it from TOPIC, TOPIC and TOPIC"#,
                self.0.user_id.to_owned(),
                topic.name(locale),
            );

            Ok(markdown)
        }
    }

    impl Markdown for fetched::UpdateLinkParentTopics {
        fn markdown(&self, locale: Locale, _context: Option<&RepoPath>) -> Result<String> {
            let added = self.added_topics();
            let removed = self.removed_topics();

            if added.is_empty() && removed.is_empty() {
                return Err(Error::Repo("no change to display".to_owned()));
            }

            let mut markdown = String::new();
            markdown.push_str(&format!("<user>{}</user> ", self.0.user_id));
            let mut changes = vec![];

            if !added.is_empty() {
                changes.push(format!("added {} to", topic_desc(locale, &added)?));
            }

            if !removed.is_empty() {
                changes.push(format!("removed {} from", topic_desc(locale, &removed)?));
            }

            let changes = changes.join(" and ");

            markdown.push_str(&changes);
            let link = self.link();
            match &link {
                Some(link) => {
                    let meta = &link.metadata;
                    markdown.push_str(&format!(" {}", reference(&meta.title, &meta.url)));
                }
                None => {
                    markdown.push_str(" [missing link]");
                }
            };

            Ok(markdown)
        }
    }

    impl Markdown for fetched::UpdateTopicParentTopics {
        fn markdown(&self, _locale: Locale, _context: Option<&RepoPath>) -> Result<String> {
            let markdown = format!(
                "<user>{}</user> added TOPIC to TOPIC and TOPIC",
                self.0.user_id
            );
            Ok(markdown)
        }
    }

    impl Markdown for fetched::UpdateTopicSynonyms {
        fn markdown(&self, _locale: Locale, _context: Option<&RepoPath>) -> Result<String> {
            let markdown = format!(
                "<user>{}</user> added NAME to and removed NAME from TOPIC",
                self.0.user_id
            );
            Ok(markdown)
        }
    }

    impl Markdown for fetched::UpsertLink {
        fn markdown(&self, _locale: Locale, _context: Option<&RepoPath>) -> Result<String> {
            let markdown = format!(
                "<user>{}</user> added LINK to TOPIC and TOPIC",
                self.0.user_id
            );
            Ok(markdown)
        }
    }

    impl Markdown for fetched::UpsertTopic {
        fn markdown(&self, _locale: Locale, _context: Option<&RepoPath>) -> Result<String> {
            let markdown = format!(
                "<user>{}</user> added TOPIC to TOPIC and TOPIC",
                self.0.user_id
            );
            Ok(markdown)
        }
    }

    impl Markdown for fetched::UpsertTopicTimerange {
        fn markdown(&self, _locale: Locale, _context: Option<&RepoPath>) -> Result<String> {
            let markdown = format!(
                "<user>{}</user> updated the timerange on to TOPIC to be",
                self.0.user_id
            );
            Ok(markdown)
        }
    }
}

pub struct FetchActivity {
    pub actor: Viewer,
    pub first: i32,
    pub topic_path: Option<RepoPath>,
}

pub struct FetchActivityResult {
    pub changes: Vec<fetched::Change>,
}

pub trait ActivityForPrefix {
    fn fetch_activity(&self, prefix: &str) -> Result<Vec<desc::Change>>;
}

impl FetchActivity {
    pub fn call<F>(&self, git: &Git, fetch: &F) -> Result<FetchActivityResult>
    where
        F: ActivityForPrefix,
    {
        let changes = match &self.topic_path {
            Some(path) => git
                .fetch_activity(path)?
                .changes()
                .iter()
                .cloned()
                .collect::<Vec<desc::Change>>(),

            // Fetch the top-level activity feed from Redis rather than Git so as to avoid
            // write contention on a single file for every update.  This could show up in the form
            // of merge conflicts when commits are being saved to Git.
            None => fetch.fetch_activity(WIKI_REPO_PREFIX)?,
        };

        let mut fetched = vec![];
        for change in changes {
            let change = change.fetch(git)?;
            fetched.push(change);
        }

        Ok(FetchActivityResult { changes: fetched })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::fetched::*;
    use super::*;
    use crate::git::{
        Link, LinkMetadata, Locale, ParentTopic, Synonym, Topic, TopicMetadata, API_VERSION,
    };

    fn topic(name: &str) -> Topic {
        let added = chrono::Utc::now();
        // Some unique path
        let path = format!("/wiki/{}", name);
        Topic {
            api_version: API_VERSION.to_owned(),
            metadata: TopicMetadata {
                added,
                path: path.to_owned(),
                root: false,
                timerange: None,
                synonyms: Vec::from([Synonym {
                    name: name.to_owned(),
                    locale: Locale::EN,
                    added,
                }]),
            },
            parent_topics: BTreeSet::from([ParentTopic { path }]),
            children: BTreeSet::new(),
        }
    }

    fn link(title: &str, url: &str) -> Link {
        let added = chrono::Utc::now();
        Link {
            api_version: API_VERSION.to_owned(),
            metadata: LinkMetadata {
                title: title.to_owned(),
                url: url.to_owned(),
                path: "/wiki/00002".to_owned(),
                added,
            },
            parent_topics: BTreeSet::from([ParentTopic {
                path: "/wiki/0001".to_owned(),
            }]),
        }
    }

    #[test]
    fn update_link_parent_topics() {
        let link = link("Reddit", "http://www.reddit.com");
        let topic1 = topic("Climate change");
        let topic2 = topic("Weather");

        println!("{}, {}", topic1.path(), topic2.path());
        let paths = BTreeMap::from([
            (link.path(), Role::Link(Some(link.clone()))),
            (topic1.path(), Role::AddedParentTopic(Some(topic1.clone()))),
            (
                topic2.path(),
                Role::RemovedParentTopic(Some(topic2.clone())),
            ),
        ]);

        let change = fetched::Change::UpdateLinkParentTopics(UpdateLinkParentTopics(Body {
            date: chrono::Utc::now(),
            user_id: "123".to_owned(),
            paths,
        }));

        let markdown = format!(
            "<user>123</user> added [Climate change]({}) to and removed [Weather]({}) from \
            [Reddit](http://www.reddit.com)",
            topic1.path(),
            topic2.path()
        );

        let context = link.path();
        assert_eq!(
            change.markdown(Locale::EN, Some(&context)).unwrap(),
            markdown
        );
    }

    #[test]
    fn delete_link() {
        let link = link("Reddit", "http://www.reddit.com");
        let topic1 = topic("Climate change");
        let topic2 = topic("Weather");

        let paths = BTreeMap::from([
            (
                link.path(),
                Role::DeletedLink {
                    title: link.metadata.title.to_owned(),
                    url: link.metadata.url,
                },
            ),
            (topic1.path(), Role::RemovedParentTopic(Some(topic1))),
            (topic2.path(), Role::RemovedParentTopic(Some(topic2))),
        ]);

        let change = fetched::Change::DeleteLink(DeleteLink(Body {
            date: chrono::Utc::now(),
            user_id: "123".to_owned(),
            paths,
        }));

        let markdown =
            "<user>123</user> deleted [Reddit](http://www.reddit.com), removing it from 1, 2 and 3"
                .to_string();

        assert_eq!(change.markdown(Locale::EN, None).unwrap(), markdown);
    }
}
