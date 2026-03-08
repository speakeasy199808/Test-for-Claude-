# P1-005 — Linear Types

## Mission
Resource types: File, Socket, Capability. Must be used exactly once. Compile-time enforcement — no runtime cost.

## Scope
- canonical Stage 0 linear resource types in the kernel
- exact-once ownership discharge over the current AST
- deterministic branch-consistency rules for linear resources
- builtin ownership contracts for File, Socket, and Capability
- normative law in `docs/lyralang/LINEARITY.md`

## Primary Ownership Root
`lyralang/`

## Deliverables
- `docs/lyralang/LINEARITY.md`
- `lyralang/src/types/ty.rs`
- `lyralang/src/builtins.rs`
- `lyralang/src/linear/`
- `lyralang/tests/seed_linear_checker_integration.rs`
- task control-plane records and traceability artifacts
