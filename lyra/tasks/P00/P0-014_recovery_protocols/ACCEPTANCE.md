# Acceptance — P0-014 Recovery Protocols

## Acceptance Criteria
1. `RecoveryAction` defines 6 canonical actions: Log, Retry, Rollback, Isolate, Escalate, Halt.
2. `RecoveryPolicy::for_kind(kind)` returns the correct action sequence for every severity tier:
   - Critical → [Log, Isolate, Halt]
   - High     → [Log, Rollback, Escalate]
   - Medium   → [Log, Retry, Escalate]
   - Low      → [Log]
3. All policies include `Log` as the first action.
4. `RecoveryProtocol::execute(incident, timestamp)` returns:
   - `Halted` for all 4 constitutional (Critical) kinds
   - `Escalated` for all High and Medium kinds
   - `Recovered` for all Low kinds
5. `RecoveryOutcome` carries incident code, actions taken, and timestamp.
6. `RecoveryOutcome::is_halted()`, `is_escalated()`, `is_recovered()` are correct.
7. `RecoveryPolicy::halts()`, `escalates()`, `rolls_back()` are correct.

## Verification Method
- Unit tests in `k0/src/recovery/protocol.rs` (22 tests)
- Integration tests in `k0/src/recovery/mod.rs` (8 tests)
- `cargo test -p k0 --lib` — 260 passed; 0 failed

## Evidence Required
- `artifacts/recovery-traceability.md`
- `fixtures/recovery/policies.json`
- `cargo test` output: 260 passed; 0 failed
