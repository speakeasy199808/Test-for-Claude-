# P1-007 — Self Reference Primitives

## Mission
`@current_program()`, `@current_receipt()`, and `@ledger_state()` as built-in self-reference primitives with executable validation.

## Scope
- direct parser support for `@name()` self-reference syntax
- canonical metadata descriptor types in the Stage 0 kernel
- deterministic lowering into dedicated self-reference IR instructions
- normative docs and interface contract
- fixture-backed validation

## Primary Ownership Root
`docs/`

## Deliverables
- `docs/lyralang/SELF_REFERENCE.md`
- `interfaces/specs/lyralang_self_reference_primitives_v1.json`
- parser/type/codegen integration across `lyralang/`
- `lyralang/tests/seed_self_reference_integration.rs`
- task control-plane records and traceability artifacts
