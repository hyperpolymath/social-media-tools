// SPDX-License-Identifier: PMPL-1.0-or-later
// NUJ Social Media Ethics Monitor - Collector Service Library
// Purpose: Public library interface for collector modules
// Author: Jonathan D.A. Jewell <6759885+hyperpolymath@users.noreply.github.com>

use std::sync::Arc;

pub mod config;
pub mod db;
pub mod handlers;
pub mod models;
pub mod platforms;
pub mod scheduler;
pub mod scraper;

pub use config::Config;
pub use models::{Platform, PolicyChange, PolicyDocument, PolicySnapshot};

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub redis: redis::Client,
    pub config: Arc<Config>,
}
