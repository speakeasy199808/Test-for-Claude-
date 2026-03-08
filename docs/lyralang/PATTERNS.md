# Pattern Matching Exhaustiveness — Formal Specification

**Task:** P1-012
**Module:** `lyralang::patterns`
**Stage:** 0

## 1. Overview

The pattern matching exhaustiveness checker statically verifies that every `match` expression in a LyraLang program covers all reachable input cases. A non-exhaustive match is a compile-time error. The checker also detects unreachable arms (arms that appear after a catch-all) and duplicate literal patterns.

## 2. Pattern Grammar (Stage 0)

```
Pattern ::=
    | "_"                    -- Wildcard
    | Identifier             -- Identifier (catch-all binding)
    | IntegerLiteral         -- Decimal integer
    | StringLiteral          -- UTF-8 string
    | BooleanLiteral         -- true | false
```

## 3. Exhaustiveness Rules

### 3.1 Wildcard and Identifier Catch-All

A match arm with pattern `_` or any identifier (e.g. `x`) is a catch-all. A match expression is always exhaustive when it contains at least one such arm.

### 3.2 Boolean Domain

The boolean type has exactly two inhabitants: `true` and `false`. A match over a boolean scrutinee (detected when the arms use boolean literal patterns) is exhaustive if and only if both `true` and `false` are explicitly covered, OR a wildcard/identifier arm is present.

**Missing cases** are reported individually: `"true"` or `"false"` as appropriate.

### 3.3 Integer and String Domains

The integer and string domains are infinite. A match using integer or string literal patterns is exhaustive only if a wildcard or identifier arm is also present. Without a catch-all, the checker reports `missing_cases: ["_"]`.

### 3.4 Opaque Identifier Scrutinee

When the scrutinee is an identifier and no boolean, integer, or string literal patterns appear in the arms, the match is considered exhaustive by Stage 0 convention (the scrutinee type is opaque).

### 3.5 Empty Match

A match with zero arms is considered exhaustive by convention (it is unreachable code).

## 4. Error Kinds

| Kind                  | Meaning                                                      |
|-----------------------|--------------------------------------------------------------|
| `ParseError`          | Source failed to parse; checking did not proceed            |
| `NonExhaustiveMatch`  | A match expression does not cover all possible input cases  |
| `UnreachablePattern`  | An arm can never be reached (arm after catch-all); recovered |
| `DuplicatePattern`    | The same literal appears more than once in a match; recovered|

## 5. Output Structure

```
PatternCheckOutput {
    normalized_source: String,
    judgment: Option<PatternProgramJudgment>,
    errors: Vec<PatternError>,
}

PatternProgramJudgment {
    module: Option<String>,
    match_expressions: Vec<MatchExhaustivenessReport>,
    exhaustive: bool,    -- true iff ALL match expressions are exhaustive
    span: SourceSpan,
}

MatchExhaustivenessReport {
    match_index: usize,         -- 0-based, source order
    arm_count: usize,
    has_wildcard: bool,
    covered_patterns: Vec<String>,  -- canonical pattern text
    exhaustive: bool,
    missing_cases: Vec<String>,
    span: SourceSpan,
}
```

## 6. Invariants

- The checker never modifies source text; it forwards `normalized_source` from the parser.
- The checker always produces a `PatternProgramJudgment` on parse success, even when match expressions have errors.
- `PatternProgramJudgment::exhaustive` is the conjunction of all `MatchExhaustivenessReport::exhaustive` values.
- `UnreachablePattern` and `DuplicatePattern` errors have `recovered: true`; the checker continues analysis.
- `NonExhaustiveMatch` errors have `recovered: false`.

## 7. API

```rust
// Zero-configuration checker
let checker = PatternChecker::new();
let output = checker.check_source(source);

// Free function shorthand
let output = lyralang::patterns::check(source);
```
