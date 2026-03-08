# Design — P1-020 Seed Bytecode Emitter

## Design Summary

Canonical bytecode emission over deterministic Stage 0 IR.

## Constraint Notes

- no nondeterministic host dependencies
- canonical boundary representations only
- conservative Stage 0 surface: do not outrun the current parser/type/codegen baseline
