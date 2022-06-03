use async_graphql::dataloader::*;
use async_graphql::Result;
use sqlx::postgres::PgPool;

use super::link::LinkLoader;
use super::organization::OrganizationLoader;
use super::repository::RepositoryLoader;
use super::topic::TopicLoader;
use crate::schema::{Link, Organization, Repository, Topic};

pub struct Repo {
    link_loader: DataLoader<LinkLoader, HashMapCache>,
    organization_loader: DataLoader<OrganizationLoader, HashMapCache>,
    repository_loader: DataLoader<RepositoryLoader, HashMapCache>,
    topic_loader: DataLoader<TopicLoader, HashMapCache>,
}

impl Repo {
    pub fn new(pool: PgPool) -> Self {
        let link_loader = LinkLoader::new(pool.clone());
        let organization_loader = OrganizationLoader::new(pool.clone());
        let repository_loader = RepositoryLoader::new(pool.clone());
        let topic_loader = TopicLoader::new(pool);

        Self {
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
            repository_loader: DataLoader::with_cache(
                repository_loader,
                actix_web::rt::spawn,
                HashMapCache::default(),
            ),
            topic_loader: DataLoader::with_cache(
                topic_loader,
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

    async fn flat_links(&self, ids: &[String]) -> Result<Vec<Link>> {
        let result = self.links(ids).await?;
        Ok(result.iter().flatten().cloned().collect())
    }

    pub async fn child_links_for_topic(&self, topic_id: String) -> Result<Vec<Link>> {
        let topic = self.topic(topic_id).await?;
        match topic {
            Some(topic) => self.flat_links(&topic.child_link_ids).await,
            None => Ok(vec![]),
        }
    }

    pub async fn child_topics_for_topic(&self, topic_id: String) -> Result<Vec<Topic>> {
        let topic = self.topic(topic_id).await?;
        match topic {
            Some(topic) => self.flat_topics(&topic.child_topic_ids).await,
            None => Ok(vec![]),
        }
    }

    pub async fn link(&self, id: String) -> Result<Option<Link>> {
        Ok(self.link_loader.load_one(id).await?)
    }

    pub async fn links(&self, ids: &[String]) -> Result<Vec<Option<Link>>> {
        let ids: Vec<String> = ids.iter().map(String::to_string).collect();
        let map = self.link_loader.load_many(ids.clone()).await?;
        let mut links: Vec<Option<Link>> = Vec::new();
        for id in ids {
            let link = map.get(&id).cloned();
            links.push(link);
        }
        Ok(links)
    }

    pub async fn organization(&self, id: String) -> Result<Option<Organization>> {
        Ok(self.organization_loader.load_one(id).await?)
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
        Ok(self.repository_loader.load_one(id).await?)
    }

    pub async fn topic(&self, id: String) -> Result<Option<Topic>> {
        Ok(self.topic_loader.load_one(id).await?)
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
}
