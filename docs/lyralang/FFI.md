# FFI Specification — Formal Specification

**Task:** P1-014
**Module:** `lyralang::ffi`
**Stage:** 0

## 1. Overview

The FFI specification checker analyzes calls to foreign functions in LyraLang programs. It identifies call sites where the callee name begins with `ffi_`, determines the target language (Rust or C), enforces that a `Capability` resource is in scope at every such call site, and documents the canonical type marshalling rules between Lyra types and foreign types.

## 2. FFI Call Site Detection

Any call expression whose callee is an identifier starting with `"ffi_"` is treated as an FFI call. No additional annotation is required in Stage 0 source code.

```
ffi_rust_read(cap, 42)    -- Rust FFI call
ffi_c_write(fd, buf)      -- C FFI call
ffi_unknown_func()        -- Unknown target
```

## 3. Target Language Detection

| Callee name pattern | Detected target |
|---------------------|-----------------|
| `ffi_rust_*`        | `FfiTarget::Rust` |
| `ffi_c_*`           | `FfiTarget::C`    |
| `ffi_*` (other)     | `FfiTarget::Unknown` |

## 4. Capability Gating

Every FFI call site must be guarded by a `Capability` resource binding that appears in scope before the call. A `Capability` binding is recognized when a `let` statement initializes a binding with the expression `Capability(...)`.

```
-- Gated (correct)
let cap = Capability()
let result = ffi_rust_read(cap, 42)

-- Ungated (error)
let result = ffi_c_write(0)
```

An ungated FFI call emits a `FfiErrorKind::MissingCapability` diagnostic. The checker continues after this error (`recovered: false` — the program is considered unsafe without capability gating).

The `SafetyBoundary::all_calls_gated` field summarizes whether all detected FFI calls in the program were properly gated.

## 5. Safety Boundary

```
SafetyBoundary {
    all_calls_gated: bool,
    unsafe_blocks_present: bool,   -- always false (crate forbids unsafe)
    boundary_description: String,
}
```

Because `#![forbid(unsafe_code)]` is enforced at the crate root, `unsafe_blocks_present` is always `false`. There is no mechanism for a Stage 0 program to introduce raw unsafe blocks.

## 6. Canonical Marshalling Rules

These rules are included in the output whenever at least one FFI call is present:

| Lyra type | Foreign type | Direction      | Notes                                     |
|-----------|-------------|----------------|-------------------------------------------|
| `Int`     | `i64`       | Bidirectional  | 64-bit signed integer                     |
| `Bool`    | `bool`      | Bidirectional  | C99 `_Bool` / Rust `bool`                 |
| `String`  | `*const u8` | LyraToForeign  | Null-terminated UTF-8; no ownership transfer |
| `Unit`    | `void`      | Bidirectional  | No value representation                   |

## 7. Argument Marshalling

The checker infers the marshalled parameter type for each argument at the call site:

| Argument expression kind | Marshalled type  |
|--------------------------|------------------|
| Integer literal          | `i64`            |
| Boolean literal          | `bool`           |
| String literal           | `*const u8`      |
| Identifier               | `marshal(<name>)`|
| Other                    | `opaque`         |

The return marshalling defaults to `i64` in Stage 0 (conservative assumption).

## 8. Error Kinds

| Kind                    | Meaning                                                        |
|-------------------------|----------------------------------------------------------------|
| `ParseError`            | Source failed to parse; checking did not proceed               |
| `MissingCapability`     | FFI call site has no `Capability` binding in scope             |
| `UnsafeTypeMarshalling` | Reserved for future stages; a type cannot be safely marshalled |
| `UnknownFfiTarget`      | Reserved for future stages; explicit unknown-target reporting   |

## 9. Output Structure

```
FfiCheckOutput {
    normalized_source: String,
    judgment: Option<FfiProgramJudgment>,
    errors: Vec<FfiError>,
}

FfiProgramJudgment {
    module: Option<String>,
    ffi_calls: Vec<FfiCallSummary>,
    safety_boundary: SafetyBoundary,
    required_capabilities: Vec<String>,
    marshalling_rules: Vec<MarshallingRule>,
    span: SourceSpan,
}

FfiCallSummary {
    callee: String,
    target_language: FfiTarget,    -- Rust | C | Unknown
    required_capability: String,   -- "Capability"
    marshalled_params: Vec<String>,
    return_marshalling: String,
    span: SourceSpan,
}
```

## 10. API

```rust
let checker = FfiChecker::new();
let output = checker.check_source(source);

// Free function shorthand
let output = lyralang::ffi::check(source);
```

## 11. Security Properties

1. **No unsafe code** — `#![forbid(unsafe_code)]` prevents any bypass of the Rust type system.
2. **Capability gating** — Every foreign call requires explicit capability authorization, preventing ambient authority.
3. **Safe-by-construction** — Stage 0 FFI cannot produce dangling pointers, use-after-free, or data races by construction; these are ruled out by the language's ownership and linear type systems.
