<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath)
-->

# Changelog

All notable changes to `social-media-tools` will be documented in this file.

This file is generated from conventional commits by the
[`changelog-reusable.yml`](https://github.com/hyperpolymath/standards/blob/main/.github/workflows/changelog-reusable.yml)
workflow (`hyperpolymath/standards#206`). Adopt the workflow in this repo's CI to keep this file in sync automatically — see
[`templates/cliff.toml`](https://github.com/hyperpolymath/standards/blob/main/templates/cliff.toml)
for the canonical config.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/);
this project aims to follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- feat(crg): add crg-grade and crg-badge justfile recipes
- feat: wire VeriSimDB into polygraph replacing ArangoDB/Redis/XTDB
- feat: achieve CRG C — 80 tests + 14 benchmarks in nuj-collector
- feat: add stapeln.toml container definition
- feat: deploy UX Manifesto infrastructure
- feat: add CLADE.a2ml — clade taxonomy declaration
- feat: add RSR template structure and project metadata
- feat: create social-media-tools monorepo

### Fixed

- fix(ci): pin upload-artifact to valid SHA in hypatia-scan.yml (Refs standards#48) (#23)
- fix(ci): bump a2ml/k9-validate-action pins to canonical (standards#85) (#21)
- fix(ci): sync hypatia-scan.yml to canonical (kill cd-scanner build drift) (#20)
- fix(ci): build Hypatia escript from repo root (estate dogfood drift)
- fix(ci): build Hypatia escript from repo root (estate dogfood drift)
- fix(ci): build Hypatia escript from repo root (estate dogfood drift)
- fix(ci): rsr-antipattern.yml duplicate heredoc (#17)
- fix(deps): force-bump vulnerable transitive crates via [patch.crates-io] (#15)
- fix: set correct Groove capability type (was: custom)
- fix(security): use exact hostname matching instead of substring checks

### Changed

- refactor: migrate 6SCM → 6A2 (.scm → .a2ml format)

### Documentation

- docs(claude): add CLAUDE.md with TypeScript Exemptions table (#13)
- docs: substantive CRG C annotation (EXPLAINME.adoc)
- docs: add EXPLAINME.adoc — prove-it file backing README claims

### CI

- build(deps): bump openssl (#25)
- build(deps): bump the actions group with 12 updates (#26)
- ci: redistribute concurrency-cancel guard to read-only check workflows (#24)
- ci: bump actions/upload-artifact SHA to current v4 (#16)
- ci: SHA-pin hyperpolymath validate-actions in dogfood-gate

## Pre-history

Prior commits to this file's introduction are recorded in git history but not formally classified into Keep-a-Changelog sections. To backfill, run `git cliff -o CHANGELOG.md` locally using the canonical [`cliff.toml`](https://github.com/hyperpolymath/standards/blob/main/templates/cliff.toml) — this is one-shot mechanical work.

---

<!-- This file was seeded by the 2026-05-26 estate tech-debt audit follow-up (Row-2 Phase 3); see [`hyperpolymath/standards/docs/audits/2026-05-26-estate-documentation-debt.md`](https://github.com/hyperpolymath/standards/blob/main/docs/audits/2026-05-26-estate-documentation-debt.md). -->
