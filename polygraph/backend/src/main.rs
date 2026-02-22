// SPDX-License-Identifier: PMPL-1.0-or-later

/**
 * Polygraph Backend — Fact-Checking and Credibility Analysis Engine.
 *
 * This binary implements the core intelligence layer for the Polygraph project.
 * It integrates heterogeneous data stores and machine learning models 
 * to evaluate the veracity of social media claims.
 *
 * ARCHITECTURE:
 * 1. **Graph Layer**: ArangoDB for tracking relationships between claims and sources.
 * 2. **Bitemporal Layer**: XTDB for maintaining an immutable audit log of claim state.
 * 3. **AI Layer**: Rust-based NLP and credibility scoring modules.
 * 4. **WASM Interface**: High-performance credibility algorithms executed in sandboxes.
 */

mod api;
mod db;
mod ml;
mod models;
mod services;
mod wasm;

use tracing::{info, warn, error};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. OBSERVABILITY: Init structured logging.
    tracing_subscriber::fmt::init();

    // 2. CONNECTIVITY: Establish links to ArangoDB and XTDB.
    let graph_db = db::arango::connect().await?;
    let temporal_db = db::xtdb::connect().await?;

    // 3. SERVICE BOOT: Initialize the GraphQL API and ML scoring services.
    let server = api::graphql::start_server(graph_db, temporal_db).await?;

    info!("Polygraph Backend operational. Fact-checking engine online.");
    server.run().await?;

    Ok(())
}
