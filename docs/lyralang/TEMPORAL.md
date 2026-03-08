
# LyraLang Stage 0 Temporal Logic

## 1. Scope

P1-023 introduces the first executable linear-temporal-logic surface for
Stage 0. The current parser-ready operator surface is expressed through
conservative builtin calls:

- `always(expr)`
- `eventually(expr)`
- `until(lhs, rhs)`
- `since(lhs, rhs)`

These operators are built into the compiler pipeline now, while richer
symbolic syntax remains deferred to later grammar-extension work.

## 2. Canonical Internal Type

The shared type kernel now includes:

- `Temporal[T]`

The executable Stage 0 temporal slice currently uses `Temporal[Bool]` as its
proposition carrier.

## 3. Operator Law

Stage 0 temporal operators are typed as follows:

- `always(Bool) -> Temporal[Bool]`
- `eventually(Bool) -> Temporal[Bool]`
- `until(Temporal[Bool], Temporal[Bool]) -> Temporal[Bool]`
- `since(Temporal[Bool], Temporal[Bool]) -> Temporal[Bool]`

The operators are recorded in source order and normalized into canonical
formula strings for fixtures, diagnostics, and future proof obligations.

## 4. Canonical Formula Normalization

The checker emits deterministic normalized formulas such as:

- `always(eq(1, 1))`
- `eventually(eq(2, 2))`
- `since(eventually(eq(2, 2)), always(eq(1, 1)))`
- `until(always(eq(1, 1)), since(eventually(eq(2, 2)), always(eq(1, 1))))`

Normalization is source-derived only. No runtime model checking is performed
in this slice.

## 5. Built-In Temporal Logic

Temporal operators are part of the language pipeline rather than library-only
helpers:

- the builtin signatures live in the shared compiler environment
- the type kernel has an explicit `Temporal[T]` constructor
- the temporal checker emits first-class operator judgments

This satisfies the built-in requirement while keeping the executable surface
coherent with the current parser.
