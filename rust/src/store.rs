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
use crate::types::Timespec;

pub struct Store {
    db: PgPool,
    git: git::Client,
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
        git: git::Client,
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
        let object_loader = graphql::ObjectLoader::new(git.clone());
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
        let topic_path = if let Some(path) = topic_path {
            Some(RepoId::try_from(&path)?)
        } else {
            None
        };

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

    fn update(&self) -> Result<git::Mutation> {
        self.git.mutation(git::IndexMode::Update)
    }

    fn update_by(&self, actor: &Viewer) -> Result<git::Mutation> {
        let git = git::Client {
            root: self.git.root.to_owned(),
            timespec: self.git.timespec.to_owned(),
            viewer: actor.to_owned(),
        };

        git.mutation(git::IndexMode::Update)
    }

    async fn flat_topics(&self, paths: &[RepoId]) -> Result<Vec<graphql::Topic>> {
        let result = self.topics(paths).await?;
        Ok(result.iter().flatten().cloned().collect())
    }

    pub async fn child_links_for_topic(
        &self,
        parent_topic: &RepoId,
        _reviewed: Option<bool>,
    ) -> Result<Vec<graphql::Link>> {
        let children = self.topic_children(parent_topic).await?;
        let mut links = vec![];

        for child in children.iter().take(50) {
            let git::SearchMatch { object, .. } = &child;
            if let git::Object::Link(link) = object {
                links.push(graphql::Link::try_from(link)?);
            }
        }

        Ok(links)
    }

    pub async fn topic_children(
        &self,
        parent_topic: &RepoId,
    ) -> Result<BTreeSet<git::SearchMatch>> {
        let topic = self
            .topic(parent_topic)
            .await?
            .ok_or_else(|| Error::NotFound(format!("no topic: {}", parent_topic)))?;
        let child_paths = topic
            .child_paths
            .iter()
            .map(RepoId::to_string)
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
        log::info!("account deletion: fetching account info for {}", user_id);
        let psql::FetchAccountInfoResult { personal_repos } = psql::FetchAccountInfo {
            user_id: user_id.to_owned(),
            viewer: self.viewer.to_owned(),
        }
        .call(&self.db)
        .await?;

        log::info!(
            "account deletion: deleting git repos for {}: {:?}",
            user_id,
            personal_repos
        );
        git::DeleteAccount {
            actor: self.viewer.to_owned(),
            user_id: user_id.to_owned(),
            personal_repos,
        }
        .call(&self.update()?)?;

        log::info!("account deletion: postgres data for {}", user_id);
        psql::DeleteAccount::new(self.viewer.clone(), user_id)
            .call(&self.db)
            .await
    }

    pub async fn delete_link(&self, link_path: &RepoId) -> Result<git::DeleteLinkResult> {
        git::DeleteLink {
            actor: self.viewer.clone(),
            link_id: link_path.clone(),
        }
        .call(self.update()?, &self.redis)
    }

    pub async fn delete_session(&self, session_id: String) -> Result<psql::DeleteSessionResult> {
        psql::DeleteSession::new(self.viewer.clone(), session_id)
            .call(&self.db)
            .await
    }

    pub async fn delete_topic(&self, path: &RepoId) -> Result<git::DeleteTopicResult> {
        git::DeleteTopic {
            actor: self.viewer.clone(),
            topic_id: path.clone(),
        }
        .call(self.update()?, &self.redis)
    }

    pub async fn remove_topic_timerange(
        &self,
        topic_path: &RepoId,
    ) -> Result<git::RemoveTopicTimerangeResult> {
        git::RemoveTopicTimerange {
            actor: self.viewer.clone(),
            topic_id: topic_path.clone(),
        }
        .call(self.update()?, &self.redis)
    }

    pub async fn link(&self, path: &RepoId) -> Result<Option<graphql::Link>> {
        let result = self
            .object_loader
            .load_one(path.to_string())
            .await?
            .ok_or_else(|| Error::NotFound(format!("no link: {}", path)))?;

        match result {
            git::Object::Link(link) => Ok(Some(graphql::Link::try_from(&link)?)),
            _ => Err(Error::NotFound(format!("no link: {}", path))),
        }
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

    pub async fn parent_topics_for_topic(&self, path: &RepoId) -> Result<Vec<graphql::Topic>> {
        let topic = self
            .topic(path)
            .await?
            .ok_or_else(|| Error::NotFound(format!("no topic for id: {}", path)))?;
        self.flat_topics(&topic.parent_topic_paths).await
    }

    pub async fn parent_topics_for_link(&self, path: &RepoId) -> Result<Vec<graphql::Topic>> {
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
        link_path: &RepoId,
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
        let viewer = &self.viewer;

        let fetcher = git::RedisFetchDownSet {
            client: self.git.clone(),
            redis: self.redis.clone(),
        };

        let git::FindMatchesResult { matches, .. } = git::FindMatches {
            limit: 100,
            locale: Locale::EN,
            recursive: true,
            repos: viewer.read_repos.to_owned(),
            search: git::Search::parse(&search_string)?,
            timespec: Timespec,
            topic_path: parent_topic.path,
            viewer: viewer.to_owned(),
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
            prefixes: vec![RepoName::wiki()],
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

    pub async fn topic(&self, path: &RepoId) -> Result<Option<graphql::Topic>> {
        let result = self
            .object_loader
            .load_one(path.to_string())
            .await?
            .ok_or_else(|| Error::NotFound(format!("no topic: {}", path)))?;

        match result {
            git::Object::Topic(topic) => Ok(Some(graphql::Topic::try_from(&topic)?)),
            _ => Err(Error::NotFound(format!("no topic: {}", path))),
        }
    }

    pub async fn topics(&self, paths: &[RepoId]) -> Result<Vec<Option<graphql::Topic>>> {
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
            link_id: RepoId::try_from(&input.link_path)?,
            parent_topic_ids: input
                .parent_topic_paths
                .iter()
                .map(RepoId::try_from)
                .collect::<Result<BTreeSet<RepoId>>>()?,
        }
        .call(self.update()?, &self.redis)
    }

    pub async fn update_topic_synonyms(
        &self,
        input: graphql::UpdateTopicSynonymsInput,
    ) -> Result<git::UpdateTopicSynonymsResult> {
        git::UpdateTopicSynonyms {
            actor: self.viewer.clone(),
            synonyms: input.synonyms.iter().map(git::Synonym::from).collect_vec(),
            topic_id: RepoId::try_from(&input.topic_path)?,
        }
        .call(self.update()?, &self.redis)
    }

    pub async fn upsert_link(
        &self,
        input: graphql::UpsertLinkInput,
    ) -> Result<git::UpsertLinkResult> {
        let add_parent_topic_path = if let Some(path) = input.add_parent_topic_path {
            Some(RepoId::try_from(&path)?)
        } else {
            None
        };

        git::UpsertLink {
            add_parent_topic_path,
            actor: self.viewer.clone(),
            repo: input.repo_prefix.try_into()?,
            title: input.title,
            url: input.url,
            fetcher: Box::new(http::Fetcher),
        }
        .call(self.update()?, &self.redis)
    }

    pub async fn update_topic_parent_topics(
        &self,
        topic_path: &RepoId,
        parent_topics: Vec<RepoId>,
    ) -> Result<git::UpdateTopicParentTopicsResult> {
        git::UpdateTopicParentTopics {
            actor: self.viewer.clone(),
            topic_id: topic_path.clone(),
            parent_topic_ids: parent_topics
                .iter()
                .map(|p| p.to_owned())
                .collect::<BTreeSet<RepoId>>(),
        }
        .call(self.update()?, &self.redis)
    }

    pub async fn upsert_session(
        &self,
        input: graphql::CreateGithubSessionInput,
    ) -> Result<psql::CreateSessionResult> {
        let login = input.github_username.to_owned();
        let result = psql::CreateGithubSession::new(input).call(&self.db).await?;

        let actor = Viewer::service_account();
        git::EnsurePersonalRepo {
            actor: actor.to_owned(),
            user_id: result.user.id.to_string(),
            personal_repo: RepoName::from_login(&login)?,
        }
        .call(self.update_by(&actor)?)?;

        Ok(result)
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
            repo: input.repo_prefix.try_into()?,
            parent_topic: RepoId::try_from(&input.parent_topic_path)?,
        }
        .call(self.update()?, &self.redis)
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
            topic_id: RepoId::try_from(&input.topic_path)?,
        }
        .call(self.update()?, &self.redis)
    }

    pub async fn user(&self, id: String) -> Result<Option<graphql::User>> {
        let user = self
            .user_loader
            .load_one(id)
            .await?
            .map(|row| graphql::User::from(&row));
        Ok(user)
    }

    pub async fn view_stats(&self) -> Result<graphql::ViewStats> {
        let git::FetchStatsResult { stats } = git::FetchStats {
            viewer: self.viewer.clone(),
        }
        .call(&self.git, self.redis.to_owned())
        .await?;

        Ok(stats.into())
    }
}
