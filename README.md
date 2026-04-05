# Otter

Otter is a local-first coding agent harness focused on fast model orchestration, disciplined context assembly, terse interaction, and safe tool execution.

The project goal is not to become another bloated coding chat. The goal is to build a pragmatic runtime for coding agents that treats local models as first-class citizens while still allowing cloud escalation when it materially improves outcomes.

## Why this exists

Current agent harnesses often make local models feel bolted on. They also tend to combine too much surface area with too little opinionated structure. Otter aims to invert that.

Core beliefs:

- Local models should be easy to discover, load, unload, warm, and route.
- Tooling should be batteries-included and tightly scoped.
- Agent communication should be direct and terse by default.
- Context should be curated, not dumped into the prompt window.
- The runtime should be fast, observable, and safe.
- The daemon should come first; GUI shells come later.

## Initial scope

Version 0 focuses on the minimum viable harness:

- Rust workspace with a headless daemon.
- Managed local model lifecycle via `llama.cpp`.
- Optional cloud provider adapter.
- Small built-in tool set: read, search, diff, patch, test, shell.
- Session persistence and context compaction.
- Lexical and symbol-aware retrieval.
- Terse response modes.

## Non-goals for v0

These are explicitly out of scope until the core loop is working:

- Full plugin marketplace.
- Broad MCP surface area.
- GraphRAG everywhere.
- Heavy GUI-first development.
- Multi-repo orchestration.
- Fancy memory systems without clear eval wins.

## Principles

1. Local-first, not local-only.
2. Safe defaults over magical behavior.
3. Small sharp tools over sprawling extensibility.
4. Terse outputs over theatrical chain-of-thought style chatter.
5. Measure with evals before expanding scope.
6. Dogfood progressively as the runtime stabilizes.

## Planned repository shape

```text
crates/
  core/
  daemon/
  provider-llamacpp/
  provider-openai/
  context/
  retrieval/
  tools/
  policy/
  store/
  cli/
  desktop/        # later
migrations/
docs/
```

## Working docs

- Repo instructions live in the agent guidance doc.
- System design and boundaries live in the architecture doc.
- Delivery phases live in the roadmap.
- Benchmarks and task-based comparisons live in the evals doc.
- Architecture decisions live in the decisions log.
- Personality/style guidance, if used at all, stays secondary to the engineering contract.

## Current status

Scaffolding. The immediate goal is to create a clean Rust workspace, establish crate boundaries, and implement the first model-supervision path against `llama.cpp`.
