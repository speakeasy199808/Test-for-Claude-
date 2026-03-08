# Design — P0-009 Virtual Time

## Architecture

Virtual time is a pure Rust module at `k0/src/time/`. It has no I/O, no wall-clock access, and no ambient nondeterminism. Time is a monotonic counter that advances only when callers explicitly request it.

### Module Decomposition

| Module | Responsibility |
|---|---|
| `mod.rs` | Public API, re-exports, integration-level tests |
| `clock.rs` | `VirtualTime`, `VirtualClock`, `TimeError` |

### Type Design

**`VirtualTime(u64)`** — newtype over `u64`.
- Derives `Copy`, `Clone`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`, `Debug`.
- `ZERO` constant for the origin.
- `new(u64)` constructor.
- `as_u64()` accessor.
- `next()` — saturating increment (no panic at `u64::MAX`).
- `Display` — formats as `t:{value}`.

**`VirtualClock`** — holds a single `VirtualTime` field `now`.
- `new()` / `Default` — starts at `VirtualTime::ZERO`.
- `now()` — returns current time without advancing.
- `tick()` — advances by 1, returns new time. Primary event-driven operation.
- `advance(n: u64)` — advances by `n` (saturating). `n=0` is no-op.
- `reset_to(t: VirtualTime) -> Result<(), TimeError>` — sets clock to `t` iff `t >= now`. Used for replay/restore.
- `merge(&other)` — sets `now = max(now, other.now)`. Causal merge for distributed clocks.

**`TimeError`** — `thiserror`-derived enum.
- `BackwardReset { current: u64, requested: u64 }` — only variant.

### Causal Ordering Invariant

The clock satisfies: for any sequence of operations, `now` is non-decreasing. This is enforced by:
1. `tick()` and `advance()` only add to `now`.
2. `reset_to()` rejects `t < now` with `TimeError::BackwardReset`.
3. `merge()` takes the maximum.

### Wall-Clock Prohibition

The module contains no imports of `std::time`. This is enforced by code review and the absence of any `SystemTime` or `Instant` usage. The `#![forbid(unsafe_code)]` and `#![deny(missing_docs)]` attributes from `k0/src/lib.rs` apply to this module.

### No Ambient Time

Callers must explicitly call `tick()` or `advance()` to move time forward. There is no background thread, no timer, and no OS time query anywhere in this module.

## Dependency Posture

- No dependencies beyond `thiserror` (already in workspace)
- Consumed by: P0-010 (entropy uses virtual time for seeding), P0-011 (determinism verifier timestamps events), P0-012 (drift detection timestamps observations), P0-023 (foundation integration)
