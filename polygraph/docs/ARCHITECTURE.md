# Architecture Documentation

## System Overview

Social Media Polygraph is a microservices-based application for AI-powered fact-checking.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                      Clients                            │
│  ┌───────────┐  ┌───────────┐  ┌──────────────────┐   │
│  │ Web UI    │  │   API     │  │ Browser Extension│   │
│  └─────┬─────┘  └─────┬─────┘  └────────┬─────────┘   │
└────────┼──────────────┼─────────────────┼──────────────┘
         │              │                  │
         └──────────────┴──────────────────┘
                        │
         ┌──────────────▼──────────────┐
         │      Nginx (Reverse Proxy)  │
         └──────────────┬──────────────┘
                        │
         ┌──────────────▼──────────────┐
         │   FastAPI Backend (Python)  │
         │  ┌────────────────────────┐ │
         │  │  API Endpoints         │ │
         │  │  - Claims              │ │
         │  │  - Auth                │ │
         │  └────────────────────────┘ │
         │  ┌────────────────────────┐ │
         │  │  Services              │ │
         │  │  - Fact Checking       │ │
         │  │  - Claim Processing    │ │
         │  └────────────────────────┘ │
         │  ┌────────────────────────┐ │
         │  │  ML/NLP                │ │
         │  │  - Entity Extraction   │ │
         │  │  - Sentiment Analysis  │ │
         │  │  - Credibility Scoring │ │
         │  └────────────────────────┘ │
         └──────────────┬──────────────┘
                        │
         ┌──────────────┴──────────────┐
         │                             │
    ┌────▼─────┐  ┌────────┐  ┌───────▼─────┐
    │ ArangoDB │  │  XTDB  │  │  Dragonfly  │
    │ (Graph)  │  │ (Time) │  │   (Cache)   │
    └──────────┘  └────────┘  └─────────────┘
```

## Components

### 1. Frontend (React + TypeScript)

**Responsibilities:**
- User interface for claim verification
- Display of verification results
- User authentication
- Claim history visualization

**Technology:**
- React 18 with TypeScript
- Vite for build
- TailwindCSS for styling
- React Query for data fetching
- React Router for navigation

**Key Files:**
- `src/pages/` - Page components
- `src/components/` - Reusable components
- `src/services/api.ts` - API client
- `src/types/` - TypeScript types

### 2. Backend (FastAPI + Python)

**Responsibilities:**
- API endpoints
- Business logic
- ML/NLP processing
- Data persistence
- Authentication & authorization

**Technology:**
- FastAPI (async web framework)
- Pydantic for validation
- spaCy for NLP
- JWT for authentication
- Poetry for dependencies

**Architecture Layers:**

```
┌─────────────────────────────────┐
│         API Layer               │
│  - Endpoints                    │
│  - Request validation           │
│  - Response formatting          │
└────────────┬────────────────────┘
             │
┌────────────▼────────────────────┐
│       Service Layer             │
│  - Business logic               │
│  - Orchestration                │
│  - External API calls           │
└────────────┬────────────────────┘
             │
┌────────────▼────────────────────┐
│        ML/NLP Layer             │
│  - Text processing              │
│  - Entity extraction            │
│  - Sentiment analysis           │
│  - Credibility scoring          │
└────────────┬────────────────────┘
             │
┌────────────▼────────────────────┐
│         Data Layer              │
│  - ArangoDB client              │
│  - XTDB client                  │
│  - Cache manager                │
└─────────────────────────────────┘
```

### 3. Databases

#### ArangoDB (Multi-Model)

**Purpose:** Primary data store

**Collections:**
- `claims` - Claim documents
- `sources` - Source credibility data
- `fact_checks` - Fact-check results
- `users` - User accounts
- `api_keys` - API key management

**Graph:**
- `knowledge_graph` - Relationships between claims, sources, entities

**Advantages:**
- Document + Graph in one system
- Flexible schema
- AQL query language
- Built-in graph traversal

#### XTDB (Temporal)

**Purpose:** Temporal claim tracking

**Use Cases:**
- Track claim verification changes over time
- Audit trail
- Historical analysis
- Trend detection

**Advantages:**
- Immutable event log
- Time-travel queries
- Full history retention

#### Dragonfly (Cache)

**Purpose:** High-performance caching

**Cached Data:**
- Verification results
- API responses
- NLP processing results
- External API calls

**Advantages:**
- Redis-compatible
- Lower memory usage
- Better performance
- Simpler deployment

### 4. Browser Extension

**Responsibilities:**
- In-context verification on social media
- Quick claim lookup
- Background processing

**Components:**
- Background service worker
- Content scripts (injected into pages)
- Popup UI
- Options page

**Supported Platforms:**
- Twitter/X
- Facebook
- Instagram
- (Extensible to others)

## Data Flow

### Claim Verification Flow

```
1. User Input
   ↓
2. API Endpoint (/claims/verify)
   ↓
3. Claim Service
   ↓ ┌─────────────────────────────┐
   ├─→ Check cache (Dragonfly)     │
   │  └─────────────────────────────┘
   ↓
4. Create Claim (ArangoDB)
   ↓
5. NLP Processing
   ├─→ Extract entities (spaCy)
   ├─→ Analyze sentiment (TextBlob)
   └─→ Compute complexity
   ↓
6. Fact-Checking Services
   ├─→ Google Fact Check API
   ├─→ Snopes (if available)
   └─→ PolitiFact (if available)
   ↓
7. Credibility Scoring
   ├─→ Source credibility
   ├─→ Fact-check consensus
   └─→ Overall score
   ↓
8. Store Results
   ├─→ ArangoDB (current state)
   └─→ XTDB (temporal record)
   ↓
9. Cache Result (Dragonfly)
   ↓
10. Return Response
```

## Security Architecture

### Authentication

```
┌──────────────────────────────────┐
│  Client                          │
└────────┬─────────────────────────┘
         │ POST /auth/login
         │ (username, password)
         ↓
┌──────────────────────────────────┐
│  Auth Endpoint                   │
│  - Verify credentials            │
│  - Generate JWT tokens           │
└────────┬─────────────────────────┘
         │ Returns tokens
         ↓
┌──────────────────────────────────┐
│  Client stores tokens            │
└────────┬─────────────────────────┘
         │ Subsequent requests
         │ Authorization: Bearer <token>
         ↓
┌──────────────────────────────────┐
│  Protected Endpoints             │
│  - Validate token                │
│  - Extract user context          │
│  - Process request               │
└──────────────────────────────────┘
```

### Authorization Levels

1. **Public** - No authentication required
   - Health checks
   - Info endpoints

2. **Authenticated** - JWT or API key required
   - Claim verification
   - User profile

3. **Admin** - Superuser required
   - User management
   - System configuration

## Scalability

### Horizontal Scaling

**Backend:**
- Stateless design
- Multiple instances behind load balancer
- Session data in cache/database

**Databases:**
- ArangoDB: Cluster mode
- XTDB: Distributed deployment
- Dragonfly: Replication

### Caching Strategy

**Levels:**
1. Application cache (in-memory)
2. Dragonfly (distributed cache)
3. CDN (static assets)

**Cache Keys:**
```
claim:verification:{claim_id}
claim:text_hash:{hash}
fact_check:{claim_text_hash}
user:{user_id}
```

**TTL Strategy:**
- Verification results: 1 hour
- User data: 15 minutes
- Fact-check results: 6 hours

## Monitoring & Observability

### Logging

**Levels:**
- DEBUG: Development only
- INFO: Normal operations
- WARNING: Potential issues
- ERROR: Errors requiring attention
- CRITICAL: System failures

**Structured Logging:**
```json
{
  "timestamp": "2024-01-15T12:00:00",
  "level": "INFO",
  "service": "backend",
  "message": "Claim verified",
  "claim_id": "abc123",
  "duration": 1.234
}
```

### Metrics

**Application:**
- Request rate
- Response time
- Error rate
- Cache hit rate

**Business:**
- Claims verified
- Verification accuracy
- User registrations

**Infrastructure:**
- CPU usage
- Memory usage
- Disk I/O
- Network traffic

### Health Checks

**Endpoint:** `/health`

**Checks:**
- Database connectivity
- Cache connectivity
- ML model availability
- External API status

## Development Workflow

```
┌─────────────┐
│   Developer │
└──────┬──────┘
       │ git push
       ↓
┌─────────────────┐
│  GitHub         │
└──────┬──────────┘
       │ webhook
       ↓
┌─────────────────┐
│  GitHub Actions │
│  - Run tests    │
│  - Build images │
│  - Deploy       │
└──────┬──────────┘
       │
       ↓
┌─────────────────┐
│  Production     │
└─────────────────┘
```

## Technology Decisions

### Why ArangoDB?

- Multi-model (document + graph)
- Claims and sources have relationships
- Flexible schema for varied data
- Strong query language (AQL)

### Why XTDB?

- Temporal tracking crucial for fact-checking
- Immutable audit trail
- Historical analysis capabilities

### Why Dragonfly over Redis?

- Better performance
- Lower memory footprint
- Redis-compatible (drop-in replacement)
- Active development

### Why FastAPI?

- Async/await support
- Automatic API documentation
- Type checking with Pydantic
- Modern Python web framework

### Why React?

- Component-based architecture
- Large ecosystem
- TypeScript support
- Performant

## Future Improvements

1. **Message Queue** - Add Celery/RabbitMQ for background jobs
2. **Real-time Updates** - WebSocket support
3. **GraphQL API** - Alternative to REST
4. **Microservices** - Split into smaller services
5. **Service Mesh** - For advanced networking
6. **Event Sourcing** - More comprehensive event tracking
