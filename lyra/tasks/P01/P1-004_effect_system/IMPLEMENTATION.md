# Implementation — P1-004 Effect System

## Implemented Assets

- Added `docs/lyralang/EFFECTS.md` as the normative Stage 0 effect law.
- Extended `lyralang/src/types/effect.rs` with persistent and linear effect tracking, effect subtyping, and canonical entries.
- Added `lyralang/src/builtins.rs` so seed builtins carry explicit latent effects in one shared place.
- Updated the seed type checker call path so call expressions now propagate builtin latent effects instead of hard-coding purity.

## Notes

- Source-level effect annotations remain deferred.
- Linear effects are represented at the effect layer now; richer resource typing remains future work.
- P1-018 consumes this same algebra directly for policy validation and diagnostics.
