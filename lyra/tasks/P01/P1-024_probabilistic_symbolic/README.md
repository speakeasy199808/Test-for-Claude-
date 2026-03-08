# P1-024 — Probabilistic Symbolic

## Mission
Distribution type, symbolic PDFs, Bayesian update primitive. No sampling — symbolic only.

## Scope
- normative specification and versioned contracts where required
- executable compiler pipeline surface in `lyralang/`
- fixtures, goldens, and integration validation
- task control-plane records and traceability artifacts

## Primary Ownership Root
`lyralang/`

## Deliverables
- `docs/lyralang/PROBABILISTIC.md`
- `interfaces/specs/lyralang_probabilistic_symbolic_v1.json`
- `lyralang/src/probabilistic/mod.rs`
- `fixtures/lyralang/probabilistic/prob_valid.lyra`
- `fixtures/lyralang/probabilistic/prob_sampling.lyra`
- `goldens/lyralang/probabilistic/prob_valid.json`
- `goldens/lyralang/probabilistic/prob_sampling.json`

## Key Design Decisions
- Symbolic only: no numeric evaluation, no sampling allowed
- `dist_sample` is a hard error (`SamplingDetected`)
- `dist_bayesian_update` produces both a `BayesianUpdateSummary` and a `Derived` `DistributionSummary`
- `symbolic_only` flag on `ProbabilisticProgramJudgment` is `true` iff no `SamplingDetected` error occurred
- The `EffectAtom::Entropy` atom exists in the broader type system; this module operates purely symbolically without requiring it
