# Acceptance — P1-013 Lifetime Annotations

## Acceptance Criteria

1. `LifetimeChecker::new()` constructs a zero-configuration checker.
2. `LifetimeChecker::check_source(source)` returns a `LifetimeCheckOutput` for any input.
3. A program with no let bindings produces an empty `bindings` list and `dangling_free: true`.
4. Integer and boolean literal bindings receive `BorrowKind::SharedRef` and a fresh named region (`'a`, `'b`, ...).
5. String literal bindings receive `BorrowKind::Static` and region `'static`.
6. Linear resource constructor calls (e.g. `Capability()`) receive `BorrowKind::Owned` and region `'static`.
7. Top-level bindings emit an `Outlives` constraint against `'program`; block-scoped bindings emit `SameScopeAs` constraints.
8. Static bindings emit a `Static` constraint against `'static`.
9. `dangling_free` is always `true` in Stage 0 (safe by construction).
10. `elision_applied` records the names of bindings where lifetime elision was applied (first `SharedRef` in each scope).
11. Parse errors are forwarded as `LifetimeErrorKind::ParseError` diagnostics with no judgment.
12. All public types derive `Clone, Debug, Serialize, Deserialize`.
13. All public items carry `///` doc comments.
14. The free function `check(source)` is equivalent to `LifetimeChecker::new().check_source(source)`.

## Verification Method

- Fixture/golden round-trip for `lifetimes_valid.lyra` and `lifetimes_short.lyra`
- `cargo check -p lyralang` passes with no warnings

## Status

All acceptance criteria satisfied. Module implemented at `lyralang/src/lifetimes/mod.rs`.
