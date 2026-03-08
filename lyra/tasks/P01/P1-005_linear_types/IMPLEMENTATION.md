# Implementation — P1-005 Linear Types

## Implemented Module Family

- `docs/lyralang/LINEARITY.md` — normative Stage 0 ownership law
- `lyralang/src/types/ty.rs` — resource types added to the shared kernel algebra
- `lyralang/src/builtins.rs` — builtin ownership contracts and resource-typed signatures
- `lyralang/src/linear/mod.rs` — public checking surface and result structures
- `lyralang/src/linear/error.rs` — ownership diagnostics
- `lyralang/src/linear/checker.rs` — exact-once ownership analyzer
- `lyralang/tests/seed_linear_checker_integration.rs` — fixture-backed verification

## Ownership Strategy

- linear bindings are tracked in lexical scope frames
- identifier use on a linear binding performs a move and marks the binding consumed
- builtin contracts can produce, consume, or forward linear resources
- scope exit fails if any outstanding linear bindings remain
- branch results fail if ownership state or returned resource kind diverge

## Follow-on Tasks Enabled

- P1-006 modal types
- P1-010 error handling
- P1-013 lifetime annotations
- P1-014 FFI specification
- P1-019 seed code generator
- P1-020 seed bytecode emitter
