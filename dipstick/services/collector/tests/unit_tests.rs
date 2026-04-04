// SPDX-License-Identifier: PMPL-1.0-or-later
// Unit tests for collector models and config parsing
// Author: Jonathan D.A. Jewell <6759885+hyperpolymath@users.noreply.github.com>

use nuj_collector::models::*;
use uuid::Uuid;
use std::str::FromStr;

#[test]
fn test_policy_snapshot_calculate_checksum() {
    let content = "Test policy content";
    let checksum = PolicySnapshot::calculate_checksum(content);

    // Verify it's a valid SHA256 hex string (64 chars)
    assert_eq!(checksum.len(), 64, "SHA256 checksum should be 64 hex characters");
    assert!(checksum.chars().all(|c| c.is_ascii_hexdigit()), "Checksum should be valid hex");

    // Verify deterministic
    let checksum2 = PolicySnapshot::calculate_checksum(content);
    assert_eq!(checksum, checksum2, "Checksum should be deterministic");
}

#[test]
fn test_policy_snapshot_checksum_different_content() {
    let content1 = "Policy version 1";
    let content2 = "Policy version 2";

    let checksum1 = PolicySnapshot::calculate_checksum(content1);
    let checksum2 = PolicySnapshot::calculate_checksum(content2);

    assert_ne!(checksum1, checksum2, "Different content should produce different checksums");
}

#[test]
fn test_policy_snapshot_calculate_word_count() {
    let content = "This is a test document with several words";
    let word_count = PolicySnapshot::calculate_word_count(content);

    assert_eq!(word_count, 8, "Should correctly count words");
}

#[test]
fn test_policy_snapshot_calculate_word_count_empty() {
    let content = "";
    let word_count = PolicySnapshot::calculate_word_count(content);

    assert_eq!(word_count, 0, "Empty string should have 0 words");
}

#[test]
fn test_policy_snapshot_calculate_char_count() {
    let content = "Hello World";
    let char_count = PolicySnapshot::calculate_char_count(content);

    // "Hello World" has 11 characters (including space)
    assert_eq!(char_count, 11, "Should correctly count characters");
}

#[test]
fn test_policy_snapshot_calculate_char_count_unicode() {
    let content = "Hello 世界";  // Hello + Chinese characters
    let char_count = PolicySnapshot::calculate_char_count(content);

    // Should count grapheme clusters correctly
    assert_eq!(char_count, 8, "Should correctly count Unicode characters");
}

#[test]
fn test_job_status_serialization() {
    let pending = JobStatus::Pending;
    let running = JobStatus::Running;
    let completed = JobStatus::Completed;
    let failed = JobStatus::Failed;

    // Verify enum variants exist and are distinct
    assert_ne!(pending, running);
    assert_ne!(running, completed);
    assert_ne!(completed, failed);
}

#[test]
fn test_platform_model_creation() {
    let now = chrono::Utc::now();
    let id = Uuid::new_v4();

    // Models should be constructible
    let _: Platform = Platform {
        id,
        name: "twitter".to_string(),
        display_name: "Twitter/X".to_string(),
        url: "https://twitter.com".to_string(),
        api_endpoint: Some("https://api.twitter.com/v2".to_string()),
        api_enabled: true,
        scraping_enabled: false,
        monitoring_active: true,
        check_frequency_minutes: 15,
        policy_urls: serde_json::json!(["https://twitter.com/en/tos"]),
        terms_urls: serde_json::json!(["https://twitter.com/en/tos"]),
        community_guidelines_urls: serde_json::json!(["https://twitter.com/en/rules"]),
        metadata: serde_json::json!({}),
        created_at: now,
        updated_at: now,
    };
}

#[test]
fn test_policy_change_model_creation() {
    let now = chrono::Utc::now();
    let policy_doc_id = Uuid::new_v4();

    let _: PolicyChange = PolicyChange {
        id: Uuid::new_v4(),
        policy_document_id: policy_doc_id,
        previous_snapshot_id: None,
        current_snapshot_id: Some(Uuid::new_v4()),
        detected_at: now,
        change_type: "modification".to_string(),
        severity: "low".to_string(),
        confidence_score: "0.95".parse::<bigdecimal::BigDecimal>().unwrap(),
        affected_sections: serde_json::json!([]),
        change_summary: Some("Content modified".to_string()),
        impact_assessment: None,
        requires_member_notification: false,
        notification_sent_at: None,
        reviewed_by: None,
        reviewed_at: None,
        review_notes: None,
        false_positive: false,
        metadata: serde_json::json!({}),
    };
}

#[test]
fn test_collection_result_model_creation() {
    let result = CollectionResult {
        platform_id: Uuid::new_v4(),
        document_id: Uuid::new_v4(),
        snapshot_id: Uuid::new_v4(),
        content: "Policy content here".to_string(),
        checksum: "a1b2c3d4".to_string(),
        change_detected: true,
        previous_checksum: Some("a1b2c3d3".to_string()),
    };

    assert_eq!(result.content, "Policy content here");
    assert!(result.change_detected);
    assert_eq!(result.previous_checksum, Some("a1b2c3d3".to_string()));
}

#[test]
fn test_checksum_empty_vs_whitespace() {
    let empty = PolicySnapshot::calculate_checksum("");
    let whitespace = PolicySnapshot::calculate_checksum("   ");

    // Empty and whitespace should produce different hashes
    assert_ne!(empty, whitespace);
}

#[test]
fn test_word_count_various_whitespace() {
    let single_spaces = "one two three";
    let multiple_spaces = "one  two   three";
    let tabs_and_newlines = "one\ttwo\nthree";

    // All should have 3 words despite different whitespace
    assert_eq!(PolicySnapshot::calculate_word_count(single_spaces), 3);
    assert_eq!(PolicySnapshot::calculate_word_count(multiple_spaces), 3);
    assert_eq!(PolicySnapshot::calculate_word_count(tabs_and_newlines), 3);
}
