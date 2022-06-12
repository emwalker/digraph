#[macro_use]
extern crate quick_error;

#[macro_use]
extern crate derivative;

use actix_web::{guard, post, web, App, HttpRequest, HttpResponse, HttpServer};
// use async_graphql::extensions::ApolloTracing;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::EmptySubscription;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use futures::lock::Mutex;
use std::env;

mod config;
mod db;
mod errors;
mod http;
mod prelude;
mod psql;
mod schema;
use config::Config;
use errors::Error;

pub type Result<T> = std::result::Result<T, Error>;

use schema::{MutationRoot, QueryRoot, Schema, State, View};

struct AuthHeader(String);

impl AuthHeader {
    fn decode(&self) -> Result<(String, String)> {
        let encoded = self.0.split(' ').last().unwrap_or_default();
        let decoded = base64::decode(&encoded)?;
        let decoded = String::from_utf8_lossy(&decoded);
        let parts = decoded
            .split(':')
            .map(str::to_string)
            .collect::<Vec<String>>();

        if parts.len() != 2 {
            return Err(Error::AuthHeader(format!(
                "unexpected message: {}",
                self.0
            )));
        }

        Ok((parts[0].clone(), parts[1].clone()))
    }
}

fn user_id_from_header(req: HttpRequest) -> Option<String> {
    match req.headers().get("authorization") {
        Some(value) => match value.to_str() {
            Ok(value) => match AuthHeader(value.into()).decode() {
                Ok((user_id, _session_id)) => {
                    log::info!("user id found in auth header: {}", user_id);
                    Some(user_id)
                }
                Err(err) => {
                    log::info!("failed to decode auth header, proceeding as guest: {}", err);
                    None
                }
            },
            Err(err) => {
                log::warn!("problem fetching authorization header value: {}", err);
                None
            }
        },
        None => {
            log::warn!("no authorization header, proceeding as guest");
            None
        }
    }
}

#[post("/graphql")]
async fn index(
    state: web::Data<State>,
    req: GraphQLRequest,
    http_req: HttpRequest,
) -> GraphQLResponse {
    let user_id = user_id_from_header(http_req);
    let viewer = state.viewer(user_id).await;
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
    let _config = Config::load()?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_header_parsing() {
        let auth = AuthHeader("Bearer NDYxYzg3YzgtZmI4Zi0xMWU4LTljYmMtYWZkZTZjNTRkODgxOmFiM2Q1MTYwYWFlNjMyYTUxNzNjMDVmOGNiMGVmMDg2ODY2ZGFkMTAzNTE3ZGQwMTRmMzhhNWIxY2E2OWI5YWE=".into());
        let (user_id, session_id) = auth.decode().unwrap();
        assert_eq!(user_id, "461c87c8-fb8f-11e8-9cbc-afde6c54d881");
        assert_eq!(
            session_id,
            "ab3d5160aae632a5173c05f8cb0ef086866dad103517dd014f38a5b1ca69b9aa"
        );
    }
}
