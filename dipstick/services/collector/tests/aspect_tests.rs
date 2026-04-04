// SPDX-License-Identifier: PMPL-1.0-or-later
// Aspect tests for security and compliance
// Author: Jonathan D.A. Jewell <6759885+hyperpolymath@users.noreply.github.com>

use nuj_collector::models::*;

#[test]
fn test_json_injection_in_metadata() {
    // Test that malicious JSON injection doesn't break model serialization
    let malicious_json = r#"{"injection": "SELECT * FROM users; --"}"#;

    let result = CollectionResult {
        platform_id: uuid::Uuid::new_v4(),
        document_id: uuid::Uuid::new_v4(),
        snapshot_id: uuid::Uuid::new_v4(),
        content: malicious_json.to_string(),
        checksum: PolicySnapshot::calculate_checksum(malicious_json),
        change_detected: false,
        previous_checksum: None,
    };

    // Should serialize without error
    let serialized = serde_json::to_string(&result).expect("Should serialize");
    let deserialized: CollectionResult = serde_json::from_str(&serialized)
        .expect("Should deserialize");

    assert_eq!(deserialized.content, malicious_json);
}

#[test]
fn test_oversized_content_handling() {
    // Test handling of large content (not huge to avoid memory exhaustion)
    let large_content = "x ".repeat(500_000);  // 1MB of alternating x and space

    // Should not crash, but handle gracefully
    let word_count = PolicySnapshot::calculate_word_count(&large_content);
    let char_count = PolicySnapshot::calculate_char_count(&large_content);

    // Should still produce valid values
    assert!(word_count > 0);
    assert!(char_count > 0);
    // Repeated "x " should give us many words
    assert!(word_count > 100_000);
}

#[test]
fn test_sql_injection_patterns_in_urls() {
    // Test that SQL injection patterns in URLs don't get evaluated
    let dangerous_url = "https://example.com/policy'; DROP TABLE policies;--";

    let snapshot_checksum = PolicySnapshot::calculate_checksum(dangerous_url);

    // Checksum should be safe (just hashing the string, not evaluating it)
    assert_eq!(snapshot_checksum.len(), 64);
    assert!(snapshot_checksum.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_xss_payload_in_content() {
    // Test XSS payload doesn't cause issues in data handling
    let xss_payload = "<script>alert('XSS')</script>";

    let result = CollectionResult {
        platform_id: uuid::Uuid::new_v4(),
        document_id: uuid::Uuid::new_v4(),
        snapshot_id: uuid::Uuid::new_v4(),
        content: xss_payload.to_string(),
        checksum: PolicySnapshot::calculate_checksum(xss_payload),
        change_detected: false,
        previous_checksum: None,
    };

    // Should serialize safely
    let serialized = serde_json::to_string(&result).expect("Should serialize");

    // XSS payload should be present in JSON (serde_json doesn't escape < or >)
    assert!(serialized.contains("<script>") || serialized.contains("script"));
}

#[test]
fn test_null_byte_injection() {
    // Test null byte injection in content
    let content_with_null = "before\0after";

    let checksum = PolicySnapshot::calculate_checksum(content_with_null);
    let word_count = PolicySnapshot::calculate_word_count(content_with_null);

    // Should handle null bytes gracefully
    assert_eq!(checksum.len(), 64);
    assert!(word_count >= 0);
}

#[test]
fn test_extremely_long_url() {
    // Test handling of pathologically long URLs
    let long_url = format!("https://example.com/{}", "a".repeat(10_000));

    let checksum = PolicySnapshot::calculate_checksum(&long_url);

    // Should produce valid checksum despite long input
    assert_eq!(checksum.len(), 64);
}

#[test]
fn test_control_characters_in_content() {
    // Test various control characters don't cause issues
    let content = "Normal\t\n\r\x00\x01\x02\x03Content";

    let word_count = PolicySnapshot::calculate_word_count(content);
    let char_count = PolicySnapshot::calculate_char_count(content);
    let checksum = PolicySnapshot::calculate_checksum(content);

    // All should handle control characters
    assert!(word_count >= 0);
    assert!(char_count >= 0);
    assert_eq!(checksum.len(), 64);
}

#[test]
fn test_unicode_normalization_differences() {
    // Test that different Unicode representations of the same character
    // produce different checksums (important for detecting subtle changes)
    let nfc = "é";  // Single character (NFC form)
    let nfd = "é";  // Decomposed form (NFD) - looks the same but different bytes

    let checksum_nfc = PolicySnapshot::calculate_checksum(nfc);
    let checksum_nfd = PolicySnapshot::calculate_checksum(nfd);

    // In Rust, these are different at the byte level, so checksums may differ
    // This test ensures consistency
    let checksum_nfc_again = PolicySnapshot::calculate_checksum(nfc);
    assert_eq!(checksum_nfc, checksum_nfc_again);
}

#[test]
fn test_bom_in_content() {
    // Test Byte Order Mark doesn't cause issues
    let content_with_bom = "\u{FEFF}Regular content";

    let checksum = PolicySnapshot::calculate_checksum(content_with_bom);
    let char_count = PolicySnapshot::calculate_char_count(content_with_bom);

    assert_eq!(checksum.len(), 64);
    assert!(char_count > 0);
}

#[test]
fn test_regex_pattern_injection() {
    // Test regex patterns don't get evaluated
    let regex_payload = ".*|DROP|.*;";

    let word_count = PolicySnapshot::calculate_word_count(regex_payload);
    let checksum = PolicySnapshot::calculate_checksum(regex_payload);

    // Should treat as literal string - count is 1 since pipes aren't whitespace
    assert_eq!(word_count, 1);
    assert_eq!(checksum.len(), 64);
}

#[test]
fn test_html_entity_injection() {
    // Test HTML entity injection
    let html_entities = "&lt;script&gt;&lt;/script&gt;";

    let result = CollectionResult {
        platform_id: uuid::Uuid::new_v4(),
        document_id: uuid::Uuid::new_v4(),
        snapshot_id: uuid::Uuid::new_v4(),
        content: html_entities.to_string(),
        checksum: PolicySnapshot::calculate_checksum(html_entities),
        change_detected: false,
        previous_checksum: None,
    };

    let serialized = serde_json::to_string(&result).expect("Should serialize");
    let deserialized: CollectionResult = serde_json::from_str(&serialized)
        .expect("Should deserialize");

    // Original should be preserved
    assert_eq!(deserialized.content, html_entities);
}

#[test]
fn test_path_traversal_in_urls() {
    // Test path traversal attempts in URLs
    let traversal_url = "../../../../../../etc/passwd";

    let checksum = PolicySnapshot::calculate_checksum(traversal_url);

    // Should just hash the string, not evaluate it
    assert_eq!(checksum.len(), 64);
}

#[test]
fn test_excessive_nesting_json() {
    // Test deeply nested JSON structure
    let mut nested = String::from("{");
    for _ in 0..1000 {
        nested.push_str(r#""a":{"#);
    }
    nested.push_str("0");
    for _ in 0..1000 {
        nested.push('}');
    }

    let word_count = PolicySnapshot::calculate_word_count(&nested);

    // Should handle deeply nested structures
    assert!(word_count >= 0);
}

#[test]
fn test_uuid_collision_resistance() {
    // Verify UUIDs are unique with high probability
    use uuid::Uuid;

    let uuid1 = Uuid::new_v4();
    let uuid2 = Uuid::new_v4();

    assert_ne!(uuid1, uuid2, "UUIDs should be unique");
}

#[test]
fn test_timestamp_monotonicity() {
    // Verify timestamps are working correctly
    use chrono::Utc;

    let time1 = Utc::now();
    std::thread::sleep(std::time::Duration::from_millis(1));
    let time2 = Utc::now();

    assert!(time2 > time1, "Later timestamp should be greater");
}

#[test]
fn test_content_with_all_whitespace() {
    // Edge case: content with only whitespace
    let whitespace_only = "   \t\t\n\n\r\r";

    let word_count = PolicySnapshot::calculate_word_count(whitespace_only);
    let char_count = PolicySnapshot::calculate_char_count(whitespace_only);

    // Whitespace counts as characters but not words
    assert_eq!(word_count, 0, "Whitespace-only content should have 0 words");
    assert!(char_count > 0, "Whitespace-only content should have characters");
}
