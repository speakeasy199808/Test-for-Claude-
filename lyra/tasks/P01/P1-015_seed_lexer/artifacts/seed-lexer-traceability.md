# Traceability — P1-015 Seed Lexer

## Acceptance to Implementation Map

- Rust lexer exists → `lyralang/src/lexer/`
- Regex-assisted token stream → `lyralang/src/lexer/lexer.rs::scan_integer`
- Unicode identifiers and reserved words → `lyralang/src/lexer/token.rs`, `lyralang/src/lexer/lexer.rs`
- Comments and whitespace → `lyralang/src/lexer/lexer.rs::{scan_line_comment,scan_block_comment,scan_horizontal_whitespace}`
- Recovery diagnostics → `lyralang/src/lexer/error.rs`, `lyralang/src/lexer/lexer.rs::{scan_invalid_character,scan_string,scan_block_comment}`
- Fixture-backed verification → `lyralang/tests/seed_lexer_integration.rs`
