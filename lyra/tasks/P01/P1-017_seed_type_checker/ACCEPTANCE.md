# Acceptance — P1-017 Seed Type Checker

## Acceptance Criteria

1. The checker performs deterministic type inference over the current Stage 0 AST.
2. Let-bound generalization and identifier instantiation are implemented.
3. Equality constraints are solved through a unifier with occurs checks.
4. Program judgments include inferred type and aggregate effects.
5. Diagnostics include stable source span information, including line and column.
6. Shared fixtures and goldens exist for successful and failing programs.
7. The checker consumes the P1-003 type kernel directly.

## Verification Method
- Inspect `lyralang/src/checker/`
- Review fixture-backed test coverage in `lyralang/tests/seed_type_checker_integration.rs`
- Compare shared goldens in `goldens/lyralang/typechecker/`

## Evidence Required
- `lyralang/src/checker/*`
- `lyralang/tests/seed_type_checker_integration.rs`
- `fixtures/lyralang/typechecker/*`
- `goldens/lyralang/typechecker/*`
- `lyra/tasks/P01/P1-017_seed_type_checker/artifacts/seed-type-checker-traceability.md`
