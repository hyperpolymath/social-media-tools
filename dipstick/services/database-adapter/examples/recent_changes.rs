// SPDX-License-Identifier: MPL-2.0
// Example usage, against the default local Virtuoso:
//   cargo run --example recent_changes

use nuj_virtuoso_adapter::{VirtuosoAdapter, VirtuosoConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let adapter = VirtuosoAdapter::new(VirtuosoConfig::default());

    // Query recent changes
    let changes = adapter
        .query_recent_changes(7, Some("critical"), None)
        .await?;
    println!("Found {} critical changes in last 7 days", changes.len());

    // Semantic search
    let results = adapter
        .semantic_search(&["journalist", "harassment", "content removal"])
        .await?;
    println!("Found {} changes matching keywords", results.len());

    // Platform statistics
    let stats = adapter.query_platform_statistics("twitter").await?;
    println!("Twitter stats: {stats}");

    Ok(())
}
