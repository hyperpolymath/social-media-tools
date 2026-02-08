#!/bin/bash
# SurrealDB restore helper

set -euo pipefail

BACKUP_FILE=$1
SURREAL_ENDPOINT=${SURREAL_ENDPOINT:-http://surrealdb:8000}
SURREAL_USER=${SURREAL_USER:-root}
SURREAL_PASS=${SURREAL_PASS:-root}
SURREAL_NAMESPACE=${SURREAL_NAMESPACE:-nuj}
SURREAL_DATABASE=${SURREAL_DATABASE:-nuj_monitor}

if [ -z "$BACKUP_FILE" ]; then
    echo "Usage: $0 <backup_file.surql.gz>"
    exit 1
fi

if [ ! -f "$BACKUP_FILE" ]; then
    echo "Error: Backup file not found: $BACKUP_FILE"
    exit 1
fi

echo "⚠️  WARNING: This will REPLACE the SurrealDB dataset!"
read -p "Are you sure? (yes/no): " CONFIRM

if [ "$CONFIRM" != "yes" ]; then
    echo "Cancelled"
    exit 0
fi

echo "📥 Restoring data from: $BACKUP_FILE"
gunzip -c "$BACKUP_FILE" | selur-compose exec -T surrealdb surreal import \
    --conn "$SURREAL_ENDPOINT" \
    --user "$SURREAL_USER" \
    --pass "$SURREAL_PASS" \
    --ns "$SURREAL_NAMESPACE" \
    --db "$SURREAL_DATABASE" -

echo "✅ SurrealDB restore completed"
