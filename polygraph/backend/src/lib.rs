// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
#![forbid(unsafe_code)]
pub mod api;
pub mod db;
pub mod ml;
pub mod models;
pub mod services;

// Re-exports for library usage
pub use models::*;
pub use services::*;
