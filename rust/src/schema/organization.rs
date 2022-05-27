use async_graphql::*;

use super::user::User;

#[derive(Clone, SimpleObject)]
pub struct Organization {
    pub id: ID,
    pub name: String,
    pub owners: Vec<User>,
}
