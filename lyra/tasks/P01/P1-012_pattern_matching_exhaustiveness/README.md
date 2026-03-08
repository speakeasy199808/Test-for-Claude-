# P1-012 — Pattern Matching Exhaustiveness

## Mission

Implement a static exhaustiveness checker for `match` expressions in LyraLang Stage 0. Every `match` must be proven to cover all reachable input cases before the program is accepted.

## Scope

- `PatternChecker` — public checker struct following the standard `new()` / `check_source(source)` API
- `PatternCheckOutput` — result bundle: `normalized_source`, `judgment`, `errors`
- `PatternProgramJudgment` — program-level exhaustiveness verdict with per-match reports
- `MatchExhaustivenessReport` — per-match report: arm count, wildcard flag, covered patterns, missing cases
- `PatternError` / `PatternErrorKind` — `ParseError`, `NonExhaustiveMatch`, `UnreachablePattern`, `DuplicatePattern`
- Free function `check(source)` as module-level shorthand

## Primary Ownership Root

`lyralang/src/patterns/`

## Secondary Touched Roots

`fixtures/lyralang/patterns/`, `goldens/lyralang/patterns/`, `interfaces/specs/`, `docs/lyralang/`

## Exhaustiveness Logic

| Scrutinee domain  | Exhaustive when                                      |
|-------------------|------------------------------------------------------|
| Wildcard / ident  | Always (catch-all arm present)                       |
| Boolean (`true`/`false`) | Both `true` and `false` arms present, OR wildcard |
| Integer literals  | Only with a wildcard arm (infinite domain)           |
| String literals   | Only with a wildcard arm (infinite domain)           |
| Opaque identifier | Always (by convention — Stage 0 approximation)       |

## Deliverables

- `lyralang/src/patterns/mod.rs` — exhaustiveness checker implementation
- `lyralang/src/patterns/error.rs` — error types
- `fixtures/lyralang/patterns/patterns_valid.lyra` — exhaustive boolean match
- `fixtures/lyralang/patterns/patterns_nonexhaustive.lyra` — non-exhaustive boolean match
- `goldens/lyralang/patterns/patterns_valid.json`
- `goldens/lyralang/patterns/patterns_nonexhaustive.json`
- `interfaces/specs/lyralang_pattern_matching_v1.json`
- `docs/lyralang/PATTERNS.md`
