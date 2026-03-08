# Recovery Traceability — P0-014

| Acceptance Criterion | Implementation | Test | Status |
|---|---|---|---|
| 6 canonical `RecoveryAction` variants | `protocol.rs` — `RecoveryAction` enum | `action_names_are_correct` | ✅ |
| Critical → [Log, Isolate, Halt] | `protocol.rs` — `ACTIONS_CONSTITUTIONAL` static | `constitutional_policy_halts`, `constitutional_policy_includes_isolate` | ✅ |
| High → [Log, Rollback, Escalate] | `protocol.rs` — `ACTIONS_HIGH` static | `high_policy_escalates_and_rolls_back` | ✅ |
| Medium → [Log, Retry, Escalate] | `protocol.rs` — `ACTIONS_MEDIUM` static | `medium_policy_escalates_and_retries` | ✅ |
| Low → [Log] | `protocol.rs` — `ACTIONS_LOW` static | `low_policy_only_logs` | ✅ |
| All policies include Log | `protocol.rs` — all static slices start with `Log` | `all_policies_include_log` | ✅ |
| `RecoveryPolicy::halts()` correct | `protocol.rs` — `actions.contains(Halt)` | `constitutional_policy_halts` | ✅ |
| `RecoveryPolicy::escalates()` correct | `protocol.rs` — `actions.contains(Escalate)` | `high_policy_escalates_and_rolls_back` | ✅ |
| `RecoveryPolicy::rolls_back()` correct | `protocol.rs` — `actions.contains(Rollback)` | `high_policy_escalates_and_rolls_back` | ✅ |
| Constitutional incident → `Halted` outcome | `protocol.rs` — `execute()` checks `halts()` first | `constitutional_incident_produces_halted_outcome` | ✅ |
| High incident → `Escalated` outcome | `protocol.rs` — `execute()` checks `escalates()` | `high_incident_produces_escalated_outcome` | ✅ |
| Medium incident → `Escalated` outcome | `protocol.rs` — `execute()` checks `escalates()` | `medium_incident_produces_escalated_outcome` | ✅ |
| Low incident → `Recovered` outcome | `protocol.rs` — `execute()` fallthrough | `low_incident_produces_recovered_outcome` | ✅ |
| `RecoveryOutcome::incident_code()` correct | `protocol.rs` — `kind.code()` stored in outcome | `outcome_incident_code_matches` | ✅ |
| `RecoveryOutcome` timestamp stored | `protocol.rs` — timestamp field in all variants | `outcome_timestamp_is_stored` | ✅ |
| All 4 constitutional kinds halt | `mod.rs` integration test | `constitutional_incident_halts_system` | ✅ |
| All High kinds escalate | `mod.rs` integration test | `high_severity_incidents_escalate` | ✅ |
| All Medium kinds escalate | `mod.rs` integration test | `medium_severity_incidents_escalate` | ✅ |
| All Low kinds recover | `mod.rs` integration test | `low_severity_incidents_recover` | ✅ |

## Fixture Reference
`lyra/tasks/P00/P0-014_recovery_protocols/fixtures/recovery/policies.json` — 4 severity-tier policy entries with action sequences, expected outcomes, and invariant assertions.

## Test Evidence
```
cargo test -p k0 --lib
test result: ok. 260 passed; 0 failed; 0 ignored
```
Recovery-specific tests: 30 (22 in `protocol.rs`, 8 in `mod.rs`)
