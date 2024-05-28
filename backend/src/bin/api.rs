use async_graphql::extensions;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::EmptySubscription;
use async_graphql::Schema;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::post;
use axum::Json;
use axum::{
    extract::{Extension, State},
    routing::get,
    Router,
};
use axum_extra::{
    headers::authorization::{Authorization, Basic},
    TypedHeader,
};
use digraph::{
    config::Config,
    db, git,
    graphql::{MutationRoot, QueryRoot},
    prelude::*,
    redis,
    types::Timespec,
};
use serde::Serialize;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::signal;
use tower_http::cors::CorsLayer;

pub(crate) type ServiceSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

async fn viewer_from_header(
    auth: Option<TypedHeader<Authorization<Basic>>>,
    state: &digraph::graphql::State,
) -> Viewer {
    if let Some(TypedHeader(auth)) = auth {
        // https://docs.rs/axum-extra/latest/axum_extra/extract/cookie/struct.CookieJar.html
        let user_id = auth.username().to_owned();
        let session_id = auth.password().to_owned();
        state.authenticate((user_id, session_id)).await
    } else {
        log::info!("no auth header found, proceeding as guest");
        Viewer::guest()
    }
}

#[derive(Serialize)]
struct Health {
    healthy: bool,
}

pub(crate) async fn health() -> impl IntoResponse {
    let health = Health { healthy: true };
    (StatusCode::OK, Json(health))
}

async fn graphql_handler(
    schema: Extension<ServiceSchema>,
    auth: Option<TypedHeader<Authorization<Basic>>>,
    State(state): State<digraph::graphql::State>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let viewer = viewer_from_header(auth, &state).await;
    let store = state.store(Arc::new(viewer), &Timespec);
    log::info!("fetching request as viewer: {:?}", store.viewer);
    let response = async move { schema.execute(req.into_inner().data(store)).await }.await;
    response.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    opentelemetry::global::shutdown_tracer_provider();
}

#[tokio::main]
async fn main() -> async_graphql::Result<()> {
    let config = Config::load()?;
    env_logger::init();

    log::info!("setting up database connection");
    let pool = db::db_connection(&config).await?;

    log::info!("reading data from {}", config.digraph_data_directory);
    let root = git::DataRoot::new(PathBuf::from(&config.digraph_data_directory));

    log::info!("running migrations");
    sqlx::migrate!("./db/migrations").run(&pool).await?;

    log::info!("loading graphql schema");
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .extension(extensions::Logger)
        .finish();

    log::info!("loading graphql schema");
    let redis = Arc::new(redis::Redis::new(config.digraph_redis_url.to_owned())?);

    log::info!("setting up app state");
    let state = digraph::graphql::State::new(
        pool,
        root,
        schema.clone(),
        config.digraph_server_secret,
        redis,
    );

    let socket = env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_owned());
    let listener = tokio::net::TcpListener::bind(socket).await.unwrap();

    // For metrics, see https://github.com/oliverjumpertz/axum-graphql/blob/main/src/main.rs
    log::info!("starting server");
    let app = Router::new()
        .route("/health", get(health))
        .route("/", get(graphql_playground))
        .route("/graphql", post(graphql_handler))
        .layer(Extension(schema))
        .layer(CorsLayer::permissive())
        .with_state(state);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
}
