# Acceptance — P0-009 Virtual Time

## Acceptance Criteria

1. `VirtualTime` is a monotonic `u64` newtype with `Ord`, `PartialOrd`, `Eq`, `Hash`.
2. `VirtualClock::new()` starts at `VirtualTime::ZERO`.
3. `VirtualClock::tick()` advances by exactly one and returns the new time.
4. `VirtualClock::advance(n)` advances by `n` ticks; `n=0` is a no-op.
5. `VirtualClock::merge(&other)` sets `self.now = max(self.now, other.now)`.
6. `VirtualClock::reset_to(t)` succeeds iff `t >= self.now()`; rejects backward resets with `TimeError::BackwardReset`.
7. No `std::time::SystemTime`, `std::time::Instant`, or any wall-clock source appears anywhere in `k0/src/time/`.
8. `VirtualTime::next()` saturates at `u64::MAX` rather than panicking.
9. All operations are deterministic: identical call sequences produce identical results.

## Verification Method
- Unit tests in `k0/src/time/` covering all operations, edge cases, and error paths
- `cargo test -p k0 --lib` passes with 0 failures

## Evidence Required
- `artifacts/time-traceability.md`
- `fixtures/time/scenarios.json`
- `cargo test -p k0 --lib` output: all time tests pass (28 time tests)
