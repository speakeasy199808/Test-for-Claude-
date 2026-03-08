//! Incident taxonomy — canonical classification of all Lyra incident kinds (P0-013).
//!
//! This module defines the authoritative incident taxonomy for the Lyra system.
//! Every subsystem that detects a failure, violation, or anomaly MUST classify
//! it using the types defined here.
//!
//! # Module Map
//! - [`taxonomy`] — [`IncidentKind`], [`IncidentSeverity`] (canonical classification)
//! - [`record`]   — [`Incident`] (structured incident event)
//!
//! # Severity Hierarchy
//! ```text
//! Critical  — constitutional violations (P0-003 invariant breaches)
//! High      — significant operational failures
//! Medium    — operational anomalies requiring investigation
//! Low       — informational / recoverable events
//! ```
//!
//! # Constitutional Kinds
//! The following kinds are constitutional (Critical severity):
//! - [`IncidentKind::DeterminismViolation`] — INC-001
//! - [`IncidentKind::ConstitutionalBreach`] — INC-002
//! - [`IncidentKind::TrustRootViolation`]   — INC-003
//! - [`IncidentKind::DigestMismatch`]       — INC-004

pub mod record;
pub mod taxonomy;

pub use record::Incident;
pub use taxonomy::{IncidentKind, IncidentSeverity};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::VirtualTime;

    fn t(n: u64) -> VirtualTime {
        VirtualTime::new(n)
    }

    // ── Taxonomy integration ──────────────────────────────────────────────

    #[test]
    fn all_constitutional_kinds_are_critical() {
        for kind in IncidentKind::constitutional_kinds() {
            assert_eq!(
                kind.severity(),
                IncidentSeverity::Critical,
                "{kind:?} must be Critical",
            );
        }
    }

    #[test]
    fn determinism_violation_maps_to_inc_001() {
        assert_eq!(IncidentKind::DeterminismViolation.code(), "INC-001");
    }

    #[test]
    fn encoding_error_maps_to_inc_005() {
        assert_eq!(IncidentKind::EncodingError.code(), "INC-005");
    }

    #[test]
    fn severity_ordering_critical_is_max() {
        assert!(IncidentSeverity::Critical > IncidentSeverity::High);
        assert!(IncidentSeverity::Critical > IncidentSeverity::Medium);
        assert!(IncidentSeverity::Critical > IncidentSeverity::Low);
    }

    // ── Incident record integration ───────────────────────────────────────

    #[test]
    fn incident_from_determinism_violation() {
        let inc = Incident::new(
            IncidentKind::DeterminismViolation,
            "codec-check",
            t(10),
            "first=cafe second=dead",
        );
        assert!(inc.is_constitutional());
        assert_eq!(inc.severity, IncidentSeverity::Critical);
        assert_eq!(inc.code(), "INC-001");
        assert_eq!(inc.label, "codec-check");
        assert_eq!(inc.context, "first=cafe second=dead");
    }

    #[test]
    fn incident_from_encoding_error() {
        let inc = Incident::new(
            IncidentKind::EncodingError,
            "enc-boundary",
            t(5),
            "bad tag 0xff",
        );
        assert!(!inc.is_constitutional());
        assert_eq!(inc.severity, IncidentSeverity::High);
    }

    #[test]
    fn incident_from_entropy_anomaly() {
        let inc = Incident::new_bare(IncidentKind::EntropyAnomaly, "pool-check", t(3));
        assert_eq!(inc.severity, IncidentSeverity::Medium);
        assert_eq!(inc.context, "");
    }

    #[test]
    fn incident_display_format() {
        let inc = Incident::new(IncidentKind::DigestMismatch, "genesis-hash", t(7), "");
        let s = format!("{inc}");
        assert!(s.contains("INC-004"));
        assert!(s.contains("critical"));
        assert!(s.contains("t:7"));
        assert!(s.contains("genesis-hash"));
    }

    #[test]
    fn incident_timestamp_preserved() {
        let inc = Incident::new(IncidentKind::TimeAnomaly, "clock", t(99), "");
        assert_eq!(inc.timestamp, t(99));
    }

    #[test]
    fn incident_kind_accessible_from_mod() {
        let kind = IncidentKind::ConstitutionalBreach;
        assert!(kind.is_constitutional());
    }

    #[test]
    fn incident_severity_accessible_from_mod() {
        let sev = IncidentSeverity::High;
        assert!(!sev.is_constitutional());
    }
}
