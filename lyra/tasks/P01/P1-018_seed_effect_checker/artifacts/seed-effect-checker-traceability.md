# Traceability — P1-018 Seed Effect Checker

## Acceptance to Implementation Map

- Deterministic effect inference → `lyralang/src/effects/infer.rs::EffectInferenceEngine`
- Effect subtyping and missing obligations → `lyralang/src/types/effect.rs::{is_sub_effect_of,missing_from}`
- Policy violation explanation → `lyralang/src/effects/infer.rs::require_allowed`
- Program judgment output → `lyralang/src/effects/mod.rs::{EffectProgramJudgment,EffectBindingJudgment,EffectCheckOutput}`
- Line/column diagnostics → `lyralang/src/effects/error.rs`, `crate::lexer::span::SourceSpan`
- Fixture-backed verification → `lyralang/tests/seed_effect_checker_integration.rs`
