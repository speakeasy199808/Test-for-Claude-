
# LyraLang Stage 0 Structured Concurrency

## 1. Scope

P1-011 introduces the first executable structured-concurrency law for Stage 0.
The surface is intentionally conservative and is expressed through builtin call
forms that fit the current parser:

- `spawn(expr)`
- `join(task)`
- `select(task_a, task_b)`
- `channel_int()`
- `send_int(channel, value)`
- `recv_int(channel)`

This slice establishes deterministic scheduling, typed channels, and a static
no-race policy without assuming later function declaration or actor syntax.

## 2. Canonical Internal Types

The shared type kernel now includes:

- `Task[T]`
- `Channel[T]`

Stage 0 currently materializes the channel surface for `Int` payloads through
the executable builtin trio `channel_int`, `send_int`, and `recv_int`.

## 3. Structured Concurrency Law

Stage 0 concurrency is lexically structured.

- `spawn(expr)` introduces a task handle in source order.
- `join(task)` waits on one task handle.
- `select(task_a, task_b)` waits on two task handles and deterministically
  chooses the leftmost ready candidate.

Detached tasks are not part of this slice.

## 4. Deterministic Scheduling Specification

The executable scheduler summary is:

- spawn sites are numbered in lexical source order
- joins are processed in lexical source order
- `select` uses the stable rule `leftmost_ready_candidate`
- channel operations are reported in source order

This is a specification artifact for the compiler pipeline, not a runtime
scheduler implementation.

## 5. No Data Races by Construction

Stage 0 currently guarantees race freedom by policy:

- spawned expressions may not capture linear resources such as `File`,
  `Socket`, or `Capability`
- communication across tasks is modeled through explicit channel operations
- there is no shared mutable variable surface in the current parser subset

The concurrency checker rejects any spawn site that captures a linear resource.

## 6. Channel Types

Channel operations are type-directed.

- `channel_int() -> Channel[Int]`
- `send_int(Channel[Int], Int) -> Unit`
- `recv_int(Channel[Int]) -> Int`

More payload kinds can be added as later syntax and type tasks expand the
language surface.
