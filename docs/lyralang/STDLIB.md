# LyraLang Seed Stdlib Minimal — Stage 0

## Status

This document freezes the first minimal standard library for LyraLang.
The sources are written in `.lyra` files and compiled through the current
seed compiler pipeline.

## Scope

- primitive data kernels
- I/O primitives
- math seeds
- deterministic manifest and versioned interface contract

## Manifest

The Stage 0 seed stdlib ships these modules:

- `std.data.bool_guard`
- `std.io.print_status`
- `std.math.int_eq`
- `std.math.rational_seed`

## Constraints

The current grammar does not yet expose user-defined functions, trait declarations,
or collection literals. This stdlib therefore freezes a minimal primitive-centric
library surface that is still genuinely written in Lyra and compiled by the seed
pipeline.

## Compilation Path

Each stdlib module is compiled through:

1. parser
2. type checker
3. code generator
4. bytecode emitter

## Interfaces

- stdlib manifest contract: `interfaces/specs/lyralang_seed_stdlib_v1.json`

## Implementation Surface

- `lyralang/src/stdlib/`
- `fixtures/lyralang/stdlib/modules/*.lyra`
