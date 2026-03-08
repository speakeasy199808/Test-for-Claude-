# P1-016 — Seed Parser

## Mission
Recursive descent parser with Pratt parsing for expressions. AST output. Every node carries source span.

## Scope
- Deterministic recursive descent parser over Stage 0 grammar
- Pratt parsing for infix and prefix expressions
- AST output for programs, statements, expressions, and patterns
- Span-carrying nodes derived from normalized lexer coordinates
- Fixture-backed parser integration tests and diagnostics

## Primary Ownership Root
`lyralang/`

## Deliverables
- `lyralang/src/parser/mod.rs`
- `lyralang/src/parser/ast.rs`
- `lyralang/src/parser/error.rs`
- `lyralang/src/parser/parser.rs`
- `lyralang/tests/seed_parser_integration.rs`
- Task control-plane records and traceability artifacts
