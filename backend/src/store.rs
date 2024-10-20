use async_graphql::dataloader::*;
use geotime::Geotime;
use sqlx::postgres::PgPool;
use std::collections::BTreeSet;
use std::convert::TryInto;
use std::sync::Arc;

use crate::git;
use crate::graphql;
use crate::http;
use crate::prelude::*;
use crate::psql;
use crate::redis;
use crate::types::Timespec;

pub struct Store {
    db: PgPool,
    git: Arc<git::Client>,
    object_loader: DataLoader<graphql::ObjectLoader>,
    organization_loader: DataLoader<psql::OrganizationLoader>,
    pub server_secret: String,
    pub user_loader: DataLoader<psql::UserLoader>,
    pub viewer: Arc<Viewer>,
    redis: Arc<redis::Redis>,
    repository_loader: DataLoader<psql::RepositoryLoader>,
}

impl Store {
    pub fn new(
        viewer: Arc<Viewer>,
        git: Arc<git::Client>,
        db: PgPool,
        server_secret: String,
        redis: Arc<redis::Redis>,
    ) -> Self {
        let organization_loader = psql::OrganizationLoader::new(Arc::clone(&viewer), db.clone());
        let repository_loader = psql::RepositoryLoader::new(Arc::clone(&viewer), db.clone());
        let object_loader = graphql::ObjectLoader::new(Arc::clone(&git));
        let user_loader = psql::UserLoader::new(Arc::clone(&viewer), db.clone());

        Self {
            db,
            git,
            redis,
            server_secret,
            viewer,

            organization_loader: DataLoader::new(organization_loader, tokio::spawn),
            repository_loader: DataLoader::new(repository_loader, tokio::spawn),
            object_loader: DataLoader::new(object_loader, tokio::spawn),
            user_loader: DataLoader::new(user_loader, tokio::spawn),
        }
    }
}

impl Store {
    pub async fn activity(
        &self,
        repo_id: RepoId,
        topic_id: &Option<ExternalId>,
        first: i32,
    ) -> Result<Vec<git::activity::Change>> {
        let result = git::activity::FetchActivity {
            actor: Arc::clone(&self.viewer),
            path: topic_id.as_ref().map(|id| (repo_id, id.to_owned())),
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

    pub async fn delete_account(&self, user_id: String) -> Result<psql::DeleteAccountResult> {
        log::info!("account deletion: fetching account info for {}", user_id);
        let psql::FetchAccountInfoResult { personal_repos } = psql::FetchAccountInfo {
            user_id: user_id.to_owned(),
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
        psql::DeleteAccount::new(Arc::clone(&self.viewer), user_id)
            .call(&self.db)
            .await
    }

    pub async fn delete_link(
        &self,
        repo_id: RepoId,
        link_id: &ExternalId,
    ) -> Result<git::DeleteLinkResult> {
        git::DeleteLink {
            actor: Arc::clone(&self.viewer),
            repo_id,
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
        repo_id: RepoId,
        topic_id: &ExternalId,
    ) -> Result<git::DeleteTopicResult> {
        git::DeleteTopic {
            actor: Arc::clone(&self.viewer),
            repo_id,
            topic_id: topic_id.to_owned(),
        }
        .call(self.mutation()?, &self.redis)
    }

    pub async fn fetch_link(&self, link_id: ExternalId) -> Result<Option<git::Link>> {
        let key = Okey(link_id, self.viewer.context_repo_id);
        match self.object_loader.load_one(key).await? {
            Some(object) => Ok(Some(object.try_into()?)),
            None => Ok(None),
        }
    }

    pub async fn fetch_links(
        &self,
        link_ids: &[ExternalId],
        take: usize,
        _reviewed: Option<bool>,
    ) -> Result<BTreeSet<git::Link>> {
        self.fetch_objects(link_ids.to_owned(), take)
            .await?
            .into_iter()
            .map(git::Link::try_from)
            .collect::<Result<BTreeSet<git::Link>>>()
    }

    pub async fn fetch_objects(
        &self,
        ids: Vec<ExternalId>,
        take: usize,
    ) -> Result<Vec<git::Object>> {
        let context_id = self.viewer.context_repo_id;
        self.fetch_objects_with_context(ids, take, context_id).await
    }

    pub async fn fetch_objects_with_context(
        &self,
        ids: Vec<ExternalId>,
        take: usize,
        context_id: RepoId,
    ) -> Result<Vec<git::Object>> {
        let keys = ids
            .into_iter()
            .take(take)
            .map(|oid| Okey(oid, context_id))
            .collect::<Vec<Okey>>();

        Ok(self
            .object_loader
            .load_many(keys)
            .await?
            .into_values()
            .collect::<Vec<git::Object>>())
    }

    pub async fn fetch_topic(&self, topic_id: ExternalId) -> Result<Option<git::Topic>> {
        let key = Okey(topic_id, self.viewer.context_repo_id);
        self.fetch_topic_by_key(key).await
    }

    pub async fn fetch_topic_by_key(&self, key: Okey) -> Result<Option<git::Topic>> {
        match self.object_loader.load_one(key).await? {
            Some(object) => Ok(Some(object.try_into()?)),
            None => Ok(None),
        }
    }

    pub async fn fetch_topics(
        &self,
        topic_ids: Vec<ExternalId>,
        take: usize,
    ) -> Result<BTreeSet<git::Topic>> {
        self.fetch_topics_with_context(topic_ids, take, self.viewer.context_repo_id)
            .await
    }

    pub async fn fetch_topics_with_context(
        &self,
        topic_ids: Vec<ExternalId>,
        take: usize,
        context_id: RepoId,
    ) -> Result<BTreeSet<git::Topic>> {
        self.fetch_objects_with_context(topic_ids, take, context_id)
            .await?
            .into_iter()
            .map(git::Topic::try_from)
            .collect::<Result<BTreeSet<git::Topic>>>()
    }

    fn mutation(&self) -> Result<git::Mutation> {
        self.git.mutation(git::IndexMode::Update)
    }

    pub async fn remove_topic_timerange(
        &self,
        repo_id: RepoId,
        topic_id: &ExternalId,
    ) -> Result<git::RemoveTopicTimerangeResult> {
        git::RemoveTopicTimerange {
            actor: Arc::clone(&self.viewer),
            repo_id,
            topic_id: topic_id.clone(),
        }
        .call(self.mutation()?, &self.redis)
    }

    pub async fn organization(&self, id: String) -> Result<Option<graphql::Organization>> {
        self.organization_loader.load_one(id).await
    }

    pub async fn repositories_for_user(&self, user_id: String) -> Result<Vec<graphql::Repository>> {
        psql::FetchWriteableRepositoriesForUser::new(Arc::clone(&self.viewer), user_id)
            .call(&self.db)
            .await
    }

    pub async fn repo(&self, id: String) -> Result<Option<graphql::Repository>> {
        self.repository_loader.load_one(id).await
    }

    pub async fn repository_by_id(&self, repo_id: String) -> Result<Option<graphql::Repository>> {
        self.repository_loader.load_one(repo_id).await
    }

    pub async fn search(
        &self,
        parent_topic: &git::Topic,
        search: &git::Search,
    ) -> Result<git::FindMatchesResult> {
        log::debug!("search: {:?}", search);

        let fetcher = git::RedisFetchDownSet {
            client: Arc::clone(&self.git),
            redis: Arc::clone(&self.redis),
        };

        git::FindMatches {
            context_repo_id: parent_topic.key.1.to_owned(),
            limit: 100,
            locale: Locale::EN,
            recursive: true,
            search: search.to_owned(),
            timespec: Timespec,
            topic_id: parent_topic.key.0.to_owned(),
            viewer: Arc::clone(&self.viewer),
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
            viewer: Arc::clone(&self.viewer),
            repos: self.viewer.read_repo_ids.to_owned(),
            search,
        }
        .call(&self.git)
    }

    pub async fn select_repository(
        &self,
        repo_id: Option<RepoId>,
    ) -> Result<psql::SelectRepositoryResult> {
        psql::SelectRepository::new(Arc::clone(&self.viewer), repo_id)
            .call(&self.db)
            .await
    }

    fn update_by(&self, actor: &Arc<Viewer>) -> Result<git::Mutation> {
        let git = git::Client {
            root: self.git.root.to_owned(),
            timespec: self.git.timespec.to_owned(),
            viewer: Arc::clone(actor),
        };

        git.mutation(git::IndexMode::Update)
    }

    pub async fn update_link_parent_topics(
        &self,
        input: graphql::UpdateLinkParentTopicsInput,
    ) -> Result<git::UpdateLinkParentTopicsResult> {
        let link_id = ExternalId::try_from(&input.link_id)?;

        git::UpdateLinkParentTopics {
            actor: Arc::clone(&self.viewer),
            link_id,
            parent_topic_ids: input
                .parent_topic_ids
                .iter()
                .map(ExternalId::try_from)
                .collect::<Result<BTreeSet<ExternalId>>>()?,
            repo_id: input.repo_id.try_into()?,
        }
        .call(self.mutation()?, &self.redis)
    }

    pub async fn update_topic_parent_topics(
        &self,
        repo_id: RepoId,
        topic_id: &ExternalId,
        parent_topic_ids: &[ExternalId],
    ) -> Result<git::UpdateTopicParentTopicsResult> {
        git::UpdateTopicParentTopics {
            actor: Arc::clone(&self.viewer),
            repo_id,
            topic_id,
            parent_topic_ids,
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
            actor: Arc::clone(&self.viewer),
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
            Some(ExternalId::try_from(id)?)
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

        let actor = Arc::new(Viewer::service_account());

        git::EnsurePersonalRepo {
            actor: Arc::clone(&actor),
            user_id: result.user.id.to_string(),
            personal_repo_ids: result.personal_repo_ids.to_owned(),
        }
        .call(self.update_by(&actor)?)?;

        Ok(result)
    }

    pub async fn upsert_topic(
        &self,
        graphql::UpsertTopicInput {
            name,
            repo_id,
            parent_topic_id,
            on_matching_synonym,
            update_topic_id,
            ..
        }: graphql::UpsertTopicInput,
    ) -> Result<git::UpsertTopicResult> {
        let parent_topic = ExternalId::try_from(&parent_topic_id)?;

        let on_matching_synonym = match &(on_matching_synonym, update_topic_id) {
            (graphql::OnMatchingSynonym::Ask, None) => git::OnMatchingSynonym::Ask,

            (graphql::OnMatchingSynonym::CreateDistinct, None) => {
                git::OnMatchingSynonym::CreateDistinct
            }

            (graphql::OnMatchingSynonym::Update, Some(topic_id)) => {
                git::OnMatchingSynonym::Update(topic_id.try_into()?)
            }

            (enum_value, update_topic_id) => {
                log::warn!(
                    "unrecognized upsert topic resolution: {:?} / {:?}",
                    enum_value,
                    update_topic_id,
                );
                git::OnMatchingSynonym::Ask
            }
        };

        git::UpsertTopic {
            actor: Arc::clone(&self.viewer),
            locale: Locale::EN,
            name,
            on_matching_synonym,
            repo_id: repo_id.try_into()?,
            parent_topic_id: parent_topic,
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
            actor: Arc::clone(&self.viewer),
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
            viewer: Arc::clone(&self.viewer),
        }
        .call(&self.git, &self.redis)
        .await
    }
}
