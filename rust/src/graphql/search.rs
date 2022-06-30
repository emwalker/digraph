use async_graphql::{connection::*, Union};

use super::{Link, Topic};

#[derive(Union)]
pub enum TopicChild {
    Link(Link),
    Topic(Topic),
}

pub type TopicChildConnection = Connection<String, TopicChild, EmptyFields, EmptyFields>;
