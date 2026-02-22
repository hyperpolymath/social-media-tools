// SPDX-License-Identifier: PMPL-1.0-or-later

//! Polygraph API — GraphQL Interface Layer.
//!
//! This module implements the external API for the fact-checking engine. 
//! It uses a schema-first approach via `async-graphql` to provide a 
//! typed, queryable interface for clients.

use async_graphql::{Context, Object, Schema, EmptySubscription, Result as GqlResult};
use axum::{extract::State, response::Html, Json};
// ... [other imports]

/// QUERY RESOLVER: Handles read-only requests.
pub struct Query;

#[Object]
impl Query {
    /// HEALTH: Basic connectivity check.
    async fn health(&self) -> &str { "OK" }

    /// CLAIM: Retrieves a specific fact-check record by its UUID.
    async fn claim(&self, ctx: &Context<'_>, id: String) -> GqlResult<Claim> {
        let service = ctx.data::<ClaimService>()?;
        service.get_claim(&id).await
    }
}

/// MUTATION RESOLVER: Handles state-changing operations.
pub struct Mutation;

#[Object]
impl Mutation {
    /// VERIFY: Ingests a new social media claim and initiates the 
    /// neurosymbolic analysis pipeline.
    async fn verify_claim(
        &self,
        ctx: &Context<'_>,
        input: ClaimInput,
    ) -> GqlResult<VerificationResult> {
        let service = ctx.data::<ClaimService>()?;
        service.verify_claim(input).await
    }
}

pub type PolygraphSchema = Schema<Query, Mutation, EmptySubscription>;
