# Lyra Dependency Inventory

> Auto-generated dependency audit for the Lyra workspace.
> Task: P0-021 Dependency Audit
> Workspace license: MIT OR Apache-2.0

## Overview

All dependencies are declared in the root `Cargo.toml` under `[workspace.dependencies]` and inherited by workspace members. The license policy is enforced by `cargo-deny` via `deny.toml`.

**Total workspace dependencies:** 9
**Allowed licenses:** MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Zlib, Unicode-3.0, Unicode-DFS-2016

## Dependency Table

| Crate | Version | License | Category | Purpose |
|---|---|---|---|---|
| `sha3` | `0.10` | MIT OR Apache-2.0 | Cryptographic | SHA-3 (Keccak) digest algorithm for constitutional hashing in k0 |
| `blake3` | `1` | MIT OR Apache-2.0 | Cryptographic | BLAKE3 high-performance hash function for content-addressable operations |
| `serde` | `1` (features: `derive`) | MIT OR Apache-2.0 | Serialization | Framework for serializing and deserializing Rust data structures |
| `serde_json` | `1` | MIT OR Apache-2.0 | Serialization | JSON serialization/deserialization for structured data interchange |
| `thiserror` | `1` | MIT OR Apache-2.0 | Error Handling | Derive macro for ergonomic custom error types with Display impl |
| `anyhow` | `1` | MIT OR Apache-2.0 | Error Handling | Flexible error type for application-level error propagation |
| `tracing` | `0.1` | MIT | Logging | Structured, event-based diagnostic instrumentation framework |
| `proptest` | `1` | MIT OR Apache-2.0 | Testing | Property-based testing framework for generative test strategies |
| `criterion` | `0.5` (features: `html_reports`) | MIT OR Apache-2.0 | Benchmarking | Statistical benchmarking harness with HTML report generation |

## Dependency Details

### sha3 `0.10`
- **License:** MIT OR Apache-2.0
- **Registry:** crates.io
- **Used by:** `k0`
- **Purpose:** Provides the SHA-3 (Keccak) family of cryptographic hash functions. Used as one of the constitutional digest algorithms in the k0 substrate for integrity verification and content addressing.
- **Security notes:** Pure Rust implementation. No unsafe code. Audited as part of the RustCrypto project.

### blake3 `1`
- **License:** MIT OR Apache-2.0
- **Registry:** crates.io
- **Used by:** `k0`
- **Purpose:** High-performance cryptographic hash function (BLAKE3). Used alongside SHA-3 for content-addressable storage and fast integrity checks. Supports SIMD acceleration.
- **Security notes:** Official reference implementation. Includes optional assembly optimizations.

### serde `1`
- **License:** MIT OR Apache-2.0
- **Registry:** crates.io
- **Features enabled:** `derive`
- **Used by:** `k0`, `k1`, `lyralang`, `shells`
- **Purpose:** Core serialization/deserialization framework. The `derive` feature enables `#[derive(Serialize, Deserialize)]` on Lyra data structures for deterministic encoding and interchange.
- **Security notes:** Widely audited. No unsafe code in derive macros.

### serde_json `1`
- **License:** MIT OR Apache-2.0
- **Registry:** crates.io
- **Used by:** `k0`, `k1`
- **Purpose:** JSON serialization and deserialization. Used for structured log output, configuration files, and human-readable data interchange formats.
- **Security notes:** Depends on serde. No known advisories.

### thiserror `1`
- **License:** MIT OR Apache-2.0
- **Registry:** crates.io
- **Used by:** `k0`, `k1`, `lyralang`
- **Purpose:** Derive macro for defining custom error types with automatic `Display` and `Error` trait implementations. Used throughout the workspace for typed error hierarchies (P0-018 Error Code System).
- **Security notes:** Proc-macro only. No runtime code.

### anyhow `1`
- **License:** MIT OR Apache-2.0
- **Registry:** crates.io
- **Used by:** `k1`, `shells`
- **Purpose:** Flexible, context-rich error type for application-level error handling. Used in higher-level crates where error type erasure is acceptable (not in k0 constitutional code).
- **Security notes:** Minimal dependency footprint. No known advisories.

### tracing `0.1`
- **License:** MIT
- **Registry:** crates.io
- **Used by:** `k0`, `k1`, `lyralang`, `shells`
- **Purpose:** Structured diagnostic instrumentation framework. Provides spans and events for observability. Integrated with the structured logging system (P0-019).
- **Security notes:** Part of the tokio project ecosystem. Widely audited.

### proptest `1`
- **License:** MIT OR Apache-2.0
- **Registry:** crates.io
- **Used by:** `k0` (dev-dependency)
- **Purpose:** Property-based testing framework. Generates random inputs to verify invariants hold across a wide range of cases. Used for constitutional invariant testing and determinism verification.
- **Security notes:** Dev-dependency only. Not included in release builds.

### criterion `0.5`
- **License:** MIT OR Apache-2.0
- **Registry:** crates.io
- **Features enabled:** `html_reports`
- **Used by:** `k0` (dev-dependency)
- **Purpose:** Statistical benchmarking harness. Provides statistically rigorous performance measurements with HTML report generation. Used by the benchmarking harness (P0-017).
- **Security notes:** Dev-dependency only. Not included in release builds.

## Transitive Dependency Notes

The following transitive dependencies introduce licenses beyond MIT/Apache-2.0:

- **`unicode-ident`** (transitive via `proc-macro2` -> `syn` -> `serde_derive`): Licensed under `Unicode-3.0 OR MIT OR Apache-2.0`. The Unicode-3.0 license is included in the allowlist.
- **Unicode data files** (transitive): Some crates consuming Unicode data tables are licensed under `Unicode-DFS-2016`. This license is included in the allowlist.

## Audit Policy

- **Tool:** `cargo-deny` (configuration: `deny.toml`)
- **CI integration:** `.github/workflows/ci.yml` (deny job)
- **Frequency:** Every push and pull request (automated)
- **Advisory database:** RustSec (https://github.com/rustsec/advisory-db)
- **Vulnerability policy:** Deny (hard failure)
- **Unmaintained policy:** Warn (informational)
- **Unknown sources:** Deny (hard failure)
