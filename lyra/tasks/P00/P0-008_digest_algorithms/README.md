# P0-008 — Digest Algorithms

## Mission
SHA-3-256 primary, BLAKE3 secondary. Implemented as Rust module family in `k0/`. All hashes used system-wide route through this module.

## Scope
- SHA-3-256 primary digest algorithm
- BLAKE3 secondary digest algorithm
- Unified routing API: all system-wide hash operations call `k0::digest::digest(algorithm, input)`
- `DigestAlgorithm` enum for explicit algorithm selection
- `DigestOutput` struct: algorithm-tagged 32-byte output with hex serialization

## Primary Archetype
Core Module Implementation

## Primary Ownership Root
`k0/`

## Secondary Touched Roots
`interfaces/`, `fixtures/`, `lyra/tasks/`

## Deliverables
- `k0/src/digest/mod.rs` — routing API, `DigestAlgorithm`, `DigestOutput`
- `k0/src/digest/sha3.rs` — SHA-3-256 implementation
- `k0/src/digest/blake3.rs` — BLAKE3 implementation
- Task control plane: README, ACCEPTANCE, DESIGN, IMPLEMENTATION, task.toml
- Fixtures: golden digest vectors
- Artifacts: digest traceability
