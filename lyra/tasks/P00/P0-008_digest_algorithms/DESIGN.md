# Design — P0-008 Digest Algorithms

## Architecture

The digest module is a pure Rust module family at `k0/src/digest/`. It provides a unified routing API over two algorithm implementations. No caller outside this module may invoke `sha3` or `blake3` crates directly in production code.

### Module Decomposition

| Module | Responsibility |
|---|---|
| `mod.rs` | `DigestAlgorithm` enum, `DigestOutput` struct, `digest()` routing fn, `sha3_256()` / `blake3()` shorthands |
| `sha3.rs` | `sha3_256_digest(input) -> DigestOutput` — wraps `sha3::Sha3_256` |
| `blake3.rs` | `blake3_digest(input) -> DigestOutput` — wraps `blake3::hash()` |

### Algorithm Policy

| Role | Algorithm | Use Cases |
|---|---|---|
| Primary | SHA-3-256 | Constitutional hashes, trust root fingerprints, canonical state sealing, all governance-critical digests |
| Secondary | BLAKE3 | High-throughput content-addressed storage, non-constitutional integrity checks |

### Public API

```rust
// Explicit routing (preferred for all production code)
pub fn digest(algorithm: DigestAlgorithm, input: &[u8]) -> DigestOutput

// Shorthands
pub fn sha3_256(input: &[u8]) -> DigestOutput
pub fn blake3(input: &[u8]) -> DigestOutput

// Algorithm enum
pub enum DigestAlgorithm { Sha3_256, Blake3 }

// Output type
pub struct DigestOutput {
    pub algorithm: DigestAlgorithm,
    // bytes: [u8; 32] (private)
}
impl DigestOutput {
    pub fn as_bytes(&self) -> &[u8; 32]
    pub fn to_hex(&self) -> String  // 64-char lowercase hex
}
```

### DigestOutput Design

`DigestOutput` carries the algorithm tag alongside the bytes. This ensures:
- Callers can verify which algorithm produced a given digest
- Digests from different algorithms cannot be silently compared as equal
- Traceability: every stored digest knows its provenance

### Determinism Guarantee

Both SHA-3-256 and BLAKE3 are deterministic pure functions. The routing layer adds no nondeterminism. The module has no mutable global state, no ambient randomness, and no I/O.

### Float Prohibition

Not applicable — this module operates on byte slices only.

## Dependency Posture

- No hard prerequisites beyond workspace crates (`sha3`, `blake3` already in `k0/Cargo.toml`)
- Consumed by: P0-001 (genesis hash should migrate to use this routing), P0-009 (virtual time sealing), P0-011 (determinism verifier), P0-023 (foundation integration)
