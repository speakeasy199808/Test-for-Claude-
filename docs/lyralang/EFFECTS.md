# LyraLang Effect System — Stage 0

## Status

This document is the normative Stage 0 effect law for Phase 1 work.
P1-004 defines the canonical effect algebra and effect subtyping. P1-018 consumes the same
law in the seed effect checker.

## 1. Purpose

LyraLang tracks observable interaction as explicit effects.
Effects are part of function types and part of program judgments.
The Stage 0 goal is modest but executable: a deterministic effect algebra, subset-based effect
subtyping, a linear-effect distinction, and stable inference rules over the current AST.

## 2. Effect Atoms

The Stage 0 kernel reserves the following effect atoms:

- `io`
- `state`
- `time`
- `entropy`
- `proof`

These names are canonical and versioned. Later phases may add more atoms, but must not rewrite
existing spellings.

## 3. Persistent and Linear Effects

Stage 0 distinguishes two classes of effects.

### 3.1 Persistent Effects

A persistent effect may be combined by ordinary set union.
Persistent atoms are written plainly.

Examples:

```text
io
proof
```

### 3.2 Linear Effects

A linear effect records a use-once or capability-sensitive interaction at the effect layer.
Stage 0 uses deterministic bookkeeping only; later linear-type tasks will connect this to richer
resource typing.

Linear atoms are written with a `!` suffix.

Examples:

```text
state!
io!
```

### 3.3 Canonical Ordering

Effect sets are serialized in deterministic sorted order:

1. persistent atoms by canonical atom name
2. linear atoms by canonical atom name with `!` suffix

Examples:

```text
pure
entropy,io,proof,time,state!
io,state!
```

## 4. Function Types Carry Explicit Effects

The canonical function form remains:

```text
Fn((T1, T2, ..., Tn) -{effects}-> R)
```

Examples:

```text
Fn((Int) -{io}-> Unit)
Fn((Int) -{state!}-> Int)
Fn((Int, Int) -{proof}-> Bool)
Fn((Int) -{pure}-> Int)
```

The effect payload is part of the function type contract.
It is never implicit in canonical type form.

## 5. Effect Subtyping

Effect subtyping is subset-based and deterministic.

Given effect sets `A` and `B`:

```text
A ≤ B  iff
  persistent(A) ⊆ persistent(B)
  and linear(A) ⊆ linear(B)
```

Important consequences:

- `pure` is a subtype of every effect set
- `io ≤ io,time`
- `state!` is **not** a subtype of `state`
- `state` is **not** a subtype of `state!`

Persistent and linear membership are compared independently.
The same atom name in persistent and linear position represents different effect obligations.

## 6. Effect Inference Rules

The Stage 0 seed checker infers effects directly from the current AST.

### 6.1 Pure Expressions

The following are effect-free:

- identifiers
- integer literals
- boolean literals
- string literals at the effect layer
- grouped expressions

### 6.2 Structural Composition

- block effect = union of statement effects and optional tail-expression effects
- `if` effect = condition ∪ then-branch ∪ else-branch
- `match` effect = scrutinee ∪ union of arm-body effects
- prefix and binary expression effect = union of operand effects

### 6.3 Calls

A call expression contributes:

```text
effects(callee) ∪ effects(arguments...) ∪ latent_effects(callee)
```

For the Stage 0 seed surface, builtin callable signatures provide the latent effect set.

## 7. Seed Builtin Surface

The seed effect checker and seed type checker share the following executable builtin contracts:

- `id : Fn((t0) -{pure}-> t0)`
- `add : Fn((Int, Int) -{pure}-> Int)`
- `eq_int : Fn((Int, Int) -{pure}-> Bool)`
- `not : Fn((Bool) -{pure}-> Bool)`
- `nat_succ : Fn((Nat) -{pure}-> Nat)`
- `ratio_from_ints : Fn((Int, Int) -{pure}-> Rational)`
- `print_int : Fn((Int) -{io}-> Unit)`
- `read_clock : Fn(() -{time}-> Int)`
- `entropy_u64 : Fn(() -{entropy}-> Int)`
- `touch_state : Fn((Int) -{state!}-> Int)`
- `prove_eq_int : Fn((Int, Int) -{proof}-> Bool)`
- `open_file : Fn(() -{io}-> File)`
- `close_file : Fn((File) -{io}-> Unit)`
- `open_socket : Fn(() -{io}-> Socket)`
- `close_socket : Fn((Socket) -{io}-> Unit)`
- `grant_capability : Fn((Int) -{proof}-> Capability)`
- `consume_capability : Fn((Capability) -{state!}-> Unit)`

## 8. Policy Ceilings and Violations

A seed effect policy is an allowed effect ceiling.
A program or call site violates policy when its required effect set is not a subtype of the ceiling.
Diagnostics must explain:

- the required effect set
- the allowed effect set
- the missing effect obligations

## 9. Determinism Requirements

Any consumer of the effect system must:

- keep effect atoms in canonical sorted order
- preserve stable diagnostic wording for equivalent failures
- avoid wall-clock or host-dependent effect classification
- treat policy checking as pure set comparison, never heuristic ranking

## 10. Deferred Surface

Later tasks extend this with:

- source-level effect annotations
- effect variables and richer rows
- linear-type coupling to resource types
- capability-safe FFI effects
- concurrency-specific effect law
