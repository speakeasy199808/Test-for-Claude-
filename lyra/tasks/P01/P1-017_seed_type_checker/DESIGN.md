# Design — P1-017 Seed Type Checker

The first type checker is intentionally narrow but executable. It consumes the parser AST exactly as it exists now,
rather than speculating about later syntax such as function declarations or user-written annotations.

## Design Decisions

- Parsing remains the gateway; parse diagnostics are promoted into checker diagnostics when needed.
- Inference is Hindley-Milner style with deterministic type-variable allocation.
- Equality constraints are solved online through a stable unifier rather than a nondeterministic solver.
- Function types already include an effect slot, but the seed checker treats the current builtin surface as pure.
- Program results are emitted as a compact judgment bundle suited for golden testing.

## Current Executable Coverage

- integer and boolean expressions
- blocks and tail expressions
- `if` expressions
- `match` expressions with seed patterns
- calls to predeclared builtin functions
- `let`-bound generalization

## Deferred Coverage

- source-level function declarations
- user-written type annotations
- effect rows and effect variables
- string/text typing
- trait resolution and advanced pattern typing
