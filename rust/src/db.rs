use sqlx::postgres::Postgres;
use sqlx::Error;
use sqlx::Pool;

use crate::config::Config;

pub async fn db_connection(config: &Config) -> Result<Pool<Postgres>, Error> {
    let database_url = config.digraph_postgres_connection.clone();
    Pool::<Postgres>::connect(&*database_url).await
}
