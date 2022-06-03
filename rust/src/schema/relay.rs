use async_graphql::connection::*;

use futures::executor;

use crate::prelude::*;

pub fn conn<N>(
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
    results: Vec<N>,
) -> Result<Connection<usize, N, EmptyFields, EmptyFields>> {
    let result = query(
        after,
        before,
        first,
        last,
        |_after, _before, _first, _last| async move {
            let mut connection = Connection::new(false, false);
            connection.append(
                results
                    .into_iter()
                    .map(|n| Edge::with_additional_fields(0_usize, n, EmptyFields)),
            );
            Ok::<_, Error>(connection)
        },
    );
    executor::block_on(result).map_err(Error::Resolver)
}
