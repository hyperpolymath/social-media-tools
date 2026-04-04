# Test & Benchmark Requirements

## CRG Grade: C — ACHIEVED 2026-04-04

80 tests passing (`cargo test`, 0 failures) + 14 benchmarks compiled. All CRG C categories met for `dipstick/services/collector`.

## Current State (UPDATED 2026-04-04)

### dipstick/services/collector (nuj-collector)

- Unit tests: 12 tests (COMPLETE)
  - tests/unit_tests.rs: model creation, serialization, checksum, word count
- Property-based tests: 16+ tests (COMPLETE)
  - tests/property_tests.rs: proptest invariants for models and checksums
- Reflexive tests: 15+ tests (COMPLETE)
  - tests/reflexive_tests.rs: round-trip serialization, model reconstruction
- Contract tests: 25+ tests (COMPLETE)
  - tests/contract_tests.rs: API contract and invariant enforcement
- Aspect tests: 12+ tests (COMPLETE)
  - tests/aspect_tests.rs: security, correctness, performance aspects
- Benchmarks: 14 benchmarks (COMPLETE)
  - benches/collector_bench.rs: Criterion benchmarks for models and checksums

## Test Categories (CRG C)

| Category | Count | Status |
|----------|-------|--------|
| Unit | 12 | COMPLETE |
| Property-based (P2P) | 16+ | COMPLETE |
| Reflexive | 15+ | COMPLETE |
| Contract | 25+ | COMPLETE |
| Aspect | 12+ | COMPLETE |
| Benchmarks | 14 | COMPLETE |
| **Total** | **80+** | **ALL PASS** |

## Remaining Work

### Other services (not yet at CRG C)
- [ ] dipstick/services/analyzer-rescript — ReScript, no tests
- [ ] dipstick/services/gateway-rescript — ReScript, no tests
- [ ] dipstick/services/dashboard — no tests
- [ ] dipstick/services/publisher-deno — Deno, no tests
- [ ] dipstick/services/scraper-julia — Julia, no tests
- [ ] polygraph/ — frontend, no unit tests

### Priority
- **MEDIUM** — Additional services need test coverage, but core collector service at CRG C.
