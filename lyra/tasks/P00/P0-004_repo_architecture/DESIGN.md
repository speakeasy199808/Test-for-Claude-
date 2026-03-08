# Design — P0-004 Repo Architecture

## Canonical Ownership Zone Model

The Lyra repository is organized into canonical ownership zones. Each zone owns a distinct behavioral domain and has a corresponding Cargo crate. Cross-zone dependencies are strictly ownership-aligned.

### Ownership Zones

| Root | Crate | Role | Depends On |
|---|---|---|---|
| `k0/` | `k0` | Trusted deterministic foundation: canonicalization, verification, constitutional substrate | (none — foundational) |
| `lyralang/` | `lyralang` | Language definition, compiler, toolchain, self-hosting pipeline | (none — language-independent) |
| `k1/` | `k1` | Higher symbolic systems: memory, reasoning, planning, retrieval | `k0` |
| `shells/` | `shells` | Interaction shells and orchestration surfaces | `k0`, `k1` |
| `slices/` | `slices` | Vertical compositions and end-to-end feature slices | `k0`, `k1` |

### Dependency Graph

```
k0  ──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────Now create<create_file>
<path>lyra/tasks/P00/P0-004_repo_architecture/DESIGN.md</path>
<content DESIGN.md for P0-004.>
# Design — P0-004 Repo Architecture

## Architectural Model

### Canonical Ownership Zones
The Lyra codebase is organized into five canonical ownership roots, each with a clear mandate:

| Root | Owner | Mandate |
|------|-------|---------|
| `k0/` | Foundation | Trusted deterministic substrate: canonicalization, verification, constitutional law, security-critical code |
| `lyralang/` | Language | Language definition, compiler, toolchain, self-hosting pipeline |
| `k1/` | Symbolic | Higher symbolic systems: memory, reasoning, planning, retrieval, coordination |
| `shells/` | Interaction | Interaction shells and environment-facing orchestration surfaces |
| `slices/` | Integration | Vertical compositions and end-to-end feature slices spanning roots |

### Dependency Rules
1. `k0` is the foundation — all other roots depend on it
2. `k1`, `shells`, `slices` depend on `k0` (and optionally `k1`)
3. `lyralang` is independent of `k1` — it compiles to bytecode consumed by k0/k1
4. No root may introduce ambient nondeterminism — all randomness/time must flow from k0 primitives

### Workspace Structure
```
LyraOS/
├── Cargo.toml          # Workspace manifest
├── k0/                 # Foundation (trusted substrate)
│   ├── Cargo.toml
│   └── src/lib.rs
├── lyralang/           # Language (compiler/toolchain)
│   ├── Cargo.toml
│   └── src/lib.rs
├── k1/                 # Symbolic (higher systems)
│   ├── Cargo.toml
│   └── src/lib.rs
├── shells/             # Interaction (surfaces)
│   ├── Cargo.toml
│   └── src/lib.rs
├── slices/             # Integration (compositions)
│   ├── Cargo.toml
│   └── src/lib.rs
├── interfaces/         # Shared interface contracts (not a crate)
├── fixtures/          # Shared test fixtures (not a crate)
├── ops/               # Operational scripts (not a crate)
└── docs/              # Documentation (not a crate)
```

### Crate Stubs
Each canonical root crate MUST contain:
- A `Cargo.toml` with proper workspace-inherited fields (`version.workspace = true`, etc.)
- A `src/lib.rs` with:
  - `#![forbid(unsafe_code)]`
  - `#![deny(missing_docs)]`
  - `#![deny(clippy::all)]`
  - A module map doc comment showing future modules
