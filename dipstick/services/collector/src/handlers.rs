// SPDX-License-Identifier: PMPL-1.0-or-later
// Author: Jonathan D.A. Jewell <6759885+hyperpolymath@users.noreply.github.com>

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use tracing::{error, info};
use uuid::Uuid;

use crate::{db, platforms, AppState};

pub async fn list_platforms(State(state): State<AppState>) -> impl IntoResponse {
    match db::get_active_platforms(&state.db).await {
        Ok(platforms) => (StatusCode::OK, Json(json!({ "platforms": platforms }))),
        Err(e) => {
            error!("Failed to fetch platforms: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to fetch platforms" })),
            )
        }
    }
}

pub async fn get_platform(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match db::get_platform_by_id(&state.db, id).await {
        Ok(Some(platform)) => (StatusCode::OK, Json(json!({ "platform": platform }))),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Platform not found" })),
        ),
        Err(e) => {
            error!("Failed to fetch platform: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to fetch platform" })),
            )
        }
    }
}

pub async fn trigger_collection(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    info!("Manual collection triggered for platform {}", id);

    match db::get_platform_by_id(&state.db, id).await {
        Ok(Some(platform)) => {
            // In a real implementation, this would be queued in Redis
            // For now, we'll run it synchronously
            match platforms::collect_platform_policies(&state, &platform).await {
                Ok(result) => {
                    info!(
                        "Collection completed for {}: {} documents, {} changes",
                        platform.name,
                        result.len(),
                        result.iter().filter(|r| r.change_detected).count()
                    );
                    (
                        StatusCode::OK,
                        Json(json!({
                            "message": "Collection completed",
                            "platform": platform.name,
                            "documents_collected": result.len(),
                            "changes_detected": result.iter().filter(|r| r.change_detected).count(),
                        })),
                    )
                }
                Err(e) => {
                    error!("Collection failed for {}: {}", platform.name, e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({
                            "error": "Collection failed",
                            "details": e.to_string()
                        })),
                    )
                }
            }
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Platform not found" })),
        ),
        Err(e) => {
            error!("Failed to fetch platform: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to fetch platform" })),
            )
        }
    }
}

pub async fn list_changes(State(state): State<AppState>) -> impl IntoResponse {
    match db::get_recent_changes(&state.db, 100).await {
        Ok(changes) => (StatusCode::OK, Json(json!({ "changes": changes }))),
        Err(e) => {
            error!("Failed to fetch changes: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to fetch changes" })),
            )
        }
    }
}

pub async fn get_change(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match db::get_change_by_id(&state.db, id).await {
        Ok(Some(change)) => (StatusCode::OK, Json(json!({ "change": change }))),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Change not found" })),
        ),
        Err(e) => {
            error!("Failed to fetch change: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to fetch change" })),
            )
        }
    }
}
