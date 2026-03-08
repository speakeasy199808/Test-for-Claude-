# Traceability — P1-004 Effect System

## Acceptance to Implementation Map

- Explicit effects in function types → `lyralang/src/types/ty.rs::FunctionType`, `lyralang/src/types/effect.rs`
- Persistent and linear effect distinction → `lyralang/src/types/effect.rs::EffectSet`
- Deterministic effect subtyping → `lyralang/src/types/effect.rs::{is_sub_effect_of,missing_from}`
- Shared latent builtin effects → `lyralang/src/builtins.rs`
- Normative effect law → `docs/lyralang/EFFECTS.md`
- Downstream consumers → `lyralang/src/checker/infer.rs`, `lyralang/src/effects/`
