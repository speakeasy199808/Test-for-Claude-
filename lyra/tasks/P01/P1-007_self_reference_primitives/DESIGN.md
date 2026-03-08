# Design — P1-007 Self Reference Primitives

## Design Summary

P1-007 is implemented as a cross-cutting but conservative Stage 0 slice:

- the parser owns the `@name()` surface directly
- the shared type kernel owns stable descriptor types
- the code generator lowers the primitives into explicit self-reference IR instructions

This keeps the feature executable now without inventing runtime schema details that belong to later phases.

## Why Metadata Descriptors

The current compiler pipeline does not yet have rich record/object surface syntax.
Representing self-reference results as canonical metadata descriptor types gives the language a stable
contract immediately while leaving payload expansion to later phases.
