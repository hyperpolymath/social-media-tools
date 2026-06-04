// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! Polygraph API — Interface Definitions.
//!
//! This module acts as the public gatekeeper for the backend API. 
//! It exposes the GraphQL handlers and schema definitions used by 
//! the Axum web server.

pub mod graphql;

// RE-EXPORTS: Canonical types for server orchestration.
pub use graphql::{create_schema, graphql_handler, graphql_playground, PolygraphSchema};
