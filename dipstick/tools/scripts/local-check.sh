#!/usr/bin/env bash
# Run the standalone QA loop while container tooling is unavailable.
set -euo pipefail

ROOT=$(git rev-parse --show-toplevel)
cd "$ROOT"

echo "🧹 Running local smoke loop"
just --justfile Justfile setup
just --justfile Justfile test

echo "✅ Local loop completed (collector + analyzer + publisher + dashboard tests)."
