use async_graphql::{Context, InputObject, Object, SimpleObject, ID};
use itertools::Itertools;

use super::{
    alert, timerange, DateTime, Link, LinkEdge, Repository, Session, SessionEdge, Synonym,
    TimerangeEdge, Topic, TopicEdge, User, UserEdge,
};
use crate::git;
use crate::prelude::*;
use crate::psql;
use crate::repo::Repo;

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
    link_path: String,
}

#[derive(Debug, SimpleObject)]
pub struct DeleteLinkPayload {
    client_mutation_id: Option<String>,
    deleted_link_path: Option<String>,
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
    topic_path: String,
}

#[derive(Debug, SimpleObject)]
pub struct DeleteTopicPayload {
    client_mutation_id: Option<String>,
    deleted_topic_path: String,
}

#[derive(Debug, InputObject)]
pub struct RemoveTopicTimerangeInput {
    client_mutation_id: Option<String>,
    topic_path: String,
}

#[derive(Debug, SimpleObject)]
pub struct RemoveTopicTimerangePayload {
    client_mutation_id: Option<String>,
    topic: Topic,
}

#[derive(Debug, InputObject)]
pub struct ReviewLinkInput {
    client_mutation_id: Option<String>,
    link_path: String,
    reviewed: bool,
}

#[derive(Debug, SimpleObject)]
pub struct ReviewLinkPayload {
    link: Link,
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
    pub link_path: String,
    pub parent_topic_paths: Vec<String>,
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

    pub fn to_synonym(&self) -> Synonym {
        Synonym {
            name: self.name.clone(),
            locale: self.locale.clone(),
        }
    }
}

#[derive(Debug, InputObject)]
pub struct UpdateTopicSynonymsInput {
    pub client_mutation_id: Option<String>,
    pub synonyms: Vec<SynonymInput>,
    pub topic_path: String,
}

#[derive(SimpleObject)]
pub struct UpdateTopicSynonymsPayload {
    alerts: Vec<alert::Alert>,
    client_mutation_id: Option<String>,
    topic: Option<Topic>,
}

#[derive(Debug, InputObject)]
pub struct UpdateTopicParentTopicsInput {
    client_mutation_id: Option<String>,
    topic_path: String,
    parent_topic_paths: Vec<String>,
}

#[derive(SimpleObject)]
pub struct UpdateTopicParentTopicsPayload {
    alerts: Vec<alert::Alert>,
    topic: Topic,
}

#[derive(Debug, InputObject)]
pub struct UpsertLinkInput {
    pub add_parent_topic_path: Option<String>,
    pub client_mutation_id: Option<String>,
    pub organization_login: String,
    pub repository_name: String,
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
    pub organization_login: String,
    pub repository_name: String,
    pub parent_topic_path: String,
}

#[derive(SimpleObject)]
pub struct UpsertTopicPayload {
    alerts: Vec<alert::Alert>,
    topic_edge: Option<TopicEdge>,
}

#[derive(Debug, InputObject)]
pub struct UpsertTopicTimerangeInput {
    pub client_mutation_id: Option<String>,
    pub topic_path: String,
    pub starts_at: DateTime,
    pub ends_at: Option<DateTime>,
    pub prefix_format: timerange::TimerangePrefixFormat,
}

#[derive(SimpleObject)]
pub struct UpsertTopicTimerangePayload {
    alerts: Vec<alert::Alert>,
    timerange_edge: Option<TimerangeEdge>,
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
            .data_unchecked::<Repo>()
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
            link_path,
        } = input;
        let link_path = RepoPath::from(&link_path);
        let git::DeleteLinkResult {
            deleted_link_path, ..
        } = ctx.data_unchecked::<Repo>().delete_link(&link_path).await?;

        Ok(DeleteLinkPayload {
            client_mutation_id,
            deleted_link_path: Some(deleted_link_path.inner),
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
            topic_path,
        } = input;
        ctx.data_unchecked::<Repo>()
            .delete_topic(&RepoPath::from(&topic_path))
            .await?;

        Ok(DeleteTopicPayload {
            client_mutation_id,
            deleted_topic_path: topic_path,
        })
    }

    async fn remove_topic_timerange(
        &self,
        ctx: &Context<'_>,
        input: RemoveTopicTimerangeInput,
    ) -> Result<RemoveTopicTimerangePayload> {
        let RemoveTopicTimerangeInput {
            client_mutation_id,
            topic_path,
        } = input;

        let topic_path = RepoPath::from(&topic_path);
        let git::RemoveTopicTimerangeResult { topic, .. } = ctx
            .data_unchecked::<Repo>()
            .remove_topic_timerange(&topic_path)
            .await?;

        Ok(RemoveTopicTimerangePayload {
            client_mutation_id,
            topic: Topic::from(&topic),
        })
    }

    async fn review_link(
        &self,
        ctx: &Context<'_>,
        input: ReviewLinkInput,
    ) -> Result<ReviewLinkPayload> {
        let ReviewLinkInput {
            link_path,
            reviewed,
            ..
        } = input;

        let psql::ReviewLinkResult { link, .. } = ctx
            .data_unchecked::<Repo>()
            .review_link(&RepoPath::from(&link_path), reviewed)
            .await?;

        Ok(ReviewLinkPayload {
            link: Link::from(&link),
        })
    }

    async fn select_repository(
        &self,
        ctx: &Context<'_>,
        input: SelectRepositoryInput,
    ) -> Result<SelectRepositoryPayload> {
        let psql::SelectRepositoryResult { repository, actor } = ctx
            .data_unchecked::<Repo>()
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
        let git::UpdateLinkParentTopicsResult { link, .. } = ctx
            .data_unchecked::<Repo>()
            .update_link_parent_topics(input)
            .await?;
        Ok(UpdateLinkParentTopicsPayload {
            link: Link::from(&link),
        })
    }

    async fn update_topic_parent_topics(
        &self,
        ctx: &Context<'_>,
        input: UpdateTopicParentTopicsInput,
    ) -> Result<UpdateTopicParentTopicsPayload> {
        let UpdateTopicParentTopicsInput {
            topic_path,
            parent_topic_paths,
            ..
        } = input;
        let git::UpdateTopicParentTopicsResult { alerts, topic } = ctx
            .data_unchecked::<Repo>()
            .update_topic_parent_topics(
                &RepoPath::from(&topic_path),
                parent_topic_paths.iter().map(RepoPath::from).collect(),
            )
            .await?;

        Ok(UpdateTopicParentTopicsPayload {
            alerts: alerts.iter().map(alert::Alert::from).collect_vec(),
            topic: Topic::from(&topic),
        })
    }

    async fn update_topic_synonyms(
        &self,
        ctx: &Context<'_>,
        input: UpdateTopicSynonymsInput,
    ) -> Result<UpdateTopicSynonymsPayload> {
        let client_mutation_id = input.client_mutation_id.clone();
        let git::UpdateTopicSynonymsResult { alerts, topic } = ctx
            .data_unchecked::<Repo>()
            .update_topic_synonyms(input)
            .await?;

        Ok(UpdateTopicSynonymsPayload {
            alerts: alerts.iter().map(alert::Alert::from).collect_vec(),
            client_mutation_id,
            topic: Some(Topic::from(&topic)),
        })
    }

    async fn upsert_link(
        &self,
        ctx: &Context<'_>,
        input: UpsertLinkInput,
    ) -> Result<UpsertLinkPayload> {
        let result = ctx.data_unchecked::<Repo>().upsert_link(input).await?;
        let edge = result
            .link
            .as_ref()
            .map(|link| LinkEdge::new(String::from("0"), Link::from(link)));

        Ok(UpsertLinkPayload {
            alerts: result.alerts.iter().map(alert::Alert::from).collect_vec(),
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
            .map(|topic| TopicEdge::new(String::from("0"), Topic::from(topic)));

        Ok(UpsertTopicPayload {
            alerts: result.alerts.iter().map(alert::Alert::from).collect_vec(),
            topic_edge: edge,
        })
    }

    async fn upsert_topic_timerange(
        &self,
        ctx: &Context<'_>,
        input: UpsertTopicTimerangeInput,
    ) -> Result<UpsertTopicTimerangePayload> {
        let git::UpsertTopicTimerangeResult {
            alerts,
            topic,
            timerange,
        } = ctx
            .data_unchecked::<Repo>()
            .upsert_topic_timerange(input)
            .await?;

        Ok(UpsertTopicTimerangePayload {
            alerts: alerts.iter().map(alert::Alert::from).collect_vec(),
            topic: Topic::from(&topic),
            timerange_edge: Some(TimerangeEdge::new(
                String::from("0"),
                timerange::Timerange::from(&timerange),
            )),
        })
    }
}
