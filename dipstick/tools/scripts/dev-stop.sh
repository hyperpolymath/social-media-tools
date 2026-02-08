#!/bin/bash
# Stop all development servers

echo "🛑 Stopping NUJ Monitor services..."

# Kill background processes
pkill -f "cargo run" || true
pkill -f "deno task dev" || true
pkill -f "deno run" || true
pkill -f "mix phx.server" || true

echo "✅ All services stopped"
