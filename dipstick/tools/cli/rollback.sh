#!/bin/bash
# Rollback a publication

set -e

PUBLICATION_ID=$1
REASON=${2:-"Manual rollback"}

if [ -z "$PUBLICATION_ID" ]; then
    echo "Usage: $0 <publication_id> [reason]"
    exit 1
fi

echo "🔄 Rolling back publication: $PUBLICATION_ID"
echo "Reason: $REASON"

# Call publisher API
curl -X POST "http://localhost:3003/api/publications/$PUBLICATION_ID/rollback" \
    -H "Content-Type: application/json" \
    -d "{\"reason\": \"$REASON\"}"

echo ""
echo "✅ Rollback initiated"
