#!/bin/bash
# SurrealDB + VerisimDB migration tool

set -euo pipefail

COMMAND=${1:-up}
SURREAL_NAMESPACE=${SURREAL_NAMESPACE:-nuj}
SURREAL_DATABASE=${SURREAL_DATABASE:-nuj_monitor}
SCHEMA_FILE=${SCHEMA_FILE:-/app/database/schema.sql}

function run_surreal_import() {
    selur-compose exec surrealdb surreal import \
        --ns "$SURREAL_NAMESPACE" \
        --db "$SURREAL_DATABASE" \
        "$SCHEMA_FILE"
}

function show_surreal_schema() {
    selur-compose exec surrealdb surreal sql \
        --ns "$SURREAL_NAMESPACE" \
        --db "$SURREAL_DATABASE" \
        "SHOW TABLES;"
}

case "$COMMAND" in
    up)
        echo "Applying SurrealDB schema from ${SCHEMA_FILE}..."
        run_surreal_import
        echo "✅ SurrealDB schema applied"
        ;;

    down)
        echo "Rolling back migrations is manual."
        echo "Review ${SCHEMA_FILE} or use VerisimDB snapshots for journaled rollbacks."
        ;;

    create)
        if [ -z "${2:-}" ]; then
            echo "Usage: $0 create <migration_name>"
            exit 1
        fi

        TIMESTAMP=$(date +%Y%m%d%H%M%S)
        FILENAME="infrastructure/database/migrations/${TIMESTAMP}_$2.surql"

        cat > "$FILENAME" <<EOF
-- Migration: $2
-- Created: $(date)

-- Add SurrealQL statements here

EOF

        echo "✅ Migration created: $FILENAME"
        ;;

    status)
        echo "SurrealDB tables in ${SURREAL_NAMESPACE}.${SURREAL_DATABASE}:"
        show_surreal_schema
        ;;

    *)
        echo "Usage: $0 {up|down|create <name>|status}"
        exit 1
        ;;
esac
