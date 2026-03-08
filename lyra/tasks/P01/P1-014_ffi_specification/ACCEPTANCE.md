# Acceptance — P1-014 FFI Specification

## Acceptance Criteria

1. `FfiChecker::new()` constructs a zero-configuration checker.
2. `FfiChecker::check_source(source)` returns an `FfiCheckOutput` for any input.
3. A program with no `ffi_*` calls produces an empty `ffi_calls` list, `all_calls_gated: true`, and empty `marshalling_rules`.
4. A call to `ffi_rust_*` is classified as `FfiTarget::Rust`; a call to `ffi_c_*` is classified as `FfiTarget::C`.
5. An FFI call present after a `Capability(...)` binding in scope produces no `MissingCapability` error and sets `all_calls_gated: true`.
6. An FFI call without a `Capability(...)` binding in scope produces a `FfiErrorKind::MissingCapability` error and sets `all_calls_gated: false`.
7. `unsafe_blocks_present` is always `false` (crate forbids unsafe code).
8. When FFI calls are present, the judgment includes all four canonical marshalling rules (Int→i64, Bool→bool, String→*const u8, Unit→void).
9. `required_capabilities` contains `"Capability"` whenever any FFI call is present.
10. Parse errors are forwarded as `FfiErrorKind::ParseError` diagnostics with no judgment.
11. All public types derive `Clone, Debug, Serialize, Deserialize`.
12. All public items carry `///` doc comments.
13. The free function `check(source)` is equivalent to `FfiChecker::new().check_source(source)`.

## Verification Method

- Fixture/golden round-trip for `ffi_valid.lyra` and `ffi_uncapable.lyra`
- `cargo check -p lyralang` passes with no warnings

## Status

All acceptance criteria satisfied. Module implemented at `lyralang/src/ffi/mod.rs`.
