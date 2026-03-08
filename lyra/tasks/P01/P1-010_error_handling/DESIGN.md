# Design — P1-010 Error Handling

## Design Summary

Stage 0 error handling is implemented as:

- explicit internal `Option` / `Result` kernel types
- postfix `?` parsing and type inference
- scope-local propagation contexts for blocks and whole programs
- deterministic error-label composition with `@trace`
- a dedicated analyzer that rejects panic-style calls

## Constraint Notes

- no new user-written type syntax beyond the existing parser surface
- conservative propagation model scoped to current executable Stage 0 blocks/programs
- deterministic spans only; no host/runtime trace capture
