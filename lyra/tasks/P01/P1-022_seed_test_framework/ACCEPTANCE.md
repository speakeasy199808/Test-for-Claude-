# Acceptance — P1-022 Seed Test Framework

## Acceptance Criteria

1. Unit-test helpers centralize fixture/golden lookup.
2. Golden JSON comparison is canonical and deterministic.
3. Property-based tests run under `cargo test`.
4. Shared fixtures/goldens validate the framework itself.

## Verification Method
- review task-local docs/specs/interfaces
- inspect `lyralang/src/testing/mod.rs`
- run `lyralang/tests/seed_test_framework_integration.rs`

## Evidence Required
- `docs/lyralang/TESTING.md`
- `interfaces/specs/lyralang_test_framework_v1.json`
- `lyralang/src/testing/mod.rs`
- `lyralang/tests/seed_test_framework_integration.rs`
