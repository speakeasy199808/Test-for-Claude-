# Traceability — P1-005 Linear Types

## Acceptance to Implementation Map

- Canonical resource types → `lyralang/src/types/ty.rs::{ResourceType,Type::Resource}`
- Shared builtin ownership contracts → `lyralang/src/builtins.rs::{LinearBuiltinBehavior,LinearCallableSignature,builtin_linear_environment}`
- Exact-once ownership analyzer → `lyralang/src/linear/checker.rs::LinearAnalyzer`
- Duplicate-use diagnostics → `lyralang/src/linear/error.rs`, `lyralang/src/linear/checker.rs::consume_identifier`
- Scope-exit leak detection → `lyralang/src/linear/checker.rs::ensure_current_scope_closed`
- Branch consistency checking → `lyralang/src/linear/checker.rs::{analyze_if,analyze_match,merge_branch_state}`
- Fixture-backed verification → `lyralang/tests/seed_linear_checker_integration.rs`
