# Design — P1-018 Seed Effect Checker

The first effect checker is intentionally narrow but executable. It reasons about the current parser AST,
shared builtin callable contracts, and an optional policy ceiling. This keeps the implementation grounded
in the current language surface while still exercising the real effect algebra from P1-004.

## Design Decisions

- Parsing remains the gateway; parse diagnostics are promoted into effect-checker diagnostics when needed.
- Effect inference is structural and deterministic over the current AST.
- Policy violations are reported at the call site when possible for clearer explanations.
- Let-bound identifiers are tracked as values only; user-defined callables remain deferred until later syntax tasks.
- The public output is a compact program judgment suited for golden testing.

## Current Executable Coverage

- literals and identifiers
- blocks and tail expressions
- `if` expressions
- `match` expressions
- calls to predeclared builtin functions
- policy validation including linear-effect distinctions

## Deferred Coverage

- user-written effect annotations
- user-defined callable values
- effect variables or row polymorphism
- advanced capability flow tied to linear types
