# P1-023 — Temporal Logic

## Mission
always, eventually, until, since operators. Linear temporal logic built into the language.

## Scope
- normative specification and versioned contracts where required
- executable compiler pipeline surface in `lyralang/`
- fixtures, goldens, and integration validation
- task control-plane records and traceability artifacts

## Primary Ownership Root
`lyralang/`

## Deliverables
- `docs/lyralang/TEMPORAL.md`
- `interfaces/specs/lyralang_temporal_logic_v1.json`
- `lyralang/src/temporal/mod.rs`
- `lyralang/src/temporal/error.rs`
- `lyralang/src/temporal/checker.rs`
- `lyralang/tests/seed_temporal_logic_integration.rs`
