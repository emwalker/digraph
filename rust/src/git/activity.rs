use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use super::{Git, Locale, Topic};
use crate::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(tag = "kind")]
pub enum Role {
    AddedChildLink(LinkInfo),
    AddedChildTopic(TopicInfo),
    AddedParentTopic(TopicInfo),
    DeletedLink(LinkInfo),
    RemovedChildLink(LinkInfo),
    RemovedChildTopic(TopicInfo),
    RemovedParentTopic(TopicInfo),
    UpdatedLink(LinkInfo),
    UpdatedTopic(TopicInfo),
}

pub type Paths = BTreeMap<String, Role>;

#[derive(Clone, Debug, Deserialize, Serialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct LinkInfo {
    pub title: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct TopicInfo {
    pub synonyms: BTreeMap<Locale, String>,
}

impl TopicInfo {
    pub fn from(topic: &Topic) -> Self {
        let mut synonyms = BTreeMap::new();

        for synonym in &topic.metadata.synonyms {
            if synonyms.contains_key(&synonym.locale) {
                continue;
            }
            synonyms.insert(synonym.locale, synonym.name.to_owned());
        }

        Self { synonyms }
    }

    pub fn get(&self, locale: Locale) -> Option<&String> {
        self.synonyms.get(&locale)
    }

    pub fn name(&self, locale: Locale) -> String {
        if let Some(name) = self.synonyms.get(&locale) {
            return name.to_owned();
        }

        if let Some(name) = self.synonyms.get(&Locale::EN) {
            return name.to_owned();
        }

        "[missing topic]".to_owned()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub date: Timestamp,
    pub paths: Paths,
    pub user_id: String,
}

fn find_topic(paths: &Paths) -> Result<&Role> {
    let result = paths
        .values()
        .find(|value| matches!(value, Role::UpdatedTopic { .. }));

    match result {
        Some(topic) => Ok(topic),
        None => Err(Error::Repo("no updated link found".to_owned())),
    }
}

fn find_link(paths: &Paths) -> Result<&Role> {
    let result = paths
        .values()
        .find(|value| matches!(value, Role::UpdatedLink { .. }));

    match result {
        Some(link) => Ok(link),
        None => Err(Error::Repo("no updated link found".to_owned())),
    }
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

impl std::cmp::Ord for Change {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse chronological order
        other.date().cmp(&self.date())
    }
}

impl std::cmp::PartialOrd for Change {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Change {
    pub fn body(&self) -> &Body {
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
        self.body().date.to_owned()
    }

    pub fn markdown(
        &self,
        locale: Locale,
        actor_name: &str,
        context: Option<&RepoPath>,
    ) -> Result<String> {
        use crate::git::activity::markdown::Markdown;
        match self {
            Self::DeleteLink(inner) => inner.markdown(locale, actor_name, context),
            Self::DeleteTopic(inner) => inner.markdown(locale, actor_name, context),
            Self::DeleteTopicTimerange(inner) => inner.markdown(locale, actor_name, context),
            Self::ImportLink(inner) => inner.markdown(locale, actor_name, context),
            Self::ImportTopic(inner) => inner.markdown(locale, actor_name, context),
            Self::UpdateLinkParentTopics(inner) => inner.markdown(locale, actor_name, context),
            Self::UpdateTopicParentTopics(inner) => inner.markdown(locale, actor_name, context),
            Self::UpdateTopicSynonyms(inner) => inner.markdown(locale, actor_name, context),
            Self::UpsertLink(inner) => inner.markdown(locale, actor_name, context),
            Self::UpsertTopic(inner) => inner.markdown(locale, actor_name, context),
            Self::UpsertTopicTimerange(inner) => inner.markdown(locale, actor_name, context),
        }
    }

    pub fn paths(&self) -> &BTreeMap<String, Role> {
        &self.body().paths
    }

    pub fn user_id(&self) -> String {
        self.body().user_id.to_owned()
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct DeleteLink(pub Body);

impl DeleteLink {
    pub fn deleted_link(&self) -> Option<(&String, &String)> {
        self.0.paths.values().find_map(|value| match value {
            Role::DeletedLink(LinkInfo { title, url }) => Some((title, url)),
            _ => None,
        })
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct DeleteTopic(pub Body);

impl DeleteTopic {
    pub fn removed_topic_name(&self, locale: Locale) -> String {
        let role = self
            .0
            .paths
            .values()
            .find(|value| matches!(value, Role::RemovedParentTopic { .. }));

        if let Some(Role::RemovedParentTopic(topic)) = role {
            topic.name(locale)
        } else {
            "[missing topic]".to_owned()
        }
        .to_owned()
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct DeleteTopicTimerange(pub Body);

impl DeleteTopicTimerange {
    pub fn topic(&self) -> Result<&Role> {
        find_topic(&self.0.paths)
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct UpdateLinkParentTopics(pub Body);

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct UpdateTopicParentTopics(pub Body);

impl UpdateLinkParentTopics {
    pub fn added_topics(&self) -> Vec<(String, &TopicInfo)> {
        self.0
            .paths
            .iter()
            .filter_map(|(path, value)| {
                if let Role::AddedParentTopic(topic) = value {
                    Some((path.to_owned(), topic))
                } else {
                    None
                }
            })
            .collect::<Vec<(String, &TopicInfo)>>()
    }

    pub fn removed_topics(&self) -> Vec<(String, &TopicInfo)> {
        self.0
            .paths
            .iter()
            .filter_map(|(path, value)| {
                if let Role::RemovedParentTopic(topic) = value {
                    Some((path.to_owned(), topic))
                } else {
                    None
                }
            })
            .collect::<Vec<(String, &TopicInfo)>>()
    }

    pub fn link(&self) -> Result<&Role> {
        find_link(&self.0.paths)
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct UpdateTopicSynonyms(pub Body);

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct UpsertLink(pub Body);

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct UpsertTopic(pub Body);

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct UpsertTopicTimerange(pub Body);

#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub struct ChangeIndexMap {
    pub api_version: String,
    pub changes: BTreeSet<Change>,
}

mod markdown {
    use super::*;

    pub trait Markdown {
        fn markdown(
            &self,
            locale: Locale,
            actor_name: &str,
            context: Option<&RepoPath>,
        ) -> Result<String>;
    }

    fn reference(text: &str, path: &str) -> String {
        format!("[{}]({})", text, path)
    }

    fn topic_reference(locale: Locale, path: &str, topic: &TopicInfo) -> String {
        let name = topic.name(locale);
        reference(&name, &path)
    }

    fn topic_desc(locale: Locale, topics: &[(String, &TopicInfo)]) -> Result<String> {
        use itertools::Itertools;
        let topics = topics.to_vec();

        let markdown = match topics.len() {
            0 => {
                return Err(Error::Repo("expected at least one topic".to_owned()));
            }
            1 => topics
                .iter()
                .map(|(path, topic)| topic_reference(locale, path, topic))
                .join(""),
            2 => topics
                .iter()
                .map(|(path, topic)| topic_reference(locale, path, topic))
                .join(" and "),
            _ => {
                let mut markdown = topics
                    .get(0..topics.len() - 1)
                    .unwrap_or_default()
                    .iter()
                    .map(|(path, topic)| topic_reference(locale, path, topic))
                    .join(", ");

                match topics.last() {
                    Some((path, topic)) => {
                        markdown
                            .push_str(&format!(" and {}", topic_reference(locale, path, topic)));
                        markdown
                    }
                    None => return Err(Error::Repo("expected a topic".to_owned())),
                }
            }
        };

        Ok(markdown)
    }

    impl Markdown for DeleteLink {
        fn markdown(
            &self,
            _locale: Locale,
            actor_name: &str,
            _context: Option<&RepoPath>,
        ) -> Result<String> {
            let (title, url) = match self.deleted_link() {
                Some((title, url)) => (title, url),
                None => return Err(Error::Repo("expected deleted link info".to_owned())),
            };

            let markdown = format!(
                "{} deleted {}, removing it from 1, 2 and 3",
                actor_name,
                reference(title, url)
            );

            Ok(markdown)
        }
    }

    impl Markdown for DeleteTopic {
        fn markdown(
            &self,
            locale: Locale,
            actor_name: &str,
            _context: Option<&RepoPath>,
        ) -> Result<String> {
            let name = self.removed_topic_name(locale);

            let markdown = format!(
                r#"<user>{}</user> deleted topic "{}", removing it from TOPIC, TOPIC and TOPIC"#,
                actor_name, name,
            );

            Ok(markdown)
        }
    }

    impl Markdown for DeleteTopicTimerange {
        fn markdown(
            &self,
            locale: Locale,
            actor_name: &str,
            _context: Option<&RepoPath>,
        ) -> Result<String> {
            let topic = if let Ok(Role::UpdatedTopic(topic)) = self.topic() {
                topic
            } else {
                return Err(Error::Repo("expected a topic".to_owned()));
            };

            let markdown = format!(
                r#"<user>{}</user> deleted topic "{}", removing it from TOPIC, TOPIC and TOPIC"#,
                actor_name,
                topic.name(locale),
            );

            Ok(markdown)
        }
    }

    impl Markdown for UpdateLinkParentTopics {
        fn markdown(
            &self,
            locale: Locale,
            actor_name: &str,
            _context: Option<&RepoPath>,
        ) -> Result<String> {
            let added = self.added_topics();
            let removed = self.removed_topics();

            if added.is_empty() && removed.is_empty() {
                return Err(Error::Repo("no change to display".to_owned()));
            }

            let mut markdown = String::new();
            markdown.push_str(&format!("{} ", actor_name));
            let mut changes = vec![];

            if !added.is_empty() {
                changes.push(format!("added {} to", topic_desc(locale, &added)?));
            }

            if !removed.is_empty() {
                changes.push(format!("removed {} from", topic_desc(locale, &removed)?));
            }

            let changes = changes.join(" and ");

            markdown.push_str(&changes);
            match self.link() {
                Ok(Role::UpdatedLink(link)) => {
                    markdown.push_str(&format!(" {}", reference(&link.title, &link.url)));
                }
                Ok(_) => {
                    markdown.push_str(" [missing link]");
                }
                Err(_) => {
                    markdown.push_str(" [missing link]");
                }
            };

            Ok(markdown)
        }
    }

    impl Markdown for UpdateTopicParentTopics {
        fn markdown(
            &self,
            _locale: Locale,
            actor_name: &str,
            _context: Option<&RepoPath>,
        ) -> Result<String> {
            let markdown = format!("{} added TOPIC to TOPIC and TOPIC", actor_name);
            Ok(markdown)
        }
    }

    impl Markdown for UpdateTopicSynonyms {
        fn markdown(
            &self,
            _locale: Locale,
            actor_name: &str,
            _context: Option<&RepoPath>,
        ) -> Result<String> {
            let markdown = format!("{} added NAME to and removed NAME from TOPIC", actor_name);
            Ok(markdown)
        }
    }

    impl Markdown for UpsertLink {
        fn markdown(
            &self,
            _locale: Locale,
            actor_name: &str,
            _context: Option<&RepoPath>,
        ) -> Result<String> {
            let markdown = format!("{} added LINK to TOPIC and TOPIC", actor_name);
            Ok(markdown)
        }
    }

    impl Markdown for UpsertTopic {
        fn markdown(
            &self,
            _locale: Locale,
            actor_name: &str,
            _context: Option<&RepoPath>,
        ) -> Result<String> {
            let markdown = format!("{} added TOPIC to TOPIC and TOPIC", actor_name);
            Ok(markdown)
        }
    }

    impl Markdown for UpsertTopicTimerange {
        fn markdown(
            &self,
            _locale: Locale,
            actor_name: &str,
            _context: Option<&RepoPath>,
        ) -> Result<String> {
            let markdown = format!("{} updated the timerange on to TOPIC to be", actor_name);
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
    pub changes: Vec<Change>,
}

pub trait ActivityForPrefix {
    fn fetch_activity(&self, prefix: &str) -> Result<Vec<Change>>;
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
                .collect::<Vec<Change>>(),

            // Fetch the top-level activity feed from Redis rather than Git so as to avoid
            // write contention on a single file for every update.  This could show up in the form
            // of merge conflicts when commits are being saved to Git.
            None => fetch.fetch_activity(WIKI_REPO_PREFIX)?,
        };

        Ok(FetchActivityResult { changes })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

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
        let link = Role::UpdatedLink(LinkInfo {
            title: "Reddit".to_owned(),
            url: "http://www.reddit.com".to_owned(),
        });
        let topic1 = topic("Climate change");
        let topic2 = topic("Weather");

        let paths = BTreeMap::from([
            ("/wiki/00010".to_owned(), link),
            (
                topic1.metadata.path.to_owned(),
                Role::AddedParentTopic(TopicInfo::from(&topic1)),
            ),
            (
                topic2.metadata.path.to_owned(),
                Role::RemovedParentTopic(TopicInfo::from(&topic2)),
            ),
        ]);

        let change = Change::UpdateLinkParentTopics(UpdateLinkParentTopics(Body {
            date: chrono::Utc::now(),
            paths,
            user_id: "2".to_owned(),
        }));

        let markdown = format!(
            "Gnusto added [Climate change]({}) to and removed [Weather]({}) from \
            [Reddit](http://www.reddit.com)",
            topic1.path(),
            topic2.path()
        );

        let context = RepoPath::from("/wiki/00010");
        assert_eq!(
            change
                .markdown(Locale::EN, "Gnusto", Some(&context))
                .unwrap(),
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
                link.metadata.path.to_owned(),
                Role::DeletedLink(LinkInfo {
                    title: link.metadata.title.to_owned(),
                    url: link.metadata.url,
                }),
            ),
            (
                topic1.metadata.path.to_owned(),
                Role::RemovedParentTopic(TopicInfo::from(&topic1)),
            ),
            (
                topic2.metadata.path.to_owned(),
                Role::RemovedParentTopic(TopicInfo::from(&topic2)),
            ),
        ]);

        let change = Change::DeleteLink(DeleteLink(Body {
            date: chrono::Utc::now(),
            paths,
            user_id: "2".to_owned(),
        }));

        assert_eq!(
            change.markdown(Locale::EN, "Gnusto", None).unwrap(),
            "Gnusto deleted [Reddit](http://www.reddit.com), removing it from 1, 2 and 3"
                .to_string()
        );
    }

    #[test]
    fn upsert_topic_timerange() {
        let topic1 = topic("Climate change");

        let paths = BTreeMap::from([(
            topic1.metadata.path.to_owned(),
            Role::UpdatedTopic(TopicInfo::from(&topic1)),
        )]);

        let change = Change::UpsertTopicTimerange(UpsertTopicTimerange(Body {
            date: chrono::Utc::now(),
            paths,
            user_id: "2".to_owned(),
        }));

        assert_eq!(
            change.markdown(Locale::EN, "Gnusto", None).unwrap(),
            "Gnusto updated the timerange on to TOPIC to be".to_string()
        );
    }
}
