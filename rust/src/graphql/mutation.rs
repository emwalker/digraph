use std::convert::TryInto;

use async_graphql::{Context, InputObject, Object, SimpleObject, ID};
use itertools::Itertools;

use super::{
    alert, time, Link, LinkEdge, RepoTopic, Repository, Session, SessionEdge, Topic, TopicEdge,
    User, UserEdge,
};
use crate::git;
use crate::prelude::*;
use crate::psql;
use crate::store::Store;

#[derive(Clone, Debug, InputObject)]
pub struct CreateGithubSessionInput {
    client_mutation_id: Option<String>,
    pub github_avatar_url: String,
    pub github_username: String,
    pub name: String,
    pub primary_email: String,
    server_secret: String,
}

#[derive(Debug, SimpleObject)]
pub struct CreateSessionPayload {
    alerts: Vec<alert::Alert>,
    user_edge: Option<UserEdge>,
    session_edge: Option<SessionEdge>,
}

#[derive(Debug, InputObject)]
pub struct DeleteAccountInput {
    client_mutation_id: Option<String>,
    user_id: ID,
}

#[derive(Debug, SimpleObject)]
pub struct DeleteAccountPayload {
    alerts: Vec<alert::Alert>,
    client_mutation_id: Option<String>,
    deleted_user_id: ID,
}

#[derive(Debug, InputObject)]
pub struct DeleteLinkInput {
    client_mutation_id: Option<String>,
    link_id: String,
    repo_id: String,
}

#[derive(Debug, SimpleObject)]
pub struct DeleteLinkPayload {
    client_mutation_id: Option<String>,
    deleted_link_id: Option<String>,
}

#[derive(Debug, InputObject)]
pub struct DeleteSessionInput {
    client_mutation_id: Option<String>,
    // TODO: add server secret
    session_id: ID,
}

#[derive(Debug, SimpleObject)]
pub struct DeleteSessionPayload {
    client_mutation_id: Option<String>,
    deleted_session_id: Option<ID>,
}

#[derive(Debug, InputObject)]
pub struct DeleteTopicInput {
    client_mutation_id: Option<String>,
    repo_id: String,
    topic_id: String,
}

#[derive(Debug, SimpleObject)]
pub struct DeleteTopicPayload {
    client_mutation_id: Option<String>,
    deleted_topic_id: String,
}

#[derive(Debug, InputObject)]
pub struct RemoveTopicTimerangeInput {
    client_mutation_id: Option<String>,
    repo_id: String,
    topic_id: String,
}

#[derive(Debug, SimpleObject)]
pub struct RemoveTopicTimerangePayload {
    client_mutation_id: Option<String>,
    updated_repo_topic: RepoTopic,
    updated_topic: Topic,
}

#[derive(Debug, InputObject)]
pub struct SelectRepositoryInput {
    pub client_mutation_id: Option<String>,
    pub repository_id: Option<ID>,
}

#[derive(Debug, SimpleObject)]
pub struct SelectRepositoryPayload {
    repository: Option<Repository>,
    viewer: User,
}

#[derive(Debug, InputObject)]
pub struct UpdateLinkParentTopicsInput {
    pub client_mutation_id: Option<String>,
    pub link_id: String,
    pub parent_topic_ids: Vec<String>,
    pub repo_id: String,
}

#[derive(Debug, SimpleObject)]
pub struct UpdateLinkParentTopicsPayload {
    link: Link,
}

#[derive(Debug, InputObject)]
pub struct SynonymInput {
    pub name: String,
    pub locale: String,
}

impl SynonymInput {
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty() && !RepoUrl::is_valid_url(&self.name)
    }
}

#[derive(Debug, InputObject)]
pub struct UpdateTopicSynonymsInput {
    pub client_mutation_id: Option<String>,
    pub repo_id: String,
    pub synonyms: Vec<SynonymInput>,
    pub topic_id: String,
}

#[derive(SimpleObject)]
pub struct UpdateTopicSynonymsPayload {
    alerts: Vec<alert::Alert>,
    client_mutation_id: Option<String>,
    updated_repo_topic: RepoTopic,
    updated_topic: Topic,
}

#[derive(Debug, InputObject)]
pub struct UpdateTopicParentTopicsInput {
    client_mutation_id: Option<String>,
    parent_topic_ids: Vec<String>,
    repo_id: String,
    topic_id: String,
}

#[derive(SimpleObject)]
pub struct UpdateTopicParentTopicsPayload {
    alerts: Vec<alert::Alert>,
    topic: Topic,
}

#[derive(Debug, InputObject)]
pub struct UpsertLinkInput {
    pub add_parent_topic_id: Option<String>,
    pub client_mutation_id: Option<String>,
    pub link_id: Option<String>,
    pub repo_id: String,
    pub title: Option<String>,
    pub url: String,
}

#[derive(SimpleObject)]
pub struct UpsertLinkPayload {
    alerts: Vec<alert::Alert>,
    link_edge: Option<LinkEdge>,
}

#[derive(Debug, InputObject)]
pub struct UpsertTopicInput {
    pub client_mutation_id: Option<String>,
    pub description: Option<String>,
    pub name: String,
    pub parent_topic_id: String,
    pub repo_id: String,
}

#[derive(SimpleObject)]
pub struct UpsertTopicPayload {
    alerts: Vec<alert::Alert>,
    topic_edge: Option<TopicEdge>,
    matching_topics: Vec<Topic>,
}

#[derive(Debug, InputObject)]
pub struct UpsertTopicTimerangeInput {
    pub client_mutation_id: Option<String>,
    pub ends_at: Option<time::DateTime>,
    pub prefix_format: time::TimerangePrefixFormat,
    pub repo_id: String,
    pub starts_at: time::DateTime,
    pub topic_id: String,
}

#[derive(SimpleObject)]
pub struct UpsertTopicTimerangePayload {
    alerts: Vec<alert::Alert>,
    timerange_edge: Option<time::TimerangeEdge>,
    updated_repo_topic: RepoTopic,
    updated_topic: Topic,
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_github_session(
        &self,
        ctx: &Context<'_>,
        input: CreateGithubSessionInput,
    ) -> Result<CreateSessionPayload> {
        log::info!("creating GitHub session: {:?}", input);
        let store = ctx.data_unchecked::<Store>();

        if store.server_secret != input.server_secret {
            log::warn!("server secret did not match secret provided by client");
            return Err(Error::Auth("failed to authenticate request".to_string()));
        }

        let result = store.upsert_session(input).await?;
        log::info!(
            "server secret matched secret provided by client, user session created: {:?}",
            result.user
        );

        Ok(CreateSessionPayload {
            alerts: result.alerts.iter().map(alert::Alert::from).collect_vec(),
            user_edge: Some(UserEdge {
                cursor: String::from("0"),
                node: User::from(&result.user),
            }),
            session_edge: Some(SessionEdge {
                cursor: String::from("0"),
                node: Session {
                    id: result.session_id,
                },
            }),
        })
    }

    async fn delete_account(
        &self,
        ctx: &Context<'_>,
        input: DeleteAccountInput,
    ) -> Result<DeleteAccountPayload> {
        let DeleteAccountInput {
            user_id,
            client_mutation_id,
        } = input;

        let psql::DeleteAccountResult {
            alerts,
            deleted_user_id,
        } = ctx
            .data_unchecked::<Store>()
            .delete_account(user_id.to_string())
            .await?;

        Ok(DeleteAccountPayload {
            alerts: alerts.iter().map(alert::Alert::from).collect_vec(),
            deleted_user_id: ID(deleted_user_id),
            client_mutation_id,
        })
    }

    async fn delete_link(
        &self,
        ctx: &Context<'_>,
        input: DeleteLinkInput,
    ) -> Result<DeleteLinkPayload> {
        let DeleteLinkInput {
            client_mutation_id,
            link_id,
            repo_id,
        } = input;
        let link_id = Oid::try_from(&link_id)?;

        let git::DeleteLinkResult {
            deleted_link_id, ..
        } = ctx
            .data_unchecked::<Store>()
            .delete_link(&repo_id.try_into()?, &link_id)
            .await?;

        Ok(DeleteLinkPayload {
            client_mutation_id,
            deleted_link_id: Some(deleted_link_id.to_string()),
        })
    }

    async fn delete_session(
        &self,
        ctx: &Context<'_>,
        input: DeleteSessionInput,
    ) -> Result<DeleteSessionPayload> {
        let DeleteSessionInput {
            client_mutation_id,
            session_id,
        } = input;
        let psql::DeleteSessionResult { deleted_session_id } = ctx
            .data_unchecked::<Store>()
            .delete_session(session_id.to_string())
            .await?;

        Ok(DeleteSessionPayload {
            client_mutation_id,
            deleted_session_id: Some(ID(deleted_session_id)),
        })
    }

    async fn delete_topic(
        &self,
        ctx: &Context<'_>,
        input: DeleteTopicInput,
    ) -> Result<DeleteTopicPayload> {
        let DeleteTopicInput {
            client_mutation_id,
            repo_id,
            topic_id,
        } = input;
        let topic_id = Oid::try_from(&topic_id)?;

        ctx.data_unchecked::<Store>()
            .delete_topic(&repo_id.try_into()?, &topic_id)
            .await?;

        Ok(DeleteTopicPayload {
            client_mutation_id,
            deleted_topic_id: topic_id.to_string(),
        })
    }

    async fn remove_topic_timerange(
        &self,
        ctx: &Context<'_>,
        input: RemoveTopicTimerangeInput,
    ) -> Result<RemoveTopicTimerangePayload> {
        let RemoveTopicTimerangeInput {
            client_mutation_id,
            repo_id,
            topic_id,
        } = &input;

        let topic_id = topic_id.try_into()?;
        let store = ctx.data_unchecked::<Store>();
        let git::RemoveTopicTimerangeResult { repo_topic, .. } = store
            .remove_topic_timerange(&repo_id.try_into()?, &topic_id)
            .await?;
        let updated_topic: Topic = store.fetch_topic(topic_id).await?.try_into()?;

        Ok(RemoveTopicTimerangePayload {
            client_mutation_id: client_mutation_id.to_owned(),
            updated_repo_topic: repo_topic.into(),
            updated_topic,
        })
    }

    async fn select_repository(
        &self,
        ctx: &Context<'_>,
        input: SelectRepositoryInput,
    ) -> Result<SelectRepositoryPayload> {
        let psql::SelectRepositoryResult { repository, actor } = ctx
            .data_unchecked::<Store>()
            .select_repository(input.repository_id.map(|id| id.to_string()))
            .await?;
        Ok(SelectRepositoryPayload {
            repository,
            viewer: User::from(&actor),
        })
    }

    async fn update_link_parent_topics(
        &self,
        ctx: &Context<'_>,
        input: UpdateLinkParentTopicsInput,
    ) -> Result<UpdateLinkParentTopicsPayload> {
        let store = ctx.data_unchecked::<Store>();
        let link_id: Oid = (&input.link_id).try_into()?;
        store.update_link_parent_topics(input).await?;
        let link: Link = store.fetch_link(link_id).await?.try_into()?;
        Ok(UpdateLinkParentTopicsPayload { link })
    }

    async fn update_topic_parent_topics(
        &self,
        ctx: &Context<'_>,
        input: UpdateTopicParentTopicsInput,
    ) -> Result<UpdateTopicParentTopicsPayload> {
        let UpdateTopicParentTopicsInput {
            topic_id,
            parent_topic_ids,
            repo_id,
            ..
        } = &input;

        let store = ctx.data_unchecked::<Store>();

        let git::UpdateTopicParentTopicsResult { alerts, repo_topic } = store
            .update_topic_parent_topics(
                &repo_id.try_into()?,
                &topic_id.try_into()?,
                parent_topic_ids
                    .iter()
                    .map(Oid::try_from)
                    .collect::<Result<Vec<Oid>>>()?,
            )
            .await?;

        let topic: Topic = store
            .fetch_topic(repo_topic.topic_id().to_owned())
            .await?
            .try_into()?;

        Ok(UpdateTopicParentTopicsPayload {
            alerts: alerts.iter().map(alert::Alert::from).collect_vec(),
            topic,
        })
    }

    async fn update_topic_synonyms(
        &self,
        ctx: &Context<'_>,
        input: UpdateTopicSynonymsInput,
    ) -> Result<UpdateTopicSynonymsPayload> {
        let store = ctx.data_unchecked::<Store>();
        let client_mutation_id = input.client_mutation_id.clone();

        let git::UpdateTopicSynonymsResult {
            alerts, repo_topic, ..
        } = store.update_topic_synonyms(input).await?;

        let updated_topic: Topic = store
            .fetch_topic(repo_topic.topic_id().to_owned())
            .await?
            .try_into()?;

        Ok(UpdateTopicSynonymsPayload {
            alerts: alerts.iter().map(alert::Alert::from).collect_vec(),
            client_mutation_id,
            updated_repo_topic: repo_topic.into(),
            updated_topic,
        })
    }

    async fn upsert_link(
        &self,
        ctx: &Context<'_>,
        input: UpsertLinkInput,
    ) -> Result<UpsertLinkPayload> {
        let store = ctx.data_unchecked::<Store>();
        let result = store.upsert_link(input).await?;

        let link_edge = if let Some(link) = &result.link {
            let link_id = link.id();
            let link: Link = store.fetch_link(link_id.to_owned()).await?.try_into()?;
            Some(LinkEdge::new(String::from("0"), link))
        } else {
            None
        };

        Ok(UpsertLinkPayload {
            alerts: result.alerts.iter().map(alert::Alert::from).collect_vec(),
            link_edge,
        })
    }

    async fn upsert_topic(
        &self,
        ctx: &Context<'_>,
        input: UpsertTopicInput,
    ) -> Result<UpsertTopicPayload> {
        log::info!("upserting topic: {:?}", input);
        let store = ctx.data_unchecked::<Store>();

        let git::UpsertTopicResult {
            alerts,
            repo_topic,
            matching_repo_topics,
            ..
        } = store.upsert_topic(input).await?;

        let topic_edge = match &repo_topic {
            Some(topic) => {
                let topic_id = topic.topic_id();
                let topic: Topic = store.fetch_topic(topic_id.to_owned()).await?.try_into()?;
                Some(TopicEdge::new(String::from("0"), topic))
            }

            None => None,
        };

        let mut matching_topics = vec![];
        for synonym_match in &matching_repo_topics {
            let topic_id = synonym_match.repo_topic.topic_id();
            let topic: Topic = store.fetch_topic(topic_id.to_owned()).await?.try_into()?;
            matching_topics.push(topic);
        }

        Ok(UpsertTopicPayload {
            alerts: alerts.iter().map(alert::Alert::from).collect_vec(),
            topic_edge,
            matching_topics,
        })
    }

    async fn upsert_topic_timerange(
        &self,
        ctx: &Context<'_>,
        input: UpsertTopicTimerangeInput,
    ) -> Result<UpsertTopicTimerangePayload> {
        let store = ctx.data_unchecked::<Store>();
        let topic_id: Oid = (&input.topic_id).try_into()?;

        let git::UpsertTopicTimerangeResult {
            alerts,
            timerange,
            updated_repo_topic,
        } = store.upsert_topic_timerange(input).await?;
        let topic: Topic = store.fetch_topic(topic_id).await?.try_into()?;

        let timerange_edge = Some(time::TimerangeEdge::new(
            String::from("0"),
            timerange.try_into()?,
        ));

        Ok(UpsertTopicTimerangePayload {
            alerts: alerts.iter().map(alert::Alert::from).collect_vec(),
            updated_topic: topic,
            updated_repo_topic: updated_repo_topic.into(),
            timerange_edge,
        })
    }
}
