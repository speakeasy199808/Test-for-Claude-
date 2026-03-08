# Acceptance — P1-024 Probabilistic Symbolic

## Acceptance Criteria

1. `dist_uniform(lo, hi)` is recognized and produces a `DistributionSummary` with `kind = Uniform`.
2. `dist_bernoulli(p)` is recognized and produces a `DistributionSummary` with `kind = Bernoulli`.
3. `dist_bayesian_update(prior, likelihood)` is recognized and produces both a `BayesianUpdateSummary` and a `Derived` `DistributionSummary`.
4. `dist_sample(...)` produces a `SamplingDetected` error and sets `symbolic_only = false`.
5. All analysis is symbolic — no numeric evaluation occurs.
6. Fixtures and goldens cover both valid and error cases.
7. `ProbabilisticCheckOutput` serializes deterministically.

## Verification Method
- Review `lyralang/src/probabilistic/mod.rs`
- Inspect fixture `fixtures/lyralang/probabilistic/prob_valid.lyra` against golden `goldens/lyralang/probabilistic/prob_valid.json`
- Inspect fixture `fixtures/lyralang/probabilistic/prob_sampling.lyra` against golden `goldens/lyralang/probabilistic/prob_sampling.json`

## Evidence Required
- `docs/lyralang/PROBABILISTIC.md`
- `interfaces/specs/lyralang_probabilistic_symbolic_v1.json`
- `lyralang/src/probabilistic/mod.rs`
- `fixtures/lyralang/probabilistic/prob_valid.lyra`
- `fixtures/lyralang/probabilistic/prob_sampling.lyra`
- `goldens/lyralang/probabilistic/prob_valid.json`
- `goldens/lyralang/probabilistic/prob_sampling.json`
