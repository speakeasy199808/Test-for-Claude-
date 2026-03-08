# P0-012 — Drift Detection

## Mission
Runtime nondeterminism monitoring layer. Wraps the Determinism Verifier (P0-011) and adds drift event classification by severity, structured reporting, and a clean API for runtime monitoring of the constitutional determinism invariant.

## Scope
- `DriftDetector` — stateful runtime monitor wrapping `DeterminismVerifier`
- `DriftSeverity` — `Constitutional` (P0-003 violation) / `Operational` (suspicious)
- `DriftEvent` — classified drift event with label, severity, hex evidence, timestamp
- `DriftReport` — structured summary: total checks, passed, drift counts by severity
- `DriftError` — `ConstitutionalDrift` / `EmptyOutputDrift` error variants

## Primary Archetype
Verification / Runtime Monitoring

## Primary Ownership Root
`k0/`

## Secondary Touched Roots
`lyra/tasks/`, `fixtures/`

## Deliverables
- `k0/src/drift/` module family (detector.rs, error.rs, mod.rs)
- 28 unit tests + integration tests against codec and digest
- Task control-plane files, fixtures, and traceability artifact
