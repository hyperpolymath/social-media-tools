// NUJ Social Media Ethics Monitor - Collector Service
// Purpose: Monitor social media platforms for policy changes
// Tech: Rust + Axum + SQLx + Tokio

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::{error, info};

mod config;
mod db;
mod handlers;
mod models;
mod platforms;
mod scheduler;
mod scraper;

use config::Config;

#[derive(Clone)]
struct AppState {
    db: sqlx::PgPool,
    redis: redis::Client,
    config: Arc<Config>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .json()
        .init();

    info!("Starting NUJ Collector Service");

    // Load configuration
    let config = Arc::new(Config::from_env()?);
    info!("Configuration loaded");

    // Connect to SurrealDB (PostgreSQL-compatible endpoint for the collector)
    let db = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await?;
    info!("Connected to SurrealDB");

    // Connect to Redis
    let redis = redis::Client::open(config.redis.url.as_str())?;
    let mut redis_conn = redis.get_connection()?;
    redis::cmd("PING").query::<String>(&mut redis_conn)?;
    info!("Connected to Redis");

    // Build application state
    let state = AppState {
        db: db.clone(),
        redis: redis.clone(),
        config: config.clone(),
    };

    // Start background scheduler
    let scheduler = scheduler::start_scheduler(state.clone()).await?;
    info!("Background scheduler started");

    // Build router
    let app = create_router(state);

    // Start server
    let addr = format!("0.0.0.0:{}", config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(scheduler))
        .await?;

    Ok(())
}

fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(metrics))
        .route("/api/platforms", get(handlers::list_platforms))
        .route("/api/platforms/:id", get(handlers::get_platform))
        .route("/api/platforms/:id/collect", post(handlers::trigger_collection))
        .route("/api/changes", get(handlers::list_changes))
        .route("/api/changes/:id", get(handlers::get_change))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "collector",
        "timestamp": chrono::Utc::now()
    }))
}

async fn metrics() -> impl IntoResponse {
    use prometheus::{Encoder, TextEncoder};

    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];

    if let Err(e) = encoder.encode(&metric_families, &mut buffer) {
        error!("Failed to encode metrics: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to encode metrics".to_string());
    }

    (StatusCode::OK, String::from_utf8(buffer).unwrap_or_default())
}

async fn shutdown_signal(mut scheduler: tokio_cron_scheduler::JobScheduler) {
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
        _ = ctrl_c => {
            info!("Received Ctrl+C, shutting down");
        },
        _ = terminate => {
            info!("Received SIGTERM, shutting down");
        },
    }

    if let Err(e) = scheduler.shutdown().await {
        error!("Error shutting down scheduler: {}", e);
    }
}
