<!-- SPDX-License-Identifier: PMPL-1.0-or-later -->
<!-- TOPOLOGY.md — Project architecture map and completion dashboard -->
<!-- Last updated: 2026-02-19 -->

# Social Media Tools — Project Topology

## System Architecture

```
                        ┌─────────────────────────────────────────┐
                        │              SOCIAL MEDIA ANALYST       │
                        │        (Metrics HUD / Verification CLI) │
                        └───────────────────┬─────────────────────┘
                                            │ API Requests
                                            ▼
                        ┌─────────────────────────────────────────┐
                        │           SOCIAL TOOLING HUB            │
                        │                                         │
                        │  ┌───────────┐  ┌───────────────────┐  │
                        │  │ dipstick/ │  │ polygraph/        │  │
                        │  │ (Metrics) │  │ (Verification)    │  │
                        │  └─────┬─────┘  └────────┬──────────┘  │
                        └────────│─────────────────│──────────────┘
                                 │                 │
                                 ▼                 ▼
                        ┌─────────────────────────────────────────┐
                        │           EXTERNAL PLATFORMS            │
                        │  ┌───────────┐  ┌───────────┐  ┌───────┐│
                        │  │ Twitter/X │  │ BlueSky   │  │ Mastdn││
                        │  └───────────┘  └───────────┘  └───────┘│
                        └─────────────────────────────────────────┘

                        ┌─────────────────────────────────────────┐
                        │          REPO INFRASTRUCTURE            │
                        │  Justfile Automation  .machine_readable/  │
                        │  .bot_directives/     0-AI-MANIFEST.a2ml  │
                        └─────────────────────────────────────────┘
```

## Completion Dashboard

```
COMPONENT                          STATUS              NOTES
─────────────────────────────────  ──────────────────  ─────────────────────────────────
SOCIAL MODULES
  dipstick (Health Metrics)         ██████░░░░  60%    Engagement metrics stable
  polygraph (Authenticity)          ████░░░░░░  40%    Bot detection stubs active
  Platform Adapters                 ██████░░░░  60%    Mastodon/BlueSky active

REPO INFRASTRUCTURE
  Justfile Automation               ██████████ 100%    Standard tasks active
  .machine_readable/                ██████████ 100%    STATE tracking active
  0-AI-MANIFEST.a2ml                ██████████ 100%    AI entry point verified

─────────────────────────────────────────────────────────────────────────────
OVERALL:                            █████░░░░░  ~50%   Core stubs stable, ML refining
```

## Key Dependencies

```
Platform API ─────► Platform Adapter ───► dipstick Engine ───► Dashboard
     │                    │                   │
     ▼                    ▼                   ▼
User Profile ──────► Content Scan ──────► polygraph Check ──► Report
```

## Update Protocol

This file is maintained by both humans and AI agents. When updating:

1. **After completing a component**: Change its bar and percentage
2. **After adding a component**: Add a new row in the appropriate section
3. **After architectural changes**: Update the ASCII diagram
4. **Date**: Update the `Last updated` comment at the top of this file

Progress bars use: `█` (filled) and `░` (empty), 10 characters wide.
Percentages: 0%, 10%, 20%, ... 100% (in 10% increments).
