use async_graphql::{Context, InputObject, Object, SimpleObject};

use super::{
    Alert, DateTime, Link, LinkEdge, Repository, Session, SessionEdge, Synonym, TimeRangeEdge,
    TimeRangePrefixFormat, Topic, TopicEdge, User, UserEdge,
};
use crate::http::repo_url;
use crate::prelude::*;
use crate::psql::{
    DeleteLinkResult, DeleteSessionResult, DeleteTopicTimeRangeResult, Repo,
    SelectRepositoryResult, UpdateLinkTopicsResult, UpdateSynonymsResult,
    UpsertTopicTimeRangeResult,
};

#[derive(Debug, InputObject)]
pub struct CreateGithubSessionInput {
    client_mutation_id: Option<String>,
    github_avatar_url: String,
    pub github_username: String,
    name: String,
    primary_email: String,
    server_secret: String,
}

#[derive(Debug, SimpleObject)]
pub struct CreateSessionPayload {
    alerts: Vec<Alert>,
    user_edge: Option<UserEdge>,
    session_edge: Option<SessionEdge>,
}

#[derive(Debug, InputObject)]
pub struct DeleteLinkInput {
    client_mutation_id: Option<String>,
    link_id: ID,
}

#[derive(Debug, SimpleObject)]
pub struct DeleteLinkPayload {
    client_mutation_id: Option<String>,
    deleted_link_id: Option<ID>,
}

#[derive(Debug, InputObject)]
pub struct DeleteSessionInput {
    client_mutation_id: Option<String>,
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
    topic_id: ID,
}

#[derive(Debug, SimpleObject)]
pub struct DeleteTopicPayload {
    client_mutation_id: Option<String>,
    deleted_topic_id: ID,
}

#[derive(Debug, InputObject)]
pub struct DeleteTopicTimeRangeInput {
    client_mutation_id: Option<String>,
    topic_id: ID,
}

#[derive(Debug, SimpleObject)]
pub struct DeleteTopicTimeRangePayload {
    client_mutation_id: Option<String>,
    deleted_time_range_id: Option<ID>,
    topic: Topic,
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
pub struct UpdateLinkTopicsInput {
    pub client_mutation_id: Option<String>,
    pub link_id: ID,
    pub parent_topic_ids: Vec<ID>,
}

#[derive(Debug, SimpleObject)]
pub struct UpdateLinkTopicsPayload {
    link: Link,
}

#[derive(Debug, InputObject)]
pub struct SynonymInput {
    pub name: String,
    pub locale: String,
}

impl SynonymInput {
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty() && !repo_url::Url::is_valid_url(&self.name)
    }

    pub fn to_synonym(&self) -> Synonym {
        Synonym {
            name: self.name.clone(),
            locale: self.locale.clone(),
        }
    }
}

#[derive(Debug, InputObject)]
pub struct UpdateSynonymsInput {
    pub client_mutation_id: Option<String>,
    pub synonyms: Vec<SynonymInput>,
    pub topic_id: ID,
}

#[derive(SimpleObject)]
pub struct UpdateSynonymsPayload {
    alerts: Vec<Alert>,
    client_mutation_id: Option<String>,
    topic: Option<Topic>,
}

#[derive(Debug, InputObject)]
pub struct UpsertLinkInput {
    pub add_parent_topic_ids: Vec<ID>,
    pub client_mutation_id: Option<String>,
    pub organization_login: String,
    pub repository_name: String,
    pub title: Option<String>,
    pub url: String,
}

#[derive(SimpleObject)]
pub struct UpsertLinkPayload {
    alerts: Vec<Alert>,
    link_edge: Option<LinkEdge>,
}

#[derive(Debug, InputObject)]
pub struct UpsertTopicInput {
    pub client_mutation_id: Option<String>,
    pub description: Option<String>,
    pub name: String,
    pub organization_login: String,
    pub repository_name: String,
    pub topic_ids: Vec<ID>,
}

#[derive(SimpleObject)]
pub struct UpsertTopicPayload {
    alerts: Vec<Alert>,
    topic_edge: Option<TopicEdge>,
}

#[derive(Debug, InputObject)]
pub struct UpsertTopicTimeRangeInput {
    pub client_mutation_id: Option<String>,
    pub topic_id: ID,
    pub starts_at: DateTime,
    pub ends_at: Option<DateTime>,
    pub prefix_format: TimeRangePrefixFormat,
}

#[derive(SimpleObject)]
pub struct UpsertTopicTimeRangePayload {
    alerts: Vec<Alert>,
    time_range_edge: Option<TimeRangeEdge>,
    topic: Topic,
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
        let repo = ctx.data_unchecked::<Repo>();

        if repo.server_secret != input.server_secret {
            log::warn!("server secret did not match secret provided by client");
            return Err(Error::Auth("failed to authenticate request".to_string()));
        }

        let result = repo.upsert_session(input).await?;
        log::info!(
            "server secret matched secret provided by client, user session created: {:?}",
            result.user
        );

        Ok(CreateSessionPayload {
            alerts: result.alerts,
            user_edge: Some(UserEdge {
                cursor: String::from("0"),
                node: result.user,
            }),
            session_edge: Some(SessionEdge {
                cursor: String::from("0"),
                node: Session {
                    id: result.session_id,
                },
            }),
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
        } = input;
        let DeleteLinkResult { deleted_link_id } = ctx
            .data_unchecked::<Repo>()
            .delete_link(link_id.to_string())
            .await?;

        Ok(DeleteLinkPayload {
            client_mutation_id,
            deleted_link_id: Some(ID(deleted_link_id)),
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
        let DeleteSessionResult { deleted_session_id } = ctx
            .data_unchecked::<Repo>()
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
            topic_id,
        } = input;
        ctx.data_unchecked::<Repo>()
            .delete_topic(topic_id.to_string())
            .await?;

        Ok(DeleteTopicPayload {
            client_mutation_id,
            deleted_topic_id: topic_id,
        })
    }

    async fn delete_topic_time_range(
        &self,
        ctx: &Context<'_>,
        input: DeleteTopicTimeRangeInput,
    ) -> Result<DeleteTopicTimeRangePayload> {
        let DeleteTopicTimeRangeInput {
            client_mutation_id,
            topic_id,
        } = input;

        let DeleteTopicTimeRangeResult {
            topic,
            deleted_time_range_id,
        } = ctx
            .data_unchecked::<Repo>()
            .delete_topic_time_range(topic_id.to_string())
            .await?;

        Ok(DeleteTopicTimeRangePayload {
            client_mutation_id,
            topic,
            deleted_time_range_id: deleted_time_range_id.map(ID),
        })
    }

    async fn select_repository(
        &self,
        ctx: &Context<'_>,
        input: SelectRepositoryInput,
    ) -> Result<SelectRepositoryPayload> {
        let SelectRepositoryResult { repository, viewer } = ctx
            .data_unchecked::<Repo>()
            .select_repository(input.repository_id.map(|id| id.to_string()))
            .await?;
        Ok(SelectRepositoryPayload { repository, viewer })
    }

    async fn update_link_topics(
        &self,
        ctx: &Context<'_>,
        input: UpdateLinkTopicsInput,
    ) -> Result<UpdateLinkTopicsPayload> {
        let UpdateLinkTopicsResult { link } = ctx
            .data_unchecked::<Repo>()
            .update_link_topics(input)
            .await?;
        Ok(UpdateLinkTopicsPayload { link })
    }

    async fn update_synonyms(
        &self,
        ctx: &Context<'_>,
        input: UpdateSynonymsInput,
    ) -> Result<UpdateSynonymsPayload> {
        let client_mutation_id = input.client_mutation_id.clone();
        let UpdateSynonymsResult { alerts, topic } =
            ctx.data_unchecked::<Repo>().update_synonyms(input).await?;

        Ok(UpdateSynonymsPayload {
            alerts,
            client_mutation_id,
            topic: Some(topic),
        })
    }

    async fn upsert_link(
        &self,
        ctx: &Context<'_>,
        input: UpsertLinkInput,
    ) -> Result<UpsertLinkPayload> {
        log::info!("upserting link: {:?}", input);
        let result = ctx.data_unchecked::<Repo>().upsert_link(input).await?;
        let edge = result
            .link
            .as_ref()
            .map(|link| LinkEdge::new(String::from("0"), link.clone()));

        Ok(UpsertLinkPayload {
            alerts: result.alerts,
            link_edge: edge,
        })
    }

    async fn upsert_topic(
        &self,
        ctx: &Context<'_>,
        input: UpsertTopicInput,
    ) -> Result<UpsertTopicPayload> {
        log::info!("upserting topic: {:?}", input);
        let result = ctx.data_unchecked::<Repo>().upsert_topic(input).await?;
        let edge = result
            .topic
            .as_ref()
            .map(|topic| TopicEdge::new(String::from("0"), topic.clone()));

        Ok(UpsertTopicPayload {
            alerts: result.alerts,
            topic_edge: edge,
        })
    }

    async fn upsert_topic_time_range(
        &self,
        ctx: &Context<'_>,
        input: UpsertTopicTimeRangeInput,
    ) -> Result<UpsertTopicTimeRangePayload> {
        let UpsertTopicTimeRangeResult {
            alerts,
            topic,
            time_range,
        } = ctx
            .data_unchecked::<Repo>()
            .upsert_topic_time_range(input)
            .await?;

        Ok(UpsertTopicTimeRangePayload {
            alerts,
            topic,
            time_range_edge: Some(TimeRangeEdge::new(String::from("0"), time_range)),
        })
    }
}
