#!/bin/bash
# SurrealDB backup helper

set -euo pipefail

BACKUP_DIR=${BACKUP_DIR:-./backups}
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="$BACKUP_DIR/surrealdb_$TIMESTAMP.surql.gz"
SURREAL_ENDPOINT=${SURREAL_ENDPOINT:-http://surrealdb:8000}
SURREAL_USER=${SURREAL_USER:-root}
SURREAL_PASS=${SURREAL_PASS:-root}
SURREAL_NAMESPACE=${SURREAL_NAMESPACE:-nuj}
SURREAL_DATABASE=${SURREAL_DATABASE:-nuj_monitor}

mkdir -p "$BACKUP_DIR"

echo "📦 Exporting SurrealDB namespace ${SURREAL_NAMESPACE}.${SURREAL_DATABASE}..."
selur-compose exec -T surrealdb surreal export \
    --conn "$SURREAL_ENDPOINT" \
    --user "$SURREAL_USER" \
    --pass "$SURREAL_PASS" \
    --ns "$SURREAL_NAMESPACE" \
    --db "$SURREAL_DATABASE" | gzip > "$BACKUP_FILE"

echo "✅ Backup created: $BACKUP_FILE"

# Cleanup old backups (keep last 7 days)
find "$BACKUP_DIR" -name "*.surql.gz" -mtime +7 -delete

echo "🧹 Old backups cleaned up"
