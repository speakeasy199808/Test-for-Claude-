# LyraLang Seed Test Framework

## 1. Scope

P1-022 freezes the first shared test framework for the `lyralang` crate under `cargo test`.

It standardizes:

- unit-test helpers
- property-based tests
- golden-file comparisons
- canonical fixture/golden path resolution

## 2. Helper Surface

The shared helper module provides:

- workspace-root resolution from the `lyralang` crate
- canonical fixture lookup under `fixtures/lyralang/<group>/`
- canonical golden lookup under `goldens/lyralang/<group>/`
- stable pretty JSON rendering
- JSON golden assertions with deterministic mismatch output

## 3. Property-Based Testing Law

Property tests must:

- run under `cargo test`
- avoid network/time/host-state dependencies
- assert deterministic behavior, not probabilistic success claims
- keep generators bounded and replayable

The first property suite proves lexical determinism for generated Stage 0 source strings.

## 4. Golden Comparison Law

Goldens are the source of truth for externalized summaries. Tests shall compare fully materialized JSON values, not ad hoc string fragments.

## 5. Determinism Requirements

- helper path construction is canonical
- JSON rendering is stable
- mismatch output is deterministic
- generated inputs are bounded and test-run safe
