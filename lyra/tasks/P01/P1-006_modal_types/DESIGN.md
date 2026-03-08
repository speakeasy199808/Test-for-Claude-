# Design — P1-006 Modal Types

## Design Summary

Stage 0 modal typing is implemented in three layers:

- the shared type kernel owns modal and evidence types
- the shared builtin environment defines all allowed introductions/promotions/eliminations
- a seed modal checker walks the typed AST and records explicit promotions

This keeps modal law executable immediately without waiting for dedicated source syntax.

## Why Builtin-Backed Promotion

The existing Stage 0 parser already supports identifiers and call expressions.
Encoding promotion through builtins lets the compiler enforce modal law now while
preserving a clean migration path to richer surface syntax later.

## Current Surface Limits

- no dedicated modal syntax yet
- promotion must flow through explicit builtin names
- evidence is modeled as canonical token types, not external proof objects yet
- the modal checker is intentionally audit-oriented: it records modal facts and promotions on top of the seed type checker
