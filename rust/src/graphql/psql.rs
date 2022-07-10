use async_graphql::ID;

use super::User;
use crate::psql;

impl From<&psql::user::Row> for User {
    fn from(row: &psql::user::Row) -> Self {
        User::Registered {
            id: ID(row.id.to_string()),
            name: row.name.to_owned(),
            avatar_url: row.avatar_url.to_owned(),
            selected_repository_id: row.selected_repository_id.map(|uuid| ID(uuid.to_string())),
        }
    }
}
