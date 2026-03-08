# Implementation Notes — P0-004 Repo Architecture

## Work Package Shape
Multi-module workspace initialization with canonical crate stubs.

## Produced Components

| Component | Path | Description |
|---|---|---|
| Workspace manifest | `Cargo.toml` | Root workspace with all members, shared deps, lint config |
| k0 crate stub | `k0/Cargo.toml` + `k0/src/lib.rs` | Foundation crate: deterministic substrate |
| lyralang crate stub | `lyralang/Cargo.toml` + `lyralang/src/lib.rs` | Language crate: compiler/toolchain |
| k1 crate stub | `k1/Cargo.toml` + `k1/src/lib.rs` | Symbolic crate: higher systems |
| shells crate stub | `shells/Cargo.toml` + `shells/src/lib.rs` | Interaction crate: surfaces |
| slices crate stub | `slices/Cargo.toml` + `slices/src/lib.rs` | Integration crate: compositions |
| Workspace manifest artifact | `artifacts/workspace-manifest.md` | Documents all declared members |
| Touched-roots artifact | `artifacts/touched-roots.md` | Documents all roots touched |

## Ownership Placement
- Primary: `k0/` (foundational substrate crate)
- Workspace root: `Cargo.toml` (workspace governance)
- All canonical roots: `lyralang/`, `k1/`, `shells/`, `slices/`
- Task control-plane evidence: `lyra/tasks/P00/P0-004_repo_architecture/`

## Dependency Posture
Parallel-capable after P0-003 (constitutional math spec). Enables all subsequent implementation tasks.

## Workspace Shared Dependencies
All workspace-level dependencies are declared in root `Cargo.toml` under `[workspace.dependencies]`:
- `sha3 = "0.10"` — SHA-3-256 primary digest
- `blake3 = "1"` — BLAKE3 secondary digest
- `serde = { version = "1", features = ["derive"] }` — serialization
- `serde_json = "1"` — JSON serialization
- `thiserror = "1"` — error type derivation
- `anyhow = "1"` — error propagation
- `tracing = "0.1"` — structured logging
- `proptest = "1"` — property-based testing

## Acceptance Checklist
- [x] Root `Cargo.toml` workspace manifest created
- [x] `k0/` crate stub created with correct ownership attributes
- [x] `lyralang/` crate stub created
- [x] `k1/` crate stub created with k0 dependency
- [x] `shells/` crate stub created with k0+k1 dependencies
- [x] `slices/` crate stub created with k0+k1 dependencies
- [x] All crates use `#![forbid(unsafe_code)]`
- [x] All crates use workspace-inherited version/edition/license
- [x] Workspace manifest artifact emitted
- [x] Touched-roots artifact emitted
