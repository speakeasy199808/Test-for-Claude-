# P1-017 — Seed Type Checker

## Mission
Hindley-Milner with effect extensions. Constraint solving. Clear error messages with line and column.

## Scope
- Deterministic HM-style inference over the current Stage 0 AST
- Online equality-constraint solving by unification
- Let generalization and instantiation
- Explicit program judgment containing inferred type and aggregate effects
- Clear diagnostics with source span, line, and column

## Primary Ownership Root
`lyralang/`

## Deliverables
- `lyralang/src/checker/mod.rs`
- `lyralang/src/checker/error.rs`
- `lyralang/src/checker/infer.rs`
- `lyralang/tests/seed_type_checker_integration.rs`
- `fixtures/lyralang/typechecker/*`
- `goldens/lyralang/typechecker/*`
- Task control-plane records and traceability artifacts
