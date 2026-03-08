# Design — P0-014 Recovery Protocols

## Constitutional Basis
Recovery protocols are the response layer for incidents classified by P0-013. The protocol is deterministic: given an incident kind, the recovery action sequence is fixed and canonical. No runtime decision-making — the policy is a pure function of severity.

## Architecture

### Module Layout
```
k0/src/recovery/
├── mod.rs        — public API, re-exports, integration tests
└── protocol.rs   — RecoveryAction, RecoveryPolicy, RecoveryOutcome, RecoveryProtocol
```

### Layering
```
RecoveryProtocol (P0-014)
    └── RecoveryPolicy (severity → action sequence)
            └── IncidentKind.severity() (P0-013)
```

### Core Types

**`RecoveryAction`** — 6 canonical actions
- `Log`      — always first; audit trail
- `Retry`    — attempt operation again (medium tier)
- `Rollback` — revert to last known-good state (high tier)
- `Isolate`  — quarantine affected subsystem (constitutional tier)
- `Escalate` — propagate to higher tier or operator
- `Halt`     — stop system execution (constitutional violations only)

**`RecoveryPolicy`** — static action sequences per severity tier
```
Critical → [Log, Isolate, Halt]
High     → [Log, Rollback, Escalate]
Medium   → [Log, Retry, Escalate]
Low      → [Log]
```
- `for_kind(kind)` — canonical factory
- `halts()`, `escalates()`, `rolls_back()` — fast-path queries

**`RecoveryOutcome`** — result of executing a protocol
- `Recovered { incident_code, actions_taken, timestamp }` — system continues
- `Escalated { incident_code, actions_taken, timestamp }` — propagated upward
- `Halted { incident_code, actions_taken, timestamp }` — system stopped

**`RecoveryProtocol`** — stateless executor
- `execute(incident, timestamp)` — looks up policy, returns outcome

## Design Decisions
1. **Static action slices** — `&'static [RecoveryAction]` avoids allocation; policies are compile-time constants
2. **Severity-keyed** — policy is derived from severity, not from individual kind; this keeps the policy table small and consistent
3. **Halt is terminal** — if a policy halts, the outcome is always `Halted` regardless of other actions
4. **Escalate before Recover** — if a policy escalates but does not halt, outcome is `Escalated`
5. **Log is universal** — every policy includes `Log` as the first action; no incident goes unrecorded
6. **Stateless executor** — `RecoveryProtocol` has no state; it is a pure function of incident + timestamp
