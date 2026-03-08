# Implementation — P1-007 Self Reference Primitives

## Implemented Surface

- `docs/lyralang/SELF_REFERENCE.md` — normative Stage 0 law
- `interfaces/specs/lyralang_self_reference_primitives_v1.json` — canonical boundary contract
- parser support for `@current_program()`, `@current_receipt()`, `@ledger_state()`
- Stage 0 metadata descriptor types in the shared type kernel
- seed type-checker support for self-reference expressions
- seed code-generator lowering into dedicated self-reference instructions
- fixture-backed validation in `lyralang/tests/seed_self_reference_integration.rs`

## Follow-on Tasks Enabled

- P1-008 formal semantics
- P1-019 seed code generator
- P1-025 proof construction
