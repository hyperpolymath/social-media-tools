<!--
SPDX-License-Identifier: MPL-2.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# CLAUDE.md

Repository-specific guidance for AI agents working in this repo. See the org-wide standard guidance in standards/.

### TypeScript Exemptions (Approved)

The hyperpolymath "no new TypeScript" policy has the following approved exemptions in this repo. These are *not* policy violations — they are documented carve-outs.

| Path | Files | Rationale | Unblock condition |
|---|---|---|---|
| `dipstick/services/agent-swarm/src/swarm.ts` | 1 | Agent-swarm orchestration service in the dipstick polyglot pipeline; Deno-runtime concerns (workers, signals). | AffineScript stdlib for service runtime + worker bindings. |
| `dipstick/services/publisher-deno/src/publisher.ts` | 1 | Explicitly Deno-named publisher service; Deno HTTP+queue ecosystem. | AffineScript Deno-target stdlib + HTTP/queue bindings. |
| `dipstick/services/pestle-observatory/src/observatory.ts` | 1 | Observatory service in dipstick pipeline; Deno-runtime metrics + log aggregation. | AffineScript Node/Deno target (affinescript#35) + observability bindings. |

Adding to this list requires explicit user approval and an unblock condition. New TypeScript files outside this list are blocked by the RSR antipattern check.
