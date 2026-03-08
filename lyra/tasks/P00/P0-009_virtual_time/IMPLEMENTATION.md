# Implementation Notes — P0-009 Virtual Time

## Work Package Shape
Two-module Rust implementation in `k0/src/time/` with task-local control plane, fixtures, and artifacts.

## Produced Components

| File | Description |
|---|---|
| `k0/src/time/mod.rs` | Public API: re-exports `VirtualClock`, `VirtualTime`, `TimeError`; integration tests |
| `k0/src/time/clock.rs` | `VirtualTime` newtype, `VirtualClock` struct, `TimeError` enum |

## Test Coverage

- **clock.rs unit tests (16):** `virtual_time_zero_is_zero`, `virtual_time_new_stores_value`, `virtual_time_next_increments`, `virtual_time_next_saturates_at_max`, `virtual_time_ordering_is_correct`, `clock_starts_at_zero`, `tick_returns_new_time`, `advance_adds_n`, `advance_zero_is_noop`, `reset_to_forward_succeeds`, `reset_to_same_value_succeeds`, `reset_to_backward_fails`, `merge_takes_max_when_other_larger`, `merge_keeps_self_when_larger`, `merge_equal_clocks_unchanged`, `default_clock_starts_at_zero`, `virtual_time_display`
- **mod.rs integration tests (12):** `new_clock_starts_at_zero`, `tick_advances_by_one`, `tick_is_monotonic`, `advance_by_n`, `merge_takes_max`, `merge_keeps_own_if_larger`, `virtual_time_ordering`, `reset_to_advances_forward`, `reset_to_rejects_backward`, `advance_zero_is_noop`, `virtual_time_as_u64`

Total: 28 time tests

## Ownership Placement
- Production code: `k0/src/time/` (primary ownership root `k0/`)
- Task control plane: `lyra/tasks/P00/P0-009_virtual_time/`

## Dependency Posture
- No new crate dependencies (`thiserror` already in workspace)
- Enables: P0-010 (entropy seeding), P0-011 (determinism verifier), P0-012 (drift detection)

## Wall-Clock Prohibition Verification
`grep -r "SystemTime\|Instant\|std::time" k0/src/time/` returns no matches.
