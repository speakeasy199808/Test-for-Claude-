# Workspace Manifest — P0-004 Repo Architecture

## Cargo Workspace Members

| Member | Crate Name | Role | Depends On |
|--------|-----------|------|------------|
| `k0/` | `k0` | Trusted deterministic foundation: canonicalization, verification, constitutional substrate, security-critical code | (foundational — no internal deps) |
| `lyralang/` | `lyralang` | Language definition, compiler pipeline, toolchain, self-hosting infrastructure | (language-independent) |
| `k1/` | `k1` | Higher symbolic systems: memory, reasoning, planning, retrieval, coordination | `k0` |
| `shells/` | `shells` | Interaction shells and environment-facing orchestration surfaces | `k0`, `k1` |
| `slices/` | `slices` | Vertical compositions and end-to-end feature slices spanning roots | `k0`, `k1` |

## Workspace-Level Shared Dependencies

| Dependency | Version | Purpose |
|-----------|---------|---------|
| `sha3` | `0.10` | SHA-3-256 primary canonical digest algorithm |
| `blake3` | `1` | BLAKE3 secondary digest algorithm |
| `serde` | `1` (features: derive) | Serialization/deserialization framework |
| `serde_json` | `1` | JSON serialization for fixtures and interchange |
| `thiserror` | `1` | Error type derivation |
| `anyhow` | `1` | Error propagation in non-library code |
| `tracing` | `0.1` | Structured deterministic logging |
| `proptest` | `1` | Property-based testing |

## Workspace Lint Configuration
- `unsafe_code = "forbid"` — no unsafe code in any workspace member
- `missing_docs = "warn"` — all public items must be documented
- `clippy::all = "warn"` — full Clippy lint suite enabled

## Workspace Version
- Version: `0.1.0`
- Edition: `2021`
- License: `MIT OR Apache-2.0`
- Rust resolver: `2`
