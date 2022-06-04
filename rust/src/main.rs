#[macro_use]
extern crate quick_error;

use actix_web::{guard, web, App, HttpResponse, HttpServer};
// use async_graphql::extensions::ApolloTracing;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use futures::lock::Mutex;
use std::env;

mod db;
mod errors;
mod prelude;
mod psql;
mod query;
mod schema;
use errors::Error;

pub type Result<T> = std::result::Result<T, Error>;

use query::{QueryRoot, Schema, State};
use schema::View;

async fn index(state: web::Data<State>, req: GraphQLRequest) -> GraphQLResponse {
    let repo = state.create_repo();
    let view = Mutex::<Option<View>>::new(None);
    state
        .schema
        .execute(req.into_inner().data(repo).data(view))
        .await
        .into()
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        )))
}

#[actix_web::main]
async fn main() -> async_graphql::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let pool = db::db_connection().await?;
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        // .extension(ApolloTracing)
        .finish();
    let state = State::new(pool, schema);

    let socket = env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_owned());
    println!("Playground: http://localhost:8080");

    // TODO: Look into switching to https://github.com/poem-web/poem
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(web::resource("/graphql").guard(guard::Post()).to(index))
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .to(index_playground),
            )
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind(socket)?
    .run()
    .await?;

    Ok(())
}
