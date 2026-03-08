//! [`DeterminismVerifier`] — double-run nondeterminism detector (P0-011).
//!
//! Runs a computation twice with identical inputs and compares outputs.
//! Any divergence is a constitutional violation.

use super::error::VerifierError;
use crate::time::{VirtualClock, VirtualTime};

/// Outcome of a single determinism verification run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationOutcome {
    /// Both runs produced identical output — determinism confirmed.
    Pass {
        /// Hex encoding of the (identical) output.
        output_hex: String,
        /// Virtual timestamp of this verification.
        timestamp: VirtualTime,
    },
    /// The two runs produced different output — constitutional violation.
    Fail {
        /// Hex encoding of the first run's output.
        first_hex: String,
        /// Hex encoding of the second run's output.
        second_hex: String,
        /// Virtual timestamp of this verification.
        timestamp: VirtualTime,
    },
}

impl VerificationOutcome {
    /// Returns `true` if this outcome is a pass.
    pub fn is_pass(&self) -> bool {
        matches!(self, VerificationOutcome::Pass { .. })
    }

    /// Returns `true` if this outcome is a fail.
    pub fn is_fail(&self) -> bool {
        matches!(self, VerificationOutcome::Fail { .. })
    }

    /// Returns the virtual timestamp of this outcome.
    pub fn timestamp(&self) -> VirtualTime {
        match self {
            VerificationOutcome::Pass { timestamp, .. } => *timestamp,
            VerificationOutcome::Fail { timestamp, .. } => *timestamp,
        }
    }
}

/// A record of a single determinism verification event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationRecord {
    /// Human-readable label for this verification point.
    pub label: String,
    /// The outcome of the verification.
    pub outcome: VerificationOutcome,
}

/// A stateful determinism verifier with a virtual clock and audit log.
///
/// `DeterminismVerifier` runs each registered computation twice and compares
/// outputs. All verification events are recorded with virtual timestamps.
///
/// # Constitutional Requirement
/// Determinism is a constitutional invariant (P0-003). Any computation that
/// produces different outputs on identical inputs is a violation.
#[derive(Debug)]
pub struct DeterminismVerifier {
    clock: VirtualClock,
    records: Vec<VerificationRecord>,
}

impl DeterminismVerifier {
    /// Create a new verifier with a fresh virtual clock.
    pub fn new() -> Self {
        DeterminismVerifier {
            clock: VirtualClock::new(),
            records: Vec::new(),
        }
    }

    /// Create a verifier with a pre-seeded virtual clock (for replay/testing).
    pub fn with_clock(clock: VirtualClock) -> Self {
        DeterminismVerifier {
            clock,
            records: Vec::new(),
        }
    }

    /// Verify that `f` is deterministic by running it twice and comparing outputs.
    ///
    /// `f` must be a pure function: same call, same output. The output is
    /// returned as `Vec<u8>` for byte-level comparison.
    ///
    /// Returns `Ok(output)` if both runs match, `Err(VerifierError)` if they differ
    /// or if the output is empty (use [`verify_allow_empty`](Self::verify_allow_empty)
    /// for intentionally empty outputs).
    ///
    /// The verification event is recorded in the audit log regardless of outcome.
    pub fn verify(
        &mut self,
        label: &str,
        f: impl Fn() -> Vec<u8>,
    ) -> Result<Vec<u8>, VerifierError> {
        self.verify_inner(label, f, false)
    }

    /// Verify determinism, allowing empty outputs.
    ///
    /// Same as [`verify`](Self::verify) but does not reject empty outputs.
    pub fn verify_allow_empty(
        &mut self,
        label: &str,
        f: impl Fn() -> Vec<u8>,
    ) -> Result<Vec<u8>, VerifierError> {
        self.verify_inner(label, f, true)
    }

    fn verify_inner(
        &mut self,
        label: &str,
        f: impl Fn() -> Vec<u8>,
        allow_empty: bool,
    ) -> Result<Vec<u8>, VerifierError> {
        let timestamp = self.clock.tick();

        let run1 = f();
        let run2 = f();

        if !allow_empty && run1.is_empty() && run2.is_empty() {
            // Record as a structural error — don't add to audit log as pass/fail
            return Err(VerifierError::EmptyOutput {
                label: label.to_string(),
            });
        }

        let outcome = if run1 == run2 {
            VerificationOutcome::Pass {
                output_hex: hex_encode(&run1),
                timestamp,
            }
        } else {
            VerificationOutcome::Fail {
                first_hex: hex_encode(&run1),
                second_hex: hex_encode(&run2),
                timestamp,
            }
        };

        let is_fail = outcome.is_fail();
        self.records.push(VerificationRecord {
            label: label.to_string(),
            outcome: outcome.clone(),
        });

        if is_fail {
            Err(VerifierError::DeterminismViolation {
                label: label.to_string(),
                first_hex: hex_encode(&run1),
                second_hex: hex_encode(&run2),
            })
        } else {
            Ok(run1)
        }
    }

    /// Return all recorded verification events.
    pub fn records(&self) -> &[VerificationRecord] {
        &self.records
    }

    /// Return the number of recorded verification events.
    pub fn record_count(&self) -> usize {
        self.records.len()
    }

    /// Return the number of passing verifications.
    pub fn pass_count(&self) -> usize {
        self.records.iter().filter(|r| r.outcome.is_pass()).count()
    }

    /// Return the number of failing verifications.
    pub fn fail_count(&self) -> usize {
        self.records.iter().filter(|r| r.outcome.is_fail()).count()
    }

    /// Return `true` if all recorded verifications passed.
    pub fn all_pass(&self) -> bool {
        self.fail_count() == 0
    }

    /// Return the current virtual clock time.
    pub fn now(&self) -> VirtualTime {
        self.clock.now()
    }
}

impl Default for DeterminismVerifier {
    fn default() -> Self {
        DeterminismVerifier::new()
    }
}

/// Stateless helper: verify a single function call without a verifier instance.
///
/// Returns `Ok(output)` if both runs match, `Err` if they differ.
pub fn verify_once(label: &str, f: impl Fn() -> Vec<u8>) -> Result<Vec<u8>, VerifierError> {
    let run1 = f();
    let run2 = f();
    if run1 == run2 {
        Ok(run1)
    } else {
        Err(VerifierError::DeterminismViolation {
            label: label.to_string(),
            first_hex: hex_encode(&run1),
            second_hex: hex_encode(&run2),
        })
    }
}

/// Encode bytes as lowercase hex string.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deterministic_fn() -> Vec<u8> {
        vec![0xde, 0xad, 0xbe, 0xef]
    }

    fn nondeterministic_fn() -> Vec<u8> {
        // Simulate nondeterminism by using a static counter
        use std::sync::atomic::{AtomicU8, Ordering};
        static COUNTER: AtomicU8 = AtomicU8::new(0);
        let v = COUNTER.fetch_add(1, Ordering::SeqCst);
        vec![v]
    }

    #[test]
    fn deterministic_fn_passes() {
        let mut v = DeterminismVerifier::new();
        let result = v.verify("test", deterministic_fn);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn nondeterministic_fn_fails() {
        let mut v = DeterminismVerifier::new();
        let result = v.verify("nondeterministic", nondeterministic_fn);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            VerifierError::DeterminismViolation { .. }
        ));
    }

    #[test]
    fn records_are_appended() {
        let mut v = DeterminismVerifier::new();
        let _ = v.verify("a", deterministic_fn);
        let _ = v.verify("b", deterministic_fn);
        assert_eq!(v.record_count(), 2);
    }

    #[test]
    fn pass_count_and_fail_count() {
        let mut v = DeterminismVerifier::new();
        let _ = v.verify("pass1", deterministic_fn);
        let _ = v.verify("pass2", deterministic_fn);
        let _ = v.verify("fail1", nondeterministic_fn);
        assert_eq!(v.pass_count(), 2);
        assert_eq!(v.fail_count(), 1);
    }

    #[test]
    fn all_pass_true_when_no_failures() {
        let mut v = DeterminismVerifier::new();
        let _ = v.verify("a", deterministic_fn);
        let _ = v.verify("b", deterministic_fn);
        assert!(v.all_pass());
    }

    #[test]
    fn all_pass_false_when_failure_present() {
        let mut v = DeterminismVerifier::new();
        let _ = v.verify("a", deterministic_fn);
        let _ = v.verify("fail", nondeterministic_fn);
        assert!(!v.all_pass());
    }

    #[test]
    fn clock_advances_on_each_verify() {
        let mut v = DeterminismVerifier::new();
        assert_eq!(v.now(), VirtualTime::ZERO);
        let _ = v.verify("a", deterministic_fn);
        assert_eq!(v.now(), VirtualTime::new(1));
        let _ = v.verify("b", deterministic_fn);
        assert_eq!(v.now(), VirtualTime::new(2));
    }

    #[test]
    fn record_timestamps_are_monotonic() {
        let mut v = DeterminismVerifier::new();
        let _ = v.verify("a", deterministic_fn);
        let _ = v.verify("b", deterministic_fn);
        let records = v.records();
        assert!(records[0].outcome.timestamp() < records[1].outcome.timestamp());
    }

    #[test]
    fn empty_output_rejected_by_default() {
        let mut v = DeterminismVerifier::new();
        let result = v.verify("empty", std::vec::Vec::new);
        assert!(matches!(result, Err(VerifierError::EmptyOutput { .. })));
    }

    #[test]
    fn empty_output_allowed_with_allow_empty() {
        let mut v = DeterminismVerifier::new();
        let result = v.verify_allow_empty("empty", std::vec::Vec::new);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn verify_once_passes_for_deterministic() {
        let result = verify_once("test", deterministic_fn);
        assert!(result.is_ok());
    }

    #[test]
    fn verify_once_fails_for_nondeterministic() {
        let result = verify_once("test", nondeterministic_fn);
        assert!(result.is_err());
    }

    #[test]
    fn outcome_is_pass_helper() {
        let outcome = VerificationOutcome::Pass {
            output_hex: "deadbeef".to_string(),
            timestamp: VirtualTime::new(1),
        };
        assert!(outcome.is_pass());
        assert!(!outcome.is_fail());
    }

    #[test]
    fn outcome_is_fail_helper() {
        let outcome = VerificationOutcome::Fail {
            first_hex: "aa".to_string(),
            second_hex: "bb".to_string(),
            timestamp: VirtualTime::new(1),
        };
        assert!(outcome.is_fail());
        assert!(!outcome.is_pass());
    }

    #[test]
    fn record_label_is_stored() {
        let mut v = DeterminismVerifier::new();
        let _ = v.verify("my-label", deterministic_fn);
        assert_eq!(v.records()[0].label, "my-label");
    }

    #[test]
    fn hex_encode_correct() {
        assert_eq!(hex_encode(&[0xde, 0xad, 0xbe, 0xef]), "deadbeef");
        assert_eq!(hex_encode(&[0x00, 0xff]), "00ff");
        assert_eq!(hex_encode(&[]), "");
    }

    #[test]
    fn default_verifier_starts_empty() {
        let v = DeterminismVerifier::default();
        assert_eq!(v.record_count(), 0);
        assert_eq!(v.now(), VirtualTime::ZERO);
    }

    #[test]
    fn verifier_with_codec_output() {
        // Verify that the canonical encoder is deterministic
        use crate::codec::{encode, Value};
        let mut v = DeterminismVerifier::new();
        let result = v.verify("codec-uint-42", || {
            encode(&Value::UInt(42)).unwrap_or_default()
        });
        assert!(result.is_ok());
        assert!(v.all_pass());
    }

    #[test]
    fn verifier_with_digest_output() {
        // Verify that SHA-3-256 is deterministic
        use crate::digest::{digest, DigestAlgorithm};
        let mut v = DeterminismVerifier::new();
        let result = v.verify("sha3-256-abc", || {
            digest(DigestAlgorithm::Sha3_256, b"abc")
                .as_bytes()
                .to_vec()
        });
        assert!(result.is_ok());
        assert!(v.all_pass());
    }
}
