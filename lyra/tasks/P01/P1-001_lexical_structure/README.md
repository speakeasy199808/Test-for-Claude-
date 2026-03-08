# P1-001 — Lexical Structure

## Mission
Unicode identifiers, reserved words, comments, whitespace normalization. Written as formal spec in `docs/lyralang/GRAMMAR.md` before implementation.

## Scope
- Unicode-aware identifier rules
- Reserved word catalog for Stage 0
- Line and block comment forms
- Deterministic whitespace normalization and newline handling
- Shared fixtures and goldens for lexical behavior

## Primary Ownership Root
`lyralang/`

## Deliverables
- `docs/lyralang/GRAMMAR.md`
- `fixtures/lyralang/lexer/lexical_sample.lyra`
- `fixtures/lyralang/lexer/lexical_invalid.lyra`
- `goldens/lyralang/lexer/lexical_sample.tokens.json`
- `goldens/lyralang/lexer/lexical_invalid.diagnostics.json`
- Task control-plane records and traceability artifacts
