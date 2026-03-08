# Lifetime Annotations — Formal Specification

**Task:** P1-013
**Module:** `lyralang::lifetimes`
**Stage:** 0

## 1. Overview

The lifetime annotations checker infers a named lifetime region for each `let` binding in a LyraLang program. It applies lifetime elision rules, records outlives constraints between regions, and guarantees that all Stage 0 programs are dangling-reference-free by construction. Linear resources (File, Socket, Capability) are already managed by the `linear` checker and receive `BorrowKind::Owned`.

## 2. Motivation

Lifetimes formalize the scope during which a value is valid. By making lifetime regions explicit in the checker output, downstream analysis (borrow checking, alias analysis, optimizations) has a canonical representation to work from. Stage 0 establishes the inference algorithm; later stages will refine it with explicit annotations in source syntax.

## 3. Region Naming

Regions are assigned fresh names drawn from the sequence `'a, 'b, 'c, ..., 'z, 'a1, 'b1, ...` in source order of binding introduction. Special regions:

| Region      | Meaning                                              |
|-------------|------------------------------------------------------|
| `'static`   | Value is valid for the entire execution lifetime     |
| `'program`  | Used as the upper bound for top-level non-static bindings |

## 4. Borrow Kind Inference

| Binding value                       | BorrowKind   | Region          |
|-------------------------------------|--------------|-----------------|
| Call to linear resource constructor  | `Owned`      | `'static`       |
| Identifier matching a resource name  | `Owned`      | `'static`       |
| String literal                       | `Static`     | `'static`       |
| Any other expression                 | `SharedRef`  | Fresh region    |

`UniqueRef` is reserved for future mutable-reference tracking.

## 5. Region Constraints

Each binding introduction generates one region constraint:

| Binding location | Constraint kind  | Description                                    |
|-----------------|------------------|------------------------------------------------|
| Top-level        | `Outlives`       | Region outlives `'program` (the whole program) |
| Block-scoped     | `SameScopeAs`    | Region shares scope with previous block binding|
| Static           | `Static`         | Region is `'static` (outlives all)             |

## 6. Lifetime Elision

Elision rule applied in Stage 0:

> When a new `SharedRef` binding is introduced and there are currently zero `SharedRef` bindings visible in the scope, the lifetime annotation is considered elided. The binding name is recorded in `LifetimeProgramJudgment::elision_applied`.

This corresponds to Rust's first elision rule (single lifetime input → output inherits it), adapted for value bindings.

## 7. Dangling-Reference Safety

All Stage 0 programs are dangling-reference-free by construction:

- No raw pointer syntax exists in Stage 0.
- The crate enforces `#![forbid(unsafe_code)]`.
- All references are checked by the linear or lifetime checker before use.

The field `dangling_free` is always `true` in Stage 0 outputs.

## 8. Error Kinds

| Kind                | Meaning                                                        |
|---------------------|----------------------------------------------------------------|
| `ParseError`        | Source failed to parse; checking did not proceed               |
| `DanglingReference` | Reserved for future stages; a reference outlives its referent  |
| `LifetimeTooShort`  | Reserved for future stages; region is shorter than required    |
| `ConflictingBorrows`| Reserved for future stages; overlapping mutable/shared refs    |

## 9. Output Structure

```
LifetimeCheckOutput {
    normalized_source: String,
    judgment: Option<LifetimeProgramJudgment>,
    errors: Vec<LifetimeError>,
}

LifetimeProgramJudgment {
    module: Option<String>,
    bindings: Vec<LifetimeBindingJudgment>,
    regions: Vec<RegionConstraint>,
    dangling_free: bool,       -- always true in Stage 0
    elision_applied: Vec<String>,
    span: SourceSpan,
}

LifetimeBindingJudgment {
    name: String,
    inferred_region: String,   -- e.g. "'a", "'static"
    borrow_kind: BorrowKind,
    span: SourceSpan,
}

RegionConstraint {
    lhs: String,
    rhs: String,
    constraint: OutlivesKind,  -- Outlives | SameScopeAs | Static
}
```

## 10. API

```rust
let checker = LifetimeChecker::new();
let output = checker.check_source(source);

// Free function shorthand
let output = lyralang::lifetimes::check(source);
```
