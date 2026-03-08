# Traceability — P1-016 Seed Parser

## Acceptance to Implementation Map

- Recursive descent parser → `lyralang/src/parser/parser.rs::{parse_program,parse_module_decl,parse_let_statement,parse_block_expression,parse_match_expression}`
- Pratt expression parsing → `lyralang/src/parser/parser.rs::parse_expression`
- AST output → `lyralang/src/parser/ast.rs`
- Every node carries source span → `lyralang/src/parser/ast.rs`, `lyralang/src/parser/parser.rs`
- Fixture-backed verification → `lyralang/tests/seed_parser_integration.rs`
- Syntax diagnostics and recovery → `lyralang/src/parser/error.rs`, `lyralang/src/parser/parser.rs`
