# Acceptance — P1-027 Metaprogramming

## Acceptance Criteria

1. Stage 0 exposes `meta_eval`, `quasi_quote`, and `quasi_unquote` surface forms.
2. `meta_eval(literal)` produces a `CompileTimeEval` with `statically_determined = true`.
3. `meta_eval(non_literal)` produces a `CompileTimeEval` with `statically_determined = false`
   and a `NonStaticMetaEval` error.
4. `quasi_quote(expr)` records a `QuasiQuote` with the canonical `quoted_form`.
5. `quasi_unquote(expr)` inside a `quasi_quote` is recorded as an unquote site.
6. `quasi_unquote` outside any `quasi_quote` produces an `UnbalancedQuasiQuote` error.
7. `all_static = true` iff every `meta_eval` argument was statically determined.
8. Fixtures and goldens cover both success and error behavior.

## Verification Method
- Review `docs/lyralang/METAPROGRAMMING.md` and `interfaces/specs/lyralang_metaprogramming_v1.json`.
- Inspect `lyralang/src/meta/mod.rs` implementation.
- Run checker over `fixtures/lyralang/meta/meta_valid.lyra` and compare to golden.
- Run checker over `fixtures/lyralang/meta/meta_nonstatic.lyra` and compare to golden.

## Evidence Required
- `docs/lyralang/METAPROGRAMMING.md`
- `interfaces/specs/lyralang_metaprogramming_v1.json`
- `lyralang/src/meta/mod.rs`
- `fixtures/lyralang/meta/meta_valid.lyra`
- `fixtures/lyralang/meta/meta_nonstatic.lyra`
- `goldens/lyralang/meta/meta_valid.json`
- `goldens/lyralang/meta/meta_nonstatic.json`
