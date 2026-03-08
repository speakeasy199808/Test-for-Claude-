# LyraLang Metaprogramming (P1-027)

## Overview

The LyraLang metaprogramming system provides compile-time code execution, AST
manipulation, and quasiquotation primitives. In Stage 0 the system is
recognized conservatively through three builtin call forms over the existing
parser surface.

## Surface Forms

### `meta_eval(expr)`

Triggers compile-time evaluation (static folding) of `expr`. When the
argument is a compile-time constant (integer, boolean, or string literal),
the result is statically determined and reported in `result_summary`.

```lyra
let x = meta_eval(42)    -- result_summary = "static_int(42)"
let y = meta_eval(true)  -- result_summary = "static_bool(true)"
```

When the argument is not a constant, a `NonStaticMetaEval` error is recorded
and `result_summary = "opaque"`.

### `quasi_quote(expr)`

Creates a quoted AST value from `expr`. The checker records the canonical
`quoted_form` of the expression. While inside a `quasi_quote`, `quasi_unquote`
calls are tracked as splice sites.

```lyra
let q = quasi_quote(x + 1)
```

### `quasi_unquote(expr)`

Splices a value back into a surrounding `quasi_quote` context. Must appear
lexically inside a `quasi_quote`; appearing outside produces an
`UnbalancedQuasiQuote` error.

```lyra
let q = quasi_quote(quasi_unquote(some_expr))
```

## Static Evaluation Rules

| Expression Form | `statically_determined` | `result_summary` |
|-----------------|------------------------|-----------------|
| Integer literal | `true` | `static_int({value})` |
| Boolean literal | `true` | `static_bool({value})` |
| String literal  | `true` | `static_string("{value}")` |
| Anything else   | `false` | `opaque` |

## Quote Depth Tracking

The checker maintains a quote nesting depth counter:
- Entering `quasi_quote` increments the depth.
- Exiting `quasi_quote` decrements the depth.
- `quasi_unquote` at depth 0 → `UnbalancedQuasiQuote` error.

## Error Kinds

| Kind | Condition |
|------|-----------|
| `ParseError` | Source could not be parsed. |
| `NonStaticMetaEval` | `meta_eval` argument is not a compile-time constant. |
| `UnbalancedQuasiQuote` | `quasi_unquote` outside a `quasi_quote` context. |
| `CyclicMetaDependency` | A cyclic meta-level dependency was detected. |

## Checker API

```rust
use lyralang::meta::check;

let output = check(source);
// output.judgment.all_static — true iff every meta_eval was statically determined
// output.errors               — Vec<MetaError>
```

## Stage 0 Scope

Stage 0 recognizes the three surface forms and performs:
- Compile-time evaluation classification (static vs. opaque).
- Quasiquote site recording with unquote tracking.
- AST manipulation record generation.
- Error recovery on non-fatal errors.

Full compile-time execution and AST rewriting are planned for later stages.
