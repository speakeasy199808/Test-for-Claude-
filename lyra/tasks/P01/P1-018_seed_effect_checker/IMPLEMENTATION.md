# Implementation — P1-018 Seed Effect Checker

## Implemented Module Family

- `mod.rs` — public effect-checking surface and result structures
- `error.rs` — effect-checker diagnostics
- `infer.rs` — environment handling, structural effect inference, callable resolution, and policy checks
- `lyralang/tests/seed_effect_checker_integration.rs` — fixture-backed effect-checking coverage

## Inference Strategy

- Parse source first and stop on parser diagnostics.
- Infer statement and expression effects in stable source order.
- Resolve builtin call signatures through the shared Stage 0 builtin registry.
- Validate each latent call effect against the optional policy ceiling.
- Return a compact `EffectProgramJudgment` containing aggregate program effects and let-binding summaries.

## Follow-on Tasks Enabled

- P1-005 linear types
- P1-010 error handling
- P1-011 concurrency primitives
- P1-019 seed code generator
- P1-020 seed bytecode emitter
