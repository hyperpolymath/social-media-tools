// SPDX-License-Identifier: PMPL-1.0-or-later

//! ArangoDB Client — Graph Relationship Management.
//!
//! This module provides the interface to the ArangoDB cluster, which 
//! stores the semantic network of social media claims, authors, 
//! and evidence.
//!
//! DATA MODEL:
//! - **Nodes**: Claims, Sources, Entities.
//! - **Edges**: `sourced_from`, `mentions`, `contradicts`.

use anyhow::Result;
use arangors::{Connection, Database};

#[derive(Clone)]
pub struct ArangoClient {
    /// SHARED DATABASE HANDLE: Multiplexed across all backend tasks.
    db: Database<Connection>,
}

impl ArangoClient {
    /// CONNECTIVITY: Establishes a JWT-authenticated link to the ArangoDB endpoint.
    pub async fn new(url: &str, db_name: &str) -> Result<Self> {
        let conn = Connection::establish_jwt(url, "root", "changeme").await?;
        let db = conn.db(db_name).await?;
        Ok(Self { db })
    }

    pub fn database(&self) -> &Database<Connection> {
        &self.db
    }
}
