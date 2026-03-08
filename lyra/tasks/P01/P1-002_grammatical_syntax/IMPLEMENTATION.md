# Implementation — P1-002 Grammatical Syntax

This task extends the grammar document from lexical law into a normative Stage 0 syntax contract.

## Implemented Assets

- Extended `docs/lyralang/GRAMMAR.md` with program, block, expression, `if`, `match`, and pattern grammar.
- Added operator precedence and associativity rules.
- Added parser fixtures and syntax goldens for valid and invalid source.
- Added task-local traceability linking grammar clauses to parser surfaces.

## Notes

- Type grammar is intentionally deferred to P1-003 and later.
- Pattern guards, destructuring, and exhaustiveness rules are intentionally deferred.
