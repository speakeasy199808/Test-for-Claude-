# P1-011 — Concurrency Primitives

## Mission
Structured concurrency: spawn, join, select. Channel types. Deterministic scheduling specification. No data races by construction.

## Scope
- normative specification and versioned contracts where required
- executable compiler pipeline surface in `lyralang/`
- fixtures, goldens, and integration validation
- task control-plane records and traceability artifacts

## Primary Ownership Root
`lyralang/`

## Deliverables
- `docs/lyralang/CONCURRENCY.md`
- `interfaces/specs/lyralang_concurrency_model_v1.json`
- `lyralang/src/concurrency/mod.rs`
- `lyralang/src/concurrency/error.rs`
- `lyralang/src/concurrency/checker.rs`
- `lyralang/tests/seed_concurrency_integration.rs`
