# NUJ Monitor - ReScript + WASM + AI Agent Swarm Architecture

## 🏗️ System Architecture v2.0

### Pure Functional + Systems Programming Stack
**NO Python, NO TypeScript, NO Node.js**

```
┌──────────────────────────────────────────────────────────────────┐
│              GraphQL Federation Gateway (ReScript + Deno)         │
│  Type-safe, functional GraphQL with federated schema stitching   │
└────┬──────────────┬──────────────┬─────────────┬─────────────┬───┘
     │              │              │             │             │
┌────▼──────┐  ┌───▼────────┐ ┌──▼─────────┐ ┌─▼──────────┐ ┌▼────────────┐
│ Collector │  │  Analyzer  │ │ Publisher  │ │Agent Swarm│ │  PESTLE     │
│  (Rust)   │  │ (ReScript) │ │   (Deno)   │ │  (Deno)   │ │ Observatory │
│           │  │  + WASM    │ │            │ │           │ │   (Deno)    │
└───────────┘  └────────────┘ └────────────┘ └───────────┘ └─────────────┘
      │              │              │             │              │
      └──────────────┴──────────────┴─────────────┴──────────────┘
                               │
          ┌────────────────────┼────────────────────┐
          │                    │                    │
    ┌─────▼──────┐      ┌─────▼──────┐      ┌─────▼──────┐
    │ Virtuoso   │      │   XTDB     │      │ Dragonfly  │
    │ RDF Store  │      │ Temporal   │      │   Cache    │
    │  PESTLE    │      │  Audit     │      │ 25x Redis  │
    └────────────┘      └────────────┘      └────────────┘
```

### Multi-Agent Intelligence Layer

```
┌────────────────────────────────────────────────────────────┐
│                  AI Agent Swarm Coordinator                 │
│           (LangChain + OpenAI + Actor Model)               │
└────┬────────┬────────┬─────────┬──────────┬──────────┬────┘
     │        │        │         │          │          │
┌────▼─┐  ┌──▼──┐  ┌──▼───┐  ┌──▼────┐  ┌─▼────┐  ┌──▼──────┐
│Policy│  │Sever│  │Guid │  │PESTLE │  │ Fact │  │ Member  │
│Analyst  │ity  │  │ance │  │Analyst│  │Check │  │ Impact  │
│      │  │Asses│  │Write│  │       │  │  er  │  │Evaluator│
└──────┘  └─────┘  └─────┘  └───────┘  └──────┘  └─────────┘
```

### PESTLE Observatory Network

```
┌────────────────────────────────────────────────────────────┐
│            PESTLE Observatory GraphQL Federation           │
│        Monitors developing guidance & best practices       │
└────┬──────────┬──────────┬──────────┬──────────┬─────────┘
     │          │          │          │          │
┌────▼────┐ ┌──▼──────┐ ┌─▼────────┐ ┌▼────────┐ ┌▼─────────┐
│  GDPR   │ │  IFEX   │ │ Ranking  │ │Article  │ │  Index   │
│  Watch  │ │ Freedom │ │ Digital  │ │   19    │ │   on     │
│         │ │ Express │ │  Rights  │ │  Free   │ │Censorship│
└─────────┘ └─────────┘ └──────────┘ └─────────┘ └──────────┘
```

## 🎯 Technology Stack v2.0

### Languages (8 total - NO Python, NO TypeScript, NO JavaScript)

1. **ReScript** - Type-safe functional programming (analyzer, gateway)
   - Compiles to optimized JavaScript
   - Runs on Deno runtime
   - 100% type safety
   - No runtime exceptions

2. **Rust** - Systems programming (collector, WASM modules)
   - Memory safety without GC
   - Zero-cost abstractions
   - AOT compilation to WASM

3. **Deno** - Secure JavaScript runtime (all ReScript services)
   - TypeScript/ReScript native
   - Secure by default
   - Built-in tooling
   - Modern ES modules

4. **Elixir** - Concurrent real-time dashboard
   - Phoenix LiveView
   - OTP supervision trees
   - Fault tolerance

5. **Julia** - High-performance parallel scraper
   - 1000+ concurrent connections
   - Distributed computing
   - Actor-based concurrency

6. **Ada** - Safety-critical TUI
   - Compile-time verification
   - SPARK subset for proofs
   - Zero undefined behavior

7. **Clojure** - XTDB temporal adapter
   - Immutable data structures
   - STM for concurrency
   - Lisp macros

8. **CUE** - Schema validation
   - Type constraints
   - Data validation
   - Configuration unification

### WASM Optimization

**Performance-critical NLP operations compiled to WebAssembly:**

- Sentiment analysis (Levenshtein distance)
- Key term extraction (TF-IDF scoring)
- Change detection (diff algorithms)
- Text similarity (cosine similarity)

**Benefits:**
- 10-100x faster than interpreted code
- AOT compilation for instant startup
- Sandboxed execution
- Memory safety guarantees

### Databases (4 specialized systems)

1. **SurrealDB + VerisimDB** - Core multi-model data and immutable time-series
   - SurrealDB stores platforms, credentials, policy documents, and member guidance with rich JSON support
   - VerisimDB captures policy snapshots, changes, and audit records as immutable series for fast queries
   - Both systems provide SQL-like queries plus graph traversal for compliance reasoning

2. **Virtuoso RDF Triple Store** - PESTLE semantic intelligence
   - SPARQL endpoint
   - Linked data
   - Ontology reasoning
   - Graph queries

3. **XTDB Bitemporal Database** - Audit and compliance
   - Valid-time tracking
   - Transaction-time tracking
   - Point-in-time queries
   - Full history retention

4. **Dragonfly Cache** - High-performance caching
   - 25x faster than Redis
   - Redis protocol compatible
   - Multi-threaded architecture
   - Vertical scaling

## 📊 Service Architecture

### Core Services

#### 1. **Collector Service** (Rust)
**Purpose**: Platform monitoring and change detection
- **Location**: `services/collector/`
- **Port**: 3001
- **Tech**: Axum, SQLx, Tokio async runtime
- **Features**:
  - Concurrent platform collection
  - SHA256 checksumming
  - Cron-based scheduling (15 min intervals)
  - Prometheus metrics
  - Support for 7 platforms (Twitter, Meta, LinkedIn, TikTok, YouTube, Bluesky)
  - WebAssembly plugin system for custom extractors

**Why Rust?**
- Memory safety without GC pauses
- Zero-cost abstractions
- Fearless concurrency
- Can compile to WASM for browser deployment

#### 2. **Analyzer Service** (ReScript + Deno + WASM)
**Purpose**: Type-safe NLP-powered policy analysis
- **Location**: `services/analyzer-rescript/`
- **Port**: 3002
- **Tech**: ReScript, Deno runtime, Rust WASM modules
- **Features**:
  - **Functional Core**: Pure functions, no side effects
  - **WASM NLP Engine**: Rust-compiled sentiment analysis, key term extraction
  - **Severity Classification**: Pattern matching on algebraic data types
  - **Guidance Generation**: OpenAI GPT-4 integration
  - **Type Safety**: 100% compile-time guarantees

**Why ReScript?**
- Sound type system (no `any`, no `undefined`)
- Immutable by default
- Excellent JavaScript interop
- Fast compilation
- No runtime exceptions

**WASM Module** (`wasm/src/lib.rs`):
```rust
// AOT-compiled performance-critical operations
#[wasm_bindgen]
pub fn analyze_sentiment(text: &str) -> f64;

#[wasm_bindgen]
pub fn extract_key_terms(text: &str, top_n: usize) -> JsValue;

#[wasm_bindgen]
pub fn detect_changes(old_text: &str, new_text: &str) -> f64;
```

#### 3. **Publisher Service** (Deno)
**Purpose**: Email delivery with 19-layer safety guardrails
- **Location**: `services/publisher-deno/`
- **Port**: 3003
- **Tech**: Deno runtime, Nodemailer
- **Features**:
  - Type-safe guardrail checks
  - 5-minute grace period
  - Test group validation
  - Auto-rollback on failures
  - Emergency stop button
  - GDPR compliance
  - Audit logging
  - Rate limiting

**Why Deno?**
- Secure by default (no file/network access without explicit permission)
- TypeScript native
- Built-in tooling (test, lint, format)
- Modern ES modules
- Single executable deployment

#### 4. **Dashboard Service** (Elixir/Phoenix)
**Purpose**: Real-time web interface with LiveView
- **Location**: `services/dashboard/`
- **Port**: 4000
- **Tech**: Phoenix LiveView, Ecto ORM
- **Features**:
  - Real-time updates (no polling)
  - Approval workflows
  - User authentication
  - Platform management
  - WebSocket connections
  - OTP supervision for fault tolerance

**Why Elixir?**
- Distributed by design
- Fault-tolerant (OTP supervision trees)
- LiveView for real-time without JavaScript
- Scalable (millions of connections)

### AI Intelligence Services

#### 5. **Agent Swarm Coordination** (Deno)
**Purpose**: Multi-agent AI system for policy analysis
- **Location**: `services/agent-swarm/`
- **Port**: 3004
- **Tech**: Deno, LangChain, OpenAI GPT-4
- **Features**:
  - **7 Specialized Agents**:
    1. Policy Analyst - Identifies key changes
    2. Severity Assessor - Rates impact
    3. Guidance Writer - Drafts member communications
    4. PESTLE Analyst - Environmental scanning
    5. Fact Checker - Verifies claims
    6. Member Impact Evaluator - Assesses journalist implications
    7. Coordinator - Orchestrates workflow
  - **DAG Execution**: Directed Acyclic Graph workflow
  - **Actor Model**: Isolated agent state
  - **Consensus Mechanisms**: Multi-agent agreement
  - **Best Practices**: Follows AI agent coordination patterns

**Agent Coordination Pattern**:
```
Phase 1 (Parallel):     Policy Analyst + Fact Checker
Phase 2 (Sequential):   Severity Assessor → PESTLE Analyst
Phase 3 (Synthesis):    Member Impact → Guidance Writer
```

**Why Multi-Agent?**
- Specialization: Each agent masters one domain
- Parallelization: Independent tasks run concurrently
- Validation: Multiple perspectives reduce errors
- Explainability: Clear agent provenance for decisions

#### 6. **PESTLE Observatory** (Deno)
**Purpose**: External intelligence gathering via GraphQL federation
- **Location**: `services/pestle-observatory/`
- **Port**: 3005
- **Tech**: Deno, GraphQL request client
- **Features**:
  - **6 External Data Sources**:
    1. GDPR Observatory (legal, political)
    2. IFEX - Freedom of Expression (political, legal, social)
    3. Ranking Digital Rights (tech, social, legal)
    4. Article 19 (political, legal)
    5. Index on Censorship (political, social)
    6. Tech Policy Lab (tech, legal)
  - **PESTLE Framework Analysis**:
    - **Political**: Regulatory changes, policy shifts
    - **Economic**: Market impacts, cost implications
    - **Social**: Public discourse, community standards
    - **Technological**: Platform features, algorithms
    - **Legal**: Terms of service, compliance
    - **Environmental**: Digital sustainability (often N/A)
  - **GraphQL Federation**: Unified queries across sources
  - **Reliability Scoring**: Weighted by source trustworthiness
  - **Real-time Subscriptions**: Live updates from observatories
  - **Caching**: 1-hour TTL with Dragonfly

**Why PESTLE Framework?**
- Holistic analysis: Captures all external factors
- Structured thinking: Systematic consideration
- Risk identification: Early warning of changes
- Strategic planning: Informed decision-making

### API Gateway

#### 7. **GraphQL Federation Gateway** (ReScript + Deno)
**Purpose**: Unified type-safe API layer
- **Location**: `services/gateway-rescript/`
- **Port**: 8000
- **Tech**: ReScript, GraphQL Yoga, Deno
- **Features**:
  - **Federated Schema**: Stitches 5 service schemas
  - **Type Safety**: ReScript types compile to GraphQL schema
  - **Query Optimization**: Automatic batching and caching
  - **Real-time Subscriptions**: WebSocket support
  - **Service Discovery**: Auto-configuration of endpoints

**Schema**:
```graphql
type Query {
  platforms: [Platform!]!
  policyChanges(limit: Int, severity: String): [PolicyChange!]!
  guidance(policyChangeId: ID!): Guidance
  agentTasks(status: String): [AgentTask!]!
  pestleAnalysis(policyChangeId: ID!): PESTLEAnalysis
}

type Mutation {
  triggerAgentSwarm(taskType: String!, context: JSON!): ID!
  approveGuidance(guidanceId: ID!): Guidance!
  publishGuidance(guidanceId: ID!): Guidance!
}

type Subscription {
  policyChangeDetected: PolicyChange!
  agentTaskProgress(taskId: ID!): AgentTask!
}
```

### Advanced Services

#### 8. **Julia Scraper** (Julia)
**Purpose**: Massively parallel web scraping
- **Location**: `services/scraper-julia/`
- **Port**: 3006
- **Tech**: HTTP.jl, Distributed stdlib
- **Features**:
  - 1000+ concurrent connections
  - Automatic load balancing
  - Rate limiting per domain
  - Retry with exponential backoff
  - Actor-based concurrency

#### 9. **XTDB Temporal Adapter** (Clojure)
**Purpose**: Bitemporal query interface
- **Location**: `services/xtdb-temporal/`
- **Port**: 3007
- **Tech**: Clojure, XTDB client
- **Features**:
  - Point-in-time queries
  - Full audit trail
  - Valid-time vs transaction-time
  - Immutable history

#### 10. **Ada TUI** (Ada)
**Purpose**: Safe configuration interface
- **Location**: `services/tui-ada/`
- **Tech**: GNAT Ada compiler
- **Features**:
  - Type-safe configuration
  - Menu-driven interface
  - Platform management
  - Service status monitoring

## 🔧 Configuration Systems

### CUE Scripts
**Location**: `config/cue/`
**Purpose**: Schema-validated extraction rules

```cue
package twitter

#Platform: {
    name: "twitter"
    enabled: true
}

#ExtractionRules: {
    keyTerms: ["journalist", "reporter", "defamation"]
    minWordCount: 50
}
```

### Nickel Configs
**Location**: `config/nlp_tuning.ncl`
**Purpose**: Self-tuning NLP parameters

```nickel
{
  supervised = {
    models = {
      severity_classifier = {
        thresholds = {
          critical = 0.85,
          high = 0.70,
        },
      },
    },
  },
}
```

## 🚀 Data Flow

### Policy Change Detection Flow

```
1. Julia Scraper → Fetches policy pages (1000 concurrent)
2. Rust Collector → SHA256 checksum, detects changes
3. Store snapshots and policy documents in SurrealDB and stream policy snapshots through VerisimDB
4. Trigger Agent Swarm → Coordinator assigns tasks
   ├─ Policy Analyst → Extracts key changes
   ├─ Fact Checker → Validates claims
   ├─ PESTLE Observatory → Queries external sources
   ├─ Severity Assessor → Calculates impact
   ├─ Member Impact → Assesses journalist effects
   └─ Guidance Writer → Drafts communication
5. ReScript Analyzer → WASM NLP processing
6. Store in XTDB → Bitemporal audit trail
7. Elixir Dashboard → LiveView notification
8. Human Approval → Comms team reviews
9. Deno Publisher → 19-layer safety checks
10. Email Delivery → Members receive guidance
```

### PESTLE Intelligence Flow

```
1. Policy Change Detected
2. PESTLE Observatory queries 6 sources (parallel)
   ├─ GDPR Observatory → Legal/political
   ├─ IFEX → Freedom of expression
   ├─ Ranking Digital Rights → Tech/social
   ├─ Article 19 → Legal/political
   ├─ Index on Censorship → Political/social
   └─ Tech Policy Lab → Tech/legal
3. Aggregate results by domain
4. Calculate confidence (weighted by reliability)
5. Store in Virtuoso RDF triple store
6. SPARQL queries for semantic analysis
7. Feed to Agent Swarm for synthesis
```

## 📈 Performance Characteristics

### Latency Targets
- Policy scraping: <5s per platform (Julia parallelism)
- Change detection: <100ms (Rust + SHA256)
- WASM NLP: <10ms per operation (AOT compilation)
- Agent swarm coordination: <2s for full workflow
- PESTLE queries: <500ms (Dragonfly caching)
- GraphQL queries: <50ms (federation optimization)

### Throughput
- Concurrent scraping: 1000 requests/second (Julia)
- WASM operations: 100k operations/second
- Dragonfly cache: 1M ops/second
- Agent tasks: 10 concurrent swarms

### Resource Usage
- Rust services: <100MB memory, <10% CPU
- ReScript/Deno: <500MB memory, <25% CPU
- Agent Swarm: <3GB memory (LLM context), <50% CPU
- Julia Scraper: <4GB memory, 8 cores utilized

## 🔒 Safety & Security

### Type Safety Guarantees
- **ReScript**: 100% compile-time type checking, no runtime `undefined`
- **Rust**: Borrow checker prevents data races and use-after-free
- **Ada**: SPARK subset allows formal verification
- **WASM**: Sandboxed execution, memory safety

### Runtime Security
- **Deno**: Explicit permissions (--allow-net, --allow-read, --allow-env)
- **Selur/Vörðr (Chainguard runtime)**: Verified container execution with policy enforcement
- **HTTPS**: All external connections encrypted
- **Secrets**: Environment variables, never in code

### Publisher Safety Guardrails (19 Layers)
1. Human approval required
2. 5-minute grace period
3. Test group validation
4. Delivery monitoring
5. Auto-rollback on failures
6. Emergency stop button
7. Platform change validation
8. Service health checks
9. False positive detection
10. Delivery success tracking
11. Anomaly detection
12. Member data encryption
13. Access control (RBAC)
14. Audit logging (immutable)
15. GDPR compliance
16. Data retention policies
17. Rate limiting
18. Graceful degradation
19. Disaster recovery

## 🌐 Best Practices Integration

### AI Agent Best Practices
- **Specialization**: Each agent has one clear role
- **Coordination**: DAG-based workflow execution
- **Validation**: Multi-agent consensus
- **Explainability**: Clear provenance of decisions
- **Fallback**: Human override always available

### PESTLE Best Practices
- **Comprehensive**: All 6 domains considered
- **Current**: Real-time observatory connections
- **Reliable**: Weighted by source trustworthiness
- **Actionable**: Feeds directly into agent analysis

### Functional Programming Best Practices
- **Immutability**: No mutation except at system boundaries
- **Pure Functions**: No side effects in core logic
- **Type Safety**: Compile-time guarantees
- **Composition**: Small, composable functions

## 📦 Deployment

### Container Architecture
- **Selur Compose**: Container graph validated by Svalinn and executed inside Vörðr
- **Multi-stage Builds**: WASM compilation in builder stage
- **Health Checks**: All services monitored
- **Resource Limits**: CPU and memory constraints
- **Network Isolation**: Bridge network (172.20.0.0/16)

### Service Ports
- 3001: Collector (Rust)
- 3002: Analyzer (ReScript + WASM)
- 3003: Publisher (Deno)
- 3004: Agent Swarm (Deno)
- 3005: PESTLE Observatory (Deno)
- 3006: Julia Scraper
- 3007: XTDB Temporal Adapter
- 4000: Dashboard (Elixir)
- 8000: GraphQL Gateway (ReScript)
- 8890: Virtuoso SPARQL
- 6379: Dragonfly Cache
- 7070: SurrealDB HTTP/SQL endpoint
- 7071: VerisimDB ingestion stream

### Monitoring Stack
- **Prometheus**: Metrics collection
- **Grafana**: Visualization dashboards
- **Loki**: Log aggregation
- **Health Checks**: HTTP endpoints for all services

## 🎓 Why This Stack?

### No Python Rationale
- **Dynamic Typing**: Runtime errors, no compile-time safety
- **GIL Limitation**: Poor multi-threading performance
- **Memory Usage**: Heavyweight runtime
- **Replaced With**: ReScript (type safety) + WASM (performance)

### No TypeScript Rationale
- **Gradual Typing**: Can opt-out of safety with `any`
- **Runtime Errors**: Still possible despite types
- **Complexity**: Type gymnastics for advanced patterns
- **Replaced With**: ReScript (sound type system)

### No Node.js Rationale
- **Security**: Unrestricted file/network access by default
- **Package Ecosystem**: npm supply chain risks
- **Module Chaos**: CommonJS vs ES modules
- **Replaced With**: Deno (secure by default, modern)

### ReScript Advantages
- **Sound Type System**: No escape hatches, no `any`
- **Fast Compilation**: 10x faster than TypeScript
- **Excellent JavaScript Interop**: Seamless npm usage
- **Functional First**: Immutability, pattern matching
- **No Runtime Exceptions**: Compile-time guarantees

### WASM Advantages
- **Performance**: Near-native speed (10-100x faster)
- **AOT Compilation**: No JIT warmup time
- **Memory Safety**: Sandboxed execution
- **Portability**: Runs in browser or server

### Deno Advantages
- **Security**: Explicit permissions model
- **Modern**: ES modules, TypeScript native
- **Built-in Tooling**: test, lint, format, bundle
- **Single Binary**: Easy deployment
- **Backward Compatible**: Can use npm packages

## 📚 Learning Resources

### ReScript
- Official Docs: https://rescript-lang.org/
- Syntax Cheatsheet: https://rescript-lang.org/docs/manual/latest/overview

### WASM with Rust
- wasm-bindgen Book: https://rustwasm.github.io/wasm-bindgen/
- WASM Performance: https://webassembly.org/docs/use-cases/

### Deno
- Official Manual: https://deno.land/manual
- Standard Library: https://deno.land/std

### AI Agents
- LangChain Docs: https://docs.langchain.com/
- Multi-Agent Systems: https://arxiv.org/abs/2308.08155

### PESTLE Framework
- Business Analysis Body of Knowledge
- Strategic Management literature

---

**Last Updated**: 2025-11-23
**Architecture Version**: 2.0 (ReScript + WASM + AI Agents)
