//! Determinism Verifier — double-run nondeterminism detector (P0-011).
//!
//! This module enforces the constitutional determinism invariant (P0-003):
//! identical inputs must produce identical outputs on every run.
//!
//! # Design
//! - [`DeterminismVerifier`] runs each computation twice and compares outputs.
//! - All verification events are recorded with [`VirtualTime`] timestamps.
//! - [`verify_once`] is a stateless helper for single-shot verification.
//! - [`VerifierError::DeterminismViolation`] is a constitutional violation.
//!
//! # Usage
//! ```rust
//! use k0::verifier::{DeterminismVerifier, verify_once};
//!
//! let mut v = DeterminismVerifier::new();
//! let output = v.verify("sha3-empty", || {
//!     use k0::digest::{DigestAlgorithm, digest};
//!     digest(DigestAlgorithm::Sha3_256, b"").as_bytes().to_vec()
//! }).unwrap();
//! assert_eq!(output.len(), 32);
//! assert!(v.all_pass());
//! ```

pub mod determinism;
pub mod error;

pub use determinism::{verify_once, DeterminismVerifier, VerificationOutcome, VerificationRecord};
pub use error::VerifierError;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::{encode, Value};
    use crate::digest::{digest, DigestAlgorithm};

    #[test]
    fn verifier_accessible_from_mod() {
        let mut v = DeterminismVerifier::new();
        let result = v.verify("test", || vec![1, 2, 3]);
        assert!(result.is_ok());
    }

    #[test]
    fn verify_once_accessible_from_mod() {
        let result = verify_once("test", || vec![0xaa, 0xbb]);
        assert!(result.is_ok());
    }

    #[test]
    fn codec_encoder_is_deterministic() {
        let mut v = DeterminismVerifier::new();
        // UInt
        v.verify("codec-uint-0", || {
            encode(&Value::UInt(0)).unwrap_or_default()
        })
        .unwrap();
        v.verify("codec-uint-127", || {
            encode(&Value::UInt(127)).unwrap_or_default()
        })
        .unwrap();
        v.verify("codec-uint-128", || {
            encode(&Value::UInt(128)).unwrap_or_default()
        })
        .unwrap();
        v.verify("codec-uint-max", || {
            encode(&Value::UInt(u64::MAX)).unwrap_or_default()
        })
        .unwrap();
        // SInt
        v.verify("codec-sint-neg1", || {
            encode(&Value::SInt(-1)).unwrap_or_default()
        })
        .unwrap();
        v.verify("codec-sint-0", || {
            encode(&Value::SInt(0)).unwrap_or_default()
        })
        .unwrap();
        // Bytes
        v.verify("codec-bytes", || {
            encode(&Value::Bytes(vec![0xde, 0xad])).unwrap_or_default()
        })
        .unwrap();
        // Str
        v.verify("codec-str", || {
            encode(&Value::Str("lyra".to_string())).unwrap_or_default()
        })
        .unwrap();
        assert!(v.all_pass());
        assert_eq!(v.fail_count(), 0);
    }

    #[test]
    fn digest_algorithms_are_deterministic() {
        let mut v = DeterminismVerifier::new();
        v.verify("sha3-empty", || {
            digest(DigestAlgorithm::Sha3_256, b"").as_bytes().to_vec()
        })
        .unwrap();
        v.verify("sha3-abc", || {
            digest(DigestAlgorithm::Sha3_256, b"abc")
                .as_bytes()
                .to_vec()
        })
        .unwrap();
        v.verify("blake3-empty", || {
            digest(DigestAlgorithm::Blake3, b"").as_bytes().to_vec()
        })
        .unwrap();
        v.verify("blake3-abc", || {
            digest(DigestAlgorithm::Blake3, b"abc").as_bytes().to_vec()
        })
        .unwrap();
        assert!(v.all_pass());
    }

    #[test]
    fn verifier_error_is_accessible() {
        let mut v = DeterminismVerifier::new();
        let err = v.verify("empty", std::vec::Vec::new).unwrap_err();
        assert!(matches!(err, VerifierError::EmptyOutput { .. }));
    }

    #[test]
    fn verification_outcome_variants_accessible() {
        let pass = VerificationOutcome::Pass {
            output_hex: "aabb".to_string(),
            timestamp: crate::time::VirtualTime::ZERO,
        };
        assert!(pass.is_pass());

        let fail = VerificationOutcome::Fail {
            first_hex: "aa".to_string(),
            second_hex: "bb".to_string(),
            timestamp: crate::time::VirtualTime::new(1),
        };
        assert!(fail.is_fail());
    }
}
