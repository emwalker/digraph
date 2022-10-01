use async_graphql::dataloader::*;
use geotime::Geotime;
use sqlx::postgres::PgPool;
use std::collections::BTreeSet;
use std::convert::TryInto;

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
    organization_loader: DataLoader<psql::OrganizationLoader, HashMapCache>,
    pub server_secret: String,
    pub user_loader: DataLoader<psql::UserLoader, HashMapCache>,
    pub viewer: Viewer,
    redis: redis::Redis,
    repository_by_name_loader: DataLoader<psql::RepositoryByNameLoader, HashMapCache>,
    repository_by_prefix_loader: DataLoader<psql::RepositoryByIdLoader, HashMapCache>,
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
        let repository_loader = psql::RepositoryLoader::new(viewer.clone(), db.clone());
        let repository_by_name_loader =
            psql::RepositoryByNameLoader::new(viewer.clone(), db.clone());
        let repository_by_prefix_loader =
            psql::RepositoryByIdLoader::new(viewer.clone(), db.clone());
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
        repo_id: &RepoId,
        topic_id: &Option<Oid>,
        first: i32,
    ) -> Result<Vec<git::activity::Change>> {
        let result = git::activity::FetchActivity {
            actor: self.viewer.clone(),
            path: topic_id
                .as_ref()
                .map(|id| (repo_id.to_owned(), id.to_owned())),
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

    fn mutation(&self) -> Result<git::Mutation> {
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
        .call(&self.mutation()?)?;

        log::info!("account deletion: postgres data for {}", user_id);
        psql::DeleteAccount::new(self.viewer.clone(), user_id)
            .call(&self.db)
            .await
    }

    pub async fn delete_link(
        &self,
        repo_id: &RepoId,
        link_id: &Oid,
    ) -> Result<git::DeleteLinkResult> {
        git::DeleteLink {
            actor: self.viewer.clone(),
            repo: repo_id.to_owned(),
            link_id: link_id.to_owned(),
        }
        .call(self.mutation()?, &self.redis)
    }

    pub async fn delete_session(&self, session_id: String) -> Result<psql::DeleteSessionResult> {
        psql::DeleteSession::new(self.viewer.clone(), session_id)
            .call(&self.db)
            .await
    }

    pub async fn delete_topic(
        &self,
        repo_id: &RepoId,
        topic_id: &Oid,
    ) -> Result<git::DeleteTopicResult> {
        git::DeleteTopic {
            actor: self.viewer.clone(),
            repo: repo_id.to_owned(),
            topic_id: topic_id.to_owned(),
        }
        .call(self.mutation()?, &self.redis)
    }

    pub async fn fetch_link(&self, link_id: Oid) -> Result<Option<git::Link>> {
        match self.object_loader.load_one(link_id).await? {
            Some(object) => Ok(Some(object.try_into()?)),
            None => Ok(None),
        }
    }

    pub async fn fetch_links(
        &self,
        link_ids: &[Oid],
        take: usize,
        _reviewed: Option<bool>,
    ) -> Result<Vec<git::Link>> {
        self.fetch_objects(link_ids.to_owned(), take)
            .await?
            .into_iter()
            .map(git::Link::try_from)
            .collect::<Result<Vec<git::Link>>>()
    }

    pub async fn fetch_objects(&self, ids: Vec<Oid>, take: usize) -> Result<Vec<git::Object>> {
        let ids = ids.into_iter().take(take).collect::<Vec<Oid>>();
        Ok(self
            .object_loader
            .load_many(ids)
            .await?
            .into_values()
            .collect::<Vec<git::Object>>())
    }

    pub async fn fetch_topic(&self, topic_id: Oid) -> Result<Option<git::Topic>> {
        match self.object_loader.load_one(topic_id).await? {
            Some(object) => Ok(Some(object.try_into()?)),
            None => Ok(None),
        }
    }

    pub async fn fetch_topics(&self, topic_ids: Vec<Oid>, take: usize) -> Result<Vec<git::Topic>> {
        self.fetch_objects(topic_ids, take)
            .await?
            .into_iter()
            .map(git::Topic::try_from)
            .collect::<Result<Vec<git::Topic>>>()
    }

    pub async fn remove_topic_timerange(
        &self,
        repo_id: &RepoId,
        topic_id: &Oid,
    ) -> Result<git::RemoveTopicTimerangeResult> {
        git::RemoveTopicTimerange {
            actor: self.viewer.clone(),
            repo_id: repo_id.to_owned(),
            topic_id: topic_id.clone(),
        }
        .call(self.mutation()?, &self.redis)
    }

    pub async fn organization(&self, id: String) -> Result<Option<graphql::Organization>> {
        self.organization_loader.load_one(id).await
    }

    pub async fn repositories_for_user(&self, user_id: String) -> Result<Vec<graphql::Repository>> {
        psql::FetchWriteableRepositoriesForUser::new(self.viewer.clone(), user_id)
            .call(&self.db)
            .await
    }

    pub async fn repo(&self, id: String) -> Result<Option<graphql::Repository>> {
        self.repository_loader.load_one(id).await
    }

    pub async fn repository_by_id(&self, prefix: String) -> Result<Option<graphql::Repository>> {
        self.repository_by_prefix_loader.load_one(prefix).await
    }

    pub async fn repository_by_name(&self, name: String) -> Result<Option<graphql::Repository>> {
        self.repository_by_name_loader.load_one(name).await
    }

    pub async fn review_link(
        &self,
        repo_id: &RepoId,
        link_id: &Oid,
        reviewed: bool,
    ) -> Result<psql::ReviewLinkResult> {
        let link: git::Link = self.fetch_link(link_id.to_owned()).await?.try_into()?;

        psql::ReviewLink {
            actor: self.viewer.clone(),
            repo_id: repo_id.to_owned(),
            link,
            reviewed,
        }
        .call(&self.db)
        .await
    }

    pub async fn search(
        &self,
        parent_topic: &git::Topic,
        search_string: String,
    ) -> Result<git::FindMatchesResult> {
        log::info!("search: search string {}", search_string);
        let viewer = &self.viewer;

        let fetcher = git::RedisFetchDownSet {
            client: self.git.clone(),
            redis: self.redis.clone(),
        };

        git::FindMatches {
            limit: 100,
            locale: Locale::EN,
            recursive: true,
            search: git::Search::parse(&search_string)?,
            timespec: Timespec,
            topic_id: parent_topic.id.to_owned(),
            viewer: viewer.to_owned(),
        }
        .call(&self.git, &fetcher)
    }

    pub async fn search_topics(
        &self,
        search_string: Option<String>,
    ) -> Result<git::FetchTopicLiveSearchResult> {
        let search = git::Search::parse(&search_string.unwrap_or_default())?;
        git::FetchTopicLiveSearch {
            limit: 10,
            viewer: self.viewer.to_owned(),
            repos: vec![RepoId::wiki()],
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

    pub async fn update_link_parent_topics(
        &self,
        input: graphql::UpdateLinkParentTopicsInput,
    ) -> Result<git::UpdateLinkParentTopicsResult> {
        let link_id = Oid::try_from(&input.link_id)?;

        git::UpdateLinkParentTopics {
            actor: self.viewer.clone(),
            link_id,
            parent_topic_ids: input
                .parent_topic_ids
                .iter()
                .map(Oid::try_from)
                .collect::<Result<BTreeSet<Oid>>>()?,
            repo_id: input.repo_id.try_into()?,
        }
        .call(self.mutation()?, &self.redis)
    }

    pub async fn update_topic_parent_topics(
        &self,
        // FIXME: Use id instead of name
        repo_id: &RepoId,
        topic_id: &Oid,
        parent_topics: Vec<Oid>,
    ) -> Result<git::UpdateTopicParentTopicsResult> {
        git::UpdateTopicParentTopics {
            actor: self.viewer.clone(),
            repo_id: repo_id.to_owned(),
            topic_id: topic_id.to_owned(),
            parent_topic_ids: parent_topics.iter().cloned().collect::<BTreeSet<Oid>>(),
        }
        .call(self.mutation()?, &self.redis)
    }

    pub async fn update_topic_synonyms(
        &self,
        input: graphql::UpdateTopicSynonymsInput,
    ) -> Result<git::UpdateTopicSynonymsResult> {
        let graphql::UpdateTopicSynonymsInput {
            repo_id,
            synonyms,
            topic_id,
            ..
        } = input;

        git::UpdateTopicSynonyms {
            actor: self.viewer.clone(),
            repo_id: repo_id.try_into()?,
            synonyms: synonyms
                .iter()
                .map(git::Synonym::try_from)
                .collect::<Result<Vec<git::Synonym>>>()?,
            topic_id: topic_id.try_into()?,
        }
        .call(self.mutation()?, &self.redis)
    }

    pub async fn upsert_link(
        &self,
        input: graphql::UpsertLinkInput,
    ) -> Result<git::UpsertLinkResult> {
        let add_parent_topic_id = if let Some(id) = &input.add_parent_topic_id {
            Some(Oid::try_from(id)?)
        } else {
            None
        };

        git::UpsertLink {
            add_parent_topic_id,
            actor: self.viewer.to_owned(),
            // FIXME: use id instead of prefix
            repo_id: input.repo_id.try_into()?,
            title: input.title,
            url: input.url,
            fetcher: Box::new(http::Fetcher),
        }
        .call(self.mutation()?, &self.redis)
    }

    pub async fn upsert_session(
        &self,
        input: graphql::CreateGithubSessionInput,
    ) -> Result<psql::CreateSessionResult> {
        let result = psql::CreateGithubSession::new(input).call(&self.db).await?;

        let actor = Viewer::service_account();

        git::EnsurePersonalRepo {
            actor: actor.to_owned(),
            user_id: result.user.id.to_string(),
            personal_repo_ids: result.personal_repo_ids.to_owned(),
        }
        .call(self.update_by(&actor)?)?;

        Ok(result)
    }

    pub async fn upsert_topic(
        &self,
        input: graphql::UpsertTopicInput,
    ) -> Result<git::UpsertTopicResult> {
        let parent_topic = Oid::try_from(&input.parent_topic_id)?;

        git::UpsertTopic {
            actor: self.viewer.clone(),
            locale: Locale::EN,
            name: input.name.to_owned(),
            on_matching_synonym: git::OnMatchingSynonym::Ask,
            repo_id: input.repo_id.try_into()?,
            parent_topic,
        }
        .call(self.mutation()?, &self.redis)
    }

    pub async fn upsert_topic_timerange(
        &self,
        input: graphql::UpsertTopicTimerangeInput,
    ) -> Result<git::UpsertTopicTimerangeResult> {
        // FIXME
        let starts = Geotime::from(&input.starts_at.0);

        git::UpsertTopicTimerange {
            actor: self.viewer.clone(),
            timerange: Timerange {
                starts: geotime::LexicalGeohash::from(starts),
                prefix_format: TimerangePrefixFormat::from(&input.prefix_format),
            },
            repo_id: input.repo_id.try_into()?,
            topic_id: input.topic_id.try_into()?,
        }
        .call(self.mutation()?, &self.redis)
    }

    pub async fn user(&self, id: String) -> Result<Option<graphql::User>> {
        let user = self
            .user_loader
            .load_one(id)
            .await?
            .map(|row| graphql::User::from(&row));
        Ok(user)
    }

    pub async fn view_stats(&self) -> Result<git::FetchStatsResult> {
        git::FetchStats {
            viewer: self.viewer.clone(),
        }
        .call(&self.git, self.redis.to_owned())
        .await
    }
}