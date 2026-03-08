# Acceptance — P0-004 Repo Architecture

## Acceptance Criteria

1. Root `Cargo.toml` exists as a valid Cargo workspace manifest declaring all member crates.
2. All canonical ownership root directories exist:
   - `k0/` with `Cargo.toml` and `src/lib.rs`
   - `lyralang/` with `Cargo.toml` and `src/lib.rs`
   - `k1/` with `Cargo.toml` and `src/lib.rs`
   - `shells/` with `Cargo.toml` and `src/lib.rs`
   - `slices/` with `Cargo.toml` and `src/lib.rs`
3. All crates compile cleanly under `cargo check --workspace`.
4. Dependency graph is ownership-aligned: k1/shells/slices depend on k0; lyralang is independent of k1.
5. No ambient nondeterminism is introduced in any crate stub.
6. Workspace manifest artifact documents all declared members and their ownership roles.

## Verification Method
- `cargo check --workspace` passes with zero errors
- Directory tree inspection confirms all canonical roots exist
- Workspace manifest artifact reviewed for completeness

## Evidence Required
- `artifacts/workspace-manifest.md`
- `artifacts/touched-roots.md`
- `cargo check --workspace` pass (recorded in implementation notes)
