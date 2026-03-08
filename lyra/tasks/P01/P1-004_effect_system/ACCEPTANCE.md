# Acceptance — P1-004 Effect System

## Acceptance Criteria

1. Function types carry explicit effect sets in canonical form.
2. The Stage 0 effect algebra defines persistent and linear effect obligations.
3. Effect subtyping is deterministic and subset-based.
4. Canonical string forms for effects remain stable.
5. Shared builtin callable contracts expose explicit latent effects.
6. The normative law is recorded in `docs/lyralang/EFFECTS.md`.
7. P1-017 and P1-018 consume the same effect structures rather than private duplicates.

## Verification Method
- Review `docs/lyralang/EFFECTS.md`
- Inspect `lyralang/src/types/effect.rs`
- Inspect `lyralang/src/builtins.rs`
- Confirm P1-017 and P1-018 consume these structures

## Evidence Required
- `docs/lyralang/EFFECTS.md`
- `lyralang/src/types/effect.rs`
- `lyralang/src/builtins.rs`
- `lyra/tasks/P01/P1-004_effect_system/artifacts/effect-system-traceability.md`
