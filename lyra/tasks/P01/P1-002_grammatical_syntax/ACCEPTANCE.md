# Acceptance — P1-002 Grammatical Syntax

## Acceptance Criteria

1. `docs/lyralang/GRAMMAR.md` normatively defines the Stage 0 expression grammar in EBNF.
2. The document explicitly specifies an expression-oriented language surface.
3. Blocks are defined as expressions with optional tail expressions.
4. Pattern matching syntax and seed pattern forms are defined.
5. Operator precedence and associativity are recorded deterministically.
6. Shared parser fixtures and goldens exist for valid and invalid syntax.
7. The specification is sufficient to drive the seed parser without hidden syntax rules.

## Verification Method
- Review `docs/lyralang/GRAMMAR.md`
- Inspect shared parser fixtures and goldens
- Confirm P1-016 parser implementation consumes this specification

## Evidence Required
- `docs/lyralang/GRAMMAR.md`
- `fixtures/lyralang/parser/*`
- `goldens/lyralang/parser/*`
- `artifacts/grammatical-syntax-traceability.md`
