//! SHA-3-256 digest implementation for the Lyra digest routing module.
//!
//! SHA-3-256 is the **primary** digest algorithm for all canonical and
//! constitutional operations in the Lyra system. This includes:
//! - Constitutional hash computation (P0-001)
//! - Trust root fingerprints (P0-001, P0-002)
//! - Canonical state sealing
//!
//! This module wraps the `sha3` crate and exposes a single routing function.
//! All callers should use [`crate::digest::digest`] or [`crate::digest::sha3_256`]
//! rather than calling this module directly.

use sha3::{Digest, Sha3_256};

use crate::digest::{DigestAlgorithm, DigestOutput};

/// Compute a SHA-3-256 digest of `input`.
///
/// Returns a [`DigestOutput`] tagged with [`DigestAlgorithm::Sha3_256`].
pub fn sha3_256_digest(input: &[u8]) -> DigestOutput {
    let hash = Sha3_256::digest(input);
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&hash);
    DigestOutput::new(DigestAlgorithm::Sha3_256, bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha3_256_empty_string_golden() {
        // Known SHA-3-256 of empty string
        let out = sha3_256_digest(b"");
        assert_eq!(
            out.to_hex(),
            "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a"
        );
    }

    #[test]
    fn sha3_256_abc_golden() {
        // Known SHA-3-256 of "abc"
        let out = sha3_256_digest(b"abc");
        assert_eq!(
            out.to_hex(),
            "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532"
        );
    }

    #[test]
    fn sha3_256_output_is_32_bytes() {
        let out = sha3_256_digest(b"test");
        assert_eq!(out.as_bytes().len(), 32);
    }

    #[test]
    fn sha3_256_is_deterministic() {
        let a = sha3_256_digest(b"lyra");
        let b = sha3_256_digest(b"lyra");
        assert_eq!(a, b);
    }

    #[test]
    fn sha3_256_algorithm_tag_is_correct() {
        let out = sha3_256_digest(b"test");
        assert_eq!(out.algorithm, DigestAlgorithm::Sha3_256);
    }
}
