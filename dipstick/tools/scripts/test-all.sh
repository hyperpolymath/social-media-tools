#!/bin/bash
# Run tests for all services

set -e

echo "🧪 Running tests for all services..."

# Collector (Rust)
echo -e "\n📦 Testing Collector (Rust)..."
cd services/collector
cargo test
cargo clippy -- -D warnings
cargo fmt -- --check
cd ../..

# Analyzer (ReScript + Deno)
echo -e "\n📦 Testing Analyzer (ReScript + Deno)..."
cd services/analyzer-rescript
rescript build
deno fmt --check
deno lint
cd ../..

# Publisher (Deno)
echo -e "\n📦 Testing Publisher (Deno)..."
cd services/publisher-deno
deno fmt --check
deno lint
cd ../..

# Dashboard (Elixir)
echo -e "\n📦 Testing Dashboard (Elixir)..."
cd services/dashboard
mix test
mix format --check-formatted
cd ../..

# Scraper (Julia)
echo -e "\n📦 Testing Scraper (Julia)..."
cd services/scraper-julia
julia --project=. -e 'using Pkg; Pkg.test()'
cd ../..

echo -e "\n✅ All tests passed!"
