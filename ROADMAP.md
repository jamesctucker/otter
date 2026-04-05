# ROADMAP.md

This roadmap is intentionally narrow. The goal is to reach a working, dogfoodable harness before expanding scope.

## Phase 0 - Foundation

Goal: establish the repo, workspace shape, and architectural contract.

Deliverables:

- Initial Rust workspace.
- Core docs.
- Crate scaffolding.
- Initial domain types.
- CI-quality local commands defined.

Exit criteria:

- The workspace compiles.
- Crate boundaries are clear.
- The docs are good enough that a coding agent can start from them without improvising the product.

## Phase 1 - Local model control

Goal: make local models first-class.

Deliverables:

- `provider-llamacpp`
- model registry/config
- process supervisor
- model load/unload/warm/cool flows
- health checks and lease state
- streaming completions through the daemon

Exit criteria:

- A user can select a configured local model and use it through the daemon.
- The daemon handles readiness and cancellation cleanly.
- Lifecycle UX is already better than the current manual setup.

## Phase 2 - Sessions and context

Goal: make responses coherent and cheap.

Deliverables:

- persistent sessions
- transcript storage
- context handles
- summaries/notes/compaction
- terse output modes
- simple routing policy

Exit criteria:

- Long sessions do not bloat uncontrollably.
- The same repo can be worked on across sessions with usable continuity.
- Output style is tighter and cheaper than baseline tooling.

## Phase 3 - Built-in coding tools

Goal: enable safe coding workflows.

Deliverables:

- read/search tools
- patch application
- test execution
- shell access with policy gates
- approval UX

Exit criteria:

- Otter can inspect code, propose changes, and run targeted validation safely.
- Risky actions are gated.
- Tool traces are observable.

## Phase 4 - Retrieval quality

Goal: improve repo understanding without overbuilding.

Deliverables:

- lexical retrieval
- symbol-aware retrieval
- ranking heuristics
- git-aware signals
- targeted repo summaries

Exit criteria:

- Repo question answering beats naive transcript stuffing.
- Multi-file debugging and navigation are noticeably stronger.
- Graph-heavy approaches are still optional, not mandatory.

## Phase 5 - Dogfooding default

Goal: use Otter to build Otter.

Deliverables:

- self-hosted day-to-day workflow for this repo
- eval comparisons against existing tools
- fallback policies for hard tasks
- clear feedback loop from real use

Exit criteria:

- Otter is the default tool for its own repo.
- Existing tools become fallback rather than primary.

## Deferred until proven necessary

- GUI-first product work.
- broad plugin/MCP surface area.
- GraphRAG as a default architecture choice.
- multi-agent swarms.
- broad multi-repo memory.
- elaborate personality systems.
