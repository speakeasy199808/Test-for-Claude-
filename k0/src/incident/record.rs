//! Incident record — structured incident event with kind, severity, and context (P0-013).

use super::taxonomy::{IncidentKind, IncidentSeverity};
use crate::time::VirtualTime;

/// A structured incident record.
///
/// An `Incident` captures a single classified event: its kind (from the
/// canonical taxonomy), severity, human-readable label, virtual timestamp,
/// and optional context string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Incident {
    /// The canonical kind of this incident.
    pub kind: IncidentKind,
    /// The severity of this incident (derived from kind).
    pub severity: IncidentSeverity,
    /// Human-readable label for this incident point.
    pub label: String,
    /// Virtual timestamp when this incident was recorded.
    pub timestamp: VirtualTime,
    /// Optional context string (error message, hex evidence, etc.).
    pub context: String,
}

impl Incident {
    /// Create a new incident with the given kind, label, timestamp, and context.
    ///
    /// Severity is derived automatically from the kind's canonical taxonomy entry.
    pub fn new(
        kind: IncidentKind,
        label: impl Into<String>,
        timestamp: VirtualTime,
        context: impl Into<String>,
    ) -> Self {
        let severity = kind.severity();
        Incident {
            kind,
            severity,
            label: label.into(),
            timestamp,
            context: context.into(),
        }
    }

    /// Create a new incident with an empty context string.
    pub fn new_bare(kind: IncidentKind, label: impl Into<String>, timestamp: VirtualTime) -> Self {
        Self::new(kind, label, timestamp, "")
    }

    /// Returns `true` if this incident is a constitutional violation.
    pub fn is_constitutional(&self) -> bool {
        self.kind.is_constitutional()
    }

    /// Returns the canonical incident code (e.g. `"INC-001"`).
    pub fn code(&self) -> &'static str {
        self.kind.code()
    }
}

impl std::fmt::Display for Incident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} at t:{} label={:?}",
            self.kind.code(),
            self.severity,
            self.timestamp,
            self.label
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(n: u64) -> VirtualTime {
        VirtualTime::new(n)
    }

    #[test]
    fn new_derives_severity_from_kind() {
        let inc = Incident::new(IncidentKind::DeterminismViolation, "test", t(1), "ctx");
        assert_eq!(inc.severity, IncidentSeverity::Critical);
    }

    #[test]
    fn new_stores_label_and_context() {
        let inc = Incident::new(IncidentKind::EncodingError, "enc-label", t(2), "bad bytes");
        assert_eq!(inc.label, "enc-label");
        assert_eq!(inc.context, "bad bytes");
    }

    #[test]
    fn new_bare_has_empty_context() {
        let inc = Incident::new_bare(IncidentKind::TimeAnomaly, "time-label", t(3));
        assert_eq!(inc.context, "");
    }

    #[test]
    fn is_constitutional_true_for_critical() {
        let inc = Incident::new(IncidentKind::ConstitutionalBreach, "x", t(0), "");
        assert!(inc.is_constitutional());
    }

    #[test]
    fn is_constitutional_false_for_non_critical() {
        let inc = Incident::new(IncidentKind::EncodingError, "x", t(0), "");
        assert!(!inc.is_constitutional());
    }

    #[test]
    fn code_matches_kind() {
        let inc = Incident::new(IncidentKind::DigestMismatch, "x", t(0), "");
        assert_eq!(inc.code(), "INC-004");
    }

    #[test]
    fn display_includes_code_severity_timestamp_label() {
        let inc = Incident::new(IncidentKind::DeterminismViolation, "my-fn", t(5), "");
        let s = format!("{inc}");
        assert!(s.contains("INC-001"));
        assert!(s.contains("critical"));
        assert!(s.contains("t:5"));
        assert!(s.contains("my-fn"));
    }

    #[test]
    fn timestamp_is_stored() {
        let inc = Incident::new(IncidentKind::Unknown, "x", t(42), "");
        assert_eq!(inc.timestamp, t(42));
    }

    #[test]
    fn kind_is_stored() {
        let inc = Incident::new(IncidentKind::TrustRootViolation, "x", t(0), "");
        assert_eq!(inc.kind, IncidentKind::TrustRootViolation);
    }
}
