use async_graphql::{connection::*, OutputType};
use futures::executor;
use itertools::Itertools;

use super::Topic;
use crate::git;
use crate::prelude::*;

pub fn connection<N: OutputType>(
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
    results: Vec<N>,
) -> Result<Connection<String, N, EmptyFields, EmptyFields, DefaultConnectionName, DefaultEdgeName>>
{
    let result = query(
        after,
        before,
        first,
        last,
        |_after, _before, _first, _last| async move {
            let mut connection = Connection::new(false, false);
            connection.edges.extend(
                results
                    .into_iter()
                    .map(|n| Edge::with_additional_fields(String::from("0"), n, EmptyFields)),
            );
            Ok::<_, Error>(connection)
        },
    );
    executor::block_on(result).map_err(Error::Resolver)
}

pub fn topics(
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
    topics: Vec<git::Topic>,
) -> Result<
    Connection<String, Topic, EmptyFields, EmptyFields, DefaultConnectionName, DefaultEdgeName>,
> {
    let results = topics.iter().map(Topic::from).collect_vec();
    connection(after, before, first, last, results)
}
