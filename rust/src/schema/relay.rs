use async_graphql::connection::*;
use async_graphql::*;

pub async fn conn<N>(results: Vec<N>) -> Result<Connection<usize, N, EmptyFields, EmptyFields>> {
    query(
        None,
        None,
        None,
        None,
        |_after, _before, _first, _last| async move {
            let mut connection = Connection::new(false, false);
            connection.append(
                results
                    .into_iter()
                    .map(|n| Edge::with_additional_fields(0 as usize, n, EmptyFields)),
            );
            Ok::<_, Error>(connection)
        },
    )
    .await
}
