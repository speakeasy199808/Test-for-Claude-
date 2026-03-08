# Design — P1-002 Grammatical Syntax

The grammatical surface is still intentionally narrow, but it is now executable. The Stage 0 syntax law is just large enough to support real expression parsing, tail-position blocks, and `match`-based control flow.

## Design Decisions

- The language remains expression-oriented even in the minimal seed compiler.
- Blocks can carry terminated statements plus an optional trailing expression.
- `if` and `match` are expressions so later semantic phases inherit a consistent shape.
- The EBNF is written as the canonical source of truth before or alongside parser implementation updates.
- Pattern grammar is intentionally small and defers destructuring or guards to later tasks.

## Ownership

- Normative grammar law: `docs/lyralang/GRAMMAR.md`
- Executable syntax consumer: `lyralang/src/parser/`
- Shared fixtures/goldens: `fixtures/lyralang/parser/`, `goldens/lyralang/parser/`
