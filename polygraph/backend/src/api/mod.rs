// SPDX-License-Identifier: PMPL-1.0-or-later

//! Polygraph API — Interface Definitions.
//!
//! This module acts as the public gatekeeper for the backend API. 
//! It exposes the GraphQL handlers and schema definitions used by 
//! the Axum web server.

pub mod graphql;

// RE-EXPORTS: Canonical types for server orchestration.
pub use graphql::{create_schema, graphql_handler, graphql_playground, PolygraphSchema};
