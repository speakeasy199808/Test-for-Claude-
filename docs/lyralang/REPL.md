# LyraLang REPL (P1-029)

## Overview

The `lyralang::repl` module provides a library-level REPL engine for LyraLang.
It is **not** a binary — it exposes a `Repl` struct that can be wired up to any
I/O surface (stdin/stdout, a web UI, an embedded interpreter, etc.).

Each call to `Repl::evaluate` runs the full compiler pipeline:

```
input → parser::parse → checker::check → semantics::analyze → ReplOutput
```

Session state (bindings, history, eval count) persists across calls inside the
`Repl` struct.

## Quick start

```rust
use lyralang::repl::Repl;

let mut repl = Repl::new();

let out = repl.evaluate("let x = 42");
// out.result == ReplResult::Value("42")
// out.type_info == Some("Int")

let out = repl.evaluate(":type true");
// out.result == ReplResult::TypeQuery("Bool")

let out = repl.evaluate(":state");
// out.result == ReplResult::StateQuery(ReplState { bindings: [..], eval_count: 1 })

repl.evaluate(":reset");
// Session cleared
```

## Meta-commands

| Command | Description |
|---------|-------------|
| `:type <expr>` | Type-check `<expr>` and return its canonical type |
| `:state` | Dump all accumulated bindings and the evaluation count |
| `:reset` | Clear all bindings, history, and the evaluation counter |

## Tab completion

`Repl::get_completions(partial)` returns suggestions that match `partial`:

- **Keywords** — `let`, `if`, `match`, `fn`, `true`, `false`
- **Builtins** — `add`, `eq`, `some`, `ok`, `err`, `spawn`, `join`
- **Session bindings** — any `let` binding defined in the current session

```rust
let suggestions = repl.get_completions("a");
// [CompletionSuggestion { text: "add", kind: Builtin, .. }]
```

## Error handling

Parse and type errors are returned as `ReplResult::Error(Vec<String>)`. The
evaluation counter is still incremented, but no binding is recorded.

## Session inspection

`Repl::inspect_state()` returns a `ReplState` with:

- `bindings` — all `let` bindings introduced so far
- `eval_count` — total number of evaluations (including errors)

## Types reference

See `interfaces/specs/lyralang_repl_v1.json` for the machine-readable type spec.

## Fixtures

- `fixtures/lyralang/repl/repl_session.json` — sample session transcript
- `fixtures/lyralang/repl/repl_completions.json` — sample completions

## Goldens

- `goldens/lyralang/repl/repl_session.json`
