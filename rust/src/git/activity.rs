use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use super::{Git, Locale, Topic};
use crate::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct SynonymInfo(pub BTreeMap<Locale, String>);

impl SynonymInfo {
    pub fn from(topic: &Topic) -> Self {
        let mut synonyms = BTreeMap::new();

        for synonym in &topic.metadata.synonyms {
            if synonyms.contains_key(&synonym.locale) {
                continue;
            }
            synonyms.insert(synonym.locale, synonym.name.to_owned());
        }

        Self(synonyms)
    }

    pub fn get(&self, locale: Locale) -> Option<&String> {
        self.0.get(&locale)
    }

    pub fn name(&self, locale: Locale) -> String {
        if let Some(name) = self.0.get(&locale) {
            return name.to_owned();
        }

        if let Some(name) = self.0.get(&Locale::EN) {
            return name.to_owned();
        }

        "[missing topic]".to_owned()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(tag = "kind")]
pub enum Role {
    AddedChildLink { title: String, url: String },
    AddedChildTopic { synonyms: SynonymInfo },
    AddedParentTopic { synonyms: SynonymInfo },
    DeletedLink { title: String, url: String },
    DeletedTopic { synonyms: SynonymInfo },
    RemovedChildLink { title: String, url: String },
    RemovedChildTopic { synonyms: SynonymInfo },
    RemovedParentTopic { synonyms: SynonymInfo },
    UpdatedLink { title: String, url: String },
    UpdatedTopic { synonyms: SynonymInfo },
}

pub type Paths = BTreeMap<String, Role>;

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
    pub fn deleted_link(&self) -> Result<(&String, &String)> {
        let result = self
            .0
            .paths
            .values()
            .find(|value| matches!(value, Role::DeletedLink { .. }));
        match result {
            Some(Role::DeletedLink { title, url }) => Ok((title, url)),
            _ => Err(Error::Repo("expected a deleted link".to_owned())),
        }
    }

    pub fn parent_topics(&self) -> Vec<(&String, &SynonymInfo)> {
        self.0
            .paths
            .iter()
            .filter_map(|(path, value)| match value {
                Role::RemovedParentTopic { synonyms } => Some((path, synonyms)),
                _ => None,
            })
            .collect()
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct DeleteTopic(pub Body);

impl DeleteTopic {
    pub fn deleted_topic(&self) -> Result<(&String, &SynonymInfo)> {
        let result = self
            .0
            .paths
            .iter()
            .find(|(_path, value)| matches!(value, Role::DeletedTopic { .. }));

        if let Some((path, Role::DeletedTopic { synonyms })) = &result {
            Ok((path, synonyms))
        } else {
            Err(Error::Repo("expected a deleted topic".to_owned()))
        }
    }

    pub fn child_links(&self) -> Vec<(&String, &String)> {
        self.0
            .paths
            .iter()
            .filter_map(|(_path, value)| match value {
                Role::RemovedChildLink { title, url } => Some((title, url)),
                _ => None,
            })
            .collect()
    }

    pub fn child_topics(&self) -> Vec<(&String, &SynonymInfo)> {
        self.0
            .paths
            .iter()
            .filter_map(|(path, value)| match value {
                Role::RemovedChildTopic { synonyms } => Some((path, synonyms)),
                _ => None,
            })
            .collect()
    }

    pub fn parent_topics(&self) -> Vec<(&String, &SynonymInfo)> {
        self.0
            .paths
            .iter()
            .filter_map(|(path, value)| match value {
                Role::RemovedParentTopic { synonyms } => Some((path, synonyms)),
                _ => None,
            })
            .collect()
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
    pub fn added_topics(&self) -> Vec<(&String, &SynonymInfo)> {
        self.0
            .paths
            .iter()
            .filter_map(|(path, value)| {
                if let Role::AddedParentTopic { synonyms } = value {
                    Some((path, synonyms))
                } else {
                    None
                }
            })
            .collect::<Vec<(&String, &SynonymInfo)>>()
    }

    pub fn removed_topics(&self) -> Vec<(&String, &SynonymInfo)> {
        self.0
            .paths
            .iter()
            .filter_map(|(path, value)| {
                if let Role::RemovedParentTopic { synonyms } = value {
                    Some((path, synonyms))
                } else {
                    None
                }
            })
            .collect::<Vec<(&String, &SynonymInfo)>>()
    }

    pub fn link(&self) -> Result<&Role> {
        find_link(&self.0.paths)
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct UpdateTopicSynonyms(pub Body);

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct UpsertLink(pub Body);

impl UpsertLink {
    pub fn topics(&self) -> Vec<(&String, &SynonymInfo)> {
        self.0
            .paths
            .iter()
            .filter_map(|(path, value)| match value {
                Role::AddedParentTopic { synonyms } => Some((path, synonyms)),
                _ => None,
            })
            .collect::<Vec<(&String, &SynonymInfo)>>()
    }
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct UpsertTopic(pub Body);

impl UpsertTopic {
    pub fn child_links(&self) -> Vec<(&String, &String)> {
        self.0
            .paths
            .iter()
            .filter_map(|(_path, value)| match value {
                Role::AddedChildLink { title, url } => Some((title, url)),
                _ => None,
            })
            .collect::<Vec<(&String, &String)>>()
    }

    pub fn child_topics(&self) -> Vec<(&String, &SynonymInfo)> {
        self.0
            .paths
            .iter()
            .filter_map(|(path, value)| match value {
                Role::AddedChildTopic { synonyms } => Some((path, synonyms)),
                _ => None,
            })
            .collect::<Vec<(&String, &SynonymInfo)>>()
    }

    pub fn parent_topics(&self) -> Vec<(&String, &SynonymInfo)> {
        self.0
            .paths
            .iter()
            .filter_map(|(path, value)| match value {
                Role::AddedParentTopic { synonyms } => Some((path, synonyms)),
                _ => None,
            })
            .collect::<Vec<(&String, &SynonymInfo)>>()
    }

    pub fn topic(&self) -> Option<(&String, &SynonymInfo)> {
        let result = self
            .0
            .paths
            .iter()
            .find(|(_path, value)| matches!(value, Role::UpdatedTopic { .. }));

        match result {
            Some((path, Role::UpdatedTopic { synonyms })) => Some((path, synonyms)),
            Some(_) => None,
            None => None,
        }
    }
}

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

    fn link_reference(link: &(&String, &String)) -> String {
        let (name, url) = link;
        reference(name, url)
    }

    fn topic_reference(locale: Locale, path: &str, topic: &SynonymInfo) -> String {
        let name = topic.name(locale);
        reference(&name, path)
    }

    fn topic_desc(topics: &[(&String, &SynonymInfo)], locale: Locale) -> String {
        use itertools::Itertools;
        let topics = topics.to_vec();

        match topics.len() {
            0 => "".to_owned(),
            1 => topics
                .iter()
                .map(|(path, topic)| topic_reference(locale, path, topic))
                .join(""),
            2 => topics
                .iter()
                .map(|(path, topic)| topic_reference(locale, path, topic))
                .join(" and "),
            3 => {
                let mut markdown = topics
                    .get(0..topics.len() - 1)
                    .unwrap_or_default()
                    .iter()
                    .map(|(path, topic)| topic_reference(locale, path, topic))
                    .join(", ");

                match topics.last() {
                    Some((path, topic)) => {
                        markdown.push_str(" and ");
                        markdown.push_str(&topic_reference(locale, path, topic));
                        markdown
                    }
                    None => "[missing topic]".to_owned(),
                }
            }
            _ => "a number of topics".to_owned(),
        }
    }

    fn link_desc(links: &[(&String, &String)]) -> String {
        use itertools::Itertools;

        match links.len() {
            0 => "".to_owned(),
            1 => links.iter().map(link_reference).join(""),
            2 => links.iter().map(link_reference).join(" and "),
            _ => "a number of links".to_owned(),
        }
    }

    impl Markdown for DeleteLink {
        fn markdown(
            &self,
            locale: Locale,
            actor_name: &str,
            _context: Option<&RepoPath>,
        ) -> Result<String> {
            let (title, url) = self.deleted_link()?;
            let parent_topics = self.parent_topics();

            let markdown = if parent_topics.is_empty() {
                format!("{} deleted {}", actor_name, reference(title, url))
            } else {
                format!(
                    "{} deleted {}, removing it from {}",
                    actor_name,
                    reference(title, url),
                    topic_desc(&parent_topics, locale)
                )
            };

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
            let (path, synonyms) = self.deleted_topic()?;
            let parent_topics = self.parent_topics();
            let child_links = self.child_links();
            let child_topics = self.child_topics();

            let mut markdown = format!(
                "{} deleted {}",
                actor_name,
                topic_reference(locale, path, synonyms)
            );
            let mut children = String::new();

            match (child_links.is_empty(), child_topics.is_empty()) {
                (true, true) => {}
                (false, true) => {
                    children.push_str(&link_desc(&child_links));
                }
                (true, false) => {
                    children.push_str(&topic_desc(&child_topics, locale));
                }
                (false, false) => {
                    children.push_str(&link_desc(&child_links));
                    children.push_str(" and ");
                    children.push_str(&topic_desc(&child_topics, locale));
                }
            }

            match (children.is_empty(), parent_topics.is_empty()) {
                (true, true) => {}
                (true, false) => {
                    markdown.push_str(", removing it from ");
                    markdown.push_str(&topic_desc(&parent_topics, locale));
                }
                (false, true) => {
                    markdown.push_str(", removing ");
                    markdown.push_str(&children);
                    markdown.push_str(" from it");
                }
                (false, false) => {
                    markdown.push_str(", removing ");
                    markdown.push_str(&children);
                    markdown.push_str(" from it, and removing it from ");
                    markdown.push_str(&topic_desc(&parent_topics, locale));
                }
            }

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
            let synonyms = if let Ok(Role::UpdatedTopic { synonyms }) = self.topic() {
                synonyms
            } else {
                return Err(Error::Repo("expected a topic".to_owned()));
            };

            let markdown = format!(
                r#"<user>{}</user> deleted topic "{}", removing it from TOPIC, TOPIC and TOPIC"#,
                actor_name,
                synonyms.name(locale),
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

            let mut markdown = actor_name.to_owned();
            markdown.push(' ');
            let mut changes = vec![];

            if !added.is_empty() {
                changes.push(format!("added {} to", topic_desc(&added, locale)));
            }

            if !removed.is_empty() {
                changes.push(format!("removed {} from", topic_desc(&removed, locale)));
            }

            let changes = changes.join(" and ");

            markdown.push_str(&changes);
            match self.link() {
                Ok(Role::UpdatedLink { title, url }) => {
                    markdown.push(' ');
                    markdown.push_str(&reference(title, url));
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
            locale: Locale,
            actor_name: &str,
            _context: Option<&RepoPath>,
        ) -> Result<String> {
            let (title, url) = if let Role::UpdatedLink { title, url } = find_link(&self.0.paths)? {
                (title, url)
            } else {
                return Err(Error::Repo("no link found".to_owned()));
            };
            let topics = topic_desc(self.topics().as_slice(), locale);

            let markdown = format!(
                "{} added {} to {}",
                actor_name,
                reference(title, url),
                topics,
            );
            Ok(markdown)
        }
    }

    impl Markdown for UpsertTopic {
        fn markdown(
            &self,
            locale: Locale,
            actor_name: &str,
            _context: Option<&RepoPath>,
        ) -> Result<String> {
            let child_topics = self.child_topics();
            let child_links = self.child_links();
            let parent_topics = self.parent_topics();
            let topic = self.topic();

            let mut children = vec![];
            if !child_topics.is_empty() {
                children.push(topic_desc(&child_topics, locale));
            }

            if !child_links.is_empty() {
                children.push(link_desc(&child_links));
            }

            let markdown = match (&topic, children.is_empty(), parent_topics.is_empty()) {
                (None, true, true) => "made a change, but the details are missing".to_owned(),
                (None, false, true) => format!("added {}", topic_desc(&child_topics, locale)),
                (None, true, false) => format!("added {}", link_desc(&child_links)),

                (None, false, false) => format!(
                    "added {} and {}",
                    topic_desc(&child_topics, locale),
                    link_desc(&child_links)
                ),

                (Some((path, synonyms)), true, true) => {
                    format!(
                        "updated {}, but the details are missing",
                        topic_reference(locale, path, synonyms)
                    )
                }

                (Some((path, synonyms)), true, false) => format!(
                    "added {} to {}",
                    topic_reference(locale, path, synonyms),
                    topic_desc(&parent_topics, locale),
                ),

                (Some((path, synonyms)), false, true) => format!(
                    "added {} to {}",
                    children.join(" and "),
                    topic_reference(locale, path, synonyms)
                ),

                (Some((path, synonyms)), false, false) => format!(
                    "added {} to {}, and added {} to {}",
                    children.join(" and "),
                    topic_reference(locale, path, synonyms),
                    topic_reference(locale, path, synonyms),
                    topic_desc(&parent_topics, locale),
                ),
            };

            let markdown = format!("{} {}", actor_name, markdown,);
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
        let link = Role::UpdatedLink {
            title: "Reddit".to_owned(),
            url: "http://www.reddit.com".to_owned(),
        };
        let topic1 = topic("Climate change");
        let topic2 = topic("Weather");

        let paths = BTreeMap::from([
            ("/wiki/00010".to_owned(), link),
            (
                topic1.metadata.path.to_owned(),
                Role::AddedParentTopic {
                    synonyms: SynonymInfo::from(&topic1),
                },
            ),
            (
                topic2.metadata.path.to_owned(),
                Role::RemovedParentTopic {
                    synonyms: SynonymInfo::from(&topic2),
                },
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
                Role::DeletedLink {
                    title: link.metadata.title.to_owned(),
                    url: link.metadata.url,
                },
            ),
            (
                topic1.metadata.path.to_owned(),
                Role::RemovedParentTopic {
                    synonyms: SynonymInfo::from(&topic1),
                },
            ),
            (
                topic2.metadata.path.to_owned(),
                Role::RemovedParentTopic {
                    synonyms: SynonymInfo::from(&topic2),
                },
            ),
        ]);

        let change = Change::DeleteLink(DeleteLink(Body {
            date: chrono::Utc::now(),
            paths,
            user_id: "2".to_owned(),
        }));

        assert_eq!(
            change.markdown(Locale::EN, "Gnusto", None).unwrap(),
            format!(
                "Gnusto deleted [Reddit](http://www.reddit.com), removing it from [Climate change]({}) and \
                [Weather]({})",
                topic1.metadata.path,
                topic2.metadata.path,
            )
        );
    }

    #[test]
    fn delete_topic() {
        let link = link("Reddit", "http://www.reddit.com");
        let topic1 = topic("Climate change");
        let topic2 = topic("Weather");

        let paths = BTreeMap::from([
            (
                link.metadata.path.to_owned(),
                Role::RemovedChildLink {
                    title: link.metadata.title.to_owned(),
                    url: link.metadata.url,
                },
            ),
            (
                topic1.metadata.path.to_owned(),
                Role::DeletedTopic {
                    synonyms: SynonymInfo::from(&topic1),
                },
            ),
            (
                topic2.metadata.path.to_owned(),
                Role::RemovedParentTopic {
                    synonyms: SynonymInfo::from(&topic2),
                },
            ),
        ]);

        let change = Change::DeleteTopic(DeleteTopic(Body {
            date: chrono::Utc::now(),
            paths,
            user_id: "2".to_owned(),
        }));

        assert_eq!(
            change.markdown(Locale::EN, "Gnusto", None).unwrap(),
            format!(
                "Gnusto deleted [Climate change]({}), removing [Reddit](http://www.reddit.com) from it, and \
                removing it from [Weather]({})",
                topic1.metadata.path,
                topic2.metadata.path,
            )
        );
    }

    #[test]
    fn upsert_topic_timerange() {
        let topic1 = topic("Climate change");

        let paths = BTreeMap::from([(
            topic1.metadata.path.to_owned(),
            Role::UpdatedTopic {
                synonyms: SynonymInfo::from(&topic1),
            },
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

    #[test]
    fn upsert_link() {
        let topic1 = topic("Climate change");
        let link = link("Reddit", "https://www.reddit.com");

        let paths = BTreeMap::from([
            (
                topic1.metadata.path.to_owned(),
                Role::AddedParentTopic {
                    synonyms: SynonymInfo::from(&topic1),
                },
            ),
            (
                link.metadata.path.to_owned(),
                Role::UpdatedLink {
                    title: link.title().to_owned(),
                    url: link.url().to_owned(),
                },
            ),
        ]);

        let change = Change::UpsertLink(UpsertLink(Body {
            date: chrono::Utc::now(),
            paths,
            user_id: "2".to_owned(),
        }));

        assert_eq!(
            change.markdown(Locale::EN, "Gnusto", None).unwrap(),
            format!(
                "Gnusto added [Reddit](https://www.reddit.com) to [Climate change]({})",
                topic1.metadata.path
            ),
        );
    }

    #[test]
    fn upsert_topic() {
        let topic1 = topic("Climate change");
        let topic2 = topic("Climate");

        let paths = BTreeMap::from([
            (
                topic1.metadata.path.to_owned(),
                Role::UpdatedTopic {
                    synonyms: SynonymInfo::from(&topic1),
                },
            ),
            (
                topic2.metadata.path.to_owned(),
                Role::AddedParentTopic {
                    synonyms: SynonymInfo::from(&topic2),
                },
            ),
        ]);

        let change = Change::UpsertTopic(UpsertTopic(Body {
            date: chrono::Utc::now(),
            paths,
            user_id: "2".to_owned(),
        }));

        assert_eq!(
            change.markdown(Locale::EN, "Gnusto", None).unwrap(),
            format!(
                "Gnusto added [Climate change]({}) to [Climate]({})",
                topic1.metadata.path, topic2.metadata.path
            ),
        );
    }
}
