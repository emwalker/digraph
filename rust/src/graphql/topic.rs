use async_graphql::connection::*;
use itertools::Itertools;

use super::{
    relay::conn, ActivityLineItem, LinkConnection, Prefix, Repository, Synonym, Synonyms,
    Timerange, TopicChildConnection,
};
use super::{ActivityLineItemConnection, LinkConnectionFields};
use crate::repo::{Repo, RepoPath};
use crate::{git, prelude::*};

#[derive(Debug, SimpleObject)]
pub struct SynonymMatch {
    pub display_name: String,
    pub path: String,
}

#[derive(Debug, SimpleObject)]
pub struct LiveSearchTopicsPayload {
    pub synonym_matches: Vec<SynonymMatch>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Topic {
    pub child_paths: Vec<RepoPath>,
    pub path: RepoPath,
    pub name: String,
    pub parent_topic_paths: Vec<RepoPath>,
    pub prefix: Prefix,
    pub root: bool,
    pub synonyms: Synonyms,
    pub timerange: Option<Timerange>,
}

pub type TopicEdge = Edge<String, Topic, EmptyFields>;
pub type TopicConnection = Connection<String, Topic, EmptyFields, EmptyFields>;

#[Object]
impl Topic {
    async fn activity(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<ActivityLineItemConnection> {
        let repo = ctx.data_unchecked::<Repo>();
        let activity = repo
            .activity(Some(self.path.inner.clone()), first.unwrap_or(3))
            .await?;

        let mut results = vec![];
        for change in activity {
            let actor = repo.user_loader.load_one(change.user_id()).await?;
            let actor_name = actor
                .map(|user| user.name)
                .unwrap_or_else(|| "[missing user]".to_owned());

            results.push(ActivityLineItem {
                created_at: change.date(),
                description: change.markdown(Locale::EN, &actor_name, Some(&self.path))?,
            });
        }

        conn(after, before, first, last, results)
    }

    async fn available_parent_topics(
        &self,
        ctx: &Context<'_>,
        search_string: Option<String>,
    ) -> Result<LiveSearchTopicsPayload> {
        let git::FetchTopicLiveSearchResult { synonym_matches } = ctx
            .data_unchecked::<Repo>()
            .search_topics(search_string)
            .await?;
        let synonym_matches = synonym_matches
            .iter()
            .map(SynonymMatch::from)
            .collect::<Vec<SynonymMatch>>();
        Ok(LiveSearchTopicsPayload { synonym_matches })
    }

    #[allow(unused_variables)]
    async fn children(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
        search_string: Option<String>,
    ) -> Result<TopicChildConnection> {
        let results = ctx
            .data_unchecked::<Repo>()
            .topic_children(&self.path)
            .await?;
        conn(after, before, first, last, results)
    }

    async fn description(&self) -> Option<String> {
        None
    }

    async fn display_name(&self) -> String {
        self.synonyms.display_name("en", &self.name, &self.prefix)
    }

    // TODO: rename to childLinks
    #[allow(unused_variables, clippy::too_many_arguments)]
    async fn links(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
        search_string: Option<String>,
        reviewed: Option<bool>,
        descendants: Option<bool>,
    ) -> Result<LinkConnection> {
        query(
            after,
            before,
            first,
            last,
            |_after, _before, _first, _last| async move {
                let results = ctx
                    .data::<Repo>()?
                    .child_links_for_topic(&self.path, reviewed)
                    .await?;
                let mut connection = Connection::with_additional_fields(
                    false,
                    false,
                    LinkConnectionFields {
                        total_count: results.len() as i64,
                    },
                );

                connection.edges.extend(
                    results
                        .into_iter()
                        .map(|n| Edge::with_additional_fields(String::from("0"), n, EmptyFields)),
                );

                Ok::<_, Error>(connection)
            },
        )
        .await
        .map_err(Error::Resolver)
    }

    async fn loading(&self) -> bool {
        false
    }

    async fn id(&self) -> String {
        self.path.to_string()
    }

    async fn name(&self) -> &str {
        self.name.as_str()
    }

    async fn newly_added(&self) -> bool {
        false
    }

    async fn parent_topics(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<TopicConnection> {
        conn(
            after,
            before,
            first,
            last,
            ctx.data_unchecked::<Repo>()
                .parent_topics_for_topic(&self.path)
                .await?,
        )
    }

    async fn path(&self) -> String {
        self.path.to_string()
    }

    async fn repository(&self, ctx: &Context<'_>) -> Result<Option<Repository>> {
        ctx.data_unchecked::<Repo>()
            .repository_by_prefix(self.path.org_login.clone())
            .await
    }

    async fn resource_path(&self) -> &str {
        self.path.inner.as_str()
    }

    async fn search(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        after: Option<String>,
        last: Option<i32>,
        before: Option<String>,
        search_string: String,
    ) -> Result<TopicChildConnection> {
        conn(
            after,
            before,
            first,
            last,
            ctx.data_unchecked::<Repo>()
                .search(self.clone(), search_string)
                .await?,
        )
    }

    async fn synonyms(&self) -> Vec<Synonym> {
        self.synonyms.into_iter().collect_vec()
    }

    async fn timerange(&self) -> Option<Timerange> {
        self.timerange.clone()
    }

    async fn viewer_can_delete_synonyms(&self, ctx: &Context<'_>) -> Result<bool> {
        if self.synonyms.len() < 2 {
            return Ok(false);
        }
        self.viewer_can_update(ctx).await
    }

    async fn viewer_can_update(&self, ctx: &Context<'_>) -> Result<bool> {
        let repo = ctx.data_unchecked::<Repo>();
        if repo.viewer.is_guest() {
            return Ok(false);
        }

        let repository = match self.repository(ctx).await {
            Ok(repository) => match &repository {
                Some(repository) => repository.clone(),
                None => return Ok(false),
            },
            Err(_) => return Ok(false),
        };

        if let Repository::Fetched {
            owner_id, private, ..
        } = repository
        {
            Ok(!private || (owner_id == repo.viewer.user_id))
        } else {
            Ok(false)
        }
    }
}
