# P1-018 — Seed Effect Checker

## Mission
Effect inference. Effect subtyping. Violation detection with explanation.

## Scope
- deterministic effect inference over the current Stage 0 AST
- policy-ceiling validation through effect subtyping
- precise violation diagnostics with stable source spans
- fixture-backed effect inference and rejection evidence

## Primary Ownership Root
`lyralang/`

## Deliverables
- `lyralang/src/effects/mod.rs`
- `lyralang/src/effects/error.rs`
- `lyralang/src/effects/infer.rs`
- `lyralang/tests/seed_effect_checker_integration.rs`
- `fixtures/lyralang/effects/*`
- `goldens/lyralang/effects/*`
- task control-plane records and traceability artifacts
