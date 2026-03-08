# LyraLang Type System Kernel — Stage 0

## Status

This document is the normative Stage 0 type-system kernel for Phase 1 work.
P1-003 defines the canonical internal type algebra and Hindley-Milner base consumed by P1-017.
Concrete user-written type annotations remain deferred to later syntax tasks.

## 1. Purpose

The Stage 0 kernel defines a deterministic semantic type language that is independent of parser sugar.
It exists so the seed type checker can reason about programs before the full annotation surface, traits,
effects, lifetimes, and proofs are implemented.

## 2. Primitive Types

The primitive kernel types are:

- `Unit`
- `Bool`
- `Int`
- `Nat`
- `Rational`

### 2.1 Primitive Notes

- `Unit` is the type of an expression that returns no meaningful value.
- `Bool` is the type of `true` and `false`.
- `Int` is the signed integer type used by the seed arithmetic surface.
- `Nat` is the non-negative integer kernel type.
- `Rational` is the exact ratio kernel type.

Stage 0 parsing currently exposes integer literals directly; concrete literal forms for `Nat` and `Rational`
are deferred. The kernel still reserves them now so later phases extend a stable semantic base rather than
rewriting it.

## 2.2 Linear Resource Types

The Stage 0 kernel also reserves three linear resource types:

- `File`
- `Socket`
- `Capability`

These are exact-once ownership types consumed by P1-005. Their operational law is specified in
`docs/lyralang/LINEARITY.md`.

## 3. Structural Types

The kernel defines three structural type constructors.

### 3.1 Product Types

A product type combines ordered component types.

```text
Product[T1, T2, ..., Tn]
```

The ordering of product components is semantically significant and must remain stable.

### 3.2 Sum Types

A sum type combines alternative member types.

```text
Sum[T1, T2, ..., Tn]
```

The ordering of sum members is canonical and deterministic.

### 3.3 Function Types

A function type consists of ordered parameter types, an explicit effect set, and a return type.

```text
Fn((T1, T2, ..., Tn) -{effects}-> R)
```

Even though the full effect system is deferred to P1-004, the function kernel already reserves the effect slot
so the seed type checker can attach effect information without changing the function representation later.

## 4. Type Variables

The Hindley-Milner base introduces deterministic type variables.

- type variables are compiler-generated, not surface syntax in Stage 0
- variable identifiers are allocated in stable creation order
- variable identities are local to the current inference session

Canonical textual form uses `t0`, `t1`, `t2`, ... in allocation order.

## 5. Type Schemes

A polymorphic binding is represented as a type scheme.

```text
forall t0, t1, ... . T
```

A scheme quantifies zero or more type variables over a monotype body.
Bindings without quantified variables are monomorphic.

## 6. Hindley-Milner Base Operations

The seed checker uses the standard HM base operations.

### 6.1 Instantiation

When a polymorphic binding is referenced, each quantified variable is replaced with a fresh type variable.

### 6.2 Generalization

When a `let` binding is formed, the inferred monotype is generalized over variables that are free in the value
but not free in the surrounding environment.

### 6.3 Unification

Type equality constraints are solved by deterministic unification.

The unifier must:

- process constraints in deterministic source order
- preserve stable variable numbering and substitution order
- reject cyclic substitutions via occurs checks
- emit stable diagnostic messages for mismatches

## 7. Effect Extension Slot

Stage 0 reserves an explicit effect set on function types.
For the current seed checker this effect set is intentionally small and deterministic.
A pure function has the empty effect set.
Later tasks extend the effect algebra without changing the enclosing function-type contract.
The normative Stage 0 effect law is specified in `docs/lyralang/EFFECTS.md`.

## 8. Current Seed Checker Coverage

P1-017 consumes this kernel for the currently executable syntax surface:

- integer and boolean expressions
- blocks with tail expressions
- `if` expressions
- `match` expressions
- pure builtin function calls
- `let`-generalized bindings

The parser currently accepts string literals as syntax, but string typing is intentionally deferred.
The type checker shall therefore reject string-typed programs with explicit diagnostics until the string/text
kernel is added in a later task.

## 9. Determinism Requirements

The type kernel and any consumer pass must:

- allocate type variables in a stable order
- iterate bindings in deterministic order
- serialize types in canonical textual form
- keep effect sets canonically sorted
- emit diagnostics in stable source order
- remain independent of wall-clock time, host locale, and network state

## 10. Deferred Surface

Later Phase 1 tasks extend this kernel with:

- user-written type annotations and declarations
- effect rows and effect inference law
- linear resource typing and exact-once discharge
- modal typing
- traits and type-class-like resolution
- lifetimes, proofs, and type-level computation


## Modal Extension

P1-006 extends the shared kernel with modality-qualified types and canonical evidence tokens. See `docs/lyralang/MODALITY.md` for the Stage 0 law governing `Fact[T]`, `Hypothesis[T]`, `Unknown[T]`, `Necessary[T]`, and `Possible[T]`.


## Self-Reference Metadata Types

P1-007 extends the shared kernel with canonical metadata descriptor types used by the self-reference primitives:

- `CurrentProgram`
- `CurrentReceipt`
- `LedgerState`

These types are produced by `@current_program()`, `@current_receipt()`, and `@ledger_state()` respectively. See `docs/lyralang/SELF_REFERENCE.md` for the normative execution law.


## Error-Handling Extension

P1-010 extends the shared type kernel with:

- `Option[T]`
- `Result[T, Error[..]]`
- `Error[label]` with deterministic composition
- `StackTrace`

The normative propagation and composition law is specified in `docs/lyralang/ERRORS.md`.

## Structured Concurrency and Temporal Kernel Extensions

The Stage 0 internal kernel now includes:

- `Task[T]`
- `Channel[T]`
- `Temporal[T]`

These are currently surfaced through builtin-backed executable forms so they remain coherent with the existing parser and checker slices.
