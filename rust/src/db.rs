use sqlx::postgres::Postgres;
use sqlx::Error;
use sqlx::Pool;
use std::env;

pub async fn db_connection() -> Result<Pool<Postgres>, Error> {
    let database_url = env::var("DATABASE_URL").expect("Required a database url");

    Pool::<Postgres>::connect(&*database_url).await
}
