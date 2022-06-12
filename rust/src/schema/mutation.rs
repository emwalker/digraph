use async_graphql::{Context, InputObject, Object, SimpleObject};

use super::{Alert, Link, LinkEdge, Session, SessionEdge, Synonym, Topic, TopicEdge, UserEdge};
use crate::http::repo_url;
use crate::prelude::*;
use crate::psql::{Repo, UpdateSynonymsResult};

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

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_github_session(
        &self,
        ctx: &Context<'_>,
        input: CreateGithubSessionInput,
    ) -> Result<CreateSessionPayload> {
        log::info!("creating GitHub session: {:?}", input);
        let result = ctx.data_unchecked::<Repo>().upsert_session(input).await?;

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

    async fn update_link_topics(
        &self,
        ctx: &Context<'_>,
        input: UpdateLinkTopicsInput,
    ) -> Result<UpdateLinkTopicsPayload> {
        let result = ctx
            .data_unchecked::<Repo>()
            .update_link_topics(input)
            .await?;
        Ok(UpdateLinkTopicsPayload { link: result.link })
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
}
