# Design — P1-009 Trait System

## Design Summary

Typeclass/trait mechanism for ad-hoc polymorphism with coherence, orphan prevention, default implementations, and deterministic derive expansion.

## Constraint Notes

- no nondeterministic host dependencies
- canonical boundary representations only
- conservative Stage 0 surface: do not outrun the current parser/type/codegen baseline
