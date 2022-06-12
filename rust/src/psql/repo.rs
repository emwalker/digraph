use async_graphql::dataloader::*;
use sqlx::postgres::PgPool;

use super::{
    CreateGithubSession, CreateSessionResult, FetchChildLinksForTopic, FetchChildTopicsForTopic,
    LinkLoader, LiveSearchTopics, OrganizationByLoginLoader, OrganizationLoader,
    RepositoryByNameLoader, RepositoryLoader, Search, TopicLoader, UpdateLinkParentTopics,
    UpdateLinkTopicsResult, UpsertLink, UpsertLinkResult, UpsertTopic, UpsertTopicResult,
    UserLoader,
};
use crate::prelude::*;
use crate::schema::{
    CreateGithubSessionInput, Link, Organization, Repository, SearchResultItem, Topic,
    UpdateLinkTopicsInput, UpsertLinkInput, UpsertTopicInput, User, Viewer,
};

pub struct Repo {
    link_loader: DataLoader<LinkLoader, HashMapCache>,
    organization_by_login_loader: DataLoader<OrganizationByLoginLoader, HashMapCache>,
    organization_loader: DataLoader<OrganizationLoader, HashMapCache>,
    pool: PgPool,
    pub viewer: Viewer,
    repository_by_name_loader: DataLoader<RepositoryByNameLoader, HashMapCache>,
    repository_loader: DataLoader<RepositoryLoader, HashMapCache>,
    topic_loader: DataLoader<TopicLoader, HashMapCache>,
    user_loader: DataLoader<UserLoader, HashMapCache>,
}

impl Repo {
    pub fn new(viewer: Viewer, pool: PgPool) -> Self {
        let link_loader = LinkLoader::new(viewer.clone(), pool.clone());
        let organization_loader = OrganizationLoader::new(pool.clone());
        let organization_by_login_loader = OrganizationByLoginLoader::new(pool.clone());
        let repository_loader = RepositoryLoader::new(pool.clone());
        let repository_by_name_loader = RepositoryByNameLoader::new(pool.clone());
        let topic_loader = TopicLoader::new(viewer.clone(), pool.clone());
        let user_loader = UserLoader::new(pool.clone());

        Self {
            pool,
            viewer,
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
        FetchChildTopicsForTopic::new(self.viewer.query_ids.clone(), topic_id)
            .call(&self.pool)
            .await
    }

    pub async fn link(&self, id: String) -> Result<Option<Link>> {
        self.link_loader.load_one(id).await
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

    pub async fn repository(&self, id: String) -> Result<Option<Repository>> {
        self.repository_loader.load_one(id).await.map_err(Error::DB)
    }

    pub async fn repository_by_name(&self, name: String) -> Result<Option<Repository>> {
        self.repository_by_name_loader
            .load_one(name)
            .await
            .map_err(Error::DB)
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

    pub async fn topic(&self, id: String) -> Result<Option<Topic>> {
        self.topic_loader.load_one(id).await
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
        UpdateLinkParentTopics::new(input).call(&self.pool).await
    }

    pub async fn upsert_link(&self, input: UpsertLinkInput) -> Result<UpsertLinkResult> {
        UpsertLink::new(input).call(&self.pool).await
    }

    pub async fn upsert_session(
        &self,
        input: CreateGithubSessionInput,
    ) -> Result<CreateSessionResult> {
        CreateGithubSession::new(input).call(&self.pool).await
    }

    pub async fn upsert_topic(&self, input: UpsertTopicInput) -> Result<UpsertTopicResult> {
        UpsertTopic::new(input).call(&self.pool).await
    }

    pub async fn user(&self, id: String) -> Result<Option<User>> {
        self.user_loader.load_one(id).await.map_err(Error::DB)
    }
}
