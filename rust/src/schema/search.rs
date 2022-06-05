use async_graphql::{connection::*, Union};

use super::{Link, Topic};

#[derive(Union)]
pub enum SearchResultItem {
    Link(Link),
    Topic(Topic),
}

pub type SearchResultItemConnection = Connection<usize, SearchResultItem, EmptyFields, EmptyFields>;
