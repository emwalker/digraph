#[macro_use]
extern crate quick_error;

use actix_web::{
    cookie::{time::Duration, SameSite},
    guard, post, web, App, HttpResponse, HttpServer,
};
// use async_graphql::extensions::ApolloTracing;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::EmptySubscription;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use futures::lock::Mutex;
use std::env;

mod config;
mod db;
mod errors;
mod prelude;
mod psql;
mod schema;
use actix_identity::Identity;
use config::Config;
use errors::Error;

pub type Result<T> = std::result::Result<T, Error>;

use schema::{MutationRoot, QueryRoot, Schema, Session, State, View};

#[post("/graphql")]
async fn index(state: web::Data<State>, req: GraphQLRequest, id: Identity) -> GraphQLResponse {
    let session = Session::from(&id).unwrap();
    let viewer = state.viewer(session.id).await;
    let repo = state.create_repo(viewer);
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
    let config = Config::load()?;
    env_logger::init();

    let pool = db::db_connection().await?;
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        // .extension(ApolloTracing)
        .finish();
    let state = State::new(pool, schema);

    let socket = env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_owned());
    println!("Playground: http://localhost:8080");

    // TODO: Look into switching to https://github.com/poem-web/poem
    HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(config.session_key.as_bytes())
                    .name("auth-cookie")
                    .max_age(Duration::days(356))
                    .same_site(SameSite::Strict)
                    .secure(true),
            ))
            .app_data(web::Data::new(state.clone()))
            .service(index)
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
