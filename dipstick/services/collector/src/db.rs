use anyhow::Result;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{Platform, PolicyDocument, PolicySnapshot, PolicyChange};

pub async fn get_active_platforms(pool: &PgPool) -> Result<Vec<Platform>> {
    let platforms = sqlx::query_as::<_, Platform>(
        r#"
        SELECT * FROM platforms
        WHERE monitoring_active = true
        ORDER BY name
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(platforms)
}

pub async fn get_platform_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Platform>> {
    let platform = sqlx::query_as::<_, Platform>(
        r#"
        SELECT * FROM platforms
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(platform)
}

pub async fn get_policy_documents_for_platform(
    pool: &PgPool,
    platform_id: Uuid,
) -> Result<Vec<PolicyDocument>> {
    let documents = sqlx::query_as::<_, PolicyDocument>(
        r#"
        SELECT * FROM policy_documents
        WHERE platform_id = $1 AND is_current = true
        ORDER BY document_type
        "#,
    )
    .bind(platform_id)
    .fetch_all(pool)
    .await?;

    Ok(documents)
}

pub async fn create_or_update_policy_document(
    pool: &PgPool,
    platform_id: Uuid,
    document_type: &str,
    url: &str,
    title: Option<&str>,
) -> Result<PolicyDocument> {
    let document = sqlx::query_as::<_, PolicyDocument>(
        r#"
        INSERT INTO policy_documents (platform_id, document_type, url, title, is_current)
        VALUES ($1, $2, $3, $4, true)
        ON CONFLICT (platform_id, url) DO UPDATE
        SET last_seen_at = NOW(), is_current = true, title = COALESCE($4, policy_documents.title)
        RETURNING *
        "#,
    )
    .bind(platform_id)
    .bind(document_type)
    .bind(url)
    .bind(title)
    .fetch_one(pool)
    .await?;

    Ok(document)
}

pub async fn create_policy_snapshot(
    pool: &PgPool,
    policy_document_id: Uuid,
    content_text: &str,
    content_html: Option<&str>,
    checksum: &str,
    capture_method: &str,
    previous_snapshot_id: Option<Uuid>,
) -> Result<PolicySnapshot> {
    let word_count = PolicySnapshot::calculate_word_count(content_text);
    let char_count = PolicySnapshot::calculate_char_count(content_text);

    let snapshot = sqlx::query_as::<_, PolicySnapshot>(
        r#"
        INSERT INTO policy_snapshots (
            policy_document_id,
            content_text,
            content_html,
            word_count,
            char_count,
            checksum,
            capture_method,
            previous_snapshot_id,
            metadata
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, '{}'::jsonb)
        RETURNING *
        "#,
    )
    .bind(policy_document_id)
    .bind(content_text)
    .bind(content_html)
    .bind(word_count)
    .bind(char_count)
    .bind(checksum)
    .bind(capture_method)
    .bind(previous_snapshot_id)
    .fetch_one(pool)
    .await?;

    Ok(snapshot)
}

pub async fn get_latest_snapshot(
    pool: &PgPool,
    policy_document_id: Uuid,
) -> Result<Option<PolicySnapshot>> {
    let snapshot = sqlx::query_as::<_, PolicySnapshot>(
        r#"
        SELECT * FROM policy_snapshots
        WHERE policy_document_id = $1
        ORDER BY captured_at DESC
        LIMIT 1
        "#,
    )
    .bind(policy_document_id)
    .fetch_optional(pool)
    .await?;

    Ok(snapshot)
}

pub async fn create_policy_change(
    pool: &PgPool,
    policy_document_id: Uuid,
    previous_snapshot_id: Option<Uuid>,
    current_snapshot_id: Option<Uuid>,
    change_type: &str,
    change_summary: Option<&str>,
) -> Result<PolicyChange> {
    let change = sqlx::query_as::<_, PolicyChange>(
        r#"
        INSERT INTO policy_changes (
            policy_document_id,
            previous_snapshot_id,
            current_snapshot_id,
            change_type,
            severity,
            confidence_score,
            change_summary,
            requires_member_notification,
            false_positive,
            affected_sections,
            metadata
        )
        VALUES ($1, $2, $3, $4, 'unknown', 0.00, $5, false, false, '[]'::jsonb, '{}'::jsonb)
        RETURNING *
        "#,
    )
    .bind(policy_document_id)
    .bind(previous_snapshot_id)
    .bind(current_snapshot_id)
    .bind(change_type)
    .bind(change_summary)
    .fetch_one(pool)
    .await?;

    Ok(change)
}

pub async fn get_recent_changes(
    pool: &PgPool,
    limit: i64,
) -> Result<Vec<PolicyChange>> {
    let changes = sqlx::query_as::<_, PolicyChange>(
        r#"
        SELECT * FROM policy_changes
        WHERE detected_at > NOW() - INTERVAL '30 days'
        ORDER BY detected_at DESC
        LIMIT $1
        "#,
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(changes)
}

pub async fn get_change_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<PolicyChange>> {
    let change = sqlx::query_as::<_, PolicyChange>(
        r#"
        SELECT * FROM policy_changes
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(change)
}

pub async fn update_platform_last_checked(
    pool: &PgPool,
    platform_id: Uuid,
) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE platforms
        SET updated_at = NOW(),
            metadata = jsonb_set(
                COALESCE(metadata, '{}'::jsonb),
                '{last_checked_at}',
                to_jsonb(NOW())
            )
        WHERE id = $1
        "#,
    )
    .bind(platform_id)
    .execute(pool)
    .await?;

    Ok(())
}
