use async_graphql::dataloader::*;
use sqlx::postgres::PgPool;

use super::{
    CreateGithubSession, CreateSessionResult, DeleteAccount, DeleteAccountResult, DeleteLink,
    DeleteLinkResult, DeleteSession, DeleteSessionResult, DeleteTopic, DeleteTopicResult,
    DeleteTopicTimeRange, DeleteTopicTimeRangeResult, FetchActivity, FetchChildLinksForTopic,
    FetchChildTopicsForTopic, FetchRepositoriesForUser, LinkLoader, LiveSearchTopics,
    OrganizationByLoginLoader, OrganizationLoader, RepositoryByNameLoader, RepositoryLoader,
    Search, SelectRepository, SelectRepositoryResult, TopicLoader, UpdateLinkParentTopics,
    UpdateLinkTopicsResult, UpdateSynonyms, UpdateSynonymsResult, UpdateTopicParentTopics,
    UpdateTopicParentTopicsResult, UpsertLink, UpsertLinkResult, UpsertTopic, UpsertTopicResult,
    UpsertTopicTimeRange, UpsertTopicTimeRangeResult, UserLoader,
};
use crate::prelude::*;
use crate::schema::{
    ActivityLineItem, CreateGithubSessionInput, Link, Organization, Repository, SearchResultItem,
    Topic, UpdateLinkTopicsInput, UpdateSynonymsInput, UpsertLinkInput, UpsertTopicInput,
    UpsertTopicTimeRangeInput, User, Viewer,
};

pub struct Repo {
    link_loader: DataLoader<LinkLoader, HashMapCache>,
    organization_by_login_loader: DataLoader<OrganizationByLoginLoader, HashMapCache>,
    organization_loader: DataLoader<OrganizationLoader, HashMapCache>,
    pool: PgPool,
    pub viewer: Viewer,
    repository_by_name_loader: DataLoader<RepositoryByNameLoader, HashMapCache>,
    repository_loader: DataLoader<RepositoryLoader, HashMapCache>,
    pub server_secret: String,
    topic_loader: DataLoader<TopicLoader, HashMapCache>,
    user_loader: DataLoader<UserLoader, HashMapCache>,
}

impl Repo {
    pub fn new(viewer: Viewer, pool: PgPool, server_secret: String) -> Self {
        let link_loader = LinkLoader::new(viewer.clone(), pool.clone());
        let organization_loader = OrganizationLoader::new(viewer.clone(), pool.clone());
        let organization_by_login_loader =
            OrganizationByLoginLoader::new(viewer.clone(), pool.clone());
        let repository_loader = RepositoryLoader::new(viewer.clone(), pool.clone());
        let repository_by_name_loader = RepositoryByNameLoader::new(viewer.clone(), pool.clone());
        let topic_loader = TopicLoader::new(viewer.clone(), pool.clone());
        let user_loader = UserLoader::new(viewer.clone(), pool.clone());

        Self {
            pool,
            viewer,
            server_secret,

            link_loader: DataLoader::with_cache(
                link_loader,
                actix_web::rt::spawn,
                HashMapCache::default(),
            ),
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
            topic_loader: DataLoader::with_cache(
                topic_loader,
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
        topic_id: Option<String>,
        first: i32,
    ) -> Result<Vec<ActivityLineItem>> {
        FetchActivity::new(self.viewer.clone(), topic_id, first)
            .call(&self.pool)
            .await
    }

    async fn flat_topics(&self, ids: &[String]) -> Result<Vec<Topic>> {
        let result = self.topics(ids).await?;
        Ok(result.iter().flatten().cloned().collect())
    }

    pub async fn child_links_for_topic(&self, topic_id: String) -> Result<Vec<Link>> {
        FetchChildLinksForTopic::new(self.viewer.query_ids.clone(), topic_id)
            .call(&self.pool)
            .await
    }

    pub async fn child_topics_for_topic(&self, topic_id: String) -> Result<Vec<Topic>> {
        FetchChildTopicsForTopic::new(self.viewer.clone(), topic_id)
            .call(&self.pool)
            .await
    }

    pub async fn delete_account(&self, user_id: String) -> Result<DeleteAccountResult> {
        DeleteAccount::new(self.viewer.clone(), user_id)
            .call(&self.pool)
            .await
    }

    pub async fn delete_link(&self, link_id: String) -> Result<DeleteLinkResult> {
        DeleteLink::new(self.viewer.clone(), link_id)
            .call(&self.pool)
            .await
    }

    pub async fn delete_session(&self, session_id: String) -> Result<DeleteSessionResult> {
        DeleteSession::new(self.viewer.clone(), session_id)
            .call(&self.pool)
            .await
    }

    pub async fn delete_topic(&self, topic_id: String) -> Result<DeleteTopicResult> {
        DeleteTopic::new(self.viewer.clone(), topic_id)
            .call(&self.pool)
            .await
    }

    pub async fn delete_topic_time_range(
        &self,
        topic_id: String,
    ) -> Result<DeleteTopicTimeRangeResult> {
        DeleteTopicTimeRange::new(self.viewer.clone(), topic_id)
            .call(&self.pool)
            .await
    }

    pub async fn link(&self, id: String) -> Result<Option<Link>> {
        self.link_loader.load_one(id).await
    }

    pub async fn link_count(&self) -> Result<i64> {
        let (count,) = sqlx::query_as::<_, (i64,)>(
            r#"select count(*)
            from links l
            join organization_members om on l.organization_id = om.organization_id
            where om.user_id = any($1::uuid[])"#,
        )
        .bind(&self.viewer.query_ids)
        .fetch_one(&self.pool)
        .await?;
        Ok(count)
    }

    pub async fn organization(&self, id: String) -> Result<Option<Organization>> {
        self.organization_loader.load_one(id).await
    }

    pub async fn organization_by_login(&self, login: String) -> Result<Option<Organization>> {
        self.organization_by_login_loader.load_one(login).await
    }

    pub async fn parent_topics_for_topic(&self, topic_id: String) -> Result<Vec<Topic>> {
        let topic = self.topic(topic_id).await?;
        match topic {
            Some(topic) => self.flat_topics(&topic.parent_topic_ids).await,
            None => Ok(vec![]),
        }
    }

    pub async fn parent_topics_for_link(&self, link_id: String) -> Result<Vec<Topic>> {
        let link = self.link(link_id).await?;
        match link {
            Some(link) => self.flat_topics(&link.parent_topic_ids).await,
            None => Ok(vec![]),
        }
    }

    pub async fn repositories_for_user(&self, user_id: String) -> Result<Vec<Repository>> {
        FetchRepositoriesForUser::new(self.viewer.clone(), user_id)
            .call(&self.pool)
            .await
    }

    pub async fn repository(&self, id: String) -> Result<Option<Repository>> {
        self.repository_loader.load_one(id).await
    }

    pub async fn repository_by_name(&self, name: String) -> Result<Option<Repository>> {
        self.repository_by_name_loader.load_one(name).await
    }

    pub async fn search(
        &self,
        parent_topic: Topic,
        search_string: String,
    ) -> Result<Vec<SearchResultItem>> {
        Search::new(
            self.viewer.query_ids.clone(),
            parent_topic,
            search_string.clone(),
        )
        .call(&self.pool)
        .await
    }

    pub async fn search_topics(&self, search_string: Option<String>) -> Result<Vec<Topic>> {
        LiveSearchTopics::new(self.viewer.query_ids.clone(), search_string.clone())
            .call(&self.pool)
            .await
    }

    pub async fn select_repository(
        &self,
        repository_id: Option<String>,
    ) -> Result<SelectRepositoryResult> {
        SelectRepository::new(self.viewer.clone(), repository_id)
            .call(&self.pool)
            .await
    }

    pub async fn topic(&self, id: String) -> Result<Option<Topic>> {
        self.topic_loader.load_one(id).await
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
        .fetch_one(&self.pool)
        .await?;
        Ok(count)
    }

    pub async fn topics(&self, ids: &[String]) -> Result<Vec<Option<Topic>>> {
        let ids: Vec<String> = ids.iter().map(String::to_string).collect();
        let map = self.topic_loader.load_many(ids.clone()).await?;
        let mut topics: Vec<Option<Topic>> = Vec::new();
        for id in ids {
            let topic = map.get(&id).cloned();
            topics.push(topic);
        }
        Ok(topics)
    }

    pub async fn update_link_topics(
        &self,
        input: UpdateLinkTopicsInput,
    ) -> Result<UpdateLinkTopicsResult> {
        UpdateLinkParentTopics::new(self.viewer.clone(), input)
            .call(&self.pool)
            .await
    }

    pub async fn update_synonyms(
        &self,
        input: UpdateSynonymsInput,
    ) -> Result<UpdateSynonymsResult> {
        UpdateSynonyms::new(self.viewer.clone(), input)
            .call(&self.pool)
            .await
    }

    pub async fn upsert_link(&self, input: UpsertLinkInput) -> Result<UpsertLinkResult> {
        UpsertLink::new(self.viewer.clone(), input)
            .call(&self.pool)
            .await
    }

    pub async fn update_topic_parent_topics(
        &self,
        topic_id: String,
        parent_topic_ids: Vec<String>,
    ) -> Result<UpdateTopicParentTopicsResult> {
        UpdateTopicParentTopics::new(self.viewer.clone(), topic_id, parent_topic_ids)
            .call(&self.pool)
            .await
    }

    pub async fn upsert_session(
        &self,
        input: CreateGithubSessionInput,
    ) -> Result<CreateSessionResult> {
        CreateGithubSession::new(input).call(&self.pool).await
    }

    pub async fn upsert_topic(&self, input: UpsertTopicInput) -> Result<UpsertTopicResult> {
        UpsertTopic::new(self.viewer.clone(), input)
            .call(&self.pool)
            .await
    }

    pub async fn upsert_topic_time_range(
        &self,
        input: UpsertTopicTimeRangeInput,
    ) -> Result<UpsertTopicTimeRangeResult> {
        UpsertTopicTimeRange::new(self.viewer.clone(), input)
            .call(&self.pool)
            .await
    }

    pub async fn user(&self, id: String) -> Result<Option<User>> {
        self.user_loader.load_one(id).await
    }
}
