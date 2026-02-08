use crate::db::{ArangoClient, XtdbClient, CacheClient};
use crate::models::*;
use anyhow::Result;
use async_graphql::Result as GqlResult;
use chrono::Utc;
use uuid::Uuid;
use sha2::{Sha256, Digest};

pub struct ClaimService {
    arango: ArangoClient,
    xtdb: XtdbClient,
    cache: CacheClient,
}

impl ClaimService {
    pub fn new(arango: ArangoClient, xtdb: XtdbClient, cache: CacheClient) -> Self {
        Self { arango, xtdb, cache }
    }

    pub async fn verify_claim(&self, input: ClaimInput) -> GqlResult<VerificationResult> {
        let claim_id = Uuid::new_v4();
        let text_hash = self.compute_hash(&input.text);

        // Create claim
        let claim = Claim {
            id: claim_id,
            text: input.text.clone(),
            url: input.url,
            platform: input.platform,
            author: input.author,
            text_hash,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: ClaimStatus::Processing,
        };

        // TODO: Implement actual verification logic
        // For now, return mock result
        Ok(VerificationResult {
            claim_id,
            verdict: Verdict::Unverifiable,
            confidence: 0.5,
            explanation: "Verification in progress".to_string(),
            sources: vec![],
            entities: vec![],
            sentiment: None,
            credibility_score: 0.5,
            checked_at: Utc::now(),
        })
    }

    pub async fn get_claim(&self, id: &str) -> GqlResult<Claim> {
        // TODO: Implement actual database query
        Err("Not implemented".into())
    }

    pub async fn list_claims(&self, skip: i32, limit: i32) -> GqlResult<Vec<Claim>> {
        // TODO: Implement actual database query
        Ok(vec![])
    }

    fn compute_hash(&self, text: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(text.to_lowercase().as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
