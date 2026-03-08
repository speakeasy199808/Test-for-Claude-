# Design — P1-003 Type System Kernel

The Stage 0 kernel is intentionally split away from concrete syntax. The parser does not yet accept user-written
type annotations, but the checker still needs a stable semantic type algebra. This task therefore defines the
canonical internal representation first and treats source-level sugar as a later extension.

## Design Decisions

- The kernel reserves `Unit`, `Bool`, `Int`, `Nat`, and `Rational` now, even if literal forms are not all exposed yet.
- Product, sum, and function types are represented canonically with ordered members.
- Function types reserve an explicit effect slot so later effect work extends rather than rewrites the type algebra.
- Type schemes quantify deterministic compiler-generated variables in creation order.
- Canonical string forms are part of the contract so tests and tools can emit stable evidence.

## Ownership

- Normative kernel law: `docs/lyralang/TYPES.md`
- Executable type structures: `lyralang/src/types/`
- Seed consumer enabled by this task: `lyralang/src/checker/`
