# P1-029 — REPL Implementation

## Summary

Implements an interactive read-eval-print loop engine for LyraLang as a
library module (`lyralang::repl`). The REPL evaluates expressions by
running the full compiler pipeline (parser → type checker → semantics) and
maintains session state across multiple invocations.

## Module

`lyralang/src/repl/mod.rs`

## Public surface

| Item | Kind | Description |
|------|------|-------------|
| `Repl` | struct | Stateful REPL engine |
| `Repl::new()` | fn | Create a fresh engine |
| `Repl::evaluate(&mut self, input: &str) -> ReplOutput` | fn | Evaluate one line |
| `Repl::get_completions(&self, partial: &str) -> Vec<CompletionSuggestion>` | fn | Tab completions |
| `Repl::inspect_state(&self) -> ReplState` | fn | Current session state |
| `Repl::reset(&mut self)` | fn | Clear session |
| `ReplOutput` | struct | Bundled evaluation result |
| `ReplResult` | enum | Payload: Value / Unit / Error / TypeQuery / StateQuery / Reset |
| `ReplSession` | struct | History + bindings accumulator |
| `ReplSessionSnapshot` | struct | Lightweight snapshot of session state |
| `ReplBinding` | struct | A recorded let binding |
| `ReplHistoryEntry` | struct | One entry in evaluation history |
| `ReplState` | struct | Inspect-state snapshot |
| `CompletionSuggestion` | struct | A tab-completion candidate |
| `CompletionKind` | enum | Keyword / Builtin / Binding / Type |
| `ReplError` | struct | REPL-level diagnostic |
| `ReplErrorKind` | enum | ParseError / TypeError / EvalError |

## Meta-commands

| Command | Behaviour |
|---------|-----------|
| `:type <expr>` | Return canonical type of `<expr>` |
| `:state` | Dump current bindings and eval count |
| `:reset` | Clear all bindings and history |

## Dependencies

- `crate::checker::check` — type inference
- `crate::semantics::analyze` — operational evaluation

## Fixtures

- `fixtures/lyralang/repl/repl_session.json`
- `fixtures/lyralang/repl/repl_completions.json`

## Goldens

- `goldens/lyralang/repl/repl_session.json`
