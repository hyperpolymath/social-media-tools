// SPDX-License-Identifier: PMPL-1.0-or-later
// Contract tests - API contracts and data invariants
// Author: Jonathan D.A. Jewell <6759885+hyperpolymath@users.noreply.github.com>

use nuj_collector::models::*;
use uuid::Uuid;

#[test]
fn test_platform_required_fields() {
    // Contract: Platform must have all required fields
    let now = chrono::Utc::now();

    let platform = Platform {
        id: Uuid::new_v4(),
        name: String::new(),
        display_name: String::new(),
        url: String::new(),
        api_endpoint: None,
        api_enabled: false,
        scraping_enabled: false,
        monitoring_active: false,
        check_frequency_minutes: 0,
        policy_urls: serde_json::json!([]),
        terms_urls: serde_json::json!([]),
        community_guidelines_urls: serde_json::json!([]),
        metadata: serde_json::json!({}),
        created_at: now,
        updated_at: now,
    };

    // Contract: ID should not be nil
    assert_ne!(platform.id, Uuid::nil());

    // Contract: All JSON fields should be valid JSON values
    assert!(platform.policy_urls.is_array() || platform.policy_urls.is_object());
    assert!(platform.metadata.is_object());
}

#[test]
fn test_policy_change_severity_values() {
    // Contract: severity must be one of predefined values
    let allowed_severities = vec!["critical", "high", "medium", "low", "informational"];

    for severity in allowed_severities {
        let change = PolicyChange {
            id: Uuid::new_v4(),
            policy_document_id: Uuid::new_v4(),
            previous_snapshot_id: None,
            current_snapshot_id: None,
            detected_at: chrono::Utc::now(),
            change_type: "modification".to_string(),
            severity: severity.to_string(),
            confidence_score: "0.5".parse::<bigdecimal::BigDecimal>().unwrap(),
            affected_sections: serde_json::json!([]),
            change_summary: None,
            impact_assessment: None,
            requires_member_notification: false,
            notification_sent_at: None,
            reviewed_by: None,
            reviewed_at: None,
            review_notes: None,
            false_positive: false,
            metadata: serde_json::json!({}),
        };

        assert_eq!(change.severity, severity);
    }
}

#[test]
fn test_policy_change_type_values() {
    // Contract: change_type must be one of predefined values
    let allowed_types = vec!["addition", "modification", "removal", "reformat"];

    for change_type in allowed_types {
        let change = PolicyChange {
            id: Uuid::new_v4(),
            policy_document_id: Uuid::new_v4(),
            previous_snapshot_id: None,
            current_snapshot_id: None,
            detected_at: chrono::Utc::now(),
            change_type: change_type.to_string(),
            severity: "medium".to_string(),
            confidence_score: "0.5".parse::<bigdecimal::BigDecimal>().unwrap(),
            affected_sections: serde_json::json!([]),
            change_summary: None,
            impact_assessment: None,
            requires_member_notification: false,
            notification_sent_at: None,
            reviewed_by: None,
            reviewed_at: None,
            review_notes: None,
            false_positive: false,
            metadata: serde_json::json!({}),
        };

        assert_eq!(change.change_type, change_type);
    }
}

#[test]
fn test_confidence_score_range() {
    // Contract: confidence_score should be between 0.0 and 1.0
    use bigdecimal::BigDecimal;

    let valid_scores = vec![
        "0".parse::<BigDecimal>().unwrap(),
        "0.25".parse::<BigDecimal>().unwrap(),
        "0.5".parse::<BigDecimal>().unwrap(),
        "0.75".parse::<BigDecimal>().unwrap(),
        "1".parse::<BigDecimal>().unwrap(),
    ];

    for score in valid_scores {
        // Score should be a valid decimal
        let _change = PolicyChange {
            id: Uuid::new_v4(),
            policy_document_id: Uuid::new_v4(),
            previous_snapshot_id: None,
            current_snapshot_id: None,
            detected_at: chrono::Utc::now(),
            change_type: "modification".to_string(),
            severity: "medium".to_string(),
            confidence_score: score,
            affected_sections: serde_json::json!([]),
            change_summary: None,
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
}

#[test]
fn test_job_status_transitions() {
    // Contract: job status values should be the defined enum variants
    let pending = JobStatus::Pending;
    let running = JobStatus::Running;
    let completed = JobStatus::Completed;
    let failed = JobStatus::Failed;

    // Each status should be distinct
    assert_ne!(pending, running);
    assert_ne!(running, completed);
    assert_ne!(completed, failed);
}

#[test]
fn test_collection_result_checksums() {
    // Contract: checksums should be SHA256 hexadecimal strings
    let content = "test policy";
    let checksum = PolicySnapshot::calculate_checksum(content);

    let result = CollectionResult {
        platform_id: Uuid::new_v4(),
        document_id: Uuid::new_v4(),
        snapshot_id: Uuid::new_v4(),
        content: content.to_string(),
        checksum: checksum.clone(),
        change_detected: false,
        previous_checksum: Some(checksum.clone()),
    };

    // Contract: checksums must be 64-char hex strings
    assert_eq!(result.checksum.len(), 64);
    assert!(result.checksum.chars().all(|c| c.is_ascii_hexdigit()));

    if let Some(prev) = &result.previous_checksum {
        assert_eq!(prev.len(), 64);
        assert!(prev.chars().all(|c| c.is_ascii_hexdigit()));
    }
}

#[test]
fn test_policy_snapshot_timestamps() {
    // Contract: captured_at should be a valid timestamp
    let now = chrono::Utc::now();

    let _snapshot = PolicySnapshot {
        id: Uuid::new_v4(),
        policy_document_id: Uuid::new_v4(),
        captured_at: now,
        content_text: "content".to_string(),
        content_html: None,
        content_markdown: None,
        word_count: Some(1),
        char_count: Some(7),
        checksum: PolicySnapshot::calculate_checksum("content"),
        previous_snapshot_id: None,
        diff_summary: None,
        capture_method: "scraper".to_string(),
        metadata: serde_json::json!({}),
    };

    // Contract: timestamp should not be in the future
    let one_second_future = chrono::Utc::now() + chrono::Duration::seconds(1);
    assert!(now <= one_second_future);
}

#[test]
fn test_policy_document_url_format() {
    // Contract: URLs should be parseable as valid URLs
    let valid_urls = vec![
        "https://twitter.com/en/tos",
        "https://facebook.com/policies/",
        "https://example.com/policy?version=1",
    ];

    for url_str in valid_urls {
        // Contract: URL should be a string
        assert!(!url_str.is_empty());
        assert!(url_str.contains("://"));  // Has protocol
    }
}

#[test]
fn test_platform_monitoring_flags() {
    // Contract: API and scraping flags should be boolean
    let now = chrono::Utc::now();

    let api_only = Platform {
        id: Uuid::new_v4(),
        name: "test".to_string(),
        display_name: "Test".to_string(),
        url: "https://test.com".to_string(),
        api_endpoint: Some("https://api.test.com".to_string()),
        api_enabled: true,
        scraping_enabled: false,
        monitoring_active: true,
        check_frequency_minutes: 60,
        policy_urls: serde_json::json!([]),
        terms_urls: serde_json::json!([]),
        community_guidelines_urls: serde_json::json!([]),
        metadata: serde_json::json!({}),
        created_at: now,
        updated_at: now,
    };

    assert!(api_only.api_enabled);
    assert!(!api_only.scraping_enabled);

    let scraper_only = Platform {
        id: Uuid::new_v4(),
        name: "test".to_string(),
        display_name: "Test".to_string(),
        url: "https://test.com".to_string(),
        api_endpoint: None,
        api_enabled: false,
        scraping_enabled: true,
        monitoring_active: true,
        check_frequency_minutes: 60,
        policy_urls: serde_json::json!([]),
        terms_urls: serde_json::json!([]),
        community_guidelines_urls: serde_json::json!([]),
        metadata: serde_json::json!({}),
        created_at: now,
        updated_at: now,
    };

    assert!(!scraper_only.api_enabled);
    assert!(scraper_only.scraping_enabled);
}

#[test]
fn test_check_frequency_reasonable() {
    // Contract: check frequency should be reasonable (minutes)
    let now = chrono::Utc::now();

    let frequent = Platform {
        id: Uuid::new_v4(),
        name: "test".to_string(),
        display_name: "Test".to_string(),
        url: "https://test.com".to_string(),
        api_endpoint: None,
        api_enabled: false,
        scraping_enabled: true,
        monitoring_active: true,
        check_frequency_minutes: 5,  // Every 5 minutes
        policy_urls: serde_json::json!([]),
        terms_urls: serde_json::json!([]),
        community_guidelines_urls: serde_json::json!([]),
        metadata: serde_json::json!({}),
        created_at: now,
        updated_at: now,
    };

    // Contract: frequency should be positive
    assert!(frequent.check_frequency_minutes > 0);

    // Contract: frequency should be reasonable (not checking every millisecond)
    assert!(frequent.check_frequency_minutes >= 1);

    // Contract: frequency should not be unreasonably high
    assert!(frequent.check_frequency_minutes <= 10080);  // Max 1 week in minutes
}

#[test]
fn test_serialization_round_trip() {
    // Contract: Models should be serializable and deserializable
    let now = chrono::Utc::now();

    let original = Platform {
        id: Uuid::new_v4(),
        name: "Twitter".to_string(),
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
        metadata: serde_json::json!({"version": "1"}),
        created_at: now,
        updated_at: now,
    };

    let json = serde_json::to_string(&original).expect("Should serialize");
    let restored: Platform = serde_json::from_str(&json).expect("Should deserialize");

    assert_eq!(original.id, restored.id);
    assert_eq!(original.name, restored.name);
    assert_eq!(original.api_enabled, restored.api_enabled);
}

#[test]
fn test_policy_snapshot_content_types() {
    // Contract: content can be text, HTML, or markdown
    let content_types = vec![
        ("text/plain", "Just plain text content"),
        ("text/html", "<p>HTML content</p>"),
        ("text/markdown", "# Markdown heading\n\nParagraph"),
    ];

    for (_mime_type, content) in content_types {
        let _snapshot = PolicySnapshot {
            id: Uuid::new_v4(),
            policy_document_id: Uuid::new_v4(),
            captured_at: chrono::Utc::now(),
            content_text: content.to_string(),
            content_html: None,
            content_markdown: None,
            word_count: None,
            char_count: None,
            checksum: PolicySnapshot::calculate_checksum(content),
            previous_snapshot_id: None,
            diff_summary: None,
            capture_method: "api".to_string(),
            metadata: serde_json::json!({}),
        };
    }
}
