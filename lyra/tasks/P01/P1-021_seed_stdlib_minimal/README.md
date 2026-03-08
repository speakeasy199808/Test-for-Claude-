# P1-021 — Seed Stdlib Minimal

## Mission
Minimal primitive/data/io/math stdlib written in Lyra source files and compiled through the seed compiler.

## Scope
- normative specification and versioned contracts where required
- executable compiler pipeline surface in `lyralang/`
- fixtures, goldens, and integration validation
- task control-plane records and traceability artifacts

## Primary Ownership Root
`lyralang/`

## Deliverables
- `docs/lyralang/STDLIB.md`
- `interfaces/specs/lyralang_seed_stdlib_v1.json`
- `lyralang/src/stdlib/mod.rs`
- `lyralang/src/stdlib/error.rs`
- `lyralang/src/stdlib/compiler.rs`
- `lyralang/tests/seed_stdlib_minimal_integration.rs`
