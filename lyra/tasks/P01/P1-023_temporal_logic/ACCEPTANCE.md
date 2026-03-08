# Acceptance — P1-023 Temporal Logic

## Acceptance Criteria

1. Stage 0 exposes executable temporal operators `always`, `eventually`, `until`, and `since`.
2. The type kernel includes an explicit `Temporal[T]` carrier type.
3. The temporal checker records deterministic normalized formulas in source order.
4. Fixtures/goldens cover both success and failure behavior.

## Verification Method
- review task-local docs/specs/interfaces
- inspect builtin environment and temporal checker implementation
- run `lyralang/tests/seed_temporal_logic_integration.rs`

## Evidence Required
- `docs/lyralang/TEMPORAL.md`
- `interfaces/specs/lyralang_temporal_logic_v1.json`
- `lyralang/src/temporal/mod.rs`
- `lyralang/src/temporal/error.rs`
- `lyralang/src/temporal/checker.rs`
- `lyralang/tests/seed_temporal_logic_integration.rs`
