# Traceability — P1-017 Seed Type Checker

## Acceptance to Implementation Map

- Deterministic HM-style inference → `lyralang/src/checker/infer.rs::InferenceEngine`
- Equality constraints and unification → `lyralang/src/checker/infer.rs::{TypeConstraint,constrain,solve_constraint,unify}`
- Let generalization and instantiation → `lyralang/src/checker/infer.rs::{generalize,instantiate}`
- Program judgment output → `lyralang/src/checker/mod.rs::{ProgramJudgment,BindingJudgment,TypeCheckOutput}`
- Line/column diagnostics → `lyralang/src/checker/error.rs`, `crate::lexer::span::SourceSpan`
- Fixture-backed verification → `lyralang/tests/seed_type_checker_integration.rs`
