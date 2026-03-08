# P1-003 — Type System Kernel

## Mission
Primitive types: Unit, Bool, Int, Nat, Rational. Product, sum, and function types. Hindley-Milner base.

## Scope
- Primitive kernel types for Stage 0
- Canonical product, sum, and function type representations
- Hindley-Milner type variables and schemes
- Deterministic canonical textual form for types and schemes
- Normative type kernel law in `docs/lyralang/TYPES.md`

## Primary Ownership Root
`lyralang/`

## Deliverables
- `docs/lyralang/TYPES.md`
- `lyralang/src/types/mod.rs`
- `lyralang/src/types/effect.rs`
- `lyralang/src/types/ty.rs`
- Task control-plane records and traceability artifacts
