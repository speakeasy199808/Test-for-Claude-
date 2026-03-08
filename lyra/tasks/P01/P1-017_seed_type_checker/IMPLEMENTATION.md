# Implementation — P1-017 Seed Type Checker

## Implemented Module Family

- `mod.rs` — public type-checking surface and result structures
- `error.rs` — checker diagnostics
- `infer.rs` — environment handling, instantiation, generalization, constraints, and unification
- `lyralang/tests/seed_type_checker_integration.rs` — fixture-backed type-checking coverage

## Inference Strategy

- Parse source first and stop on parser diagnostics.
- Infer statements and expressions in stable source order.
- Generalize `let` bindings against the current environment.
- Solve equality constraints immediately through deterministic unification.
- Return a compact `ProgramJudgment` containing inferred program type, aggregate effects, and user bindings.

## Follow-on Tasks Enabled

- P1-004 effect system
- P1-009 trait system
- P1-012 exhaustiveness with type-aware analysis
- P1-018 seed effect checker
- P1-019 seed code generator
