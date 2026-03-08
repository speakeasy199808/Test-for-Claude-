# Acceptance — P1-001 Lexical Structure

## Acceptance Criteria

1. A formal grammar document exists at `docs/lyralang/GRAMMAR.md` before lexer implementation.
2. The document normatively defines source normalization, identifiers, reserved words, comments, and whitespace behavior.
3. Unicode-aware identifier rules are explicit and deterministic.
4. Reserved words are listed and stable for Stage 0.
5. Comment behavior includes line comments, block comments, and unterminated-block diagnostics.
6. Shared fixtures and goldens exist for valid and invalid lexical input.
7. The specification is sufficient to drive the seed lexer implementation without hidden lexical rules.

## Verification Method
- Review `docs/lyralang/GRAMMAR.md`
- Inspect shared fixtures and goldens
- Confirm P1-015 lexer implementation consumes this specification

## Evidence Required
- `docs/lyralang/GRAMMAR.md`
- `fixtures/lyralang/lexer/*`
- `goldens/lyralang/lexer/*`
- `artifacts/lexical-structure-traceability.md`
