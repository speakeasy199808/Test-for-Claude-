# Implementation — P1-015 Seed Lexer

## Implemented Module Family

- `span.rs` — normalized source positions and spans
- `token.rs` — token/keyword definitions and stable labels
- `error.rs` — lexical diagnostics
- `lexer.rs` — normalization, scanning, recovery, and unit tests
- `mod.rs` — module surface and re-exports
- `lyralang/tests/seed_lexer_integration.rs` — fixture-backed integration coverage

## Recovery Strategy

- Invalid characters produce a recoverable diagnostic and `Error` token.
- Unterminated block comments produce a recoverable diagnostic and `Error` token.
- Unterminated strings produce a recoverable diagnostic and `Error` token.

## Follow-on Tasks Enabled

- P1-002 grammatical syntax
- P1-016 seed parser
- P1-031 source map generation
