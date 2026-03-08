# Design — P1-021 Seed Stdlib Minimal

## Design Summary

Minimal primitive/data/io/math stdlib written in Lyra source files and compiled through the seed compiler.

## Constraint Notes

- no nondeterministic host dependencies
- canonical boundary representations only
- conservative Stage 0 surface: do not outrun the current parser/type/codegen baseline
