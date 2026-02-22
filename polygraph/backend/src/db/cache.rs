// SPDX-License-Identifier: PMPL-1.0-or-later

//! Redis Cache Client — Distributed State Persistence.
//!
//! This module implements the caching layer for the Polygraph backend. 
//! It provides high-speed access to ephemeral data (e.g., verification 
//! results, session tokens) to reduce load on the primary graph databases.
//!
//! CONCURRENCY: Uses `redis::aio::ConnectionManager` to provide a 
//! multiplexed, thread-safe asynchronous connection pool.

use anyhow::Result;
use redis::aio::ConnectionManager;
use redis::AsyncCommands;

#[derive(Clone)]
pub struct CacheClient {
    /// ASYNC HANDLE: Manages the active Redis session.
    conn: ConnectionManager,
}

impl CacheClient {
    /// CONNECTIVITY: Establishes an asynchronous link to the Redis/Dragonfly endpoint.
    pub async fn new(url: &str) -> Result<Self> {
        let client = redis::Client::open(url)?;
        let conn = ConnectionManager::new(client).await?;
        Ok(Self { conn })
    }

    /// RETRIEVAL: Attempts to fetch a value by key. 
    /// Returns `Ok(None)` if the key has expired or does not exist.
    pub async fn get(&mut self, key: &str) -> Result<Option<String>> {
        Ok(self.conn.get(key).await?)
    }

    /// PERSISTENCE: Stores a value with a mandatory Time-To-Live (TTL) in seconds.
    pub async fn set(&mut self, key: &str, value: &str, ttl: usize) -> Result<()> {
        self.conn.set_ex(key, value, ttl).await?;
        Ok(())
    }
}
