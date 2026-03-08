# Traceability — P1-001 Lexical Structure

## Spec to Asset Map

- Source normalization → `docs/lyralang/GRAMMAR.md` §1 and `lyralang/src/lexer/lexer.rs::normalize_source`
- Unicode identifiers → `docs/lyralang/GRAMMAR.md` §2 and `lyralang/src/lexer/lexer.rs::scan_identifier_or_keyword`
- Reserved words → `docs/lyralang/GRAMMAR.md` §3 and `lyralang/src/lexer/token.rs::Keyword`
- Comments → `docs/lyralang/GRAMMAR.md` §4 and `lyralang/src/lexer/lexer.rs::{scan_line_comment,scan_block_comment}`
- Whitespace normalization → `docs/lyralang/GRAMMAR.md` §5 and `lyralang/src/lexer/lexer.rs::{normalize_source,scan_horizontal_whitespace}`
- Deterministic fixtures/goldens → `fixtures/lyralang/lexer/*`, `goldens/lyralang/lexer/*`
