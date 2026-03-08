# Design — P1-005 Linear Types

## Design Summary

The Stage 0 linear slice is implemented as a dedicated ownership pass over the parsed AST.
It intentionally avoids introducing runtime reference tracking. Instead, the pass:

- recognizes kernel resource types in the shared type algebra
- tracks scope-local binding state as non-linear, outstanding linear, or consumed linear
- treats identifier references on linear bindings as moves
- validates explicit builtin ownership contracts
- requires control-flow branches to converge to identical ownership state

## Why a Dedicated Pass

The current seed type checker already proves type compatibility, but exact-once discharge is a distinct
semantic invariant. Keeping it as a separate pass preserves a clean boundary for later tasks such as
lifetimes, borrow checking, and richer capability-safe flow.

## Current Surface Limits

Stage 0 linear flow is intentionally conservative:

- explicit builtin ownership contracts are supported
- generic passthrough is limited to `id`
- binary/prefix operators cannot transport linear values
- unsupported higher-order linear flow is rejected explicitly
