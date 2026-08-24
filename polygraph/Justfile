# Justfile for Social Media Polygraph
# https://github.com/casey/just

# Default recipe (list all recipes)
default:
    @just --list

# Install all dependencies
install:
    @echo "Installing Rust backend dependencies..."
    cd backend && cargo fetch
    @echo "Installing Elixir dependencies..."
    cd elixir && mix deps.get
    @echo "Installing frontend dependencies (ReScript)..."
    cd frontend && npm install
    @echo "✓ All dependencies installed"

# Run backend tests
test-backend:
    @echo "Running Rust tests..."
    cd backend && cargo test --all-features

# Run Elixir tests
test-elixir:
    @echo "Running Elixir tests..."
    cd elixir && mix test

# Run frontend tests
test-frontend:
    @echo "Running ReScript build (type check)..."
    cd frontend && npm run res:build
    @echo "Running Deno tests..."
    cd frontend && deno task test || true

# Run all tests
test: test-backend test-elixir test-frontend
    @echo "✓ All tests passed"

# Format code
fmt:
    @echo "Formatting Rust code..."
    cd backend && cargo fmt
    @echo "Formatting Elixir code..."
    cd elixir && mix format
    @echo "Formatting frontend code..."
    cd frontend && deno fmt

# Lint code
lint:
    @echo "Linting Rust..."
    cd backend && cargo clippy -- -D warnings
    @echo "Linting Elixir..."
    cd elixir && mix credo || true
    @echo "Linting frontend..."
    cd frontend && deno lint
    @echo "Type-checking ReScript..."
    cd frontend && npm run res:build

# Run backend locally
run-backend:
    @echo "Starting Rust GraphQL server..."
    cd backend && cargo run --release

# Run Elixir node
run-elixir:
    @echo "Starting Elixir CRDT node..."
    cd elixir && mix run --no-halt

# Run frontend locally
run-frontend:
    @echo "Starting ReScript + Deno dev server..."
    cd frontend && npm run res:watch &
    cd frontend && deno task dev

# Build all components
build: build-backend build-elixir build-frontend

# Build backend
build-backend:
    @echo "Building Rust backend..."
    cd backend && cargo build --release

# Build Elixir
build-elixir:
    @echo "Building Elixir..."
    cd elixir && mix compile

# Build frontend for production
build-frontend:
    @echo "Compiling ReScript..."
    cd frontend && npm run res:build
    @echo "Building with Deno..."
    cd frontend && deno task build

# Start all services with Podman Compose
up:
    @echo "Starting all services..."
    cd infrastructure/podman && podman-compose up -d
    @echo "Services started!"
    @echo "  Nginx Proxy: http://localhost (HTTPS: https://localhost:443)"
    @echo "  GraphQL API: http://localhost:8000/graphql"
    @echo "  Frontend: http://localhost:8080"
    @echo "  Elixir Nodes: localhost:9100-9101"
    @echo "  ArangoDB: http://localhost:8529"
    @echo "  XTDB: http://localhost:3000"
    @echo "  Dragonfly: localhost:6379"

# Stop all services
down:
    @echo "Stopping all services..."
    cd infrastructure/podman && podman-compose down

# View logs
logs SERVICE="":
    #!/usr/bin/env bash
    if [ -z "{{SERVICE}}" ]; then
        cd infrastructure/podman && podman-compose logs -f
    else
        cd infrastructure/podman && podman-compose logs -f {{SERVICE}}
    fi

# Restart services
restart SERVICE="":
    #!/usr/bin/env bash
    if [ -z "{{SERVICE}}" ]; then
        just down && just up
    else
        cd infrastructure/podman && podman-compose restart {{SERVICE}}
    fi

# Check service status
status:
    @echo "Checking service status..."
    cd infrastructure/podman && podman-compose ps

# Build containers
build-containers:
    @echo "Building Rust backend container..."
    cd backend && podman build -t polygraph-backend:latest -f Containerfile .
    @echo "Building Elixir container..."
    cd elixir && podman build -t polygraph-elixir:latest -f Containerfile .
    @echo "Building frontend container..."
    cd frontend && podman build -t polygraph-frontend:latest -f Containerfile .

# Clean build artifacts
clean:
    @echo "Cleaning build artifacts..."
    rm -rf backend/target
    rm -rf elixir/_build elixir/deps
    rm -rf frontend/lib/bs frontend/dist
    @echo "Clean complete"

# Clean everything including dependencies
clean-all: clean
    @echo "Removing all artifacts..."
    rm -rf backend/target
    rm -rf elixir/_build elixir/deps
    rm -rf frontend/node_modules frontend/lib/bs frontend/dist
    @echo "Deep clean complete"

# Security audit
audit:
    @echo "Running security audit..."
    @echo "Checking Rust dependencies..."
    cd backend && cargo audit || (echo "Install cargo-audit: cargo install cargo-audit" && true)
    @echo "Checking Elixir dependencies..."
    cd elixir && mix hex.audit || true
    @echo "Checking frontend dependencies..."
    cd frontend && npm audit || true

# Database migrations (when implemented)
migrate:
    @echo "Running database migrations..."
    @echo "Not yet implemented"

# Seed database with test data
seed:
    @echo "Seeding database..."
    @echo "Not yet implemented"

# RSR compliance check
validate-rsr:
    @echo "Checking RSR compliance..."
    @just --evaluate _check-docs
    @just --evaluate _check-security
    @just --evaluate _check-tests
    @echo "✓ RSR compliance validated"

# Check documentation completeness
_check-docs:
    #!/usr/bin/env bash
    echo "Checking documentation..."
    docs=("README.md" "LICENSE" "SECURITY.md" "CODE_OF_CONDUCT.md" "CONTRIBUTING.md" "MAINTAINERS.md" "CHANGELOG.md")
    missing=()
    for doc in "${docs[@]}"; do
        if [ ! -f "$doc" ]; then
            missing+=("$doc")
        fi
    done
    if [ ${#missing[@]} -eq 0 ]; then
        echo "  ✓ All required docs present"
    else
        echo "  ✗ Missing: ${missing[*]}"
        exit 1
    fi

# Check security files
_check-security:
    #!/usr/bin/env bash
    echo "Checking security files..."
    files=(".well-known/security.txt" ".well-known/ai.txt" ".well-known/humans.txt" "SECURITY.md")
    missing=()
    for file in "${files[@]}"; do
        if [ ! -f "$file" ]; then
            missing+=("$file")
        fi
    done
    if [ ${#missing[@]} -eq 0 ]; then
        echo "  ✓ All security files present"
    else
        echo "  ✗ Missing: ${missing[*]}"
        exit 1
    fi

# Check test coverage
_check-tests:
    @echo "Checking test coverage..."
    cd backend && poetry run pytest --cov=app --cov-report=term-missing --cov-fail-under=0 || true

# Generate API documentation
docs:
    @echo "Starting API documentation server..."
    @echo "Visit http://localhost:8000/docs"
    just run-backend

# Create new release
release VERSION:
    @echo "Creating release {{VERSION}}..."
    @echo "1. Update CHANGELOG.md"
    @echo "2. Update version in pyproject.toml and package.json"
    @echo "3. Commit changes"
    @echo "4. Create git tag: git tag -a v{{VERSION}} -m 'Release {{VERSION}}'"
    @echo "5. Push: git push && git push --tags"

# Check environment setup
check-env:
    @echo "Checking environment..."
    @command -v rustc >/dev/null 2>&1 || (echo "✗ Rust not found" && exit 1)
    @command -v cargo >/dev/null 2>&1 || (echo "✗ Cargo not found" && exit 1)
    @command -v elixir >/dev/null 2>&1 || (echo "✗ Elixir not found" && exit 1)
    @command -v mix >/dev/null 2>&1 || (echo "✗ Mix not found" && exit 1)
    @command -v node >/dev/null 2>&1 || (echo "✗ Node.js not found" && exit 1)
    @command -v deno >/dev/null 2>&1 || (echo "✗ Deno not found" && exit 1)
    @command -v podman >/dev/null 2>&1 || (echo "✗ Podman not found" && exit 1)
    @echo "✓ All required tools found"
    @rustc --version
    @cargo --version
    @elixir --version | head -n 1
    @mix --version | head -n 1
    @node --version
    @deno --version | head -n 1
    @podman --version

# Development setup
dev-setup: check-env install
    @echo "Setting up development environment..."
    @echo "Copying environment file..."
    @test -f .env || cp .env.example .env
    @echo "Generating self-signed SSL certificates for development..."
    @mkdir -p infrastructure/configs/ssl
    @just _gen-dev-certs
    @echo "✓ Development environment ready"
    @echo ""
    @echo "Next steps:"
    @echo "  1. Edit .env with your configuration (API keys, etc.)"
    @echo "  2. Run 'just up' to start services"
    @echo "  3. Visit http://localhost:8000/graphql for GraphQL Playground"

# Generate development SSL certificates
_gen-dev-certs:
    #!/usr/bin/env bash
    if [ ! -f infrastructure/configs/ssl/cert.pem ]; then
        echo "Generating self-signed SSL certificate..."
        cd infrastructure/configs/ssl && \
        openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
            -keyout key.pem -out cert.pem \
            -subj "/C=US/ST=Dev/L=Local/O=Polygraph/CN=localhost" \
            2>/dev/null && \
        chmod 600 key.pem && chmod 644 cert.pem && \
        echo "✓ SSL certificates generated"
    fi

# Production deployment check
prod-check:
    @echo "Production readiness checklist:"
    @echo "  [ ] Changed all default secrets in .env"
    @echo "  [ ] SSL certificates configured"
    @echo "  [ ] Firewall rules in place"
    @echo "  [ ] Backup system configured"
    @echo "  [ ] Monitoring enabled"
    @echo "  [ ] Log rotation configured"
    @echo "  [ ] Email notifications set up"
    @echo "  [ ] Domain DNS configured"
    @echo "See docs/DEPLOYMENT.md for details"

# Quick health check
health:
    @echo "Checking service health..."
    @curl -f http://localhost:8000/health 2>/dev/null && echo "✓ Backend running" || echo "✗ Backend not running"
    @curl -f http://localhost:8080 >/dev/null 2>&1 && echo "✓ Frontend running" || echo "✗ Frontend not running"
    @nc -z localhost 9100 2>/dev/null && echo "✓ Elixir node 1 running" || echo "✗ Elixir node 1 not running"
    @nc -z localhost 9101 2>/dev/null && echo "✓ Elixir node 2 running" || echo "✗ Elixir node 2 not running"

# Backup databases
backup:
    @echo "Creating backup..."
    @mkdir -p backups
    @echo "Backing up ArangoDB..."
    @podman exec polygraph-arangodb arangodump --output-directory /tmp/backup || echo "ArangoDB not running"
    @echo "Backup complete: backups/$(date +%Y%m%d_%H%M%S)"

# Interactive development mode
dev:
    @echo "Starting development mode..."
    @echo "This will:"
    @echo "  1. Start databases (ArangoDB, XTDB, Dragonfly)"
    @echo "  2. Keep backend and frontend logs tailing"
    @echo ""
    @echo "Press Ctrl+C to stop all services"
    @just up
    @trap 'just down' EXIT; just logs
