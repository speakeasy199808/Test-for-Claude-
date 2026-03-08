# P0-013 — Incident Taxonomy

## Mission
Canonical classification of all Lyra system incidents by kind and severity. Provides the authoritative taxonomy that all subsystems use to classify failures, violations, and anomalies.

## Scope
- `IncidentKind` — 14 canonical incident kinds with codes INC-000 through INC-013
- `IncidentSeverity` — Critical / High / Medium / Low (ordered)
- `Incident` — structured incident record with kind, severity, label, timestamp, context
- Constitutional kinds: DeterminismViolation, ConstitutionalBreach, TrustRootViolation, DigestMismatch

## Primary Archetype
Formal Specification + Type System

## Primary Ownership Root
`k0/`

## Secondary Touched Roots
`lyra/tasks/`, `fixtures/`

## Deliverables
- `k0/src/incident/` module family (taxonomy.rs, record.rs, mod.rs)
- 39 unit tests + integration tests
- Task control-plane files, fixtures, and traceability artifact
