use async_graphql::{Context, Object, Schema, EmptySubscription, Result as GqlResult};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    Json,
};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};

use crate::db::{ArangoClient, XtdbClient, CacheClient};
use crate::models::{Claim, ClaimInput, VerificationResult};
use crate::services::ClaimService;

pub struct Query;

#[Object]
impl Query {
    async fn health(&self) -> &str {
        "OK"
    }

    async fn claim(&self, ctx: &Context<'_>, id: String) -> GqlResult<Claim> {
        let service = ctx.data::<ClaimService>()?;
        service.get_claim(&id).await
    }

    async fn claims(
        &self,
        ctx: &Context<'_>,
        skip: Option<i32>,
        limit: Option<i32>,
    ) -> GqlResult<Vec<Claim>> {
        let service = ctx.data::<ClaimService>()?;
        service.list_claims(skip.unwrap_or(0), limit.unwrap_or(100)).await
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
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

pub async fn create_schema(
    arango: ArangoClient,
    xtdb: XtdbClient,
    cache: CacheClient,
) -> PolygraphSchema {
    let claim_service = ClaimService::new(arango, xtdb, cache);

    Schema::build(Query, Mutation, EmptySubscription)
        .data(claim_service)
        .finish()
}

pub async fn graphql_handler(
    State(schema): State<PolygraphSchema>,
    req: Json<async_graphql::Request>,
) -> impl IntoResponse {
    let res = schema.execute(req.0).await;
    Json(res)
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
