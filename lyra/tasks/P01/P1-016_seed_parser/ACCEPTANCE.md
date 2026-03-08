# Acceptance — P1-016 Seed Parser

## Acceptance Criteria

1. A Rust parser exists in `lyralang/src/parser/`.
2. The parser uses recursive descent with Pratt parsing for expressions.
3. The parser emits a structured AST for programs, statements, expressions, and patterns.
4. Every emitted AST node carries a source span derived from normalized source coordinates.
5. Blocks parse as expressions with statements plus optional tail expressions.
6. `match` expressions parse ordered arms and body expressions.
7. Integration tests cover valid syntax, precedence, blocks, and syntax diagnostics.

## Verification Method
- `cargo test -p lyralang`
- Inspect shared parser fixtures and integration tests
- Compare AST/diagnostic expectations against parser goldens

## Evidence Required
- `lyralang/src/parser/*`
- `lyralang/tests/seed_parser_integration.rs`
- `fixtures/lyralang/parser/*`
- `goldens/lyralang/parser/*`
- `artifacts/seed-parser-traceability.md`
