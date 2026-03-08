# Implementation Notes — P0-011 Determinism Verifier

## Work Package Shape
Verification module with stateful verifier, stateless helper, audit log, and integration tests against codec and digest.

## Produced Components

### `k0/src/verifier/error.rs`
- `VerifierError::DeterminismViolation { label, first_hex, second_hex }` — constitutional violation
- `VerifierError::EmptyOutput { label }` — suspicious empty output guard

### `k0/src/verifier/determinism.rs`
- `VerificationOutcome` — `Pass { output_hex, timestamp }` / `Fail { first_hex, second_hex, timestamp }`
- `VerificationRecord` — `{ label, outcome }`
- `DeterminismVerifier` — stateful verifier with `VirtualClock` and `Vec<VerificationRecord>` audit log
- `verify_once` — stateless helper
- `hex_encode` — private byte→hex utility
- 21 unit tests

### `k0/src/verifier/mod.rs`
- Public re-exports of all types
- 6 integration tests: codec encoder determinism (8 value types), digest algorithm determinism (4 cases), error/outcome accessibility

## Ownership Placement
- Primary: `k0/src/verifier/` (constitutional enforcement substrate)
- Task control plane: `lyra/tasks/P00/P0-011_determinism_verifier/`

## Test Evidence
```
cargo test -p k0 --lib
test result: ok. 163 passed; 0 failed; 0 ignored
```
Verifier-specific tests: 27 (21 in determinism.rs, 6 in mod.rs)

## Dependency Posture
- Consumes: `k0::time` (VirtualClock, VirtualTime), `k0::codec` (encode, Value), `k0::digest` (digest, DigestAlgorithm)
- Enables: P0-012 (drift detection), P0-023 (foundation integration)
