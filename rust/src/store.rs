use async_graphql::dataloader::*;
use geotime::Geotime;
use itertools::Itertools;
use sqlx::postgres::PgPool;
use std::collections::BTreeSet;

use crate::git;
use crate::graphql;
use crate::http;
use crate::prelude::*;
use crate::psql;
use crate::redis;

pub struct Store {
    db: PgPool,
    git: git::Git,
    object_loader: DataLoader<graphql::ObjectLoader, HashMapCache>,
    organization_by_login_loader: DataLoader<psql::OrganizationByLoginLoader, HashMapCache>,
    organization_loader: DataLoader<psql::OrganizationLoader, HashMapCache>,
    pub server_secret: String,
    pub user_loader: DataLoader<psql::UserLoader, HashMapCache>,
    pub viewer: Viewer,
    redis: redis::Redis,
    repository_by_name_loader: DataLoader<psql::RepositoryByNameLoader, HashMapCache>,
    repository_by_prefix_loader: DataLoader<psql::RepositoryByPrefixLoader, HashMapCache>,
    repository_loader: DataLoader<psql::RepositoryLoader, HashMapCache>,
}

impl Store {
    pub fn new(
        viewer: Viewer,
        git: git::Git,
        db: PgPool,
        server_secret: String,
        redis: redis::Redis,
    ) -> Self {
        let organization_loader = psql::OrganizationLoader::new(viewer.clone(), db.clone());
        let organization_by_login_loader =
            psql::OrganizationByLoginLoader::new(viewer.clone(), db.clone());
        let repository_loader = psql::RepositoryLoader::new(viewer.clone(), db.clone());
        let repository_by_name_loader =
            psql::RepositoryByNameLoader::new(viewer.clone(), db.clone());
        let repository_by_prefix_loader =
            psql::RepositoryByPrefixLoader::new(viewer.clone(), db.clone());
        let object_loader = graphql::ObjectLoader::new(viewer.clone(), git.clone());
        let user_loader = psql::UserLoader::new(viewer.clone(), db.clone());

        Self {
            db,
            git,
            redis,
            server_secret,
            viewer,

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

impl Store {
    pub async fn activity(
        &self,
        topic_path: Option<String>,
        first: i32,
    ) -> Result<Vec<git::activity::Change>> {
        let topic_path = topic_path.map(|s| RepoPath::from(&s));
        let result = git::activity::FetchActivity {
            actor: self.viewer.clone(),
            topic_path,
            first: first.try_into().unwrap_or(3),
        }
        .call(&self.git, &self.redis);

        match result {
            Ok(git::activity::FetchActivityResult { changes, .. }) => Ok(changes),
            Err(err) => {
                log::error!("problem fetching activity: {}", err);
                Ok(vec![])
            }
        }
    }

    async fn flat_topics(&self, paths: &[RepoPath]) -> Result<Vec<graphql::Topic>> {
        let result = self.topics(paths).await?;
        Ok(result.iter().flatten().cloned().collect())
    }

    pub async fn child_links_for_topic(
        &self,
        parent_topic: &RepoPath,
        _reviewed: Option<bool>,
    ) -> Result<Vec<graphql::Link>> {
        let children = self.topic_children(parent_topic).await?;
        let mut links = vec![];

        for child in children.iter().take(50) {
            let git::SearchMatch { object, .. } = &child;
            if let git::Object::Link(link) = object {
                links.push(graphql::Link::from(link));
            }
        }

        Ok(links)
    }

    pub async fn topic_children(
        &self,
        parent_topic: &RepoPath,
    ) -> Result<BTreeSet<git::SearchMatch>> {
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

        let empty = git::Search::empty();
        let mut children = BTreeSet::new();

        for child_path in child_paths.iter().take(50) {
            let child = map.get(child_path);
            if child.is_none() {
                // Probably not visible to the current user
                continue;
            }

            let child = child.unwrap().to_owned();
            children.insert(child.to_search_match(Locale::EN, &empty));
        }

        Ok(children)
    }

    pub async fn delete_account(&self, user_id: String) -> Result<psql::DeleteAccountResult> {
        psql::DeleteAccount::new(self.viewer.clone(), user_id)
            .call(&self.db)
            .await
    }

    pub async fn delete_link(&self, link_path: &RepoPath) -> Result<git::DeleteLinkResult> {
        git::DeleteLink {
            actor: self.viewer.clone(),
            link_path: link_path.clone(),
        }
        .call(&self.git, &self.redis)
    }

    pub async fn delete_session(&self, session_id: String) -> Result<psql::DeleteSessionResult> {
        psql::DeleteSession::new(self.viewer.clone(), session_id)
            .call(&self.db)
            .await
    }

    pub async fn delete_topic(&self, path: &RepoPath) -> Result<git::DeleteTopicResult> {
        git::DeleteTopic {
            actor: self.viewer.clone(),
            topic_path: path.clone(),
        }
        .call(&self.git, &self.redis)
    }

    pub async fn remove_topic_timerange(
        &self,
        topic_path: &RepoPath,
    ) -> Result<git::RemoveTopicTimerangeResult> {
        git::RemoveTopicTimerange {
            actor: self.viewer.clone(),
            topic_path: topic_path.clone(),
        }
        .call(&self.git, &self.redis)
    }

    pub async fn link(&self, path: &RepoPath) -> Result<Option<graphql::Link>> {
        let result = self
            .object_loader
            .load_one(path.to_string())
            .await?
            .ok_or_else(|| Error::NotFound(format!("no link: {}", path)))?;

        match result {
            git::Object::Link(link) => Ok(Some(graphql::Link::from(&link))),
            _ => return Err(Error::NotFound(format!("no link: {}", path))),
        }
    }

    pub async fn link_count(&self) -> Result<i64> {
        let git::FetchLinkCountResult { count, .. } = git::FetchLinkCount {
            actor: self.viewer.clone(),
        }
        .call(&self.git)?;
        Ok(count.try_into().unwrap_or_default())
    }

    pub async fn organization(&self, id: String) -> Result<Option<graphql::Organization>> {
        self.organization_loader.load_one(id).await
    }

    pub async fn organization_by_login(
        &self,
        login: String,
    ) -> Result<Option<graphql::Organization>> {
        self.organization_by_login_loader.load_one(login).await
    }

    pub async fn parent_topics_for_topic(&self, path: &RepoPath) -> Result<Vec<graphql::Topic>> {
        let topic = self
            .topic(path)
            .await?
            .ok_or_else(|| Error::NotFound(format!("no topic for id: {}", path)))?;
        self.flat_topics(&topic.parent_topic_paths).await
    }

    pub async fn parent_topics_for_link(&self, path: &RepoPath) -> Result<Vec<graphql::Topic>> {
        let link = self
            .link(path)
            .await?
            .ok_or_else(|| Error::NotFound(format!("no link for id: {}", path)))?;
        self.flat_topics(&link.parent_topic_paths).await
    }

    pub async fn repositories_for_user(&self, user_id: String) -> Result<Vec<graphql::Repository>> {
        psql::FetchWriteableRepositoriesForUser::new(self.viewer.clone(), user_id)
            .call(&self.db)
            .await
    }

    pub async fn repository(&self, id: String) -> Result<Option<graphql::Repository>> {
        self.repository_loader.load_one(id).await
    }

    pub async fn repository_by_prefix(
        &self,
        prefix: String,
    ) -> Result<Option<graphql::Repository>> {
        self.repository_by_prefix_loader.load_one(prefix).await
    }

    pub async fn repository_by_name(&self, name: String) -> Result<Option<graphql::Repository>> {
        self.repository_by_name_loader.load_one(name).await
    }

    pub async fn review_link(
        &self,
        link_path: &RepoPath,
        reviewed: bool,
    ) -> Result<psql::ReviewLinkResult> {
        let object = self
            .object_loader
            .load_one(link_path.inner.to_owned())
            .await?;

        match object {
            Some(git::Object::Link(link)) => {
                psql::ReviewLink::new(self.viewer.clone(), link.clone(), reviewed)
                    .call(&self.db)
                    .await
            }

            Some(other) => Err(Error::Repo(format!("expected a link: {:?}", other))),

            None => Err(Error::Repo(format!("no link found: {}", link_path))),
        }
    }

    pub async fn search(
        &self,
        parent_topic: graphql::Topic,
        search_string: String,
    ) -> Result<Vec<graphql::TopicChild>> {
        let fetcher = git::RedisFetchDownSet {
            git: self.git.clone(),
            redis: self.redis.clone(),
        };

        let git::SearchWithinTopicResult { matches, .. } = git::SearchWithinTopic {
            limit: 100,
            locale: Locale::EN,
            prefixes: vec![RepoPrefix::from("/wiki/")],
            recursive: true,
            search: git::Search::parse(&search_string)?,
            topic_path: parent_topic.path,
            viewer: self.viewer.clone(),
        }
        .call(&self.git, &fetcher)?;

        let mut results = vec![];
        for row in matches {
            results.push(graphql::TopicChild::try_from(&row.object)?);
        }

        Ok(results)
    }

    pub async fn search_topics(
        &self,
        search_string: Option<String>,
    ) -> Result<git::FetchTopicLiveSearchResult> {
        let search = git::Search::parse(&search_string.unwrap_or_default())?;
        git::FetchTopicLiveSearch {
            limit: 10,
            viewer: self.viewer.to_owned(),
            prefixes: vec![RepoPrefix::from("/wiki/")],
            search,
        }
        .call(&self.git)
    }

    pub async fn select_repository(
        &self,
        repository_id: Option<String>,
    ) -> Result<psql::SelectRepositoryResult> {
        psql::SelectRepository::new(self.viewer.clone(), repository_id)
            .call(&self.db)
            .await
    }

    pub async fn topic(&self, path: &RepoPath) -> Result<Option<graphql::Topic>> {
        let result = self
            .object_loader
            .load_one(path.to_string())
            .await?
            .ok_or_else(|| Error::NotFound(format!("no topic: {}", path)))?;

        match result {
            git::Object::Topic(topic) => Ok(Some(graphql::Topic::try_from(&topic)?)),
            _ => return Err(Error::NotFound(format!("no topic: {}", path))),
        }
    }

    pub async fn topic_count(&self) -> Result<i64> {
        let git::FetchTopicCountResult { count, .. } = git::FetchTopicCount {
            actor: self.viewer.clone(),
        }
        .call(&self.git)?;
        Ok(count.try_into().unwrap_or_default())
    }

    pub async fn topics(&self, paths: &[RepoPath]) -> Result<Vec<Option<graphql::Topic>>> {
        let paths = paths.iter().map(|p| p.to_string()).collect::<Vec<String>>();
        let map = self.object_loader.load_many(paths.clone()).await?;
        let mut topics: Vec<Option<graphql::Topic>> = Vec::new();
        for path in paths.iter().take(50) {
            let topic = map
                .get(path)
                .ok_or_else(|| Error::NotFound(format!("no topic: {}", path)))?;

            let topic = match &topic {
                git::Object::Topic(topic) => Some(graphql::Topic::try_from(topic)?),
                _ => None,
            };

            topics.push(topic);
        }
        Ok(topics)
    }

    pub async fn update_link_parent_topics(
        &self,
        input: graphql::UpdateLinkParentTopicsInput,
    ) -> Result<git::UpdateLinkParentTopicsResult> {
        git::UpdateLinkParentTopics {
            actor: self.viewer.clone(),
            link_path: RepoPath::from(&input.link_path),
            parent_topic_paths: input
                .parent_topic_paths
                .iter()
                .map(RepoPath::from)
                .collect::<BTreeSet<RepoPath>>(),
        }
        .call(&self.git, &self.redis)
    }

    pub async fn update_topic_synonyms(
        &self,
        input: graphql::UpdateTopicSynonymsInput,
    ) -> Result<git::UpdateTopicSynonymsResult> {
        git::UpdateTopicSynonyms {
            actor: self.viewer.clone(),
            synonyms: input.synonyms.iter().map(git::Synonym::from).collect_vec(),
            topic_path: RepoPath::from(&input.topic_path),
        }
        .call(&self.git, &self.redis)
    }

    pub async fn upsert_link(
        &self,
        input: graphql::UpsertLinkInput,
    ) -> Result<git::UpsertLinkResult> {
        let add_parent_topic_path = input
            .add_parent_topic_path
            .map(|path| RepoPath::from(&path));

        git::UpsertLink {
            add_parent_topic_path,
            actor: self.viewer.clone(),
            prefix: RepoPrefix::from(&input.repo_prefix),
            title: input.title,
            url: input.url,
            fetcher: Box::new(http::Fetcher),
        }
        .call(&self.git, &self.redis)
        .await
    }

    pub async fn update_topic_parent_topics(
        &self,
        topic_path: &RepoPath,
        parent_topics: Vec<RepoPath>,
    ) -> Result<git::UpdateTopicParentTopicsResult> {
        git::UpdateTopicParentTopics {
            actor: self.viewer.clone(),
            topic_path: topic_path.clone(),
            parent_topic_paths: parent_topics
                .iter()
                .map(|p| p.to_owned())
                .collect::<BTreeSet<RepoPath>>(),
        }
        .call(&self.git, &self.redis)
    }

    pub async fn upsert_session(
        &self,
        input: graphql::CreateGithubSessionInput,
    ) -> Result<psql::CreateSessionResult> {
        psql::CreateGithubSession::new(input).call(&self.db).await
    }

    pub async fn upsert_topic(
        &self,
        input: graphql::UpsertTopicInput,
    ) -> Result<git::UpsertTopicResult> {
        git::UpsertTopic {
            actor: self.viewer.clone(),
            locale: Locale::EN,
            name: input.name.to_owned(),
            on_matching_synonym: git::OnMatchingSynonym::Ask,
            prefix: RepoPrefix::from(&input.repo_prefix),
            parent_topic: RepoPath::from(&input.parent_topic_path),
        }
        .call(&self.git, &self.redis)
    }

    pub async fn upsert_topic_timerange(
        &self,
        input: graphql::UpsertTopicTimerangeInput,
    ) -> Result<git::UpsertTopicTimerangeResult> {
        let starts = Geotime::from(&input.starts_at.0);

        git::UpsertTopicTimerange {
            actor: self.viewer.clone(),
            timerange: Timerange {
                starts: geotime::LexicalGeohash::from(starts),
                prefix_format: TimerangePrefixFormat::from(&input.prefix_format),
            },
            topic_path: RepoPath::from(&input.topic_path),
        }
        .call(&self.git, &self.redis)
    }

    pub async fn user(&self, id: String) -> Result<Option<graphql::User>> {
        let user = self
            .user_loader
            .load_one(id)
            .await?
            .map(|row| graphql::User::from(&row));
        Ok(user)
    }
}