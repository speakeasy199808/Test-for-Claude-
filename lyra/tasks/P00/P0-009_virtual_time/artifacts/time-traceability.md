# Time Traceability — P0-009

| Acceptance Criterion | Implementation | Test | Status |
|---|---|---|---|
| `VirtualTime` is monotonic `u64` newtype with `Ord` | `k0/src/time/clock.rs` — `VirtualTime(u64)` with derived `Ord` | `virtual_time_ordering_is_correct`, `virtual_time_ordering` | ✅ |
| `VirtualClock::new()` starts at `ZERO` | `clock.rs` — `VirtualClock { now: VirtualTime::ZERO }` | `clock_starts_at_zero`, `new_clock_starts_at_zero` | ✅ |
| `tick()` advances by 1, returns new time | `clock.rs` — `self.now = self.now.next(); self.now` | `tick_returns_new_time`, `tick_advances_by_one`, `tick_is_monotonic` | ✅ |
| `advance(n)` adds n; n=0 is no-op | `clock.rs` — `saturating_add(n)` | `advance_adds_n`, `advance_by_n`, `advance_zero_is_noop` | ✅ |
| `merge(&other)` takes max | `clock.rs` — `if other.now > self.now { self.now = other.now }` | `merge_takes_max_when_other_larger`, `merge_keeps_self_when_larger`, `merge_equal_clocks_unchanged`, `merge_takes_max`, `merge_keeps_own_if_larger` | ✅ |
| `reset_to(t)` succeeds iff `t >= now` | `clock.rs` — returns `Err(TimeError::BackwardReset)` if `t < now` | `reset_to_forward_succeeds`, `reset_to_same_value_succeeds`, `reset_to_backward_fails`, `reset_to_advances_forward`, `reset_to_rejects_backward` | ✅ |
| No wall-clock source anywhere | No `std::time` import in `k0/src/time/` | Code review — no `SystemTime`, `Instant` | ✅ |
| `next()` saturates at `u64::MAX` | `clock.rs` — `self.0.saturating_add(1)` | `virtual_time_next_saturates_at_max` | ✅ |
| All operations deterministic | Pure functions, no ambient state | All 28 time tests deterministic | ✅ |

## Fixture Reference
`lyra/tasks/P00/P0-009_virtual_time/fixtures/time/scenarios.json` — 12 scenario vectors covering all operations and edge cases.

## Test Evidence
```
cargo test -p k0 --lib
test result: ok. 113 passed; 0 failed; 0 ignored
```
Time-specific tests: 28 (16 in `clock.rs`, 12 in `mod.rs`)
