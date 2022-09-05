use async_graphql::{Context, InputObject, Object, SimpleObject, ID};
use itertools::Itertools;

use super::{
    alert, timerange, DateTime, Link, LinkEdge, Repository, Session, SessionEdge, Synonym,
    TimerangeEdge, Topic, TopicEdge, User, UserEdge,
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
    topic: Topic,
}

#[derive(Debug, InputObject)]
pub struct ReviewLinkInput {
    client_mutation_id: Option<String>,
    link_id: String,
    repo_id: String,
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
    pub repo_id: String,
    pub synonyms: Vec<SynonymInput>,
    pub topic_id: String,
}

#[derive(SimpleObject)]
pub struct UpdateTopicSynonymsPayload {
    alerts: Vec<alert::Alert>,
    client_mutation_id: Option<String>,
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
}

#[derive(Debug, InputObject)]
pub struct UpsertTopicTimerangeInput {
    pub client_mutation_id: Option<String>,
    pub ends_at: Option<DateTime>,
    pub prefix_format: timerange::TimerangePrefixFormat,
    pub repo_id: String,
    pub starts_at: DateTime,
    pub topic_id: String,
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
        store
            .remove_topic_timerange(&repo_id.try_into()?, &topic_id)
            .await?;
        let topic = store.fetch_topic(&topic_id).await?;

        Ok(RemoveTopicTimerangePayload {
            client_mutation_id: client_mutation_id.to_owned(),
            topic,
        })
    }

    async fn review_link(
        &self,
        ctx: &Context<'_>,
        input: ReviewLinkInput,
    ) -> Result<ReviewLinkPayload> {
        let ReviewLinkInput {
            link_id,
            repo_id,
            reviewed,
            ..
        } = &input;
        let link_id: Oid = link_id.try_into()?;
        let store = ctx.data_unchecked::<Store>();

        let _ = store
            .review_link(&repo_id.try_into()?, &link_id, *reviewed)
            .await?;

        let link: Link = store.fetch_link(&link_id).await?.try_into()?;
        Ok(ReviewLinkPayload { link })
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
        let git::UpdateLinkParentTopicsResult { link, .. } = ctx
            .data_unchecked::<Store>()
            .update_link_parent_topics(input)
            .await?;

        Ok(UpdateLinkParentTopicsPayload {
            link: Link::try_from(&link)?,
        })
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

        let git::UpdateTopicParentTopicsResult { alerts, .. } = ctx
            .data_unchecked::<Store>()
            .update_topic_parent_topics(
                &repo_id.try_into()?,
                &topic_id.try_into()?,
                parent_topic_ids
                    .iter()
                    .map(Oid::try_from)
                    .collect::<Result<Vec<Oid>>>()?,
            )
            .await?;

        Ok(UpdateTopicParentTopicsPayload {
            alerts: alerts.iter().map(alert::Alert::from).collect_vec(),
        })
    }

    async fn update_topic_synonyms(
        &self,
        ctx: &Context<'_>,
        input: UpdateTopicSynonymsInput,
    ) -> Result<UpdateTopicSynonymsPayload> {
        let client_mutation_id = input.client_mutation_id.clone();
        let git::UpdateTopicSynonymsResult { alerts, .. } = ctx
            .data_unchecked::<Store>()
            .update_topic_synonyms(input)
            .await?;

        Ok(UpdateTopicSynonymsPayload {
            alerts: alerts.iter().map(alert::Alert::from).collect_vec(),
            client_mutation_id,
        })
    }

    async fn upsert_link(
        &self,
        ctx: &Context<'_>,
        input: UpsertLinkInput,
    ) -> Result<UpsertLinkPayload> {
        let result = ctx.data_unchecked::<Store>().upsert_link(input).await?;

        let edge = if let Some(link) = &result.link {
            Some(LinkEdge::new(String::from("0"), Link::try_from(link)?))
        } else {
            None
        };

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
        let store = ctx.data_unchecked::<Store>();
        let result = store.upsert_topic(input).await?;

        let edge = match &result.repo_topic {
            Some(repo_topic) => {
                let topic = store.fetch_topic(repo_topic.id()).await?;
                Some(TopicEdge::new(String::from("0"), topic))
            }

            None => None,
        };

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
        let store = ctx.data_unchecked::<Store>();
        let topic_id = (&input.topic_id).try_into()?;

        let git::UpsertTopicTimerangeResult { alerts, timerange } =
            store.upsert_topic_timerange(input).await?;

        let topic = store.fetch_topic(&topic_id).await?;

        Ok(UpsertTopicTimerangePayload {
            alerts: alerts.iter().map(alert::Alert::from).collect_vec(),
            topic,
            timerange_edge: Some(TimerangeEdge::new(
                String::from("0"),
                timerange::Timerange::try_from(&timerange)?,
            )),
        })
    }
}
