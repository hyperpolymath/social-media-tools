// SPDX-License-Identifier: PMPL-1.0-or-later
// Benchmarks for collector performance-critical operations
// Author: Jonathan D.A. Jewell <6759885+hyperpolymath@users.noreply.github.com>

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nuj_collector::models::PolicySnapshot;

fn bench_checksum_calculation(c: &mut Criterion) {
    c.bench_function("checksum_small_content", |b| {
        b.iter(|| {
            let content = black_box("Small policy content");
            PolicySnapshot::calculate_checksum(content)
        })
    });

    c.bench_function("checksum_large_content", |b| {
        let large_content = black_box("policy content ".repeat(1000));
        b.iter(|| PolicySnapshot::calculate_checksum(&large_content))
    });

    c.bench_function("checksum_very_large_content", |b| {
        let very_large_content = black_box("policy content ".repeat(10000));
        b.iter(|| PolicySnapshot::calculate_checksum(&very_large_content))
    });
}

fn bench_word_count(c: &mut Criterion) {
    c.bench_function("word_count_small", |b| {
        b.iter(|| {
            let content = black_box("This is a short policy document");
            PolicySnapshot::calculate_word_count(content)
        })
    });

    c.bench_function("word_count_large", |b| {
        let content = black_box("policy content ".repeat(1000));
        b.iter(|| PolicySnapshot::calculate_word_count(&content))
    });
}

fn bench_char_count(c: &mut Criterion) {
    c.bench_function("char_count_small", |b| {
        b.iter(|| {
            let content = black_box("Short content");
            PolicySnapshot::calculate_char_count(content)
        })
    });

    c.bench_function("char_count_large", |b| {
        let content = black_box("x".repeat(100000));
        b.iter(|| PolicySnapshot::calculate_char_count(&content))
    });
}

fn bench_serialization(c: &mut Criterion) {
    use nuj_collector::models::Platform;

    c.bench_function("serialize_platform", |b| {
        let now = chrono::Utc::now();
        let platform = black_box(Platform {
            id: uuid::Uuid::new_v4(),
            name: "test".to_string(),
            display_name: "Test Platform".to_string(),
            url: "https://test.com".to_string(),
            api_endpoint: Some("https://api.test.com".to_string()),
            api_enabled: true,
            scraping_enabled: false,
            monitoring_active: true,
            check_frequency_minutes: 15,
            policy_urls: serde_json::json!(["https://test.com/policy"]),
            terms_urls: serde_json::json!(["https://test.com/terms"]),
            community_guidelines_urls: serde_json::json!(["https://test.com/guidelines"]),
            metadata: serde_json::json!({}),
            created_at: now,
            updated_at: now,
        });

        b.iter(|| serde_json::to_string(&platform).unwrap())
    });
}

fn bench_deserialization(c: &mut Criterion) {
    use nuj_collector::models::Platform;

    let now = chrono::Utc::now();
    let json = serde_json::to_string(&Platform {
        id: uuid::Uuid::new_v4(),
        name: "test".to_string(),
        display_name: "Test Platform".to_string(),
        url: "https://test.com".to_string(),
        api_endpoint: Some("https://api.test.com".to_string()),
        api_enabled: true,
        scraping_enabled: false,
        monitoring_active: true,
        check_frequency_minutes: 15,
        policy_urls: serde_json::json!(["https://test.com/policy"]),
        terms_urls: serde_json::json!(["https://test.com/terms"]),
        community_guidelines_urls: serde_json::json!(["https://test.com/guidelines"]),
        metadata: serde_json::json!({}),
        created_at: now,
        updated_at: now,
    }).unwrap();

    c.bench_function("deserialize_platform", |b| {
        let json_ref = black_box(&json);
        b.iter(|| serde_json::from_str::<Platform>(json_ref).unwrap())
    });
}

criterion_group!(
    benches,
    bench_checksum_calculation,
    bench_word_count,
    bench_char_count,
    bench_serialization,
    bench_deserialization
);

criterion_main!(benches);
