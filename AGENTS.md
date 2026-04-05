# AGENTS.md

This file tells coding agents and collaborators how to work inside the Otter repository.

## Mission

Build a local-first coding agent harness in Rust that makes local models operationally excellent: easy to manage, cheap to run, terse in communication, and reliable in tool execution.

## Current phase

Scaffolding and foundation work only.

Priorities, in order:

1. Create a clean Rust workspace.
2. Establish crate boundaries and domain types.
3. Implement local model supervision for `llama.cpp`.
4. Add persistent sessions and streaming.
5. Add a minimal safe built-in tool set.

Do not introduce broad plugin systems, advanced memory systems, or large GUI work in this phase unless explicitly requested.

## Product intent

Otter is not trying to be a generic chat UI. It is a runtime for coding agents.

The core experience should feel:

- Local-first.
- Fast.
- Terse.
- Safe.
- Observable.
- Opinionated.

## Working style

When making changes in this repo:

- Prefer boring, idiomatic Rust over clever abstractions.
- Keep modules small and responsibilities explicit.
- Preserve a headless-daemon-first architecture.
- Optimize for operational clarity before extensibility.
- Add comments sparingly; prefer clear naming and type design.
- Avoid introducing dependencies unless they clearly reduce complexity.
- Keep agent/user-facing text brief and direct.

## Response style for agents

Default to terse, high-signal communication.

Good:

- "Found likely bug in session compaction. Preparing patch + test."
- "`provider-llamacpp` should own HTTP mapping; supervisor should own process lifecycle."

Avoid:

- Long motivational text.
- Repeating the obvious.
- Explaining Rust basics unless asked.
- Producing speculative architecture changes without tying them to the current roadmap.

## Architecture guardrails

Maintain these boundaries unless there is a strong reason to change them:

- `core` contains domain types and traits, not runtime orchestration.
- `daemon` owns orchestration, session flow, policy enforcement, and streaming.
- Provider crates adapt external inference systems.
- `tools` contains built-in tool implementations and schemas.
- `retrieval` handles repo search/index/retrieval logic.
- `store` owns persistence details.
- `desktop` is optional and comes after the daemon/CLI path is solid.

## Safe defaults

Assume these defaults unless a task says otherwise:

- One active repo context at a time.
- SQLite for persistence.
- `tokio` + `axum` + `reqwest` for async/server/networking.
- `serde` for serialization.
- SSE or equivalent simple streaming before more complex transport layers.
- Lexical + symbol-aware retrieval before graph-heavy approaches.

## Commands

Use these commands once the workspace is scaffolded:

```bash
cargo fmt --all
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

If adding a new crate, ensure it is wired into the workspace and included in checks.

## Editing rules

- Update docs when architecture or scope changes.
- Do not silently change crate boundaries.
- Capture irreversible choices in the decisions log.
- Avoid placeholder implementations that hide unfinished behavior.
- Prefer explicit `TODO:` notes over pretending work is complete.

## Approval rules

Require explicit approval before:

- Adding major dependencies.
- Reworking top-level crate boundaries.
- Introducing a GUI-first workflow.
- Adding plugin/MCP systems.
- Introducing GraphRAG or vector infrastructure.
- Making destructive shell/file changes outside the repo.

## Definition of done

A change is done when:

- The code compiles.
- Relevant tests pass.
- The change fits the current roadmap phase.
- Docs are updated if architecture, commands, or behavior changed.
- The implementation is narrower and clearer than the easiest sloppy version.

## First files to read

Before major work, read in this order:

1. `README.md`
2. `AGENTS.md`
3. `ARCHITECTURE.md`
4. `ROADMAP.md`
5. `EVALS.md`
6. `DECISIONS.md`
