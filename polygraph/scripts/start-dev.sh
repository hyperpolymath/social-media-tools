#!/bin/bash

# Start development environment with Podman Compose

set -e

echo "Starting Social Media Polygraph development environment..."

# Check if podman-compose is installed
if ! command -v podman-compose &> /dev/null; then
    echo "Error: podman-compose is not installed"
    echo "Install with: pip install podman-compose"
    exit 1
fi

# Navigate to infrastructure directory
cd "$(dirname "$0")/../infrastructure/podman"

# Check if .env exists
if [ ! -f .env ]; then
    echo "Creating .env file from .env.example..."
    cp .env.example .env
    echo "Please update .env with your configuration"
fi

# Start services
echo "Starting services with Podman Compose..."
podman-compose up -d

echo ""
echo "✓ Services started successfully!"
echo ""
echo "Available services:"
echo "  - ArangoDB:  http://localhost:8529"
echo "  - XTDB:      http://localhost:3000"
echo "  - Dragonfly: localhost:6379"
echo "  - Backend:   http://localhost:8000"
echo "  - Frontend:  http://localhost:3000"
echo ""
echo "API Documentation: http://localhost:8000/docs"
echo ""
echo "To view logs: podman-compose logs -f"
echo "To stop: podman-compose down"
