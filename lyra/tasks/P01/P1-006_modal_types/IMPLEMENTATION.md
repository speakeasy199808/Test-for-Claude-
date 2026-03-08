# Implementation — P1-006 Modal Types

## Implemented Module Family

- `docs/lyralang/MODALITY.md` — normative Stage 0 modal law
- `lyralang/src/types/ty.rs` — modal kinds and evidence token types added to the shared kernel algebra
- `lyralang/src/builtins.rs` — builtin contracts for explicit modal introduction, promotion, and elimination
- `lyralang/src/modal/mod.rs` — public modal-checking surface and result structures
- `lyralang/src/modal/error.rs` — modal diagnostics
- `lyralang/src/modal/checker.rs` — promotion tracer over the typed AST
- `lyralang/tests/seed_modal_checker_integration.rs` — fixture-backed verification

## Strategy

- represent epistemic state directly in the kernel type algebra
- require explicit evidence token types for promotion builtins
- reuse the seed type checker as the modal soundness gate
- layer a deterministic modal analyzer over the typed AST to emit audit-friendly summaries

## Follow-on Tasks Enabled

- P1-007 self reference primitives
- P1-008 formal semantics
- P1-010 error handling
- P1-025 proof construction
