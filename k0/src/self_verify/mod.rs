//! Self Verification Loop — runtime code integrity verification (P0-005).
//!
//! This module provides a self-verification mechanism that computes a
//! cryptographic digest of code bytes and compares it against an expected
//! hash. This enables runtime tamper detection: if the code has been
//! modified, the verification will fail.
//!
//! # Design
//! - [`SelfVerifier`] holds an expected SHA-3-256 digest.
//! - [`SelfVerifier::verify`] computes the actual digest of provided code
//!   bytes and compares against the expected value.
//! - [`VerificationReceipt`] records the result with virtual timestamps.
//! - On mismatch, the receipt indicates failure — the caller can emit an
//!   incident via [`crate::incident`].
//!
//! # Determinism Guarantee
//! SHA-3-256 is deterministic: identical code bytes always produce the
//! same digest. Virtual timestamps ensure reproducible receipts.
//!
//! # Usage
//! ```rust
//! use k0::self_verify::{SelfVerifier, SelfVerifyError};
//! use k0::time::VirtualClock;
//! use k0::digest::{DigestAlgorithm, digest};
//!
//! let code = b"fn main() {}";
//! let expected = digest(DigestAlgorithm::Sha3_256, code);
//! let verifier = SelfVerifier::new(*expected.as_bytes());
//! let mut clock = VirtualClock::new();
//! let receipt = verifier.verify(code, &mut clock).unwrap();
//! assert!(receipt.passed);
//! ```

pub mod error;
pub mod verifier;

pub use error::SelfVerifyError;
pub use verifier::{SelfVerifier, VerificationReceipt};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::digest::{digest, DigestAlgorithm};
    use crate::time::VirtualClock;

    #[test]
    fn matching_hash_passes() {
        let code = b"constitutional law v1";
        let expected = digest(DigestAlgorithm::Sha3_256, code);
        let verifier = SelfVerifier::new(*expected.as_bytes());
        let mut clock = VirtualClock::new();
        let receipt = verifier.verify(code, &mut clock).unwrap();
        assert!(receipt.passed);
        assert_eq!(receipt.expected_hash, receipt.actual_hash);
    }

    #[test]
    fn mismatched_hash_fails() {
        let code = b"original code";
        let tampered = b"tampered code";
        let expected = digest(DigestAlgorithm::Sha3_256, code);
        let verifier = SelfVerifier::new(*expected.as_bytes());
        let mut clock = VirtualClock::new();
        let receipt = verifier.verify(tampered, &mut clock).unwrap();
        assert!(!receipt.passed);
        assert_ne!(receipt.expected_hash, receipt.actual_hash);
    }

    #[test]
    fn empty_input_produces_valid_receipt() {
        let code: &[u8] = b"";
        let expected = digest(DigestAlgorithm::Sha3_256, code);
        let verifier = SelfVerifier::new(*expected.as_bytes());
        let mut clock = VirtualClock::new();
        let receipt = verifier.verify(code, &mut clock).unwrap();
        assert!(receipt.passed);
    }

    #[test]
    fn verification_is_deterministic() {
        let code = b"deterministic payload";
        let expected = digest(DigestAlgorithm::Sha3_256, code);
        let verifier = SelfVerifier::new(*expected.as_bytes());

        let mut clock1 = VirtualClock::new();
        let receipt1 = verifier.verify(code, &mut clock1).unwrap();

        let mut clock2 = VirtualClock::new();
        let receipt2 = verifier.verify(code, &mut clock2).unwrap();

        assert_eq!(receipt1.expected_hash, receipt2.expected_hash);
        assert_eq!(receipt1.actual_hash, receipt2.actual_hash);
        assert_eq!(receipt1.passed, receipt2.passed);
        assert_eq!(receipt1.timestamp, receipt2.timestamp);
    }

    #[test]
    fn clock_advances_on_verify() {
        let code = b"test";
        let expected = digest(DigestAlgorithm::Sha3_256, code);
        let verifier = SelfVerifier::new(*expected.as_bytes());
        let mut clock = VirtualClock::new();
        let receipt = verifier.verify(code, &mut clock).unwrap();
        assert_eq!(receipt.timestamp.as_u64(), 1);
        let receipt2 = verifier.verify(code, &mut clock).unwrap();
        assert_eq!(receipt2.timestamp.as_u64(), 2);
    }

    #[test]
    fn receipt_hex_strings_are_64_chars() {
        let code = b"hex test";
        let expected = digest(DigestAlgorithm::Sha3_256, code);
        let verifier = SelfVerifier::new(*expected.as_bytes());
        let mut clock = VirtualClock::new();
        let receipt = verifier.verify(code, &mut clock).unwrap();
        assert_eq!(receipt.expected_hex().len(), 64);
        assert_eq!(receipt.actual_hex().len(), 64);
    }

    #[test]
    fn self_verifier_accessible_from_mod() {
        let verifier = SelfVerifier::new([0u8; 32]);
        let mut clock = VirtualClock::new();
        let receipt = verifier.verify(b"anything", &mut clock).unwrap();
        // Hash of "anything" won't match [0u8; 32], so should fail
        assert!(!receipt.passed);
    }
}
