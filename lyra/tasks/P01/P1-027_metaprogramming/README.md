# P1-027 — Metaprogramming

## Mission
Compile-time code execution, AST manipulation, quasiquotation primitives.

## Scope
- normative specification and versioned contracts where required
- executable compiler pipeline surface in `lyralang/`
- fixtures, goldens, and integration validation
- task control-plane records and traceability artifacts

## Primary Ownership Root
`lyralang/`

## Deliverables
- `docs/lyralang/METAPROGRAMMING.md`
- `interfaces/specs/lyralang_metaprogramming_v1.json`
- `lyralang/src/meta/mod.rs`
- `fixtures/lyralang/meta/meta_valid.lyra`
- `fixtures/lyralang/meta/meta_nonstatic.lyra`
- `goldens/lyralang/meta/meta_valid.json`
- `goldens/lyralang/meta/meta_nonstatic.json`

## Surface Forms (Stage 0)
- `meta_eval(expr)` — triggers compile-time evaluation (static folding).
- `quasi_quote(expr)` — creates a quoted AST value.
- `quasi_unquote(expr)` — splices a value back into a quasi-quoted context.

## Static Evaluation
Integer, boolean, and string literals are statically determined.
`result_summary` is `"static_{type}({val})"` for known-static values, or
`"opaque"` for runtime expressions.

## Quasiquote Depth Tracking
`quasi_unquote` outside any `quasi_quote` is an `UnbalancedQuasiQuote` error.
The quote nesting depth is tracked monotonically during the AST walk.
