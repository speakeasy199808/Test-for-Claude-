# Acceptance — P1-011 Concurrency Primitives

## Acceptance Criteria

1. Stage 0 exposes executable structured concurrency via `spawn`, `join`, and `select` call surfaces.
2. Typed channel surfaces are available and summarized deterministically.
3. The concurrency checker enforces a no-linear-capture policy for spawned expressions.
4. Fixtures/goldens cover both success and failure behavior.

## Verification Method
- review task-local docs/specs/interfaces
- inspect builtin environment and concurrency checker implementation
- run `lyralang/tests/seed_concurrency_integration.rs`

## Evidence Required
- `docs/lyralang/CONCURRENCY.md`
- `interfaces/specs/lyralang_concurrency_model_v1.json`
- `lyralang/src/concurrency/mod.rs`
- `lyralang/src/concurrency/error.rs`
- `lyralang/src/concurrency/checker.rs`
- `lyralang/tests/seed_concurrency_integration.rs`
