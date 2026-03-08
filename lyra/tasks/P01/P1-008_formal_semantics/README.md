# P1-008 — Formal Semantics

## Mission
Denotational and operational semantics with a mathematical specification and Stage 0 soundness statement.

## Scope
- normative specification and versioned contracts where required
- executable compiler pipeline surface in `lyralang/`
- fixtures, goldens, and integration validation
- task control-plane records and traceability artifacts

## Primary Ownership Root
`lyralang/`

## Deliverables
- `docs/lyralang/SEMANTICS.md`
- `lyralang/src/semantics/mod.rs`
- `lyralang/src/semantics/error.rs`
- `lyralang/src/semantics/evaluator.rs`
- `lyralang/tests/seed_formal_semantics_integration.rs`
