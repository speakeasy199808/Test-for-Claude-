# LyraLang Stage 0 Self Reference Primitives

## Purpose

Stage 0 reserves three built-in self-reference primitives that let a program
interrogate its own execution context without consulting any external service.
They are deterministic, offline-first, and canonically typed.

Supported primitives:

- `@current_program()`
- `@current_receipt()`
- `@ledger_state()`

## Surface Form

Self-reference primitives are zero-argument expressions.

```text
self_reference = "@" , ("current_program" | "current_receipt" | "ledger_state") , "(" , ")" ;
```

No arguments are permitted. The `@` prefix is reserved for built-in language
introspection rather than user-defined identifiers.

## Canonical Types

The Stage 0 type kernel assigns fixed metadata types:

- `@current_program() : CurrentProgram`
- `@current_receipt() : CurrentReceipt`
- `@ledger_state() : LedgerState`

These are canonical metadata descriptors. They are values in the type system,
not effect atoms and not external network lookups.

## Determinism Law

Any implementation of these primitives must satisfy all of the following:

1. **Stable ordering** — repeated lowering of the same source emits the same primitive opcode ordering.
2. **Offline sovereignty** — values are sourced from current local execution state only.
3. **Canonical boundaries** — each primitive crosses the compiler/runtime boundary with a versioned, typed representation.
4. **No implicit effects** — the Stage 0 type/effect surface treats self-reference as pure metadata access.

## Stage 0 Execution Strategy

The parser recognizes the `@name()` syntax directly.
The seed type checker maps each primitive to a canonical metadata type.
The seed code generator lowers each primitive to a dedicated self-reference IR instruction:

- `selfref @current_program`
- `selfref @current_receipt`
- `selfref @ledger_state`

## Deferred Surface

Later tasks may extend the payload schemas behind these descriptors, but must not
rewrite the primitive names, remove the `@` prefix, or weaken the determinism law.
