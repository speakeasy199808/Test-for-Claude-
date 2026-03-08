//! BLAKE3 digest implementation for the Lyra digest routing module.
//!
//! BLAKE3 is the **secondary** digest algorithm in the Lyra system, used for:
//! - High-throughput content-addressed storage
//! - Non-constitutional integrity checks
//! - Performance-sensitive hashing paths where SHA-3-256 is not required
//!
//! This module wraps the `blake3` crate and exposes a single routing function.
//! All callers should use [`crate::digest::digest`] or [`crate::digest::blake3`]
//! rather than calling this module directly.

use crate::digest::{DigestAlgorithm, DigestOutput};

/// Compute a BLAKE3 digest of `input`.
///
/// Returns a [`DigestOutput`] tagged with [`DigestAlgorithm::Blake3`].
pub fn blake3_digest(input: &[u8]) -> DigestOutput {
    let hash = ::blake3::hash(input);
    let bytes: [u8; 32] = *hash.as_bytes();
    DigestOutput::new(DigestAlgorithm::Blake3, bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blake3_empty_string_golden() {
        // Known BLAKE3 of empty string
        let out = blake3_digest(b"");
        assert_eq!(
            out.to_hex(),
            "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
        );
    }

    #[test]
    fn blake3_output_is_32_bytes() {
        let out = blake3_digest(b"test");
        assert_eq!(out.as_bytes().len(), 32);
    }

    #[test]
    fn blake3_is_deterministic() {
        let a = blake3_digest(b"lyra");
        let b = blake3_digest(b"lyra");
        assert_eq!(a, b);
    }

    #[test]
    fn blake3_algorithm_tag_is_correct() {
        let out = blake3_digest(b"test");
        assert_eq!(out.algorithm, DigestAlgorithm::Blake3);
    }

    #[test]
    fn blake3_different_inputs_differ() {
        let a = blake3_digest(b"input_a");
        let b = blake3_digest(b"input_b");
        assert_ne!(a.as_bytes(), b.as_bytes());
    }
}
