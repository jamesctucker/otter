# DECISIONS.md

This file records important architectural decisions so they do not get re-litigated every session.

Use a lightweight ADR style.

***

## ADR-0001: Headless daemon first

Status: Accepted

Context:

The system needs strong control over sessions, model lifecycle, policy, approvals, streaming, and tool execution. A thin UI can sit on top later.

Decision:

Build the daemon and CLI/TUI path before a desktop shell.

Consequences:

- Better architectural discipline.
- Easier dogfooding in a terminal.
- Tauri remains an option, not a forcing function.

***

## ADR-0002: Local-first, not local-only

Status: Accepted

Context:

Local models are central to the product idea, but cloud escalation may still be useful for selected tasks.

Decision:

Design the provider abstraction to support both local and cloud backends from the start, while optimizing first for local lifecycle quality.

Consequences:

- More flexible routing.
- Slightly more abstraction early.
- Better long-term product fit.

***

## ADR-0003: Terse response modes are a product feature

Status: Accepted

Context:

Many agent harnesses allow wasteful verbosity, especially with local models.

Decision:

Bake explicit modes like `ack`, `plan`, `act`, and `result` into the runtime and policy layer.

Consequences:

- Lower token waste.
- Cleaner operator UX.
- Slightly more implementation complexity than a raw chat loop.

***

## ADR-0004: Retrieval before GraphRAG

Status: Accepted

Context:

Repo understanding needs improvement, but graph-heavy solutions add scope quickly.

Decision:

Start with lexical retrieval, symbol awareness, git signals, summaries, and ranking heuristics before considering GraphRAG.

Consequences:

- Faster path to value.
- Lower complexity.
- Clearer basis for evaluating whether graph methods are actually needed.
