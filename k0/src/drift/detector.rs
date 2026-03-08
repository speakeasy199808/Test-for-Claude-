//! [`DriftDetector`] — runtime nondeterminism monitoring layer (P0-012).
//!
//! Wraps [`DeterminismVerifier`] and adds drift event classification,
//! severity tagging, and structured reporting.

use super::error::DriftError;
use crate::time::VirtualTime;
use crate::verifier::{DeterminismVerifier, VerifierError};

/// Severity classification of a detected drift event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DriftSeverity {
    /// Operational drift: suspicious but not a constitutional violation.
    /// Example: empty output where non-empty was expected.
    Operational,
    /// Constitutional drift: a computation produced different outputs on
    /// identical runs. This is a direct violation of P0-003.
    Constitutional,
}

impl DriftSeverity {
    /// Returns the human-readable name of this severity level.
    pub fn name(&self) -> &'static str {
        match self {
            DriftSeverity::Operational => "operational",
            DriftSeverity::Constitutional => "constitutional",
        }
    }
}

impl std::fmt::Display for DriftSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// A single detected drift event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriftEvent {
    /// Human-readable label for the drift point.
    pub label: String,
    /// Severity of this drift event.
    pub severity: DriftSeverity,
    /// Hex encoding of the first run's output (empty string for operational drift).
    pub first_hex: String,
    /// Hex encoding of the second run's output (empty string for operational drift).
    pub second_hex: String,
    /// Virtual timestamp when this event was detected.
    pub timestamp: VirtualTime,
}

impl DriftEvent {
    /// Returns `true` if this is a constitutional drift event.
    pub fn is_constitutional(&self) -> bool {
        self.severity == DriftSeverity::Constitutional
    }

    /// Returns `true` if this is an operational drift event.
    pub fn is_operational(&self) -> bool {
        self.severity == DriftSeverity::Operational
    }
}

/// A structured summary of all drift events detected in a session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriftReport {
    /// All detected drift events, in detection order.
    pub events: Vec<DriftEvent>,
    /// Total number of checks performed (pass + drift).
    pub total_checks: usize,
    /// Number of checks that passed (no drift).
    pub passed: usize,
}

impl DriftReport {
    /// Returns the total number of drift events detected.
    pub fn drift_count(&self) -> usize {
        self.events.len()
    }

    /// Returns the number of constitutional drift events.
    pub fn constitutional_count(&self) -> usize {
        self.events.iter().filter(|e| e.is_constitutional()).count()
    }

    /// Returns the number of operational drift events.
    pub fn operational_count(&self) -> usize {
        self.events.iter().filter(|e| e.is_operational()).count()
    }

    /// Returns `true` if no drift was detected.
    pub fn is_clean(&self) -> bool {
        self.events.is_empty()
    }

    /// Returns `true` if any constitutional drift was detected.
    pub fn has_constitutional_drift(&self) -> bool {
        self.constitutional_count() > 0
    }
}

/// Runtime nondeterminism monitor.
///
/// `DriftDetector` wraps a [`DeterminismVerifier`] and adds drift event
/// classification and structured reporting. It is the runtime monitoring
/// layer for the constitutional determinism invariant (P0-003).
///
/// # Severity Model
/// - **Constitutional**: outputs differ on identical runs — P0-003 violation.
/// - **Operational**: empty output where non-empty expected — suspicious.
///
/// # Usage
/// ```rust
/// use k0::drift::DriftDetector;
///
/// let mut d = DriftDetector::new();
/// d.check("my-fn", || vec![1, 2, 3]).unwrap();
/// let report = d.report();
/// assert!(report.is_clean());
/// ```
#[derive(Debug)]
pub struct DriftDetector {
    verifier: DeterminismVerifier,
    events: Vec<DriftEvent>,
    total_checks: usize,
}

impl DriftDetector {
    /// Create a new drift detector with a fresh internal verifier.
    pub fn new() -> Self {
        DriftDetector {
            verifier: DeterminismVerifier::new(),
            events: Vec::new(),
            total_checks: 0,
        }
    }

    /// Check `f` for drift by running it twice and comparing outputs.
    ///
    /// Returns `Ok(output)` if no drift is detected.
    /// Returns `Err(DriftError)` if drift is detected, and records the event.
    ///
    /// Empty outputs are treated as operational drift.
    pub fn check(&mut self, label: &str, f: impl Fn() -> Vec<u8>) -> Result<Vec<u8>, DriftError> {
        self.total_checks += 1;
        let _timestamp = self.verifier.now();

        match self.verifier.verify(label, f) {
            Ok(output) => Ok(output),
            Err(VerifierError::DeterminismViolation {
                label,
                first_hex,
                second_hex,
            }) => {
                // Advance timestamp to match what verify() consumed
                let ts = self.verifier.now();
                self.events.push(DriftEvent {
                    label: label.clone(),
                    severity: DriftSeverity::Constitutional,
                    first_hex: first_hex.clone(),
                    second_hex: second_hex.clone(),
                    timestamp: ts,
                });
                Err(DriftError::ConstitutionalDrift {
                    label,
                    first_hex,
                    second_hex,
                })
            }
            Err(VerifierError::EmptyOutput { label }) => {
                let ts = self.verifier.now();
                self.events.push(DriftEvent {
                    label: label.clone(),
                    severity: DriftSeverity::Operational,
                    first_hex: String::new(),
                    second_hex: String::new(),
                    timestamp: ts,
                });
                Err(DriftError::EmptyOutputDrift { label })
            }
        }
    }

    /// Check `f` for drift, allowing empty outputs (no operational drift for empty).
    pub fn check_allow_empty(
        &mut self,
        label: &str,
        f: impl Fn() -> Vec<u8>,
    ) -> Result<Vec<u8>, DriftError> {
        self.total_checks += 1;

        match self.verifier.verify_allow_empty(label, f) {
            Ok(output) => Ok(output),
            Err(VerifierError::DeterminismViolation {
                label,
                first_hex,
                second_hex,
            }) => {
                let ts = self.verifier.now();
                self.events.push(DriftEvent {
                    label: label.clone(),
                    severity: DriftSeverity::Constitutional,
                    first_hex: first_hex.clone(),
                    second_hex: second_hex.clone(),
                    timestamp: ts,
                });
                Err(DriftError::ConstitutionalDrift {
                    label,
                    first_hex,
                    second_hex,
                })
            }
            Err(VerifierError::EmptyOutput { label }) => {
                // Should not happen with verify_allow_empty, but handle defensively
                Err(DriftError::EmptyOutputDrift { label })
            }
        }
    }

    /// Return a structured report of all drift events detected so far.
    pub fn report(&self) -> DriftReport {
        DriftReport {
            events: self.events.clone(),
            total_checks: self.total_checks,
            passed: self.total_checks.saturating_sub(self.events.len()),
        }
    }

    /// Return all detected drift events.
    pub fn events(&self) -> &[DriftEvent] {
        &self.events
    }

    /// Return the total number of checks performed.
    pub fn total_checks(&self) -> usize {
        self.total_checks
    }

    /// Return `true` if no drift has been detected.
    pub fn is_clean(&self) -> bool {
        self.events.is_empty()
    }

    /// Return `true` if any constitutional drift has been detected.
    pub fn has_constitutional_drift(&self) -> bool {
        self.events.iter().any(|e| e.is_constitutional())
    }
}

impl Default for DriftDetector {
    fn default() -> Self {
        DriftDetector::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn det() -> Vec<u8> {
        vec![0xca, 0xfe]
    }

    fn nondet() -> Vec<u8> {
        use std::sync::atomic::{AtomicU8, Ordering};
        static C: AtomicU8 = AtomicU8::new(0);
        vec![C.fetch_add(1, Ordering::SeqCst)]
    }

    #[test]
    fn clean_check_passes() {
        let mut d = DriftDetector::new();
        let result = d.check("det", det);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![0xca, 0xfe]);
    }

    #[test]
    fn nondeterministic_check_fails_with_constitutional_drift() {
        let mut d = DriftDetector::new();
        let result = d.check("nondet", nondet);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DriftError::ConstitutionalDrift { .. }
        ));
    }

    #[test]
    fn empty_output_produces_operational_drift() {
        let mut d = DriftDetector::new();
        let result = d.check("empty", std::vec::Vec::new);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DriftError::EmptyOutputDrift { .. }
        ));
    }

    #[test]
    fn check_allow_empty_passes_for_empty() {
        let mut d = DriftDetector::new();
        let result = d.check_allow_empty("empty", std::vec::Vec::new);
        assert!(result.is_ok());
    }

    #[test]
    fn is_clean_true_when_no_drift() {
        let mut d = DriftDetector::new();
        d.check("a", det).unwrap();
        d.check("b", det).unwrap();
        assert!(d.is_clean());
    }

    #[test]
    fn is_clean_false_after_drift() {
        let mut d = DriftDetector::new();
        let _ = d.check("nondet", nondet);
        assert!(!d.is_clean());
    }

    #[test]
    fn has_constitutional_drift_true_after_violation() {
        let mut d = DriftDetector::new();
        let _ = d.check("nondet", nondet);
        assert!(d.has_constitutional_drift());
    }

    #[test]
    fn has_constitutional_drift_false_for_operational_only() {
        let mut d = DriftDetector::new();
        let _ = d.check("empty", std::vec::Vec::new);
        assert!(!d.has_constitutional_drift());
    }

    #[test]
    fn report_is_clean_when_no_drift() {
        let mut d = DriftDetector::new();
        d.check("a", det).unwrap();
        let r = d.report();
        assert!(r.is_clean());
        assert_eq!(r.total_checks, 1);
        assert_eq!(r.passed, 1);
        assert_eq!(r.drift_count(), 0);
    }

    #[test]
    fn report_counts_constitutional_drift() {
        let mut d = DriftDetector::new();
        d.check("a", det).unwrap();
        let _ = d.check("nondet", nondet);
        let r = d.report();
        assert_eq!(r.total_checks, 2);
        assert_eq!(r.passed, 1);
        assert_eq!(r.constitutional_count(), 1);
        assert_eq!(r.operational_count(), 0);
        assert!(r.has_constitutional_drift());
    }

    #[test]
    fn report_counts_operational_drift() {
        let mut d = DriftDetector::new();
        let _ = d.check("empty", std::vec::Vec::new);
        let r = d.report();
        assert_eq!(r.operational_count(), 1);
        assert_eq!(r.constitutional_count(), 0);
        assert!(!r.has_constitutional_drift());
    }

    #[test]
    fn total_checks_increments_on_each_call() {
        let mut d = DriftDetector::new();
        assert_eq!(d.total_checks(), 0);
        d.check("a", det).unwrap();
        assert_eq!(d.total_checks(), 1);
        let _ = d.check("nondet", nondet);
        assert_eq!(d.total_checks(), 2);
    }

    #[test]
    fn drift_event_label_is_stored() {
        let mut d = DriftDetector::new();
        let _ = d.check("my-label", nondet);
        assert_eq!(d.events()[0].label, "my-label");
    }

    #[test]
    fn drift_event_severity_constitutional() {
        let mut d = DriftDetector::new();
        let _ = d.check("nondet", nondet);
        assert_eq!(d.events()[0].severity, DriftSeverity::Constitutional);
        assert!(d.events()[0].is_constitutional());
    }

    #[test]
    fn drift_event_severity_operational() {
        let mut d = DriftDetector::new();
        let _ = d.check("empty", std::vec::Vec::new);
        assert_eq!(d.events()[0].severity, DriftSeverity::Operational);
        assert!(d.events()[0].is_operational());
    }

    #[test]
    fn drift_severity_ordering() {
        assert!(DriftSeverity::Constitutional > DriftSeverity::Operational);
    }

    #[test]
    fn drift_severity_names() {
        assert_eq!(DriftSeverity::Constitutional.name(), "constitutional");
        assert_eq!(DriftSeverity::Operational.name(), "operational");
    }

    #[test]
    fn drift_error_is_constitutional() {
        let e = DriftError::ConstitutionalDrift {
            label: "x".to_string(),
            first_hex: "aa".to_string(),
            second_hex: "bb".to_string(),
        };
        assert!(e.is_constitutional());
        assert!(!e.is_operational());
    }

    #[test]
    fn drift_error_is_operational() {
        let e = DriftError::EmptyOutputDrift {
            label: "x".to_string(),
        };
        assert!(e.is_operational());
        assert!(!e.is_constitutional());
    }

    #[test]
    fn default_detector_starts_clean() {
        let d = DriftDetector::default();
        assert!(d.is_clean());
        assert_eq!(d.total_checks(), 0);
    }

    #[test]
    fn multiple_drift_events_accumulated() {
        let mut d = DriftDetector::new();
        let _ = d.check("empty1", std::vec::Vec::new);
        let _ = d.check("empty2", std::vec::Vec::new);
        let _ = d.check("nondet", nondet);
        let r = d.report();
        assert_eq!(r.drift_count(), 3);
        assert_eq!(r.operational_count(), 2);
        assert_eq!(r.constitutional_count(), 1);
    }
}
