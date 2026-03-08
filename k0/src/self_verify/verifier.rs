//! [`SelfVerifier`] and [`VerificationReceipt`] — runtime code integrity check.
//!
//! Computes SHA-3-256 of provided code bytes and compares against an
//! expected hash. Returns a receipt recording the result.

use crate::digest::{digest, DigestAlgorithm};
use crate::time::{VirtualClock, VirtualTime};

use super::error::SelfVerifyError;

/// A receipt recording the outcome of a self-verification check.
///
/// Contains the expected and actual hashes, the virtual timestamp of
/// the check, and whether the verification passed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationReceipt {
    /// The expected SHA-3-256 hash (32 bytes).
    pub expected_hash: [u8; 32],
    /// The actual SHA-3-256 hash computed from the code bytes (32 bytes).
    pub actual_hash: [u8; 32],
    /// The virtual timestamp when this verification was performed.
    pub timestamp: VirtualTime,
    /// Whether the verification passed (expected == actual).
    pub passed: bool,
}

impl VerificationReceipt {
    /// Return the expected hash as a 64-character lowercase hex string.
    pub fn expected_hex(&self) -> String {
        self.expected_hash
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect()
    }

    /// Return the actual hash as a 64-character lowercase hex string.
    pub fn actual_hex(&self) -> String {
        self.actual_hash
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect()
    }
}

impl std::fmt::Display for VerificationReceipt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.passed { "PASS" } else { "FAIL" };
        write!(
            f,
            "[self-verify] {} at {} expected={} actual={}",
            status,
            self.timestamp,
            self.expected_hex(),
            self.actual_hex()
        )
    }
}

/// A self-verifier that checks code integrity against an expected hash.
///
/// Holds the expected SHA-3-256 digest of the code. On each call to
/// [`verify`](SelfVerifier::verify), it computes the actual digest of
/// the provided bytes and compares.
///
/// # Determinism
/// SHA-3-256 is deterministic. Identical code bytes always produce the
/// same digest, so verification results are reproducible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelfVerifier {
    /// The expected SHA-3-256 hash of the code (32 bytes).
    expected_hash: [u8; 32],
}

impl SelfVerifier {
    /// Create a new self-verifier with the given expected hash.
    pub fn new(expected_hash: [u8; 32]) -> Self {
        Self { expected_hash }
    }

    /// Return the expected hash.
    pub fn expected_hash(&self) -> &[u8; 32] {
        &self.expected_hash
    }

    /// Verify the integrity of `code_bytes` against the expected hash.
    ///
    /// Computes SHA-3-256 of `code_bytes`, advances the clock by one tick,
    /// and returns a [`VerificationReceipt`] recording the result.
    ///
    /// The receipt's `passed` field is `true` if the computed hash matches
    /// the expected hash, `false` otherwise. On mismatch, the caller should
    /// emit an incident via [`crate::incident`].
    pub fn verify(
        &self,
        code_bytes: &[u8],
        clock: &mut VirtualClock,
    ) -> Result<VerificationReceipt, SelfVerifyError> {
        let timestamp = clock.tick();
        let actual = digest(DigestAlgorithm::Sha3_256, code_bytes);
        let actual_hash = *actual.as_bytes();
        let passed = actual_hash == self.expected_hash;

        Ok(VerificationReceipt {
            expected_hash: self.expected_hash,
            actual_hash,
            timestamp,
            passed,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::digest::{digest, DigestAlgorithm};

    fn compute_hash(data: &[u8]) -> [u8; 32] {
        *digest(DigestAlgorithm::Sha3_256, data).as_bytes()
    }

    #[test]
    fn verifier_stores_expected_hash() {
        let hash = [0xab; 32];
        let v = SelfVerifier::new(hash);
        assert_eq!(v.expected_hash(), &hash);
    }

    #[test]
    fn pass_when_hashes_match() {
        let code = b"fn main() { println!(\"hello\"); }";
        let hash = compute_hash(code);
        let v = SelfVerifier::new(hash);
        let mut clock = VirtualClock::new();
        let receipt = v.verify(code, &mut clock).unwrap();
        assert!(receipt.passed);
    }

    #[test]
    fn fail_when_hashes_differ() {
        let original = b"original";
        let tampered = b"tampered";
        let hash = compute_hash(original);
        let v = SelfVerifier::new(hash);
        let mut clock = VirtualClock::new();
        let receipt = v.verify(tampered, &mut clock).unwrap();
        assert!(!receipt.passed);
    }

    #[test]
    fn receipt_display_contains_status() {
        let code = b"test";
        let hash = compute_hash(code);
        let v = SelfVerifier::new(hash);
        let mut clock = VirtualClock::new();
        let receipt = v.verify(code, &mut clock).unwrap();
        let display = format!("{receipt}");
        assert!(display.contains("PASS"));
        assert!(display.contains("self-verify"));
    }

    #[test]
    fn receipt_display_fail_status() {
        let v = SelfVerifier::new([0u8; 32]);
        let mut clock = VirtualClock::new();
        let receipt = v.verify(b"not-matching", &mut clock).unwrap();
        let display = format!("{receipt}");
        assert!(display.contains("FAIL"));
    }

    #[test]
    fn empty_code_bytes_hash_is_sha3_empty() {
        let empty_hash = compute_hash(b"");
        let v = SelfVerifier::new(empty_hash);
        let mut clock = VirtualClock::new();
        let receipt = v.verify(b"", &mut clock).unwrap();
        assert!(receipt.passed);
        // SHA-3-256("") is a known value
        assert_eq!(
            receipt.actual_hex(),
            "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a"
        );
    }
}
