use async_graphql::{Context, InputObject, Object, SimpleObject};

use super::{Alert, LinkEdge, Session, SessionEdge, TopicEdge, UserEdge};
use crate::prelude::*;
use crate::psql::Repo;

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
pub struct UpsertLinkInput {
    pub add_parent_topic_ids: Vec<String>,
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
    pub topic_ids: Vec<String>,
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
