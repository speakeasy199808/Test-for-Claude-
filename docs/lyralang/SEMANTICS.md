# LyraLang Formal Semantics — Stage 0

## Status

This document is the normative Stage 0 semantics law for Phase 1 work.
P1-008 defines both the mathematical specification and the executable semantic
surface used to validate the current parser, type checker, and code generator.

## 1. Purpose

Stage 0 needs a semantic contract that is stronger than syntax and types alone.
This task fixes three things in one place:

- a **denotational** mapping from well-typed Stage 0 programs to canonical values
- an **operational** small-step account of evaluation order
- a **type soundness statement** for the currently executable subset

The current executable subset is intentionally narrow: integers, booleans, blocks,
`if`, `match`, a seed builtin surface, and the three self-reference primitives.

## 2. Judgment Forms

The Stage 0 semantic presentation uses the following canonical judgments.

```text
Γ ⊢ e : τ
Σ ; ρ ⊢ e ⇓ v ; Σ'
[[ e ]]ρ = v
```

Where:

- `Γ` is the typing environment
- `ρ` is the runtime environment
- `Σ` is the deterministic machine state
- `τ` is a canonical kernel type
- `v` is a canonical semantic value

`Σ` is not a wall-clock or ambient host state. It is a deterministic symbolic state
containing only compiler-owned counters and explicit builtin effects.

## 3. Canonical Semantic Domains

Stage 0 semantic values inhabit the following domains.

```text
V ::= Unit
    | Bool(b)
    | Int(z)
    | Rational(n / d)
    | Resource(kind, id)
    | Meta(current_program | current_receipt | ledger_state)
    | Evidence(kind)
    | Modal(modality, v)
```

All domains are canonical and deterministic. No floating-point, ambient entropy,
or host object identity is permitted.

## 4. Denotational Semantics

The denotational meaning of a program is the canonical value of its tail expression
after all prior statements have extended the environment in source order.
A program without a tail expression denotes `Unit`.

### 4.1 Statements

```text
[[ let p = e ]]ρ = ρ[p ↦ [[ e ]]ρ]
[[ e; ]]ρ       = ρ
```

### 4.2 Expressions

```text
[[ n ]]ρ                  = Int(n)
[[ true ]]ρ               = Bool(true)
[[ false ]]ρ              = Bool(false)
[[ x ]]ρ                  = ρ(x)
[[ { s* ; e } ]]ρ         = [[ e ]]ρ'      where ρ' is ρ extended by s*
[[ if c { t } else { f } ]]ρ = [[ t ]]ρ if [[ c ]]ρ = Bool(true), else [[ f ]]ρ
```

The seed builtin surface is denoted by total deterministic functions on the current
semantic domains. Unsupported or deferred surface is outside the Stage 0 executable
subset and must be rejected before semantic execution.

## 5. Operational Semantics

Evaluation is strict and left-to-right.
The executable semantics pass records this as an ordered trace of small steps.

Representative rules:

```text
ρ ⊢ e1 ⇓ v1    ρ ⊢ e2 ⇓ v2
------------------------------------  [E-Add]
ρ ⊢ add(e1, e2) ⇓ Int(v1 + v2)

ρ ⊢ c ⇓ Bool(true)    ρ ⊢ t ⇓ v
------------------------------------  [E-IfTrue]
ρ ⊢ if c { t } else { f } ⇓ v

ρ ⊢ s ⇓ Meta(current_program)
------------------------------------  [E-SelfProgram]
ρ ⊢ @current_program() ⇓ Meta(current_program)
```

The current implementation emits a canonical operational trace with stable step indices,
rule names, and rendered values.

## 6. Determinism Law

For any fixed normalized source, the semantic evaluator must produce exactly one of:

1. a stable diagnostic set, or
2. a stable semantic judgment with identical denotation and step trace.

No ambient host time, random source, pointer identity, hash-map iteration order, or
platform-dependent ordering may affect the result.

## 7. Type Soundness Statement

For the current executable Stage 0 subset:

> If `Γ ⊢ e : τ` and the seed semantic evaluator succeeds with `Σ ; ρ ⊢ e ⇓ v ; Σ'`,
> then `v ∈ [[τ]]`.

This is a Stage 0 soundness statement, not a full metatheoretic proof.
The practical witness is that semantic execution is only entered after successful
parsing and seed type checking, and the evaluator only constructs canonical values
that correspond to the already-established kernel types.

## 8. Relationship to Other Tasks

- P1-003 defines the canonical type algebra consumed by these judgments
- P1-007 provides the self-reference values used in semantic execution
- P1-019 lowers the same well-typed AST to deterministic IR
- P1-020 serializes that IR into canonical LyraVM bytecode

## 9. Deferred Surface

The following remain explicitly out of the Stage 0 executable semantic subset:

- user-defined functions and closures
- trait dispatch
- string runtime values
- heap allocation semantics
- concurrency and scheduling semantics
- proof terms beyond modal/evidence wrappers

Those will extend this document in later tasks without rewriting the canonical Stage 0 laws above.
