# Implementation — P1-003 Type System Kernel

## Implemented Assets

- Added `docs/lyralang/TYPES.md` as the normative Stage 0 type kernel.
- Added `lyralang/src/types/effect.rs` for deterministic effect-set primitives attached to function types.
- Added `lyralang/src/types/ty.rs` for primitives, structural types, type variables, and schemes.
- Added canonical string and substitution/free-variable support for stable inference and evidence generation.
- Linked the grammar document back to the type kernel as the semantic source of truth for later passes.

## Notes

- Concrete type-annotation syntax remains deferred.
- String/text typing is intentionally deferred even though string literals are already parsed.
- Higher-level trait, linear, modal, and lifetime semantics remain future tasks.
