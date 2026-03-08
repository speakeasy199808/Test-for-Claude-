# Acceptance — P0-013 Incident Taxonomy

## Acceptance Criteria
1. `IncidentKind` defines 14 canonical kinds with unique codes INC-000 through INC-013.
2. `IncidentSeverity` has 4 levels: Critical > High > Medium > Low (ordered).
3. Constitutional kinds (DeterminismViolation, ConstitutionalBreach, TrustRootViolation, DigestMismatch) all map to `Critical`.
4. `IncidentKind::severity()` returns the correct canonical severity for every kind.
5. `IncidentKind::code()` returns unique codes for all 14 kinds.
6. `IncidentKind::description()` returns non-empty descriptions for all kinds.
7. `IncidentKind::constitutional_kinds()` returns exactly the 4 Critical kinds.
8. `Incident::new(kind, label, timestamp, context)` derives severity from kind automatically.
9. `Incident::is_constitutional()` returns true iff severity is Critical.
10. `Incident` Display format includes code, severity, timestamp, and label.

## Verification Method
- Unit tests in `k0/src/incident/taxonomy.rs` (21 tests)
- Unit tests in `k0/src/incident/record.rs` (10 tests)
- Integration tests in `k0/src/incident/mod.rs` (8 tests)
- `cargo test -p k0 --lib` — 230 passed; 0 failed

## Evidence Required
- `artifacts/incident-traceability.md`
- `fixtures/incident/taxonomy.json`
- `cargo test` output: 230 passed; 0 failed
