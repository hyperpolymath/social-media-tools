# NUJ Social Media Ethics Monitor

## Project Overview

Automated system to deliver NUJ DM motion requiring monitoring of social media platform policies and 6-monthly guidance to members. Replaces manual monitoring (£18k/year) with automated solution (£400/year) while generating potential £30k/year revenue.

**Repository**: `social-media-dipstick` (monitoring/measurement tool for social media platforms)

## Business Case

- **Cost Savings**: £18k/year manual monitoring → £400/year automated
- **Revenue Potential**: £30k/year (training £10k, union licensing £15k, API access £5k)
- **Net Benefit**: £32k/year
- **Time Savings**: Manual hours → <10 min/day human oversight
- **Success Metrics**: 99% uptime, 70% draft accuracy, £32k revenue year 1

## Project Purpose

### Core Mission
Monitor social media platform policy changes and deliver 6-monthly guidance to NUJ members as required by Democratic Motion. System prevents crises through continuous monitoring rather than reactive response.

### Key Features
- **24/7 Policy Monitoring**: Automated tracking of platform terms/policies
- **Change Detection**: NLP-powered analysis of policy modifications
- **Impact Assessment**: Automated severity scoring and member impact analysis
- **Guidance Generation**: AI-assisted draft creation (human approved)
- **Member Communication**: Automated delivery with safety guardrails
- **Reporting Dashboard**: Real-time visibility for comms team
- **Archive & Compliance**: Complete audit trail for DM compliance

### Phil's Requirements (Project Foundation)
1. Simple starting point, not complex portal initially
2. Matt Capon's 5 principles as foundation
3. Health & safety approach to social media abuse
4. 6-monthly reviews (automated)
5. Zero/negative budget
6. Lightest touch supervision, clear guardrails

## Technology Stack v2.0

**Architecture**: ReScript + WASM + Deno + Rust + AI Agent Swarms
**NO Python, NO TypeScript, NO Node.js** - Pure functional + systems programming

### Core Services (Selur/Chainguard orchestration)
- **Collector Service** (Rust): Platform monitoring, SHA256 change detection, WASM plugins
- **Analyzer Service** (ReScript + Deno + WASM): Type-safe NLP with Rust WASM modules for performance
- **Dashboard Service** (Elixir/Phoenix): Real-time LiveView monitoring, approval workflows
- **Publisher Service** (Deno): Email delivery with 19-layer safety guardrails
- **GraphQL Gateway** (ReScript + Deno): Federated API layer, type-safe schema stitching

### AI Intelligence Layer
- **Agent Swarm** (Deno + LangChain): 7 specialized AI agents (Policy Analyst, Severity Assessor, Guidance Writer, PESTLE Analyst, Fact Checker, Member Impact Evaluator, Coordinator)
- **PESTLE Observatory** (Deno): Connects to 6 external GraphQL sources (GDPR Observatory, IFEX, Ranking Digital Rights, Article 19, Index on Censorship, Tech Policy Lab)

### Advanced Services
- **Julia Scraper**: Massively parallel (1000+ concurrent connections)
- **XTDB Temporal Adapter** (Clojure): Bitemporal queries, audit trails
- **Ada TUI**: Type-safe configuration interface

- **SurrealDB + VerisimDB**: SurrealDB holds the platform metadata, policy documents, credentials, and guidance drafts (multi-model JSON storage), while VerisimDB captures the immutable policy snapshot and change history time-series that used to rely on hypertables.
- **Virtuoso RDF**: PESTLE semantic intelligence, SPARQL endpoint
- **XTDB**: Bitemporal audit trail (valid-time + transaction-time)
- **Dragonfly**: High-performance cache (25x faster than Redis)

### Infrastructure
- **Selur/Vörðr**: Verified container orchestration (14 services managed through Svalinn policy)
- **WASM**: AOT-compiled NLP (10-100x faster than interpreted)
- **Deno**: Secure-by-default runtime (explicit permissions)
- **GitHub Actions**: CI/CD pipeline
- **Monitoring**: Prometheus + Grafana + Loki

**Why This Stack**:
- **Type Safety**: ReScript (sound), Rust (borrow checker), Ada (SPARK proofs)
- **Performance**: WASM AOT compilation, Dragonfly cache, Julia parallelism
- **Security**: Deno permissions, Chainguard runtime isolation, sandboxed WASM
- **AI Intelligence**: Multi-agent swarms, PESTLE external observatories
- **Maintainability**: Functional programming, no runtime exceptions
- **Modern Portfolio**: Cutting-edge tech stack valuable for ICT team

## Project Structure

```
social-media-dipstick/
├── services/
│   ├── collector/                # Rust: Platform monitoring, WASM plugins
│   ├── analyzer-rescript/        # ReScript + Deno + WASM: Type-safe NLP
│   │   ├── src/                  # ReScript source (compiles to .bs.js)
│   │   └── wasm/                 # Rust WASM modules (sentiment, extraction)
│   ├── gateway-rescript/         # ReScript + Deno: GraphQL federation
│   ├── dashboard/                # Elixir/Phoenix: Real-time LiveView UI
│   ├── publisher-deno/           # Deno: Email delivery, 19 guardrails
│   ├── agent-swarm/              # Deno: AI multi-agent coordination
│   │   ├── src/swarm.ts          # Agent swarm coordinator
│   │   └── agents/               # Individual agent definitions
│   ├── pestle-observatory/       # Deno: External GraphQL federation
│   ├── scraper-julia/            # Julia: Massively parallel scraping
│   ├── xtdb-temporal/            # Clojure: Bitemporal query adapter
│   └── tui-ada/                  # Ada: Type-safe configuration TUI
├── shared/
│   ├── schemas/                  # API contracts, OpenAPI, GraphQL schemas
│   ├── libraries/                # Shared utilities
│   └── types/                    # Common data models (ReScript, Rust)
├── infrastructure/
│   ├── database/                 # SurrealDB + VerisimDB schemas and migration scripts
│   │   ├── virtuoso/             # RDF ontologies, SPARQL queries
│   │   └── xtdb/                 # Temporal database config
│   ├── containerfiles/           # Chainguard Containerfile + Svalinn/Selur/Vörðr tooling
│   └── monitoring/               # Prometheus, Grafana, Loki configs
├── config/
│   ├── cue/                      # CUE schema validation scripts
│   ├── nlp_tuning.ncl            # Nickel self-tuning config
│   └── observatories/            # External GraphQL endpoints
├── tools/
│   ├── cli/                      # Development & ops tools
│   ├── scripts/                  # Automation utilities
│   └── autoconfig/               # SMTP, GraphQL autoconfiguration
├── docs/
│   ├── architecture/             # System design (ARCHITECTURE.md)
│   ├── api/                      # GraphQL schema, REST endpoints
│   └── user-guides/              # End-user documentation
├── tests/
│   ├── integration/              # Cross-service tests
│   └── e2e/                      # End-to-end scenarios
├── .well-known/                  # RFC 9116 (security.txt, ai.txt, humans.txt)
├── .github/
│   └── workflows/                # CI/CD pipelines
├── ARCHITECTURE.md               # Deep technical documentation
├── RSR_COMPLIANCE.md             # 86% Gold level compliance
└── justfile                      # Build automation (53+ recipes)
```

## Safety Guardrails (19 Layers)

### Never Auto-Publish Without Approval
1. All guidance requires human approval
2. 5-minute grace period before sending
3. Test group send-first capability
4. Auto-rollback on delivery failures
5. Emergency stop button

### Monitoring & Alerting
6. Platform change notifications
7. Service health monitoring
8. False positive detection
9. Delivery success tracking
10. Anomaly detection

### Data Protection
11. Member data encryption
12. Access control (role-based)
13. Audit logging (all actions)
14. GDPR compliance
15. Data retention policies

### Operational Safety
16. Rate limiting (prevent API abuse)
17. Graceful degradation
18. Backup systems
19. Disaster recovery

**Philosophy**: "Err on side of caution" + "Like a cricomms plugin"

## Development Phases

### Phase 1: Manual Process (Weeks 1-4, £0)
- No code, manual monitoring to demonstrate DM compliance
- Build process documentation
- Identify automation opportunities
- **Deliverable**: First 6-monthly guidance sent manually

### Phase 2: Data Collection (Weeks 5-8, £0)
- Collector service (automated scraping)
- Database setup
- Basic dashboard
- **Deliverable**: Automated policy change detection

### Phase 3: Analysis (Weeks 9-12, £400)
- NLP integration
- Impact assessment
- Guidance drafting
- **Deliverable**: AI-assisted draft generation

### Phase 4: Publishing (Weeks 13-16, £0)
- Publisher service
- Email delivery
- Safety guardrails
- **Deliverable**: End-to-end automation

### Phase 5: Enhancement (Weeks 17-18, £0)
- Advanced analytics
- Pattern recognition
- Optimization
- **Deliverable**: Production-ready system

### Phase 6: Commercialization (Weeks 19-20, £0)
- API for external unions
- Training materials
- Licensing framework
- **Deliverable**: Revenue generation ready

## Setup Instructions

# Prerequisites
```bash
# Required
- Chainguard container toolchain (chainguard/chainguard base image plus selur-compose/Svalinn/Vörðr)
- SurrealDB + VerisimDB runtimes for the core storage layer
- Rust 1.72+
- Deno 2+
- Elixir 1.15+
- Julia 1.10+

# Optional (for development)
- Graphical SurrealDB/VerisimDB consoles
- `chainguard-cli` and `selur-compose` helpers
```

### Quick Start
```bash
# Clone repository
git clone https://github.com/Hyperpolymath/social-media-dipstick.git
cd social-media-dipstick

# Start all services
selur-compose up

# Run migrations
./tools/cli/migrate.sh

# Access dashboard
open http://localhost:4000
```

### Development Environment
```bash
# Install dependencies for all services
./scripts/setup-dev.sh

# Run tests
./scripts/test-all.sh

# Start development servers
./scripts/dev-start.sh
```

## API Integration

### Supported Platforms
- Twitter/X API v2
- Meta Graph API (Facebook, Instagram)
- LinkedIn API
- TikTok Business API
- YouTube Data API
- Bluesky API (AT Protocol)

### Monitoring Methods
1. **Official APIs**: Primary source for policy documents
2. **Web Scraping**: Backup for pages without API access
3. **Change Detection**: Diff-based tracking with NLP validation
4. **Archive Comparison**: Wayback Machine integration

## Configuration

### Environment Variables
```bash
# Database
DATABASE_URL=postgresql://user:pass@localhost:5432/nuj_monitor
TIMESCALEDB_ENABLED=true

# Services
COLLECTOR_PORT=3001
ANALYZER_PORT=3002
DASHBOARD_PORT=4000
PUBLISHER_PORT=3003

# External APIs
TWITTER_API_KEY=...
META_ACCESS_TOKEN=...
OPENAI_API_KEY=...

# Email Delivery
SMTP_HOST=...
SMTP_PORT=587
FROM_EMAIL=monitor@nuj.org.uk

# Safety
APPROVAL_REQUIRED=true
GRACE_PERIOD_MINUTES=5
TEST_GROUP_EMAILS=comms@nuj.org.uk
```

### Configuration Files
- `config/collector.toml`: Platform definitions, scraping rules
- `config/analyzer.yml`: NLP models, severity thresholds
- `config/publisher.json`: Email templates, delivery rules
- `config/dashboard.exs`: UI settings, user roles

## Contributing

### Code Style
- **Rust**: `cargo fmt`, `cargo clippy`
- **ReScript + Deno**: `rescript build`, `deno fmt`, `deno lint`
- **Elixir**: `mix format`

### Testing Requirements
- Unit tests: 80% coverage minimum
- Integration tests: Critical paths covered
- E2E tests: User workflows validated
- Run `./scripts/test-all.sh` before commits

### Git Workflow
- **Branches**: `feature/`, `bugfix/`, `claude/` prefixes
- **Commits**: Conventional Commits format
- **PRs**: Require CI pass + one approval
- **Deploy**: Automatic to staging, manual to production

## Stakeholder Communication

### ICT Team (15-min pitch)
- **Hook**: Modern tech stack, portfolio value
- **Pain**: Manual monitoring unsustainable
- **Solution**: Low-maintenance automation
- **Ask**: £400/year NEC approval support
- **Template**: See `docs/stakeholder/ict-pitch.md`

### Comms Team (30-min demo)
- **Hook**: <10 min/day vs hours/week
- **Demo**: Live change detection + draft generation
- **Safety**: 19 guardrails walkthrough
- **Ask**: Process feedback, test group participation
- **Script**: See `docs/stakeholder/comms-demo.md`

### NEC Paper
- **Template**: `docs/stakeholder/nec-paper-template.md`
- **Budget**: £400/year (OpenAI API)
- **Business Case**: £32k net benefit year 1
- **Risk**: Low (manual fallback always available)

## Dependencies

### Service Dependencies
- See individual `Cargo.toml`, `pyproject.toml`, `mix.exs`, `package.json`
- Managed via dependency lock files
- Automated security updates via Dependabot

### External Services
- OpenAI API (guidance generation): £400/year
- Platform APIs: Free tiers + monitoring
- Email delivery: Existing NUJ SMTP

## Build and Deployment

### Local Development
```bash
./scripts/dev-start.sh     # Start all services in dev mode
./scripts/dev-stop.sh      # Stop all services
./scripts/dev-logs.sh      # Tail logs from all services
```

### Staging Deployment
```bash
# Automated via GitHub Actions on merge to main
# Manual trigger:
gh workflow run deploy-staging.yml
```

### Production Deployment
```bash
# Manual approval required
gh workflow run deploy-production.yml

# Rollback
./tools/cli/rollback.sh
```

### Database Migrations
```bash
# Create new migration
./tools/cli/migrate.sh create add_platform_field

# Run migrations
./tools/cli/migrate.sh up

# Rollback last migration
./tools/cli/migrate.sh down
```

## Monitoring & Observability

### Dashboards
- **Grafana**: System health, service metrics
- **Phoenix Dashboard**: Real-time policy changes
- **Email Reports**: Weekly summaries to comms team

### Alerts
- Service down → ICT team
- Platform change detected → Comms team
- Delivery failure → Auto-rollback + alert
- Anomaly detected → Human review required

### Logging
- Structured JSON logs
- Centralized via Loki
- 90-day retention
- Search via Grafana

## Troubleshooting

### Services Won't Start
```bash
# Check selur-compose status
selur-compose ps

# View service logs
selur-compose logs collector

# Restart specific service (if supported)
selur-compose restart collector
```

### Database Connection Errors
```bash
# Check SurrealDB status (via selur-compose)
selur-compose exec surrealdb surreal sql

# Check VerisimDB ingest stream
selur-compose exec verisimdb verisim status

# Verify connection string
echo $DATABASE_URL

# Reset data (DEV ONLY) - pipeline will replay Surreal/Verisim migrations
./scripts/reset-db.sh
```

### Platform API Failures
- Check API quotas in dashboard
- Verify credentials in `.env`
- Fallback to web scraping (automatic)
- See `docs/troubleshooting/api-errors.md`

### Email Delivery Issues
- Check test group delivery first
- Verify SMTP credentials
- Review delivery logs in dashboard
- Manual send available as fallback

## License

To be determined (likely GPL-3.0 for union movement benefit)

## Contact

- **Project Owner**: Jonathan (NUJ)
- **Comms Team**: 2 members (one new, one experienced)
- **ICT Team**: Phantom (hard to reach - use email)
- **Issues**: GitHub Issues in this repository
- **Emergency**: [Emergency contact TBD]

## Revenue Model

### Training (£10k/year)
- Social media ethics workshops
- Platform policy webinars
- Member guidance on policy changes

### Union Licensing (£15k/year)
- TUC unions: £5k/year
- International unions: £10k/year
- White-label deployment support

### API Access (£5k/year)
- Platform change webhooks
- Historical policy data
- Custom analysis endpoints

**Total**: £30k revenue - £400 costs = £29.6k net (including £18k manual monitoring savings = £47.6k total benefit)

## Project Status

- **Design**: Complete ✓
- **Code**: In progress
- **Stakeholder Approval**: Pending
- **Budget Approval**: Pending (£400/year)
- **Volunteers**: Recruiting (need 3-5 developers)
- **Launch**: Phase 1 target week 3-4

---

**Key Insight**: This system prevents crises rather than responding to them. Comms team approves (5 min/day), technology does everything else (24/7 monitoring, drafting, sending, reporting).
