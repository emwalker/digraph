use async_graphql::dataloader::*;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use sqlx::postgres::PgPool;

use crate::git;
use crate::graphql::{
    self, ActivityLineItem, CreateGithubSessionInput, Link, Organization, Repository, Topic,
    TopicChild, UpdateLinkTopicsInput, UpdateSynonymsInput, UpsertTopicInput,
    UpsertTopicTimeRangeInput, User, Viewer,
};
use crate::http;
use crate::prelude::*;
use crate::psql;
use crate::psql::{
    CreateGithubSession, CreateSessionResult, DeleteAccount, DeleteAccountResult, DeleteLink,
    DeleteLinkResult, DeleteSession, DeleteSessionResult, DeleteTopic, DeleteTopicResult,
    DeleteTopicTimeRange, DeleteTopicTimeRangeResult, FetchActivity, FetchRepositoriesForUser,
    LiveSearchTopics, ReviewLink, ReviewLinkResult, Search, SelectRepository,
    SelectRepositoryResult, UpdateLinkParentTopics, UpdateLinkTopicsResult, UpdateSynonyms,
    UpdateSynonymsResult, UpdateTopicParentTopics, UpdateTopicParentTopicsResult,
    UpsertTopicTimeRange, UpsertTopicTimeRangeResult,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RepoPath {
    pub inner: String,
    pub org_login: String,
    pub prefix: String,
    pub short_id: String,
    pub valid: bool,
}

impl std::fmt::Display for RepoPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl From<&String> for RepoPath {
    fn from(input: &String) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(/([\w-]+))/([\w-]+)$").unwrap();
        }

        let cap = match RE.captures(input) {
            Some(cap) => cap,
            _ => return Self::invalid_path(input),
        };

        let (prefix, org_login, short_id) = match (cap.get(1), cap.get(2), cap.get(3)) {
            (Some(prefix), Some(org_login), Some(short_id)) => {
                (prefix.as_str(), org_login.as_str(), short_id.as_str())
            }
            _ => return Self::invalid_path(input),
        };

        RepoPath {
            inner: input.to_string(),
            org_login: org_login.to_string(),
            prefix: prefix.to_string(),
            short_id: short_id.to_string(),
            valid: true,
        }
    }
}

impl From<&str> for RepoPath {
    fn from(input: &str) -> Self {
        Self::from(&input.to_string())
    }
}

impl RepoPath {
    fn invalid_path(input: &String) -> Self {
        Self {
            inner: input.clone(),
            org_login: "wiki".into(),
            prefix: "wiki".into(),
            short_id: input.into(),
            valid: false,
        }
    }
}

pub struct Repo {
    db: PgPool,
    git: git::Git,
    object_loader: DataLoader<git::ObjectLoader, HashMapCache>,
    organization_by_login_loader: DataLoader<psql::OrganizationByLoginLoader, HashMapCache>,
    organization_loader: DataLoader<psql::OrganizationLoader, HashMapCache>,
    pub server_secret: String,
    pub viewer: Viewer,
    repository_by_name_loader: DataLoader<psql::RepositoryByNameLoader, HashMapCache>,
    repository_by_prefix_loader: DataLoader<psql::RepositoryByPrefixLoader, HashMapCache>,
    repository_loader: DataLoader<psql::RepositoryLoader, HashMapCache>,
    user_loader: DataLoader<psql::UserLoader, HashMapCache>,
}

impl Repo {
    pub fn new(viewer: Viewer, git: git::Git, db: PgPool, server_secret: String) -> Self {
        let organization_loader = psql::OrganizationLoader::new(viewer.clone(), db.clone());
        let organization_by_login_loader =
            psql::OrganizationByLoginLoader::new(viewer.clone(), db.clone());
        let repository_loader = psql::RepositoryLoader::new(viewer.clone(), db.clone());
        let repository_by_name_loader =
            psql::RepositoryByNameLoader::new(viewer.clone(), db.clone());
        let repository_by_prefix_loader =
            psql::RepositoryByPrefixLoader::new(viewer.clone(), db.clone());
        let object_loader = git::ObjectLoader::new(viewer.clone(), git.clone());
        let user_loader = psql::UserLoader::new(viewer.clone(), db.clone());

        Self {
            db,
            git,
            viewer,
            server_secret,

            organization_loader: DataLoader::with_cache(
                organization_loader,
                actix_web::rt::spawn,
                HashMapCache::default(),
            ),
            organization_by_login_loader: DataLoader::with_cache(
                organization_by_login_loader,
                actix_web::rt::spawn,
                HashMapCache::default(),
            ),
            repository_loader: DataLoader::with_cache(
                repository_loader,
                actix_web::rt::spawn,
                HashMapCache::default(),
            ),
            repository_by_name_loader: DataLoader::with_cache(
                repository_by_name_loader,
                actix_web::rt::spawn,
                HashMapCache::default(),
            ),
            repository_by_prefix_loader: DataLoader::with_cache(
                repository_by_prefix_loader,
                actix_web::rt::spawn,
                HashMapCache::default(),
            ),
            object_loader: DataLoader::with_cache(
                object_loader,
                actix_web::rt::spawn,
                HashMapCache::default(),
            ),
            user_loader: DataLoader::with_cache(
                user_loader,
                actix_web::rt::spawn,
                HashMapCache::default(),
            ),
        }
    }
}

impl Repo {
    pub async fn activity(
        &self,
        topic_path: Option<String>,
        first: i32,
    ) -> Result<Vec<ActivityLineItem>> {
        let topic_path = topic_path.map(|s| RepoPath::from(&s));
        FetchActivity::new(self.viewer.clone(), topic_path, first)
            .call(&self.db)
            .await
    }

    async fn flat_topics(&self, paths: &[RepoPath]) -> Result<Vec<Topic>> {
        let result = self.topics(paths).await?;
        Ok(result.iter().flatten().cloned().collect())
    }

    pub async fn child_links_for_topic(
        &self,
        parent_topic: &RepoPath,
        _reviewed: Option<bool>,
    ) -> Result<Vec<Link>> {
        let children = self.topic_children(parent_topic).await?;
        let mut links = vec![];

        for child in &children {
            if let TopicChild::Link(link) = &child {
                links.push(link.to_owned());
            }
        }

        Ok(links)
    }

    pub async fn topic_children(&self, parent_topic: &RepoPath) -> Result<Vec<TopicChild>> {
        let topic = self
            .topic(parent_topic)
            .await?
            .ok_or_else(|| Error::NotFound(format!("no topic: {}", parent_topic)))?;
        let child_paths = topic
            .child_paths
            .iter()
            .map(RepoPath::to_string)
            .collect_vec();
        let map = self.object_loader.load_many(child_paths.clone()).await?;

        let mut children = vec![];

        for child_path in &child_paths {
            let child = map
                .get(child_path)
                .ok_or_else(|| Error::NotFound(format!("no child: {}", child_path)))?;

            let child = match child {
                git::Object::Topic(topic) => TopicChild::Topic(Topic::from(topic)),
                git::Object::Link(link) => TopicChild::Link(Link::from(link)),
            };

            children.push(child);
        }

        Ok(children)
    }

    pub async fn delete_account(&self, user_id: String) -> Result<DeleteAccountResult> {
        DeleteAccount::new(self.viewer.clone(), user_id)
            .call(&self.db)
            .await
    }

    pub async fn delete_link(&self, link_path: &RepoPath) -> Result<DeleteLinkResult> {
        DeleteLink::new(self.viewer.clone(), link_path.clone())
            .call(&self.db)
            .await
    }

    pub async fn delete_session(&self, session_id: String) -> Result<DeleteSessionResult> {
        DeleteSession::new(self.viewer.clone(), session_id)
            .call(&self.db)
            .await
    }

    pub async fn delete_topic(&self, path: &RepoPath) -> Result<DeleteTopicResult> {
        DeleteTopic::new(self.viewer.clone(), path.clone())
            .call(&self.db)
            .await
    }

    pub async fn delete_topic_time_range(
        &self,
        topic_path: &RepoPath,
    ) -> Result<DeleteTopicTimeRangeResult> {
        DeleteTopicTimeRange::new(self.viewer.clone(), topic_path.clone())
            .call(&self.db)
            .await
    }

    pub async fn link(&self, path: &RepoPath) -> Result<Option<Link>> {
        let result = self
            .object_loader
            .load_one(path.to_string())
            .await?
            .ok_or_else(|| Error::NotFound(format!("no link: {}", path)))?;

        match result {
            git::Object::Link(link) => Ok(Some(Link::from(&link))),
            _ => return Err(Error::NotFound(format!("no link: {}", path))),
        }
    }

    pub async fn link_count(&self) -> Result<i64> {
        let (count,) = sqlx::query_as::<_, (i64,)>(
            r#"select count(*)
            from links l
            join organization_members om on l.organization_id = om.organization_id
            where om.user_id = any($1::uuid[])"#,
        )
        .bind(&self.viewer.query_ids)
        .fetch_one(&self.db)
        .await?;
        Ok(count)
    }

    pub async fn organization(&self, id: String) -> Result<Option<Organization>> {
        self.organization_loader.load_one(id).await
    }

    pub async fn organization_by_login(&self, login: String) -> Result<Option<Organization>> {
        self.organization_by_login_loader.load_one(login).await
    }

    pub async fn parent_topics_for_topic(&self, path: &RepoPath) -> Result<Vec<Topic>> {
        let topic = self
            .topic(path)
            .await?
            .ok_or_else(|| Error::NotFound(format!("no topic for id: {}", path)))?;
        self.flat_topics(&topic.parent_topic_paths).await
    }

    pub async fn parent_topics_for_link(&self, path: &RepoPath) -> Result<Vec<Topic>> {
        let link = self
            .link(path)
            .await?
            .ok_or_else(|| Error::NotFound(format!("no link for id: {}", path)))?;
        self.flat_topics(&link.parent_topic_paths).await
    }

    pub async fn repositories_for_user(&self, user_id: String) -> Result<Vec<Repository>> {
        FetchRepositoriesForUser::new(self.viewer.clone(), user_id)
            .call(&self.db)
            .await
    }

    pub async fn repository(&self, id: String) -> Result<Option<Repository>> {
        self.repository_loader.load_one(id).await
    }

    pub async fn repository_by_prefix(&self, prefix: String) -> Result<Option<Repository>> {
        self.repository_by_prefix_loader.load_one(prefix).await
    }

    pub async fn repository_by_name(&self, name: String) -> Result<Option<Repository>> {
        self.repository_by_name_loader.load_one(name).await
    }

    pub async fn review_link(&self, link: &RepoPath, reviewed: bool) -> Result<ReviewLinkResult> {
        ReviewLink::new(self.viewer.clone(), link.clone(), reviewed)
            .call(&self.db)
            .await
    }

    pub async fn search(
        &self,
        parent_topic: Topic,
        search_string: String,
    ) -> Result<Vec<TopicChild>> {
        Search::new(
            self.viewer.query_ids.clone(),
            parent_topic,
            search_string.clone(),
        )
        .call(&self.db)
        .await
    }

    pub async fn search_topics(&self, search_string: Option<String>) -> Result<Vec<Topic>> {
        LiveSearchTopics::new(self.viewer.query_ids.clone(), search_string.clone())
            .call(&self.db)
            .await
    }

    pub async fn select_repository(
        &self,
        repository_id: Option<String>,
    ) -> Result<SelectRepositoryResult> {
        SelectRepository::new(self.viewer.clone(), repository_id)
            .call(&self.db)
            .await
    }

    pub async fn topic(&self, path: &RepoPath) -> Result<Option<Topic>> {
        let result = self
            .object_loader
            .load_one(path.to_string())
            .await?
            .ok_or_else(|| Error::NotFound(format!("no topic: {}", path)))?;

        match result {
            git::Object::Topic(topic) => Ok(Some(Topic::from(&topic))),
            _ => return Err(Error::NotFound(format!("no topic: {}", path))),
        }
    }

    pub async fn topic_count(&self) -> Result<i64> {
        let (count,) = sqlx::query_as::<_, (i64,)>(
            r#"select count(*)
            from topics t
            join organization_members om on t.organization_id = om.organization_id
            where om.user_id = any($1::uuid[])
            "#,
        )
        .bind(&self.viewer.query_ids)
        .fetch_one(&self.db)
        .await?;
        Ok(count)
    }

    pub async fn topics(&self, paths: &[RepoPath]) -> Result<Vec<Option<Topic>>> {
        let paths = paths.iter().map(|p| p.to_string()).collect::<Vec<String>>();
        let map = self.object_loader.load_many(paths.clone()).await?;
        let mut topics: Vec<Option<Topic>> = Vec::new();
        for path in paths {
            let topic = map
                .get(&path)
                .ok_or_else(|| Error::NotFound(format!("no topic: {}", path)))?;

            let topic = match &topic {
                git::Object::Topic(topic) => Some(Topic::from(topic)),
                _ => None,
            };

            topics.push(topic);
        }
        Ok(topics)
    }

    pub async fn update_link_topics(
        &self,
        input: UpdateLinkTopicsInput,
    ) -> Result<UpdateLinkTopicsResult> {
        UpdateLinkParentTopics::new(self.viewer.clone(), input)
            .call(&self.db)
            .await
    }

    pub async fn update_synonyms(
        &self,
        input: UpdateSynonymsInput,
    ) -> Result<UpdateSynonymsResult> {
        UpdateSynonyms::new(self.viewer.clone(), input)
            .call(&self.db)
            .await
    }

    pub async fn upsert_link(
        &self,
        input: graphql::UpsertLinkInput,
    ) -> Result<git::UpsertLinkResult> {
        let add_parent_topic_paths = input
            .add_parent_topic_paths
            .iter()
            .map(RepoPath::from)
            .collect_vec();

        git::UpsertLink {
            actor: self.viewer.clone(),
            add_parent_topic_paths,
            prefix: "/wiki".to_owned(),
            title: input.title,
            url: input.url,
            fetcher: Box::new(http::Fetcher),
        }
        .call(&self.git)
        .await
    }

    pub async fn update_topic_parent_topics(
        &self,
        topic: &RepoPath,
        parent_topics: Vec<RepoPath>,
    ) -> Result<UpdateTopicParentTopicsResult> {
        UpdateTopicParentTopics::new(self.viewer.clone(), topic.clone(), parent_topics)
            .call(&self.db)
            .await
    }

    pub async fn upsert_session(
        &self,
        input: CreateGithubSessionInput,
    ) -> Result<CreateSessionResult> {
        CreateGithubSession::new(input).call(&self.db).await
    }

    pub async fn upsert_topic(&self, input: UpsertTopicInput) -> Result<git::UpsertTopicResult> {
        git::UpsertTopic {
            actor: self.viewer.clone(),
            locale: "en".to_owned(),
            name: input.name.to_owned(),
            on_matching_synonym: git::OnMatchingSynonym::Ask,
            prefix: "/wiki".to_owned(),
            parent_topic: RepoPath::from(&input.parent_topic_path),
        }
        .call(&self.git)
    }

    pub async fn upsert_topic_time_range(
        &self,
        input: UpsertTopicTimeRangeInput,
    ) -> Result<UpsertTopicTimeRangeResult> {
        UpsertTopicTimeRange::new(self.viewer.clone(), input)
            .call(&self.db)
            .await
    }

    pub async fn user(&self, id: String) -> Result<Option<User>> {
        self.user_loader.load_one(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let path = RepoPath::from("/wiki/00001");
        assert!(path.valid);
        assert_eq!("/wiki/00001", path.inner);
        assert_eq!("/wiki", path.prefix);
        assert_eq!("wiki", path.org_login);
        assert_eq!("00001", path.short_id);
    }
}
