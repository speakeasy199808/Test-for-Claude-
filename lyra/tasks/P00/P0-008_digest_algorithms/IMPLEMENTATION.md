# Implementation Notes — P0-008 Digest Algorithms

## Work Package Shape
Multi-module Rust implementation in `k0/src/digest/` with task-local control plane, fixtures, and artifacts.

## Produced Components

| File | Description |
|---|---|
| `k0/src/digest/mod.rs` | Routing API: `digest()`, `sha3_256()`, `blake3()`, `DigestAlgorithm`, `DigestOutput` |
| `k0/src/digest/sha3.rs` | `sha3_256_digest(input) -> DigestOutput` — wraps `sha3::Sha3_256` |
| `k0/src/digest/blake3.rs` | `blake3_digest(input) -> DigestOutput` — wraps `blake3::hash()` |

## Test Coverage

- **sha3 module**: empty string golden, "abc" golden, output length, determinism, algorithm tag (5 tests)
- **blake3 module**: empty string golden, output length, determinism, algorithm tag, different inputs differ (5 tests)
- **routing (mod.rs)**: sha3 produces 32 bytes, blake3 produces 32 bytes, hex is 64 chars, determinism, different inputs differ, sha3 vs blake3 differ, algorithm names, output lengths, sha3 shorthand, blake3 shorthand, sha3 empty golden (11 tests)
- **doc-test**: routing example in module doc comment (1 test)

Total: 22 digest tests (21 unit + 1 doc-test)

## Ownership Placement
- Production code: `k0/src/digest/` (primary ownership root `k0/`)
- Task control plane: `lyra/tasks/P00/P0-008_digest_algorithms/`

## Dependency Posture
- No new crate dependencies required (`sha3` and `blake3` already in `k0/Cargo.toml`)
- Enables: P0-009 (virtual time), P0-011 (determinism verifier), P0-023 (foundation integration)

## Migration Note
`k0/src/genesis/hash.rs` uses `sha3::Sha3_256` directly (pre-P0-008). A future integration pass (P0-023) should migrate it to use `k0::digest::sha3_256()` for full routing compliance. The golden hash value is unchanged.
