# P1-022 — Seed Test Framework

## Mission
Unit tests, property-based tests, golden file comparisons. All under cargo test.

## Scope
- shared helper module in `lyralang/`
- fixture/golden path canonicalization
- property-based coverage for deterministic lexer behavior
- task control-plane records and traceability artifacts

## Primary Ownership Root
`lyralang/`

## Deliverables
- `docs/lyralang/TESTING.md`
- `interfaces/specs/lyralang_test_framework_v1.json`
- `lyralang/src/testing/mod.rs`
- `lyralang/tests/seed_test_framework_integration.rs`
