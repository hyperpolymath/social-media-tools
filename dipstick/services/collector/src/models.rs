// SPDX-License-Identifier: PMPL-1.0-or-later
// Author: Jonathan D.A. Jewell <6759885+hyperpolymath@users.noreply.github.com>

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Platform {
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
    pub url: String,
    pub api_endpoint: Option<String>,
    pub api_enabled: bool,
    pub scraping_enabled: bool,
    pub monitoring_active: bool,
    pub check_frequency_minutes: i32,
    pub policy_urls: serde_json::Value,
    pub terms_urls: serde_json::Value,
    pub community_guidelines_urls: serde_json::Value,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PolicyDocument {
    pub id: Uuid,
    pub platform_id: Uuid,
    pub document_type: String,
    pub url: String,
    pub title: Option<String>,
    pub language: String,
    pub version: Option<String>,
    pub is_current: bool,
    pub discovered_at: DateTime<Utc>,
    pub first_seen_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub archived_at: Option<DateTime<Utc>>,
    pub checksum: Option<String>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PolicySnapshot {
    pub id: Uuid,
    pub policy_document_id: Uuid,
    pub captured_at: DateTime<Utc>,
    pub content_text: String,
    pub content_html: Option<String>,
    pub content_markdown: Option<String>,
    pub word_count: Option<i32>,
    pub char_count: Option<i32>,
    pub checksum: String,
    pub previous_snapshot_id: Option<Uuid>,
    pub diff_summary: Option<serde_json::Value>,
    pub capture_method: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PolicyChange {
    pub id: Uuid,
    pub policy_document_id: Uuid,
    pub previous_snapshot_id: Option<Uuid>,
    pub current_snapshot_id: Option<Uuid>,
    pub detected_at: DateTime<Utc>,
    pub change_type: String,
    pub severity: String,
    pub confidence_score: BigDecimal,
    pub affected_sections: serde_json::Value,
    pub change_summary: Option<String>,
    pub impact_assessment: Option<String>,
    pub requires_member_notification: bool,
    pub notification_sent_at: Option<DateTime<Utc>>,
    pub reviewed_by: Option<String>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub review_notes: Option<String>,
    pub false_positive: bool,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionJob {
    pub platform_id: Uuid,
    pub platform_name: String,
    pub scheduled_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: JobStatus,
    pub documents_collected: u32,
    pub changes_detected: u32,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionResult {
    pub platform_id: Uuid,
    pub document_id: Uuid,
    pub snapshot_id: Uuid,
    pub content: String,
    pub checksum: String,
    pub change_detected: bool,
    pub previous_checksum: Option<String>,
}

impl PolicySnapshot {
    pub fn calculate_checksum(content: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn calculate_word_count(content: &str) -> i32 {
        content.split_whitespace().count() as i32
    }

    pub fn calculate_char_count(content: &str) -> i32 {
        content.chars().count() as i32
    }
}
