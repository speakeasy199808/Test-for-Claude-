# Acceptance — P1-015 Seed Lexer

## Acceptance Criteria

1. A Rust lexer exists in `lyralang/src/lexer/`.
2. The lexer normalizes line endings deterministically before tokenization.
3. The lexer classifies Unicode identifiers and reserved words according to P1-001.
4. Line comments, block comments, and whitespace trivia are emitted in deterministic source order.
5. The lexer recovers from invalid characters and unterminated strings/comments while continuing to scan later tokens.
6. All lexical-structure tokens required by the spec are handled.
7. Unit and integration tests cover valid Unicode input, reserved words, trivia, and recovery diagnostics.

## Verification Method
- `cargo test -p lyralang`
- Inspect shared fixtures and integration tests
- Compare outputs against lexer goldens

## Evidence Required
- `lyralang/src/lexer/*`
- `lyralang/tests/seed_lexer_integration.rs`
- `fixtures/lyralang/lexer/*`
- `goldens/lyralang/lexer/*`
- `artifacts/seed-lexer-traceability.md`
