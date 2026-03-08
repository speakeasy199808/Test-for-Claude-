//! Digest Algorithms — SHA-3-256 primary, BLAKE3 secondary (P0-008).
//!
//! All hash operations system-wide route through this module.
//! Direct use of `sha3` or `blake3` crates outside this module is
//! forbidden in production code — callers must use the routing API here.
//!
//! # Algorithm Policy
//! - **Primary:** SHA-3-256 (Keccak) — used for all canonical digests,
//!   constitutional hashes, and trust root fingerprints.
//! - **Secondary:** BLAKE3 — used for high-throughput content-addressed
//!   storage and non-constitutional integrity checks.
//!
//! # Usage
//! ```rust
//! use k0::digest::{digest, DigestAlgorithm, DigestOutput};
//!
//! let output: DigestOutput = digest(DigestAlgorithm::Sha3_256, b"hello");
//! assert_eq!(output.as_bytes().len(), 32);
//! assert_eq!(output.to_hex().len(), 64);
//! ```

pub mod blake3;
pub mod sha3;

pub use blake3::blake3_digest;
pub use sha3::sha3_256_digest;

/// The set of digest algorithms available in the Lyra system.
///
/// All callers must select an algorithm explicitly — there is no ambient default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigestAlgorithm {
    /// SHA-3-256 (Keccak). Primary algorithm for all canonical and constitutional digests.
    Sha3_256,
    /// BLAKE3. Secondary algorithm for high-throughput content-addressed storage.
    Blake3,
}

impl DigestAlgorithm {
    /// Return the human-readable name of this algorithm.
    pub fn name(&self) -> &'static str {
        match self {
            DigestAlgorithm::Sha3_256 => "sha3-256",
            DigestAlgorithm::Blake3 => "blake3",
        }
    }

    /// Return the output length in bytes for this algorithm.
    pub fn output_len(&self) -> usize {
        match self {
            DigestAlgorithm::Sha3_256 => 32,
            DigestAlgorithm::Blake3 => 32,
        }
    }
}

/// A 32-byte digest output, algorithm-tagged for traceability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigestOutput {
    /// The algorithm that produced this digest.
    pub algorithm: DigestAlgorithm,
    bytes: [u8; 32],
}

impl DigestOutput {
    /// Construct a `DigestOutput` from raw bytes and algorithm tag.
    pub fn new(algorithm: DigestAlgorithm, bytes: [u8; 32]) -> Self {
        Self { algorithm, bytes }
    }

    /// Return the raw 32-byte digest.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }

    /// Return the lowercase hex-encoded digest string (64 characters).
    pub fn to_hex(&self) -> String {
        self.bytes.iter().map(|b| format!("{b:02x}")).collect()
    }
}

/// Compute a digest of `input` using the specified `algorithm`.
///
/// This is the primary routing entry point. All system-wide hash operations
/// must call this function rather than invoking algorithm crates directly.
pub fn digest(algorithm: DigestAlgorithm, input: &[u8]) -> DigestOutput {
    match algorithm {
        DigestAlgorithm::Sha3_256 => sha3_256_digest(input),
        DigestAlgorithm::Blake3 => blake3_digest(input),
    }
}

/// Compute a SHA-3-256 digest of `input` (primary algorithm shorthand).
pub fn sha3_256(input: &[u8]) -> DigestOutput {
    sha3_256_digest(input)
}

/// Compute a BLAKE3 digest of `input` (secondary algorithm shorthand).
pub fn blake3(input: &[u8]) -> DigestOutput {
    blake3_digest(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digest_sha3_256_produces_32_bytes() {
        let out = digest(DigestAlgorithm::Sha3_256, b"");
        assert_eq!(out.as_bytes().len(), 32);
        assert_eq!(out.algorithm, DigestAlgorithm::Sha3_256);
    }

    #[test]
    fn digest_blake3_produces_32_bytes() {
        let out = digest(DigestAlgorithm::Blake3, b"");
        assert_eq!(out.as_bytes().len(), 32);
        assert_eq!(out.algorithm, DigestAlgorithm::Blake3);
    }

    #[test]
    fn digest_hex_is_64_chars() {
        let out = digest(DigestAlgorithm::Sha3_256, b"lyra");
        assert_eq!(out.to_hex().len(), 64);
    }

    #[test]
    fn digest_is_deterministic() {
        let a = digest(DigestAlgorithm::Sha3_256, b"determinism");
        let b = digest(DigestAlgorithm::Sha3_256, b"determinism");
        assert_eq!(a, b);
    }

    #[test]
    fn different_inputs_produce_different_digests() {
        let a = digest(DigestAlgorithm::Sha3_256, b"input_a");
        let b = digest(DigestAlgorithm::Sha3_256, b"input_b");
        assert_ne!(a.as_bytes(), b.as_bytes());
    }

    #[test]
    fn sha3_and_blake3_produce_different_outputs_for_same_input() {
        let a = digest(DigestAlgorithm::Sha3_256, b"same");
        let b = digest(DigestAlgorithm::Blake3, b"same");
        assert_ne!(a.as_bytes(), b.as_bytes());
    }

    #[test]
    fn algorithm_names_are_correct() {
        assert_eq!(DigestAlgorithm::Sha3_256.name(), "sha3-256");
        assert_eq!(DigestAlgorithm::Blake3.name(), "blake3");
    }

    #[test]
    fn algorithm_output_len_is_32() {
        assert_eq!(DigestAlgorithm::Sha3_256.output_len(), 32);
        assert_eq!(DigestAlgorithm::Blake3.output_len(), 32);
    }

    #[test]
    fn sha3_256_shorthand_matches_routing() {
        let a = sha3_256(b"test");
        let b = digest(DigestAlgorithm::Sha3_256, b"test");
        assert_eq!(a, b);
    }

    #[test]
    fn blake3_shorthand_matches_routing() {
        let a = blake3(b"test");
        let b = digest(DigestAlgorithm::Blake3, b"test");
        assert_eq!(a, b);
    }

    #[test]
    fn sha3_256_empty_input_golden() {
        // SHA-3-256("") = a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a
        let out = sha3_256(b"");
        assert_eq!(
            out.to_hex(),
            "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a"
        );
    }
}
