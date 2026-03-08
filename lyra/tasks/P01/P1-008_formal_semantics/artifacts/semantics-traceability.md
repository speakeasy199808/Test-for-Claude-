# Traceability — P1-008 Formal Semantics

## Acceptance to Implementation Map

- denotational + operational semantics law → `docs/lyralang/SEMANTICS.md`
- executable semantic evaluator → `lyralang/src/semantics/{mod.rs,error.rs,evaluator.rs}`
- type soundness statement carried with judgment → `docs/lyralang/SEMANTICS.md`, `lyralang/src/semantics/mod.rs`
- fixture-backed validation → `fixtures/lyralang/semantics/*`, `goldens/lyralang/semantics/*`, `lyralang/tests/seed_formal_semantics_integration.rs`
