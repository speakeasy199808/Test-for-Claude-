# LyraLang Trait System — Stage 0 Seed Registry

## Status

This document freezes the first executable trait/typeclass baseline for LyraLang.
Full source-level trait declarations are deferred; Stage 0 exports deterministic
trait-backed call surfaces through an internal registry and compiler-provided
method names.

## Mission

- ad-hoc polymorphism over the current Stage 0 kernel
- coherence-safe instance resolution
- orphan-instance prevention
- default implementations
- deterministic derive expansion for kernel-owned types

## Exported Trait Surface

### `Eq`

- method: `eq(T, T) -> Bool`
- default implementation: canonical rendered equality
- explicit builtin target for `Int`: `eq_int`
- derived kernel instances: `Bool`, `CurrentProgram`, `CurrentReceipt`, `LedgerState`, `File`, `Socket`, `Capability`

### `Print`

- method: `print(T) -> Unit`
- latent effect: `io`
- default implementation: canonical rendered I/O output
- explicit builtin target for `Int`: `print_int`
- derived kernel instances: `Bool`, `CurrentProgram`, `CurrentReceipt`, `LedgerState`, `File`, `Socket`, `Capability`

## Coherence

The seed registry admits at most one instance per `(trait, type, target)` key.
Registry validation is deterministic and stable-ordered.

## Orphan Rule

A trait instance is valid only when the instance owner matches either:

- the owner root of the trait, or
- the owner root of the target type

For the Stage 0 seed registry, kernel traits and kernel types are owned by `lyralang/`.

## Default Implementations

Default bodies are frozen as canonical implementation styles rather than host callbacks:

- `default_canonical_equality`
- `default_canonical_print`

These strategies remain deterministic and require no runtime plugin lookup.

## Derive Expansion

The seed registry records explicit derive-expansion manifests:

- `derive(Eq)`
- `derive(Print)`

Expansion is deterministic and emits only kernel-owned instances in stable lexical order.

## Interfaces

- trait registry contract: `interfaces/specs/lyralang_trait_registry_v1.json`

## Implementation Surface

- `lyralang/src/traits/`
- overloaded call surfaces in the builtin/type/effect/semantic layers
