# Drift Traceability — P0-012

| Acceptance Criterion | Implementation | Test | Status |
|---|---|---|---|
| `check(label, f)` detects constitutional drift | `detector.rs` — wraps verifier, classifies `DeterminismViolation` | `nondeterministic_check_fails_with_constitutional_drift` | ✅ |
| `check(label, f)` detects operational drift (empty) | `detector.rs` — classifies `EmptyOutput` | `empty_output_produces_operational_drift` | ✅ |
| Clean check → `Ok(output)`, no event | `detector.rs` — pass-through on `Ok` | `clean_check_passes` | ✅ |
| `check_allow_empty` permits empty output | `detector.rs` — routes to `verify_allow_empty` | `check_allow_empty_passes_for_empty` | ✅ |
| `DriftReport.total_checks` correct | `detector.rs` — incremented on every `check()` call | `report_is_clean_when_no_drift`, `report_counts_constitutional_drift` | ✅ |
| `DriftReport.passed` correct | `detector.rs` — `total_checks - events.len()` | `report_is_clean_when_no_drift`, `report_counts_constitutional_drift` | ✅ |
| `constitutional_count()` correct | `detector.rs` — filter by severity | `report_counts_constitutional_drift` | ✅ |
| `operational_count()` correct | `detector.rs` — filter by severity | `report_counts_operational_drift` | ✅ |
| `is_clean()` true when no drift | `detector.rs` — `events.is_empty()` | `is_clean_true_when_no_drift` | ✅ |
| `is_clean()` false after drift | `detector.rs` | `is_clean_false_after_drift` | ✅ |
| `has_constitutional_drift()` true after violation | `detector.rs` | `has_constitutional_drift_true_after_violation` | ✅ |
| `has_constitutional_drift()` false for operational only | `detector.rs` | `has_constitutional_drift_false_for_operational_only` | ✅ |
| `DriftSeverity::Constitutional > Operational` | `detector.rs` — `#[derive(PartialOrd, Ord)]` | `drift_severity_ordering` | ✅ |
| SHA-3 and BLAKE3 are drift-free | `mod.rs` integration test | `sha3_digest_is_drift_free` | ✅ |
| Codec encoder is drift-free | `mod.rs` integration test | `codec_encoder_is_drift_free` | ✅ |
| Constitutional drift detected and reported | `mod.rs` integration test | `constitutional_drift_detected_and_reported` | ✅ |

## Fixture Reference
`lyra/tasks/P00/P0-012_drift_detection/fixtures/drift/scenarios.json` — 7 scenario vectors covering clean, constitutional, operational, allow-empty, mixed session, SHA-3, and codec.

## Test Evidence
```
cargo test -p k0 --lib
test result: ok. 191 passed; 0 failed; 0 ignored
```
Drift-specific tests: 28 (22 in `detector.rs`, 6 in `mod.rs`)
