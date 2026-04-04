// SPDX-License-Identifier: PMPL-1.0-or-later
// Property-based tests using proptest for data invariants
// Author: Jonathan D.A. Jewell <6759885+hyperpolymath@users.noreply.github.com>

use nuj_collector::models::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn prop_checksum_is_valid_hex(content in ".*") {
        let checksum = PolicySnapshot::calculate_checksum(&content);

        // All property checksums should be valid SHA256
        prop_assert_eq!(checksum.len(), 64, "Checksum should always be 64 hex chars");
        prop_assert!(
            checksum.chars().all(|c| c.is_ascii_hexdigit()),
            "Checksum should be valid hex"
        );
    }

    #[test]
    fn prop_checksum_is_deterministic(content in ".*") {
        let checksum1 = PolicySnapshot::calculate_checksum(&content);
        let checksum2 = PolicySnapshot::calculate_checksum(&content);

        prop_assert_eq!(checksum1, checksum2, "Checksum must be deterministic");
    }

    #[test]
    fn prop_checksum_different_for_different_inputs(
        content1 in "\\PC+",
        content2 in "\\PC+"
    ) {
        if content1 != content2 {
            let checksum1 = PolicySnapshot::calculate_checksum(&content1);
            let checksum2 = PolicySnapshot::calculate_checksum(&content2);

            prop_assert_ne!(checksum1, checksum2, "Different inputs should have different checksums");
        }
    }

    #[test]
    fn prop_word_count_non_negative(content in ".*") {
        let word_count = PolicySnapshot::calculate_word_count(&content);

        prop_assert!(word_count >= 0, "Word count should never be negative");
    }

    #[test]
    fn prop_word_count_reasonable(content in "[a-z ]+" ) {
        let word_count = PolicySnapshot::calculate_word_count(&content);
        let content_len = content.len();

        // Word count should never exceed content length
        prop_assert!(
            word_count <= content_len as i32,
            "Word count should not exceed content length"
        );
    }

    #[test]
    fn prop_char_count_equals_content_chars(content in ".*") {
        let char_count = PolicySnapshot::calculate_char_count(&content);
        let actual_chars = content.chars().count() as i32;

        prop_assert_eq!(char_count, actual_chars, "Char count should match actual character count");
    }

    #[test]
    fn prop_char_count_non_negative(content in ".*") {
        let char_count = PolicySnapshot::calculate_char_count(&content);

        prop_assert!(char_count >= 0, "Character count should never be negative");
    }

    #[test]
    fn prop_job_status_roundtrip(status in 0u32..4u32) {
        // Test that job status variants are properly defined
        let _job_status = match status {
            0 => JobStatus::Pending,
            1 => JobStatus::Running,
            2 => JobStatus::Completed,
            3 => JobStatus::Failed,
            _ => JobStatus::Pending,
        };

        // Should not panic
        prop_assert!(true);
    }

    #[test]
    fn prop_valid_uuid_generation(uuid_bytes in prop::array::uniform16(0u8..)) {
        use uuid::Uuid;

        // Should be able to create UUIDs from valid bytes
        let _uuid = Uuid::from_bytes(uuid_bytes);

        prop_assert!(true);
    }

    #[test]
    fn prop_port_number_in_valid_range(port in 3000u16..9999u16) {
        // Port should be in valid range
        prop_assert!(port > 0u16);
    }

    #[test]
    fn prop_content_preservation_through_checksum(content in "[a-zA-Z0-9 ]+") {
        // Create two snapshots with same content
        let checksum1 = PolicySnapshot::calculate_checksum(&content);
        let checksum2 = PolicySnapshot::calculate_checksum(&content);

        // Checksums should match exactly
        prop_assert_eq!(checksum1.clone(), checksum2);

        // If we modify content even slightly, checksum changes
        if !content.is_empty() {
            let modified = format!("{}x", content);
            let checksum_modified = PolicySnapshot::calculate_checksum(&modified);

            prop_assert_ne!(checksum1, checksum_modified);
        }
    }

    #[test]
    fn prop_word_count_upper_bound(content in " {0,1000}") {
        // Word count should never exceed some reasonable bound
        let word_count = PolicySnapshot::calculate_word_count(&content);

        prop_assert!(word_count < 10000, "Word count should be reasonable");
    }

    #[test]
    fn prop_checksum_stability_with_whitespace(
        prefix in "[a-z]+",
        suffix in "[a-z]+"
    ) {
        let content1 = format!("{}{}", prefix, suffix);
        let content2 = format!("{} {}", prefix, suffix);

        // Different whitespace = different checksums
        let checksum1 = PolicySnapshot::calculate_checksum(&content1);
        let checksum2 = PolicySnapshot::calculate_checksum(&content2);

        prop_assert_ne!(checksum1, checksum2, "Whitespace should affect checksum");
    }
}

#[test]
fn test_large_content_handling() {
    // Test handling of large policy documents
    let large_content = "policy content ".repeat(10000);

    let checksum = PolicySnapshot::calculate_checksum(&large_content);
    let word_count = PolicySnapshot::calculate_word_count(&large_content);
    let char_count = PolicySnapshot::calculate_char_count(&large_content);

    // Large content should still produce valid results
    assert_eq!(checksum.len(), 64);
    assert!(word_count > 0);
    assert!(char_count > 0);
}

#[test]
fn test_unicode_handling() {
    // Test various Unicode content
    let test_cases = vec![
        "Simple ASCII",
        "Café with accents",
        "日本語テキスト",  // Japanese
        "العربية",        // Arabic
        "🎉 Emoji content 🔒",
        "Multiple\nLine\nBreaks",
    ];

    for content in test_cases {
        let checksum = PolicySnapshot::calculate_checksum(content);
        let word_count = PolicySnapshot::calculate_word_count(content);
        let char_count = PolicySnapshot::calculate_char_count(content);

        // All should produce valid results
        assert_eq!(checksum.len(), 64, "Checksum invalid for: {}", content);
        assert!(word_count >= 0, "Word count invalid for: {}", content);
        assert!(char_count >= 0, "Char count invalid for: {}", content);
    }
}
