#!/bin/bash
# Start all development servers (service shells)

set -euo pipefail

echo "🚀 Starting NUJ Monitor development services..."

echo "Ensure infrastructure stack (SurrealDB, VerisimDB, Redis, Prometheus, Grafana, Loki) is running via selur-compose up"

echo "Waiting for infrastructure readiness..."
sleep 5

# Start collector (Rust)
echo "Starting Collector service..."
cd services/collector
cargo run &
COLLECTOR_PID=$!
cd ../..

# Start analyzer (ReScript + Deno)
echo "Starting Analyzer service..."
cd services/analyzer-rescript
rescript build
deno task dev &
ANALYZER_PID=$!
cd ../..

# Start publisher (Deno)
echo "Starting Publisher service..."
cd services/publisher-deno
deno task dev &
PUBLISHER_PID=$!
cd ../..

# Start dashboard (Elixir)
echo "Starting Dashboard service..."
cd services/dashboard
mix phx.server &
DASHBOARD_PID=$!
cd ../..

echo ""
echo "✅ Services started"
echo "  Collector:  http://localhost:3001"
echo "  Analyzer:   http://localhost:3002"
echo "  Publisher:  http://localhost:3003"
echo "  Dashboard:  http://localhost:4000"
echo ""
echo "📈 Monitoring (selur-compose hosts data plane)"
echo "  Prometheus: http://localhost:9090"
echo "  Grafana:    http://localhost:3000"
echo ""
echo "Press Ctrl+C to stop all services"

trap "echo 'Stopping services...'; kill $COLLECTOR_PID $ANALYZER_PID $PUBLISHER_PID $DASHBOARD_PID; exit" INT

wait
