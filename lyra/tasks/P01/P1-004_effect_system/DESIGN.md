# Design — P1-004 Effect System

The Stage 0 effect system must be real infrastructure, not a speculative comment inside the type checker.
The function type representation already reserved an effect slot in P1-003, so this task formalizes the
shared effect algebra and makes builtin callable signatures carry explicit latent effects.

## Design Decisions

- Persistent effects and linear effects are tracked separately but serialized through one canonical contract.
- Effect subtyping is subset-based for deterministic reasoning.
- Linear effects are represented now as bookkeeping at the effect layer; deeper resource coupling is deferred to P1-005.
- Shared builtin callable signatures are centralized so the seed type checker and seed effect checker stay coherent.
- Policy ceilings are pure set comparisons, not heuristic ranks.

## Ownership

- Normative effect law: `docs/lyralang/EFFECTS.md`
- Canonical effect data structures: `lyralang/src/types/effect.rs`
- Shared builtin effect-bearing contracts: `lyralang/src/builtins.rs`
