# P0-011 — Determinism Verifier

## Mission
Double-run nondeterminism detector. Runs computations twice with identical inputs and compares outputs byte-for-byte. Any divergence is a constitutional violation (P0-003).

## Scope
- `DeterminismVerifier` — stateful verifier with virtual clock audit log
- `verify_once` — stateless single-shot helper
- `VerificationOutcome` — Pass/Fail with hex evidence and VirtualTime timestamp
- `VerificationRecord` — audit log entry per verification event
- `VerifierError` — `DeterminismViolation` and `EmptyOutput` error variants

## Primary Archetype
Verification / Constitutional Enforcement

## Primary Ownership Root
`k0/`

## Secondary Touched Roots
`lyra/tasks/`, `fixtures/`, `docs/`

## Deliverables
- `k0/src/verifier/` module family (determinism.rs, error.rs, mod.rs)
- 25 unit tests + integration tests across codec and digest
- Task control-plane files, fixtures, and traceability artifact
