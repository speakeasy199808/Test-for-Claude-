# Touched Roots — P0-004 Repo Architecture

## Production Roots Created

| Root | Type | Files Created | Notes |
|------|------|--------------|-------|
| `k0/` | Cargo crate | `Cargo.toml`, `src/lib.rs` | Foundation crate; primary ownership root for P0-004 |
| `lyralang/` | Cargo crate | `Cargo.toml`, `src/lib.rs` | Language crate; independent of k1 |
| `k1/` | Cargo crate | `Cargo.toml`, `src/lib.rs` | Symbolic crate; depends on k0 |
| `shells/` | Cargo crate | `Cargo.toml`, `src/lib.rs` | Interaction crate; depends on k0, k1 |
| `slices/` | Cargo crate | `Cargo.toml`, `src/lib.rs` | Integration crate; depends on k0, k1 |

## Workspace Root

| File | Notes |
|------|-------|
| `Cargo.toml` | Workspace manifest; declares all members, shared deps, lint config |

## Pre-existing Roots Extended

| Root | Files Pre-existing | Notes |
|------|-------------------|-------|
| `k0/` | `k0/specs/constitutional_math.md` | Spec existed from P0-003; crate files added by P0-004 |
| `interfaces/` | `interfaces/specs/lyracodec.md` | Spec existed from P0-006; not a Cargo crate |

## Non-Crate Roots (Directories Only)

| Root | Purpose | Notes |
|------|---------|-------|
| `interfaces/` | Shared interface contracts and specs | Not a Cargo crate; owned by interface governance |
| `fixtures/` | Shared test fixtures | Not a Cargo crate; referenced by multiple tasks |
| `ops/` | Operational scripts and CI/CD | Not a Cargo crate; created in P0-015 |
| `docs/` | Documentation | Not a Cargo crate; created as needed |
| `lyra/tasks/` | Task control planes | Not a Cargo crate; governance only |

## Dependency Graph (as built)

```
k0  <── k1  <── shells
 ^               ^
 └───────────────┘
 ^
 └── slices

lyralang  (independent)
