# NUJ Collector Service

Platform monitoring and policy collection service built in Rust.

## Purpose

Continuously monitors social media platforms for policy changes by:
- Scraping policy/terms/community guidelines pages
- Fetching via platform APIs where available
- Detecting changes through content checksumming
- Storing snapshots in SurrealDB with VerisimDB time-series backups
- Creating change records for analyzer service

## Features

- **Multi-platform support**: Twitter/X, Facebook, Instagram, LinkedIn, TikTok, YouTube, Bluesky
- **Dual collection methods**: API-first with scraping fallback
- **Change detection**: SHA256 checksumming with diff tracking
- **Concurrent collection**: Configurable parallel platform monitoring
- **Scheduled collection**: Cron-based automatic collection (default: every 15 minutes)
- **Manual triggers**: HTTP API for on-demand collection
- **Health monitoring**: Prometheus metrics + health checks

## API Endpoints

### Health & Monitoring
- `GET /health` - Service health check
- `GET /metrics` - Prometheus metrics

### Platforms
- `GET /api/platforms` - List all monitored platforms
- `GET /api/platforms/:id` - Get specific platform details
- `POST /api/platforms/:id/collect` - Trigger manual collection

### Changes
- `GET /api/changes` - List recent policy changes
- `GET /api/changes/:id` - Get specific change details

## Configuration

Via environment variables (see `.env.example` in project root):

```bash
# Service
COLLECTOR_PORT=3001
DATABASE_URL=postgresql://...
REDIS_URL=redis://...

# Collection behavior
MAX_CONCURRENT_COLLECTIONS=10
DEFAULT_CHECK_FREQUENCY=60
USER_AGENT="NUJ Social Media Monitor/1.0"

# Platform credentials
TWITTER_API_KEY=...
META_ACCESS_TOKEN=...
# ... etc
```

## Development

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build

# Run tests
cargo test

# Run locally
cargo run

# Run with auto-reload (requires cargo-watch)
cargo install cargo-watch
cargo watch -x run

# Format code
cargo fmt

# Lint
cargo clippy
```

## Container Runtime

```bash
# Build Chainguard container
just container-build

# Start collector via selur-compose (inside Svalinn/Vörðr stack)
selur-compose up collector
```

## Architecture

```
main.rs
├── config.rs         - Configuration management
├── db.rs             - Database operations (SQLx)
├── models.rs         - Data models
├── handlers.rs       - HTTP request handlers
├── platforms.rs      - Platform collection orchestration
├── scraper.rs        - Web scraping & API fetching
└── scheduler.rs      - Cron-based scheduling
```

## Collection Flow

1. Scheduler triggers collection cycle (or manual HTTP trigger)
2. Fetch active platforms from database
3. For each platform (up to MAX_CONCURRENT_COLLECTIONS parallel):
   - Extract policy URLs from platform record
   - Fetch content (API or scraper)
   - Calculate SHA256 checksum
   - Compare with latest snapshot
   - If changed: Create new snapshot + policy change record
   - If unchanged: Create snapshot (for audit trail)
4. Update platform "last checked" timestamp
5. Return results

## Change Detection

Changes detected by comparing SHA256 checksums of content:
- **Identical checksum**: No change, record snapshot only
- **Different checksum**: Change detected, create policy_change record

Change records sent to analyzer service for NLP processing.

## Error Handling

- **API failures**: Automatic fallback to web scraping
- **Network errors**: Retry with exponential backoff
- **Parsing errors**: Logged and reported via metrics
- **Database errors**: Propagated to caller, logged

## Monitoring

Prometheus metrics exposed at `/metrics`:
- `collector_collections_total` - Total collection attempts
- `collector_collections_success` - Successful collections
- `collector_collections_failed` - Failed collections
- `collector_changes_detected` - Changes detected
- `collector_collection_duration_seconds` - Collection duration

## Performance

- **Concurrent collections**: 10 platforms simultaneously (configurable)
- **Request timeout**: 30 seconds
- **Database connection pool**: 20 connections
- **Memory usage**: ~50-100MB idle, ~200-500MB during collection

## Future Enhancements

- [ ] Platform-specific API implementations
- [ ] Wayback Machine integration for historical comparison
- [ ] Content diffing at section level
- [ ] Intelligent retry with circuit breaker
- [ ] Webhook notifications for critical changes
- [ ] GraphQL API
