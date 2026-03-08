# LyraLang Linear Types — Stage 0

## Status

This document is the normative Stage 0 linear-ownership law for Phase 1 work.
P1-005 defines the canonical resource types and the exact-once discharge rules consumed by the
seed linear checker.

## 1. Purpose

LyraLang uses linear types to model owned resources that must be discharged exactly once.
The Stage 0 goal is intentionally narrow but executable:

- `File`, `Socket`, and `Capability` are first-class resource types in the kernel
- moving a linear binding transfers ownership
- dropping, duplicating, or ambiguously branching ownership is rejected at compile time
- runtime bookkeeping is not required for exact-once enforcement

## 2. Linear Resource Types

The Stage 0 kernel reserves three canonical resource types:

- `File`
- `Socket`
- `Capability`

These names are semantic kernel types, not host handles.
Later tasks may refine their capabilities, lifetimes, and borrow behavior, but must not rewrite the
canonical names.

## 3. Exact-Once Ownership Law

A value of linear type must be discharged exactly once within its ownership scope.
Valid discharge actions in Stage 0 are explicit builtin contracts.

### 3.1 Moves

Referencing a linear binding moves the value out of that binding.
After the move, the original binding is consumed and cannot be used again.

### 3.2 Discharge

A moved linear value must be:

- consumed by a builtin sink such as `close_file`, `close_socket`, or `consume_capability`, or
- forwarded through an explicit passthrough contract such as `id`, then eventually consumed

A linear value may not be:

- discarded via `_`
- dropped by a terminated expression statement
- left outstanding at scope exit
- used twice
- consumed on one branch and not the other

## 4. Branch Consistency

Control flow must preserve a single deterministic ownership state.
For `if` and `match`:

- every branch must leave the same set of outstanding linear bindings
- every branch must return the same linear resource kind, or no linear resource at all

This prevents ownership ambiguity without runtime reference counting or tracing.

## 5. Stage 0 Builtin Ownership Contracts

The executable Stage 0 surface reserves these builtin contracts:

- `open_file : Fn(() -{io}-> File)`
- `close_file : Fn((File) -{io}-> Unit)`
- `open_socket : Fn(() -{io}-> Socket)`
- `close_socket : Fn((Socket) -{io}-> Unit)`
- `grant_capability : Fn((Int) -{proof}-> Capability)`
- `consume_capability : Fn((Capability) -{state!}-> Unit)`
- `id : Fn((t0) -{pure}-> t0)`

`id` is a pure passthrough at the type level and a deterministic ownership forwarder at the
linearity layer.

## 6. Interaction with the Type Kernel

Linear resource types are kernel types, not effect atoms.
They coexist with the effect system:

- effects describe observable interaction
- linear types describe ownership obligations over values

A function type may therefore both carry effects and consume or produce resource-typed values.

## 7. Determinism Requirements

Any consumer of the Stage 0 linearity law must:

- preserve lexical/source order when diagnosing ownership failures
- avoid host-dependent or runtime-dependent ownership heuristics
- treat exact-once discharge as a compile-time property
- serialize resource kinds using canonical names

## 8. Deferred Surface

Later tasks extend this law with:

- borrow-checking semantics for non-linear references
- lifetime elision and region inference
- richer capability subtyping
- linear pattern destructuring and user-defined linear functions
- FFI-safe ownership bridges
