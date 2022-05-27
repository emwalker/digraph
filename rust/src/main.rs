use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use std::env;

mod db;
mod psql;
mod query;
mod schema;
mod server;
mod state;

use query::{QueryRoot, Schema};
use state::State;

async fn index(schema: web::Data<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
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

    let conn = db::db_connection().await?;
    let state = State::new(conn);
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(state)
        .finish();

    let socket = env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:8000".to_owned());
    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
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
