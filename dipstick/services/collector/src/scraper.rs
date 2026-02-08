use anyhow::{anyhow, Result};
use reqwest::Client;
use scraper::{Html, Selector};
use tracing::{info, warn};

use crate::{models::Platform, AppState};

pub async fn fetch_via_scraper(state: &AppState, url: &str) -> Result<String> {
    info!("Fetching via scraper: {}", url);

    let client = Client::builder()
        .user_agent(&state.config.collector.user_agent)
        .build()?;

    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(anyhow!("HTTP error: {}", response.status()));
    }

    let html = response.text().await?;
    let document = Html::parse_document(&html);

    // Extract main content (this is a simplified version)
    // In production, you'd want platform-specific selectors
    let content = extract_main_content(&document)?;

    Ok(content)
}

pub async fn fetch_via_api(
    state: &AppState,
    platform: &Platform,
    url: &str,
) -> Result<String> {
    info!("Fetching via API: {} for platform {}", url, platform.name);

    match platform.name.as_str() {
        "twitter" => fetch_twitter_api(state, url).await,
        "facebook" | "instagram" => fetch_meta_api(state, url).await,
        "linkedin" => fetch_linkedin_api(state, url).await,
        "youtube" => fetch_youtube_api(state, url).await,
        "bluesky" => fetch_bluesky_api(state, url).await,
        _ => {
            warn!("No API implementation for {}, falling back to scraper", platform.name);
            fetch_via_scraper(state, url).await
        }
    }
}

fn extract_main_content(document: &Html) -> Result<String> {
    // Try common content selectors
    let selectors = vec![
        "main",
        "article",
        ".content",
        "#content",
        ".main-content",
        "body",
    ];

    for selector_str in selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            if let Some(element) = document.select(&selector).next() {
                let text = element.text().collect::<Vec<_>>().join("\n");
                if !text.trim().is_empty() && text.len() > 100 {
                    return Ok(text);
                }
            }
        }
    }

    // Fallback: get all text from body
    if let Ok(body_selector) = Selector::parse("body") {
        if let Some(body) = document.select(&body_selector).next() {
            let text = body.text().collect::<Vec<_>>().join("\n");
            if !text.trim().is_empty() {
                return Ok(text);
            }
        }
    }

    Err(anyhow!("Could not extract content from page"))
}

async fn fetch_twitter_api(state: &AppState, url: &str) -> Result<String> {
    // Twitter API implementation would go here
    // For now, fallback to scraper
    warn!("Twitter API not fully implemented, falling back to scraper");
    fetch_via_scraper(state, url).await
}

async fn fetch_meta_api(state: &AppState, url: &str) -> Result<String> {
    // Meta Graph API implementation would go here
    // For now, fallback to scraper
    warn!("Meta API not fully implemented, falling back to scraper");
    fetch_via_scraper(state, url).await
}

async fn fetch_linkedin_api(state: &AppState, url: &str) -> Result<String> {
    // LinkedIn API implementation would go here
    // For now, fallback to scraper
    warn!("LinkedIn API not fully implemented, falling back to scraper");
    fetch_via_scraper(state, url).await
}

async fn fetch_youtube_api(state: &AppState, url: &str) -> Result<String> {
    // YouTube API implementation would go here
    // For now, fallback to scraper
    warn!("YouTube API not fully implemented, falling back to scraper");
    fetch_via_scraper(state, url).await
}

async fn fetch_bluesky_api(state: &AppState, url: &str) -> Result<String> {
    // Bluesky AT Protocol implementation would go here
    // For now, fallback to scraper
    warn!("Bluesky API not fully implemented, falling back to scraper");
    fetch_via_scraper(state, url).await
}
