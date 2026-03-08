# Design — P1-001 Lexical Structure

The lexical specification is intentionally defined in a normative markdown grammar document before code. This keeps Phase 1 aligned with the canon's spec-first requirement and prevents the lexer from inventing undocumented source rules.

## Design Decisions

- Line ending normalization is the only cross-platform source rewrite at the lexical layer.
- Identifier legality follows Unicode XID start/continue classes plus `_`.
- A lone underscore is reserved as a dedicated wildcard token.
- Comments are trivia, but still appear in the token stream for tooling and diagnostics.
- Nested block comments are supported so lexical recovery stays structured.

## Ownership

- Grammar and lexical law: `docs/lyralang/GRAMMAR.md`
- Executable implementation: `lyralang/src/lexer/`
- Shared fixtures/goldens: `fixtures/lyralang/lexer/`, `goldens/lyralang/lexer/`
