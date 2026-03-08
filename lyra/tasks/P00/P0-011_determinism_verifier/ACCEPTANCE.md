# Acceptance — P0-011 Determinism Verifier

## Acceptance Criteria
1. `DeterminismVerifier::verify(label, f)` runs `f` twice and compares outputs byte-for-byte.
2. Identical outputs → `Ok(output)` + `Pass` record in audit log.
3. Divergent outputs → `Err(DeterminismViolation)` + `Fail` record with hex evidence.
4. Empty output → `Err(EmptyOutput)` unless `verify_allow_empty` is used.
5. Every verification event is timestamped with `VirtualTime` from the internal clock.
6. `all_pass()`, `pass_count()`, `fail_count()`, `record_count()` correctly reflect audit log.
7. `verify_once` stateless helper works without a verifier instance.
8. Integration tests confirm codec encoder and digest algorithms are deterministic.

## Verification Method
- Unit tests in `k0/src/verifier/determinism.rs` (21 tests)
- Integration tests in `k0/src/verifier/mod.rs` (6 tests)
- `cargo test -p k0 --lib` — all 163 pass

## Evidence Required
- `artifacts/verifier-traceability.md`
- `fixtures/verifier/scenarios.json`
- `cargo test` output: 163 passed; 0 failed
