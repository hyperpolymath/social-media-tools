use anyhow::Result;
use tracing::{info, warn};

use crate::{db, models::{CollectionResult, Platform, PolicySnapshot}, scraper, AppState};

pub async fn collect_platform_policies(
    state: &AppState,
    platform: &Platform,
) -> Result<Vec<CollectionResult>> {
    info!("Starting collection for platform: {}", platform.name);

    let mut results = Vec::new();

    // Get all policy URLs for this platform
    let policy_urls = extract_urls(&platform.policy_urls)?;
    let terms_urls = extract_urls(&platform.terms_urls)?;
    let community_urls = extract_urls(&platform.community_guidelines_urls)?;

    // Collect policy documents
    for url in policy_urls {
        if let Ok(result) = collect_document(state, platform, &url, "policy").await {
            results.push(result);
        }
    }

    // Collect terms documents
    for url in terms_urls {
        if let Ok(result) = collect_document(state, platform, &url, "terms").await {
            results.push(result);
        }
    }

    // Collect community guidelines
    for url in community_urls {
        if let Ok(result) = collect_document(state, platform, &url, "community_guidelines").await {
            results.push(result);
        }
    }

    // Update last checked timestamp
    db::update_platform_last_checked(&state.db, platform.id).await?;

    info!("Collection completed for {}: {} documents", platform.name, results.len());
    Ok(results)
}

async fn collect_document(
    state: &AppState,
    platform: &Platform,
    url: &str,
    document_type: &str,
) -> Result<CollectionResult> {
    info!("Collecting {} from {}", document_type, url);

    // Fetch content using scraper
    let content = if platform.api_enabled {
        scraper::fetch_via_api(state, platform, url).await?
    } else {
        scraper::fetch_via_scraper(state, url).await?
    };

    // Calculate checksum
    let checksum = PolicySnapshot::calculate_checksum(&content);

    // Create or update policy document record
    let doc = db::create_or_update_policy_document(
        &state.db,
        platform.id,
        document_type,
        url,
        None,
    )
    .await?;

    // Get previous snapshot to check for changes
    let previous_snapshot = db::get_latest_snapshot(&state.db, doc.id).await?;
    let previous_snapshot_id = previous_snapshot.as_ref().map(|s| s.id);
    let previous_checksum = previous_snapshot.as_ref().map(|s| s.checksum.clone());
    let change_detected = previous_checksum.as_ref() != Some(&checksum);

    // Create new snapshot
    let snapshot = db::create_policy_snapshot(
        &state.db,
        doc.id,
        &content,
        None,
        &checksum,
        if platform.api_enabled { "api" } else { "scraper" },
        previous_snapshot_id,
    )
    .await?;

    // If change detected, create policy change record
    if change_detected {
        info!("Change detected in {} for {}", document_type, platform.name);

        db::create_policy_change(
            &state.db,
            doc.id,
            previous_snapshot_id,
            Some(snapshot.id),
            "modification",
            Some(&format!("Content changed from checksum {} to {}",
                previous_checksum.as_deref().unwrap_or("none"),
                checksum
            )),
        )
        .await?;
    } else {
        info!("No change detected in {} for {}", document_type, platform.name);
    }

    Ok(CollectionResult {
        platform_id: platform.id,
        document_id: doc.id,
        snapshot_id: snapshot.id,
        content,
        checksum,
        change_detected,
        previous_checksum,
    })
}

fn extract_urls(json_value: &serde_json::Value) -> Result<Vec<String>> {
    match json_value.as_array() {
        Some(arr) => Ok(arr
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect()),
        None => Ok(vec![]),
    }
}
