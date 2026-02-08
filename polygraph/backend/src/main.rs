use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod db;
mod ml;
mod models;
mod services;

use api::graphql::{create_schema, graphql_handler, graphql_playground};
use db::{ArangoClient, XtdbClient, CacheClient};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "polygraph=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = config::Config::builder()
        .add_source(config::Environment::with_prefix("POLYGRAPH"))
        .build()?;

    // Initialize database clients
    let arango = ArangoClient::new(
        config.get_string("arango_url")?.as_str(),
        config.get_string("arango_db")?.as_str(),
    ).await?;

    let xtdb = XtdbClient::new(
        config.get_string("xtdb_url")?.as_str(),
    ).await?;

    let cache = CacheClient::new(
        config.get_string("redis_url")?.as_str(),
    ).await?;

    // Create GraphQL schema
    let schema = create_schema(arango, xtdb, cache).await;

    // Build application router
    let app = Router::new()
        .route("/", get(|| async { "Social Media Polygraph GraphQL API" }))
        .route("/health", get(health_check))
        .route("/graphql", post(graphql_handler).get(graphql_playground))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(schema);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::info!("GraphQL server listening on {}", addr);
    tracing::info!("Playground: http://localhost:8000/graphql");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
