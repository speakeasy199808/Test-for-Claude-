# P1-015 — Seed Lexer

## Mission
Rust lexer for Lyra. Regex-based token stream. Error recovery. Handles all `lexical_structure` tokens.

## Scope
- Deterministic source normalization
- Unicode-aware identifiers and reserved words
- Comment and whitespace trivia tokens
- Foundational literals and punctuation for parser bring-up
- Recoverable diagnostics for invalid characters and unterminated constructs
- Unit and integration tests over shared fixtures

## Primary Ownership Root
`lyralang/`

## Deliverables
- `lyralang/src/lexer/mod.rs`
- `lyralang/src/lexer/span.rs`
- `lyralang/src/lexer/token.rs`
- `lyralang/src/lexer/error.rs`
- `lyralang/src/lexer/lexer.rs`
- `lyralang/tests/seed_lexer_integration.rs`
- Task control-plane records and traceability artifacts
