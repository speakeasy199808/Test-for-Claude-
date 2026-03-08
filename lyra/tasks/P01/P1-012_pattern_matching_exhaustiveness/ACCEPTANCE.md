# Acceptance — P1-012 Pattern Matching Exhaustiveness

## Acceptance Criteria

1. `PatternChecker::new()` constructs a zero-configuration checker.
2. `PatternChecker::check_source(source)` returns a `PatternCheckOutput` for any input.
3. A program with no match expressions produces a `PatternProgramJudgment` with `exhaustive: true` and an empty `match_expressions` list.
4. A match over `true` / `false` that covers both arms produces `exhaustive: true` with `missing_cases: []`.
5. A match over `true` / `false` that covers only `true` produces `exhaustive: false` with `missing_cases: ["false"]`.
6. A match containing a wildcard (`_`) or identifier arm is always exhaustive regardless of other arms.
7. Any arm appearing after a catch-all arm emits a `PatternErrorKind::UnreachablePattern` diagnostic with `recovered: true`.
8. A repeated literal pattern emits a `PatternErrorKind::DuplicatePattern` diagnostic with `recovered: true`.
9. An integer or string literal match without a wildcard emits `PatternErrorKind::NonExhaustiveMatch` with `missing_cases: ["_"]`.
10. All public types derive `Clone, Debug, Serialize, Deserialize`.
11. All public items carry `///` doc comments.
12. The free function `check(source)` is equivalent to `PatternChecker::new().check_source(source)`.

## Verification Method

- Unit tests covering each exhaustiveness case
- Fixture/golden round-trip for `patterns_valid.lyra` and `patterns_nonexhaustive.lyra`
- `cargo check -p lyralang` passes with no warnings

## Status

All acceptance criteria satisfied. Module implemented at `lyralang/src/patterns/mod.rs`.
