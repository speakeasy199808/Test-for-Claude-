# Probabilistic Symbolic — LyraLang P1-024

## Overview

The probabilistic symbolic module introduces a `Dist[T]` type concept for symbolic probability distributions. All analysis is purely structural: the checker records distribution shapes and Bayesian update flows but never evaluates or samples numeric values.

Sampling is actively forbidden. Any call to `dist_sample` is a hard error.

## Builtin Operators

### `dist_uniform(lo, hi)`

Creates a symbolic uniform distribution over the interval `[lo, hi]`.

- **Distribution kind:** `Uniform`
- **PDF description:** `Uniform[lo=<lo>, hi=<hi>]: pdf(x) = 1/(hi-lo) for x in [lo, hi]`
- Parameters `lo` and `hi` are symbolic — they may be integer literals, identifiers, or arbitrary expressions.

### `dist_bernoulli(p)`

Creates a symbolic Bernoulli distribution with symbolic success probability `p`.

- **Distribution kind:** `Bernoulli`
- **PDF description:** `Bernoulli[p=<p>]: P(1)=p, P(0)=1-p (symbolic)`
- Parameter `p` is symbolic and is never evaluated to a numeric value.

### `dist_bayesian_update(prior, likelihood)`

Produces the symbolic posterior distribution by applying Bayes' theorem.

- **Distribution kind:** `Derived`
- **Update description:** `P(posterior) ∝ P(likelihood|prior)`
- Records both a `BayesianUpdateSummary` and a `Derived` `DistributionSummary` for the posterior.

### `dist_sample(dist)` — **Forbidden**

Numeric sampling is forbidden in symbolic probabilistic mode. Any call to `dist_sample` produces a `SamplingDetected` error (non-recovered) and sets `symbolic_only = false` on the program judgment.

## Output Types

### `ProbabilisticCheckOutput`

```
normalized_source: String
judgment: Option<ProbabilisticProgramJudgment>
errors: Vec<ProbabilisticError>
```

### `ProbabilisticProgramJudgment`

```
module: Option<String>
program_type: String          -- canonical type name
distributions: Vec<DistributionSummary>
bayesian_updates: Vec<BayesianUpdateSummary>
symbolic_only: bool           -- true iff no SamplingDetected error
span: SourceSpan
```

### `DistributionSummary`

```
name: String                  -- binding name or "<anonymous>"
distribution_kind: DistributionKind
parameter_summaries: Vec<String>
pdf_description: String
span: SourceSpan
```

### `BayesianUpdateSummary`

```
prior_name: String
likelihood_name: String
posterior_name: String
update_description: String    -- "P(posterior) ∝ P(likelihood|prior)"
span: SourceSpan
```

## Error Kinds

| Kind | Label | Recovered |
|------|-------|-----------|
| `ParseError` | `parse_error` | varies |
| `SamplingDetected` | `sampling_detected` | `false` |
| `InvalidDistribution` | `invalid_distribution` | `true` |
| `MalformedBayesianUpdate` | `malformed_bayesian_update` | `true` |

## Entry Point

```rust
use lyralang::probabilistic::check;
let output = check(source);
```

Or via the checker struct:

```rust
use lyralang::probabilistic::ProbabilisticChecker;
let checker = ProbabilisticChecker::new();
let output = checker.check_source(source);
```

## Relation to Effect System

The broader type system carries `EffectAtom::Entropy` for deterministic entropy interactions. The probabilistic symbolic checker operates purely structurally and does not require or produce entropy effects. When numeric sampling is desired in future stages, it would be gated behind `EffectAtom::Entropy`.
