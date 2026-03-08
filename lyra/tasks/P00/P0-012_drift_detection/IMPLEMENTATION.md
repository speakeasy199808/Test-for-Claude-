# Implementation Notes — P0-012 Drift Detection

## Work Package Shape
Runtime monitoring module wrapping P0-011 verifier, with severity classification, structured reporting, and integration tests.

## Produced Components

### `k0/src/drift/error.rs`
- `DriftError::ConstitutionalDrift { label, first_hex, second_hex }` — P0-003 violation
- `DriftError::EmptyOutputDrift { label }` — operational anomaly
- `is_constitutional()` / `is_operational()` helpers

### `k0/src/drift/detector.rs`
- `DriftSeverity` — `Operational` / `Constitutional` (ordered, Constitutional > Operational)
- `DriftEvent` — `{ label, severity, first_hex, second_hex, timestamp: VirtualTime }`
- `DriftReport` — `{ events, total_checks, passed }` + query methods
- `DriftDetector` — wraps `DeterminismVerifier`, accumulates `Vec<DriftEvent>`
- `check(label, f)` — detects and classifies drift
- `check_allow_empty(label, f)` — same but empty output is not operational drift
- `report()` — returns cloneable `DriftReport` snapshot
- 22 unit tests

### `k0/src/drift/mod.rs`
- Public re-exports of all types
- 6 integration tests: SHA-3 drift-free, BLAKE3 drift-free, codec drift-free, constitutional drift detected and reported

## Ownership Placement
- Primary: `k0/src/drift/` (constitutional enforcement substrate)
- Task control plane: `lyra/tasks/P00/P0-012_drift_detection/`

## Test Evidence
```
cargo test -p k0 --lib
test result: ok. 191 passed; 0 failed; 0 ignored
```
Drift-specific tests: 28 (22 in `detector.rs`, 6 in `mod.rs`)

## Dependency Posture
- Consumes: `k0::verifier` (DeterminismVerifier, VerifierError), `k0::time` (VirtualTime)
- Enables: P0-013 (incident taxonomy), P0-023 (foundation integration)
