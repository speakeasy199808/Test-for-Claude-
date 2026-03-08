# P1-013 — Lifetime Annotations

## Mission

Implement a borrow-checking and region-inference pass for LyraLang Stage 0 non-linear bindings. The pass infers a lifetime region for each `let` binding, applies lifetime elision where possible, records outlives constraints between regions, and guarantees that all Stage 0 programs are dangling-reference-free by construction.

## Scope

- `LifetimeChecker` — public checker struct following the standard `new()` / `check_source(source)` API
- `LifetimeCheckOutput` — result bundle: `normalized_source`, `judgment`, `errors`
- `LifetimeProgramJudgment` — binding judgments, region constraints, dangling-free flag, elision record
- `LifetimeBindingJudgment` — per-binding: name, inferred region, borrow kind
- `BorrowKind` — `Owned`, `SharedRef`, `UniqueRef`, `Static`
- `RegionConstraint` — `lhs`, `rhs`, `OutlivesKind`
- `OutlivesKind` — `Outlives`, `SameScopeAs`, `Static`
- `LifetimeError` / `LifetimeErrorKind` — `ParseError`, `DanglingReference`, `LifetimeTooShort`, `ConflictingBorrows`
- Free function `check(source)` as module-level shorthand

## Primary Ownership Root

`lyralang/src/lifetimes/`

## Secondary Touched Roots

`fixtures/lyralang/lifetimes/`, `goldens/lyralang/lifetimes/`, `interfaces/specs/`, `docs/lyralang/`

## Region Inference Rules (Stage 0)

| Binding kind             | BorrowKind  | Inferred region              |
|--------------------------|-------------|------------------------------|
| Linear resource (File/Socket/Capability) | `Owned` | `'static` (delegated to linear checker) |
| String literal RHS       | `Static`    | `'static`                    |
| Any other RHS            | `SharedRef` | Fresh `'a`, `'b`, ... region |

## Elision Rule

When there is exactly one `SharedRef` binding visible in the current scope at the point of a new `SharedRef` binding introduction, lifetime elision is applied and the binding name is recorded in `elision_applied`.

## Dangling-Free Guarantee

All Stage 0 programs are dangling-free by construction. No raw pointers, no unsafe code. The `dangling_free` field is always `true`.

## Deliverables

- `lyralang/src/lifetimes/mod.rs` — lifetime checker implementation
- `lyralang/src/lifetimes/error.rs` — error types
- `fixtures/lyralang/lifetimes/lifetimes_valid.lyra`
- `fixtures/lyralang/lifetimes/lifetimes_short.lyra`
- `goldens/lyralang/lifetimes/lifetimes_valid.json`
- `goldens/lyralang/lifetimes/lifetimes_short.json`
- `interfaces/specs/lyralang_lifetime_annotations_v1.json`
- `docs/lyralang/LIFETIMES.md`
