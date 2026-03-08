# Design — P1-015 Seed Lexer

The seed lexer is intentionally deterministic and span-aware. It is not yet the full compiler front-end, but it establishes the stable token stream that later parser and typechecker tasks will consume.

## Design Decisions

- Source is normalized once up front so line ending behavior is platform-independent.
- Regex is used for bounded token classes such as integers; Unicode identifier legality uses `unicode-ident` because XID handling is part of the language contract.
- Trivia remains in the token stream to support diagnostics, tooling, and later source-map work.
- Error recovery emits `Error` tokens plus deterministic diagnostics rather than aborting at the first issue.
- Spans are defined over normalized source and use stable byte offsets plus line/column coordinates.
