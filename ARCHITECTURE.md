# ARCHITECTURE.md

This document describes the intended system architecture for Otter v0.

## System summary

Otter is a headless Rust daemon for coding-agent workflows. It manages sessions, context assembly, local/cloud model routing, safe tool execution, and streaming responses to clients such as a CLI, TUI, or future desktop shell.

## Top-level design

```text
client (CLI/TUI/Desktop)
  -> daemon
    -> providers
    -> retrieval
    -> tool runtime
    -> store
```

The daemon is the control plane. Clients should remain thin.

## Runtime actors

### 1. Client

The user-facing surface.

Examples:

- CLI.
- TUI.
- Tauri desktop shell later.

Responsibilities:

- Send user messages and commands.
- Stream status and token events.
- Display approvals and results.

### 2. Daemon

The system brain.

Responsibilities:

- Own session state.
- Assemble prompts/context.
- Apply policy.
- Route to providers.
- Supervise tool calls.
- Stream events.
- Persist summaries, notes, and transcripts.

### 3. Provider layer

Adapters for inference backends.

Planned providers:

- `provider-llamacpp`
- `provider-openai`

Responsibilities:

- Normalize model listing.
- Map internal request structs to backend APIs.
- Handle streaming responses.
- Report capabilities and health.

### 4. Model supervisor

A daemon-owned subsystem for model lifecycle.

Responsibilities:

- Discover configured models.
- Start and stop `llama.cpp` processes.
- Warm and cool models.
- Track ports, PIDs, health, and lease state.
- Avoid duplicate loads.

### 5. Retrieval

Repo-aware context retrieval.

V0 strategy:

- File tree and path heuristics.
- Ripgrep/text search.
- Symbol extraction.
- Git diff/recency awareness.
- Session notes and compact summaries.

Not in v0 by default:

- Full GraphRAG.
- Heavy vector infrastructure unless evals prove clear value.

### 6. Tools

Built-in coding tools only at first.

Planned tools:

- `read_file`
- `search_code`
- `apply_patch`
- `run_tests`
- `shell`
- `git_status`

Design rules:

- Small schemas.
- Explicit inputs.
- Safe defaults.
- Clear approvals for risky actions.

### 7. Store

Persistence for sessions and compact memory.

V0 default:

- SQLite.

Planned tables:

- sessions
- messages
- memories
- context_chunks
- model_leases
- approvals
- decisions or migration metadata later

## Request lifecycle

1. User sends message.
2. Daemon loads session state and repo context.
3. Retrieval builds a compact working set.
4. Policy selects model, tool budget, and output budget.
5. Provider streams response.
6. Tool calls are validated, approved if needed, and executed.
7. Results are folded back into context.
8. Session is compacted into summaries/notes for reuse.

## Output modes

Otter should support a few explicit response modes:

- `ack`
- `plan`
- `act`
- `result`

These are product constraints, not just prompt tricks. Different modes can carry different output budgets and allowed tools.

## Crate plan

```text
crates/
  core/              # domain types, traits, shared enums, event models
  daemon/            # orchestration, APIs, streaming, supervisor coordination
  provider-llamacpp/ # llama.cpp adapter + process integration
  provider-openai/   # cloud model adapter
  context/           # summaries, notes, compaction, prompt assembly
  retrieval/         # search, symbols, ranking, context handles
  tools/             # tool registry + built-in tools
  policy/            # approvals, routing, budgets, escalation rules
  store/             # SQLite persistence
  cli/               # command-line entrypoint
  desktop/           # future Tauri shell
```

## Technical defaults

- Runtime: `tokio`
- HTTP server: `axum`
- HTTP client: `reqwest`
- Serialization: `serde`
- Persistence: SQLite via `sqlx` or `rusqlite`
- Logging: `tracing`
- Streaming: SSE first

## Boundary rules

- Do not let UI layers talk directly to inference backends.
- Do not let provider crates own policy.
- Do not let retrieval expand into a general memory system too early.
- Do not let tools bypass approval/policy checks.
- Do not make desktop concerns drive daemon architecture.

## Open questions

- Exact symbol-indexing approach for v0.
- Whether JSON-mode/tool-mode behavior varies materially across chosen local models.
- Whether SQLite is enough for all event storage or whether some logs stay append-only on disk.
- When to add cloud escalation heuristics beyond explicit user choice.
