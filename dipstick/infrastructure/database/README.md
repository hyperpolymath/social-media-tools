# Database Infrastructure

## Overview

SurrealDB serves as the primary multi-model document store for platform metadata, policy documents, credentials, and guidance drafts. VerisimDB captures the immutable, append-only time-series (policy snapshots, change events, delivery history) that previously relied on hypertables. Both stores expose SQL-like interfaces and the schema lives alongside `infrastructure/database/schema.sql` until the Surreal/Verisim migration scripts replace it.

## Schema Components

### Core Documents
- **platforms**: Surreal table for monitored platforms with JSON metadata
- **platform_credentials**: Encrypted credentials stored in Surreal
- **policy_documents**: Documents tracked per platform with revisions and versions
- **policy_snapshots**: VerisimDB streams that capture immutable snapshots and are indexed by timestamp
- **policy_changes**: VerisimDB events emitted when diffs are detected, referenced by policy IDs

-### Communication Collections
- **guidance_drafts**: Member guidance documents
- **member_segments**: Targeted communication groups
- **guidance_publications**: Published guidance with delivery tracking
- **delivery_events**: Individual email delivery events

### Safety & Audit
- **approval_requests**: Human approval workflow
- **audit_log**: Complete system action audit trail
- **system_metrics**: Service health metrics
- **system_config**: System-wide configuration

### User Management
- **users**: System users with role-based access
- **user_sessions**: Active user sessions

## Surreal & Verisim Models

Surreal maintains the structured tables described above, while Verisim defines the following time-series streams:
- `policy_snapshots` - high-resolution Verisim stream partitioned by day
- `policy_changes` - immutable events with severity tags
- `delivery_events` - email/webhook delivery timelines
- `audit_log` - action stream for human approvals
- `system_metrics` - operational observability stream (ingested into Prometheus via exporters)

## Migrations

Surreal/Verisim migrations live under `migrations/` and are executed via the CLI (`./tools/cli/migrate.sh`). The current `schema.sql` acts as the blueprint for translating the table definitions into Surreal `DEFINE TABLE/STREAM` statements and the Verisim event definitions.

## Initial Data

Default configuration includes:
- 7 major platforms (X, Facebook, Instagram, LinkedIn, TikTok, YouTube, Bluesky)
- Default system configuration
- Default admin user (change password!)
- Default member segments

## Security Features

1. **Encryption**: Platform credentials encrypted with pgcrypto
2. **Audit Logging**: All sensitive actions logged
3. **Row-Level Security**: Can be enabled per deployment
4. **Hashed Emails**: Delivery tracking uses hashed recipient IDs

## Performance Optimizations

1. **Indexes**: Strategic indexes on frequently queried columns
2. **Partial Indexes**: For filtered queries (e.g., active records only)
3. **GIN Indexes**: For JSONB and full-text search
4. **VerisimDB**: Append-only stream partitioning, compression, and retention controls

## Views

Pre-built views for common queries:
- `recent_unreviewed_changes` - Changes needing review
- `pending_approvals` - Actions awaiting approval
- `platform_status` - Current monitoring status
- `guidance_metrics` - Publication performance metrics

## Backup & Recovery

See `../monitoring/backup/` for automated backup scripts.

Retention policy:
- Daily backups: 7 days
- Weekly backups: 4 weeks
- Monthly backups: 12 months

## Monitoring

Metrics exposed:
- Table sizes
- Query performance
- Connection pool usage
- Replication lag (if applicable)

See `../monitoring/` for Prometheus exporters.
