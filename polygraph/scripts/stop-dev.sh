#!/bin/bash

# Stop development environment

set -e

echo "Stopping Social Media Polygraph development environment..."

cd "$(dirname "$0")/../infrastructure/podman"

podman-compose down

echo "✓ Services stopped successfully!"
