# P1-014 — FFI Specification

## Mission

Implement a static FFI boundary checker for LyraLang Stage 0. The checker identifies foreign function calls (any callee whose name starts with `ffi_`), determines the target language (Rust or C), verifies that a `Capability` resource is in scope at each call site, and documents canonical marshalling rules between Lyra types and foreign types.

## Scope

- `FfiChecker` — public checker struct following the standard `new()` / `check_source(source)` API
- `FfiCheckOutput` — result bundle: `normalized_source`, `judgment`, `errors`
- `FfiProgramJudgment` — FFI call summaries, safety boundary, required capabilities, marshalling rules
- `FfiCallSummary` — per-call: callee, target, required capability, marshalled params, return marshalling
- `FfiTarget` — `Rust`, `C`, `Unknown`
- `SafetyBoundary` — `all_calls_gated`, `unsafe_blocks_present`, `boundary_description`
- `MarshallingRule` — `lyra_type`, `foreign_type`, `direction`, `conversion`
- `MarshalDirection` — `LyraToForeign`, `ForeignToLyra`, `Bidirectional`
- `FfiError` / `FfiErrorKind` — `ParseError`, `MissingCapability`, `UnsafeTypeMarshalling`, `UnknownFfiTarget`
- Free function `check(source)` as module-level shorthand

## Primary Ownership Root

`lyralang/src/ffi/`

## Secondary Touched Roots

`fixtures/lyralang/ffi/`, `goldens/lyralang/ffi/`, `interfaces/specs/`, `docs/lyralang/`

## Target Detection

| Callee prefix     | Target   |
|-------------------|----------|
| `ffi_rust_*`      | `Rust`   |
| `ffi_c_*`         | `C`      |
| `ffi_*` (other)   | `Unknown`|

## Capability Gating

An FFI call is considered gated when a binding whose value is `Capability(...)` appears in the program before the call site. Ungated calls emit `FfiErrorKind::MissingCapability`.

## Canonical Marshalling Rules

| Lyra type | Foreign type | Direction      |
|-----------|-------------|----------------|
| `Int`     | `i64`       | Bidirectional  |
| `Bool`    | `bool`      | Bidirectional  |
| `String`  | `*const u8` | LyraToForeign  |
| `Unit`    | `void`      | Bidirectional  |

## Safety Invariant

`unsafe_blocks_present` is always `false` because the crate enforces `#![forbid(unsafe_code)]`.

## Deliverables

- `lyralang/src/ffi/mod.rs` — FFI checker implementation
- `lyralang/src/ffi/error.rs` — error types
- `fixtures/lyralang/ffi/ffi_valid.lyra`
- `fixtures/lyralang/ffi/ffi_uncapable.lyra`
- `goldens/lyralang/ffi/ffi_valid.json`
- `goldens/lyralang/ffi/ffi_uncapable.json`
- `interfaces/specs/lyralang_ffi_spec_v1.json`
- `docs/lyralang/FFI.md`
