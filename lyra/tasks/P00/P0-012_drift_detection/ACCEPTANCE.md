# Acceptance — P0-012 Drift Detection

## Acceptance Criteria
1. `DriftDetector::check(label, f)` detects constitutional drift (divergent outputs) and operational drift (empty output).
2. Constitutional drift → `Err(DriftError::ConstitutionalDrift)` + event recorded with `DriftSeverity::Constitutional`.
3. Operational drift → `Err(DriftError::EmptyOutputDrift)` + event recorded with `DriftSeverity::Operational`.
4. Clean checks → `Ok(output)`, no event recorded.
5. `DriftReport` correctly counts: `total_checks`, `passed`, `constitutional_count()`, `operational_count()`.
6. `is_clean()` and `has_constitutional_drift()` reflect report state correctly.
7. `check_allow_empty` permits empty outputs without operational drift.
8. Integration tests confirm SHA-3, BLAKE3, and codec encoder are drift-free.

## Verification Method
- Unit tests in `k0/src/drift/detector.rs` (22 tests)
- Integration tests in `k0/src/drift/mod.rs` (6 tests)
- `cargo test -p k0 --lib` — all 191 pass

## Evidence Required
- `artifacts/drift-traceability.md`
- `fixtures/drift/scenarios.json`
- `cargo test` output: 191 passed; 0 failed
