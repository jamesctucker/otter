# EVALS.md

Otter should be judged on real coding tasks, not vibes.

## Evaluation principles

- Prefer task completion over benchmark theater.
- Measure local and cloud-backed modes separately.
- Track token usage, latency, and intervention rate.
- Keep the task set stable enough to compare changes over time.
- Add tasks only when they represent real work.

## Primary scorecard

For each run, capture:

- task id
- repo
- model used
- local or cloud
- latency to first token
- total wall-clock time
- input tokens
- output tokens
- tool calls
- human interventions
- success/failure
- notes on failure mode

## Starter task set

### Repo understanding

- Find the entrypoint for a subsystem.
- Explain how a request flows through the daemon.
- Identify where model routing decisions should live.

### Debugging

- Trace a failing test to likely root cause.
- Identify why a provider request is malformed.
- Explain a session-compaction bug with evidence.

### Implementation

- Add a small field to a domain struct and thread it through.
- Implement a new read-only tool.
- Add a basic health endpoint.

### Refactoring

- Split a mixed-responsibility module into cleaner parts.
- Move policy logic out of a provider.
- Extract a shared event type into `core`.

### Safety and restraint

- Refuse risky actions without approval.
- Stay terse on simple requests.
- Avoid loading irrelevant repo context.

## Comparison baselines

Use at least these baselines:

- current OpenCode workflow
- Otter local-only
- Otter local-first with cloud fallback

## Suggested win conditions

Otter is improving if it can:

- reduce token usage materially on simple repo tasks
- keep or improve success rate on debugging tasks
- reduce manual model-lifecycle friction
- produce shorter, clearer operator-facing responses

## Run template

```text
Task:
Repo:
Commit SHA:
Model:
Provider:
Mode:

Result:
- Success:
- Time to first token:
- Total time:
- Input tokens:
- Output tokens:
- Tool calls:
- Interventions:

Notes:
- What worked:
- What failed:
- What to fix in Otter:
```

## Anti-goals

Do not claim progress based on:

- one impressive cherry-picked run
- vague feelings that the model seemed smarter
- broader scope without improved task outcomes
- adding retrieval complexity without measurable gains
