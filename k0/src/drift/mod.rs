//! Drift Detection — runtime nondeterminism monitoring (P0-012).
//!
//! This module provides the runtime monitoring layer for the constitutional
//! determinism invariant (P0-003). It wraps the [`DeterminismVerifier`]
//! (P0-011) and adds drift event classification, severity tagging, and
//! structured reporting.
//!
//! # Severity Model
//! - **Constitutional** ([`DriftSeverity::Constitutional`]): a computation
//!   produced different outputs on two identical runs — direct P0-003 violation.
//! - **Operational** ([`DriftSeverity::Operational`]): a computation produced
//!   an empty output where non-empty was expected — suspicious but not a
//!   constitutional violation.
//!
//! # Usage
//! ```rust
//! use k0::drift::DriftDetector;
//!
//! let mut d = DriftDetector::new();
//! d.check("sha3-empty", || {
//!     use k0::digest::{DigestAlgorithm, digest};
//!     digest(DigestAlgorithm::Sha3_256, b"").as_bytes().to_vec()
//! }).unwrap();
//! let report = d.report();
//! assert!(report.is_clean());
//! assert_eq!(report.total_checks, 1);
//! assert_eq!(report.passed, 1);
//! ```

pub mod detector;
pub mod error;

pub use detector::{DriftDetector, DriftEvent, DriftReport, DriftSeverity};
pub use error::DriftError;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::{encode, Value};
    use crate::digest::{digest, DigestAlgorithm};

    #[test]
    fn drift_detector_accessible_from_mod() {
        let mut d = DriftDetector::new();
        let result = d.check("test", || vec![0x01, 0x02]);
        assert!(result.is_ok());
        assert!(d.is_clean());
    }

    #[test]
    fn drift_report_accessible_from_mod() {
        let mut d = DriftDetector::new();
        d.check("a", || vec![0xaa]).unwrap();
        let r = d.report();
        assert!(r.is_clean());
        assert_eq!(r.total_checks, 1);
        assert_eq!(r.passed, 1);
    }

    #[test]
    fn drift_severity_accessible_from_mod() {
        assert_eq!(DriftSeverity::Constitutional.name(), "constitutional");
        assert_eq!(DriftSeverity::Operational.name(), "operational");
    }

    #[test]
    fn drift_error_accessible_from_mod() {
        let e = DriftError::ConstitutionalDrift {
            label: "x".to_string(),
            first_hex: "aa".to_string(),
            second_hex: "bb".to_string(),
        };
        assert!(e.is_constitutional());
    }

    #[test]
    fn sha3_digest_is_drift_free() {
        let mut d = DriftDetector::new();
        d.check("sha3-empty", || {
            digest(DigestAlgorithm::Sha3_256, b"").as_bytes().to_vec()
        })
        .unwrap();
        d.check("sha3-lyra", || {
            digest(DigestAlgorithm::Sha3_256, b"lyra")
                .as_bytes()
                .to_vec()
        })
        .unwrap();
        d.check("blake3-empty", || {
            digest(DigestAlgorithm::Blake3, b"").as_bytes().to_vec()
        })
        .unwrap();
        let r = d.report();
        assert!(r.is_clean());
        assert_eq!(r.total_checks, 3);
        assert_eq!(r.passed, 3);
    }

    #[test]
    fn codec_encoder_is_drift_free() {
        let mut d = DriftDetector::new();
        d.check("uint-0", || encode(&Value::UInt(0)).unwrap_or_default())
            .unwrap();
        d.check("uint-255", || encode(&Value::UInt(255)).unwrap_or_default())
            .unwrap();
        d.check("str-lyra", || {
            encode(&Value::Str("lyra".to_string())).unwrap_or_default()
        })
        .unwrap();
        let r = d.report();
        assert!(r.is_clean());
        assert!(!r.has_constitutional_drift());
    }

    #[test]
    fn constitutional_drift_detected_and_reported() {
        use std::sync::atomic::{AtomicU8, Ordering};
        static C: AtomicU8 = AtomicU8::new(0);
        let mut d = DriftDetector::new();
        let _ = d.check("nondet", || vec![C.fetch_add(1, Ordering::SeqCst)]);
        let r = d.report();
        assert!(!r.is_clean());
        assert!(r.has_constitutional_drift());
        assert_eq!(r.constitutional_count(), 1);
    }
}
