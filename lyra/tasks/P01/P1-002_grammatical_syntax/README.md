# P1-002 — Grammatical Syntax

## Mission
Expression-oriented language. Blocks as expressions. Pattern matching. EBNF grammar document.

## Scope
- Expression-oriented Stage 0 surface
- Blocks as expressions with tail expression semantics
- Pattern matching syntax and seed pattern grammar
- Operator precedence and associativity law
- EBNF grammar document in `docs/lyralang/GRAMMAR.md`

## Primary Ownership Root
`lyralang/`

## Deliverables
- `docs/lyralang/GRAMMAR.md`
- `fixtures/lyralang/parser/parser_sample.lyra`
- `fixtures/lyralang/parser/parser_invalid.lyra`
- `goldens/lyralang/parser/parser_sample.ast.json`
- `goldens/lyralang/parser/parser_invalid.diagnostics.json`
- Task control-plane records and traceability artifacts
