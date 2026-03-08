# P0-022 — Reproducible Builds

## Mission
Establish bit-for-bit reproducible builds from source for the entire Lyra workspace. Lock all dependency versions, pin the Rust toolchain, document the build environment, and ensure the build pipeline requires no Docker — only Cargo and standard Rust tooling.

Reproducible builds are a constitutional requirement for Lyra: identical source inputs must always produce identical binary outputs. This is the build-system corollary of k0's determinism invariant.

## Scope
- **Cargo.lock committed** — all dependency versions locked and tracked in version control
- **rust-toolchain.toml** — Rust toolchain channel and components pinned at the repo root
- **BUILD.md** — comprehensive build environment documentation at the repo root
- **Docker-free pipeline** — no Docker dependency; builds use only `cargo` and `rustup`
- **Verification procedure** — documented method to verify bit-for-bit reproducibility (build twice, compare SHA-256 hashes)
- **CI alignment** — reproducible build configuration aligns with the existing CI pipeline (P0-015)

## Primary Archetype
Core Module Implementation

## Primary Ownership Root
`k0/`

## Secondary Touched Roots
- Repository root (`Cargo.lock`, `rust-toolchain.toml`, `BUILD.md`)
- `lyra/tasks/`
- `.github/` (CI workflow already uses `dtolnay/rust-toolchain@stable`)

## Deliverables
- `rust-toolchain.toml` — pinned toolchain at repo root
- `BUILD.md` — build environment documentation at repo root
- `Cargo.lock` — verified committed to version control (pre-existing)
- `lyra/tasks/P00/P0-022_reproducible_builds/artifacts/reproducible-builds-spec.md` — evidence artifact
- Task control-plane files (README, ACCEPTANCE, DESIGN, IMPLEMENTATION, task.toml)

## Dependencies
- **P0-004** (Repo Architecture) — workspace structure must exist
- **P0-015** (CI/CD Pipeline) — CI workflow must be aligned with reproducible build config
- **P0-016** (Versioning Strategy) — version policy governs Cargo.lock management

## Constitutional Alignment
This task directly supports the following Lyra constitutional invariants:
1. **Determinism** — identical source produces identical binaries
2. **Offline-first** — builds require no network access beyond initial dependency fetch
3. **Explicit ownership** — build configuration lives at the repo root where it governs the workspace
