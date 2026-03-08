# Acceptance — P1-018 Seed Effect Checker

## Acceptance Criteria

1. The checker performs deterministic effect inference over the current Stage 0 AST.
2. Effect policy validation uses subset-based effect subtyping.
3. Violations explain required effects, allowed effects, and missing obligations.
4. Diagnostics include stable source span information, including line and column.
5. Shared fixtures and goldens exist for successful and failing programs.
6. The checker consumes the P1-004 effect system directly.

## Verification Method
- Inspect `lyralang/src/effects/`
- Review fixture-backed test coverage in `lyralang/tests/seed_effect_checker_integration.rs`
- Compare shared goldens in `goldens/lyralang/effects/`

## Evidence Required
- `lyralang/src/effects/*`
- `lyralang/tests/seed_effect_checker_integration.rs`
- `fixtures/lyralang/effects/*`
- `goldens/lyralang/effects/*`
- `lyra/tasks/P01/P1-018_seed_effect_checker/artifacts/seed-effect-checker-traceability.md`
