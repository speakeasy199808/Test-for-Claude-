# P0-009 — Virtual Time

## Mission
Monotonic counter, no wall clock. Advances on events only. Causal ordering enforced. No ambient time inputs permitted.

## Scope
- `VirtualTime` — monotonic `u64` counter newtype with causal ordering
- `VirtualClock` — event-driven clock: `tick()`, `advance(n)`, `merge()`, `reset_to()`
- `TimeError` — structured error for backward-reset violations
- No `std::time::SystemTime`, no `std::time::Instant`, no wall-clock anywhere

## Primary Archetype
Core Module Implementation

## Primary Ownership Root
`k0/`

## Secondary Touched Roots
`interfaces/`, `fixtures/`, `lyra/tasks/`

## Deliverables
- `k0/src/time/mod.rs` — public API, re-exports, integration tests
- `k0/src/time/clock.rs` — `VirtualTime`, `VirtualClock`, `TimeError`
- Task control plane: README, ACCEPTANCE, DESIGN, IMPLEMENTATION, task.toml
- Fixtures: time scenario vectors
- Artifacts: time traceability
