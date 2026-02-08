// Real fact-checking API integrations
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactCheckResult {
    pub source: String,
    pub verdict: String,
    pub rating: f64,
    pub url: Option<String>,
    pub explanation: Option<String>,
}

pub struct FactCheckClient {
    client: Client,
    google_api_key: Option<String>,
    newsapi_key: Option<String>,
}

impl FactCheckClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            google_api_key: env::var("GOOGLE_FACTCHECK_API_KEY").ok(),
            newsapi_key: env::var("NEWSAPI_KEY").ok(),
        }
    }

    /// Check claim with Google Fact Check Tools API
    /// https://toolbox.google.com/factcheck/apis
    pub async fn google_fact_check(&self, claim: &str) -> Result<Vec<FactCheckResult>> {
        let api_key = self.google_api_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Google Fact Check API key not configured"))?;

        let url = format!(
            "https://factchecktools.googleapis.com/v1alpha1/claims:search?query={}&key={}",
            urlencoding::encode(claim),
            api_key
        );

        let response: GoogleFactCheckResponse = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        let results = response.claims.unwrap_or_default()
            .into_iter()
            .filter_map(|claim| {
                claim.claim_review.first().map(|review| {
                    FactCheckResult {
                        source: review.publisher.site.clone(),
                        verdict: review.textual_rating.clone(),
                        rating: normalize_rating(&review.textual_rating),
                        url: Some(review.url.clone()),
                        explanation: review.title.clone(),
                    }
                })
            })
            .collect();

        Ok(results)
    }

    /// Check claim with NewsAPI (for source credibility)
    /// https://newsapi.org/docs
    pub async fn check_news_sources(&self, query: &str) -> Result<Vec<String>> {
        let api_key = self.newsapi_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("NewsAPI key not configured"))?;

        let url = format!(
            "https://newsapi.org/v2/everything?q={}&apiKey={}",
            urlencoding::encode(query),
            api_key
        );

        let response: NewsApiResponse = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        let sources: Vec<String> = response.articles
            .into_iter()
            .map(|article| article.source.name)
            .collect();

        Ok(sources)
    }

    /// Aggregate results from all fact-checking sources
    pub async fn aggregate_fact_checks(&self, claim: &str) -> Result<Vec<FactCheckResult>> {
        let mut results = Vec::new();

        // Google Fact Check
        if let Ok(google_results) = self.google_fact_check(claim).await {
            results.extend(google_results);
        }

        // Add more fact-checking sources here as needed
        // - Snopes (if API available)
        // - PolitiFact (if API available)
        // - FactCheck.org (if API available)

        Ok(results)
    }
}

// Google Fact Check API response types
#[derive(Debug, Deserialize)]
struct GoogleFactCheckResponse {
    claims: Option<Vec<GoogleClaim>>,
}

#[derive(Debug, Deserialize)]
struct GoogleClaim {
    #[serde(rename = "claimReview")]
    claim_review: Vec<GoogleClaimReview>,
}

#[derive(Debug, Deserialize)]
struct GoogleClaimReview {
    publisher: GooglePublisher,
    url: String,
    title: Option<String>,
    #[serde(rename = "textualRating")]
    textual_rating: String,
}

#[derive(Debug, Deserialize)]
struct GooglePublisher {
    site: String,
}

// NewsAPI response types
#[derive(Debug, Deserialize)]
struct NewsApiResponse {
    articles: Vec<NewsArticle>,
}

#[derive(Debug, Deserialize)]
struct NewsArticle {
    source: NewsSource,
}

#[derive(Debug, Deserialize)]
struct NewsSource {
    name: String,
}

/// Normalize textual ratings to 0-1 scale
fn normalize_rating(rating: &str) -> f64 {
    let rating_lower = rating.to_lowercase();

    if rating_lower.contains("true") && !rating_lower.contains("mostly") {
        1.0
    } else if rating_lower.contains("mostly true") {
        0.8
    } else if rating_lower.contains("mixed") || rating_lower.contains("half") {
        0.5
    } else if rating_lower.contains("mostly false") {
        0.2
    } else if rating_lower.contains("false") {
        0.0
    } else {
        0.5 // unverifiable
    }
}
