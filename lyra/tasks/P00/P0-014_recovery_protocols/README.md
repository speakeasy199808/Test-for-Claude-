# P0-014 — Recovery Protocols

## Mission
Structured recovery procedures for every incident kind in the Lyra system. Defines the canonical action sequence to execute when an incident is detected, keyed by incident severity tier.

## Scope
- `RecoveryAction` — Log / Retry / Rollback / Isolate / Escalate / Halt
- `RecoveryPolicy` — ordered action sequence per incident kind (derived from severity)
- `RecoveryOutcome` — Recovered / Escalated / Halted
- `RecoveryProtocol` — stateless executor: incident → outcome

## Primary Archetype
Formal Specification + Type System

## Primary Ownership Root
`k0/`

## Secondary Touched Roots
`lyra/tasks/`, `fixtures/`

## Deliverables
- `k0/src/recovery/` module family (protocol.rs, mod.rs)
- 30 unit tests + integration tests
- Task control-plane files, fixtures, and traceability artifact
