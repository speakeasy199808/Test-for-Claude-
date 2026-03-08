# Incident Traceability — P0-013

| Acceptance Criterion | Implementation | Test | Status |
|---|---|---|---|
| 14 canonical kinds with unique INC-NNN codes | `taxonomy.rs` — `IncidentKind` enum + `code()` | `incident_codes_are_unique` | ✅ |
| Severity ordering: Critical > High > Medium > Low | `taxonomy.rs` — `#[derive(PartialOrd, Ord)]` on `IncidentSeverity` | `severity_ordering` | ✅ |
| DeterminismViolation → Critical (INC-001) | `taxonomy.rs` — `severity()` match arm | `determinism_violation_is_critical` | ✅ |
| ConstitutionalBreach → Critical (INC-002) | `taxonomy.rs` — `severity()` match arm | `constitutional_breach_is_critical` | ✅ |
| TrustRootViolation → Critical (INC-003) | `taxonomy.rs` — `severity()` match arm | `trust_root_violation_is_critical` | ✅ |
| DigestMismatch → Critical (INC-004) | `taxonomy.rs` — `severity()` match arm | `digest_mismatch_is_critical` | ✅ |
| EncodingError → High (INC-005) | `taxonomy.rs` — `severity()` match arm | `encoding_error_is_high` | ✅ |
| EntropyAnomaly → Medium (INC-009) | `taxonomy.rs` — `severity()` match arm | `entropy_anomaly_is_medium` | ✅ |
| RecoverableError → Low (INC-013) | `taxonomy.rs` — `severity()` match arm | `recoverable_error_is_low` | ✅ |
| `is_constitutional()` true only for Critical | `taxonomy.rs` — delegates to `severity().is_constitutional()` | `severity_is_constitutional_only_for_critical` | ✅ |
| `constitutional_kinds()` returns exactly 4 Critical kinds | `taxonomy.rs` — static slice | `constitutional_kinds_count`, `constitutional_kinds_are_all_critical` | ✅ |
| Descriptions non-empty for all kinds | `taxonomy.rs` — `description()` match | `descriptions_are_non_empty` | ✅ |
| `Incident::new` derives severity from kind | `record.rs` — `kind.severity()` in constructor | `new_derives_severity_from_kind` | ✅ |
| `Incident::is_constitutional()` correct | `record.rs` — delegates to `kind.is_constitutional()` | `is_constitutional_true_for_critical`, `is_constitutional_false_for_non_critical` | ✅ |
| `Incident` Display includes code, severity, timestamp, label | `record.rs` — `Display` impl | `display_includes_code_severity_timestamp_label` | ✅ |
| Integration: all constitutional kinds are Critical | `mod.rs` — `all_constitutional_kinds_are_critical` | `all_constitutional_kinds_are_critical` | ✅ |
| Integration: Incident from DeterminismViolation | `mod.rs` — `incident_from_determinism_violation` | `incident_from_determinism_violation` | ✅ |

## Fixture Reference
`lyra/tasks/P00/P0-013_incident_taxonomy/fixtures/incident/taxonomy.json` — 14 kind entries with codes, severities, constitutional flags, and severity ordering.

## Test Evidence
```
cargo test -p k0 --lib
test result: ok. 230 passed; 0 failed; 0 ignored
```
Incident-specific tests: 39 (21 in `taxonomy.rs`, 10 in `record.rs`, 8 in `mod.rs`)
