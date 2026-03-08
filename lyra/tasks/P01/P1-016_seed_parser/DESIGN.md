# Design — P1-016 Seed Parser

The seed parser is deliberately conservative: it accepts a stable Stage 0 grammar and builds a span-carrying AST without reaching ahead into type checking or code generation.

## Design Decisions

- Recursive descent handles outer syntactic structure such as programs, blocks, `let`, and `match` arms.
- Pratt parsing provides deterministic precedence and associativity for arithmetic, comparison, and logical expressions.
- The AST keeps grouped constructs and block tails explicit so later semantic phases receive a faithful syntax tree.
- Parsing consumes normalized lexer coordinates and never invents source positions.
- Syntax recovery is boundary-based and deterministic rather than speculative.
