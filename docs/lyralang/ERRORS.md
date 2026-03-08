# LyraLang Stage 0 Error Handling

## 1. Scope

P1-010 introduces the first executable error-handling law for Stage 0:

- `Option[T]`
- `Result[T, Error[..]]`
- postfix propagation via `?`
- panic-free subset enforcement
- deterministic error-type composition
- stack-trace integration on propagated failures

This slice is intentionally conservative. It is designed to fit the current Stage 0 parser and checker surface without assuming later function declaration syntax.

## 2. Canonical Internal Types

The shared type kernel now includes:

- `Option[T]`
- `Result[T, Error[..]]`
- `Error[label]`
- `StackTrace`

Error labels are ordered, deduplicated, and serialized canonically.

## 3. Postfix Propagation Law

The Stage 0 grammar admits postfix propagation:

```ebnf
postfix_expr = primary_expr , { call_suffix | try_suffix } ;
try_suffix   = "?" ;
```

Propagation is scope-local.

- `operand?` on `Option[T]` yields `T` within the current scope and upgrades the enclosing block/program result to `Option[U]`.
- `operand?` on `Result[T, Error[E]]` yields `T` within the current scope and upgrades the enclosing block/program result to `Result[U, Error[E @trace]]`.
- mixed `Option` and `Result` propagation in the same Stage 0 scope is rejected.

## 4. Error Composition

When multiple `Result` propagation sites occur in one scope, their error labels are unioned in deterministic sorted order.

Example:

- `Error[IoFailure]`
- `Error[DivideByZero]`

compose to:

- `Error[DivideByZero | IoFailure @trace]`

## 5. Stack Trace Integration

Each propagation site contributes one canonical frame:

- line
- column
- stable operand summary

Stage 0 stack traces are compile-time summaries used for diagnostics and proof of propagation shape. Rich runtime traces are deferred.

## 6. Panic-Free Subset

The Stage 0 subset forbids panic-style escape hatches. The analyzer rejects calls such as:

- `panic()`
- `unwrap(...)`
- `expect(...)`

Programs must stay within `Option`/`Result` propagation or explicit branching.

## 7. Determinism Requirements

- propagation sites are recorded in source order
- composed error labels are sorted canonically
- stack-trace frames are stable and span-derived only
- panic-free violations emit deterministic diagnostics
