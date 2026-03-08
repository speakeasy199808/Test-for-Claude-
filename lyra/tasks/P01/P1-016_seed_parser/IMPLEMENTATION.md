# Implementation — P1-016 Seed Parser

## Implemented Module Family

- `ast.rs` — program, statement, expression, pattern, and operator nodes
- `error.rs` — parser diagnostics and parse output bundle
- `parser.rs` — recursive descent, Pratt parsing, block/tail handling, and syntax recovery
- `mod.rs` — module surface and re-exports
- `lyralang/tests/seed_parser_integration.rs` — fixture-backed parser integration coverage

## Recovery Strategy

- Lexical diagnostics are promoted into parser diagnostics and stop AST construction.
- Statement parsing recovers to newline, `;`, `}`, or end-of-file boundaries.
- Match arm parsing recovers to comma, newline, `}`, or end-of-file boundaries.

## Follow-on Tasks Enabled

- P1-017 seed type checker
- P1-022 seed test framework
- P1-031 source map generation
- P1-032 language integration
