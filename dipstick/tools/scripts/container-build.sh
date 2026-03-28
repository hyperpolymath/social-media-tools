#!/bin/bash
# Build the Chainguard container artifact for selur-compose deployments.
set -euo pipefail

REPO_ROOT=$(git rev-parse --show-toplevel)
CONTAINERFILE="containerfiles/chainguard.Containerfile"
TAG=${TAG:-ghcr.io/hyperpolymath/social-media-monitor:latest}
LOCAL_REPOS=${LOCAL_REPOS:-/var$REPOS_DIR}
EXTRA_ARGS=${EXTRA_ARGS:-}

if ! command -v docker >/dev/null 2>&1; then
    echo "Error: docker is required to build the Chainguard container."
    exit 1
fi

cd "$REPO_ROOT"

echo "🧱 Building Chainguard container ($TAG) using $CONTAINERFILE"

docker buildx build \
    --progress=plain \
    --file "$CONTAINERFILE" \
    --build-arg LOCAL_REPOS="$LOCAL_REPOS" \
    --tag "$TAG" \
    --output type=image,push=false \
    $EXTRA_ARGS \
    .

echo "✅ Container build complete: $TAG"
