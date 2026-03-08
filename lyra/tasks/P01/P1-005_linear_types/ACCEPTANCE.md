# Acceptance — P1-005 Linear Types

## Acceptance Criteria

1. The Stage 0 type kernel defines canonical resource types `File`, `Socket`, and `Capability`.
2. Linear bindings must be discharged exactly once at compile time, with no required runtime bookkeeping.
3. Re-use after move is rejected deterministically.
4. Scope exit with an outstanding linear binding is rejected deterministically.
5. Branches must leave identical ownership state and identical returned resource shape.
6. Shared builtin ownership contracts define constructor, consumer, and passthrough behavior.
7. The normative law is recorded in `docs/lyralang/LINEARITY.md`.
8. Fixture-backed verification covers both a successful discharge path and a duplicate-use failure.

## Verification Method
- Review `docs/lyralang/LINEARITY.md`
- Inspect `lyralang/src/types/ty.rs`
- Inspect `lyralang/src/builtins.rs`
- Inspect `lyralang/src/linear/`
- Run `lyralang/tests/seed_linear_checker_integration.rs`

## Evidence Required
- `docs/lyralang/LINEARITY.md`
- `lyralang/src/types/ty.rs`
- `lyralang/src/builtins.rs`
- `lyralang/src/linear/`
- `fixtures/lyralang/linear/*`
- `goldens/lyralang/linear/*`
- `lyra/tasks/P01/P1-005_linear_types/artifacts/linear-types-traceability.md`
