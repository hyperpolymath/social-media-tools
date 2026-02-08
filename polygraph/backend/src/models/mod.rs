use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Claim {
    pub id: Uuid,
    pub text: String,
    pub url: Option<String>,
    pub platform: Option<String>,
    pub author: Option<String>,
    pub text_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: ClaimStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::Enum, Copy, PartialEq, Eq)]
pub enum ClaimStatus {
    Pending,
    Processing,
    Verified,
    Failed,
}

#[derive(Debug, Clone, InputObject)]
pub struct ClaimInput {
    pub text: String,
    pub url: Option<String>,
    pub platform: Option<String>,
    pub author: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct VerificationResult {
    pub claim_id: Uuid,
    pub verdict: Verdict,
    pub confidence: f64,
    pub explanation: String,
    pub sources: Vec<FactCheckSource>,
    pub entities: Vec<String>,
    pub sentiment: Option<SentimentAnalysis>,
    pub credibility_score: f64,
    pub checked_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::Enum, Copy, PartialEq, Eq)]
pub enum Verdict {
    True,
    MostlyTrue,
    Mixed,
    MostlyFalse,
    False,
    Unverifiable,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct FactCheckSource {
    pub name: String,
    pub verdict: Verdict,
    pub rating: f64,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct SentimentAnalysis {
    pub polarity: f64,
    pub subjectivity: f64,
    pub classification: String,
}
