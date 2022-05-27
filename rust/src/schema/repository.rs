use async_graphql::*;

use super::organization::Organization;
use super::topic::Topic;

#[derive(Clone, SimpleObject)]
pub struct Repository {
    pub id: ID,
    pub name: String,
    pub organization: Organization,
    pub root_topic: Topic,
    pub url: String,
}
