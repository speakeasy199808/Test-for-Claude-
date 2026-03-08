//! Incident taxonomy — canonical classification of all Lyra incident kinds (P0-013).

/// Severity level of an incident.
///
/// Ordered from lowest to highest: `Low < Medium < High < Critical`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IncidentSeverity {
    /// Low severity — informational, no immediate action required.
    Low,
    /// Medium severity — operational anomaly, investigation warranted.
    Medium,
    /// High severity — significant operational failure, action required.
    High,
    /// Critical severity — constitutional violation, system integrity at risk.
    Critical,
}

impl IncidentSeverity {
    /// Return the human-readable name of this severity level.
    pub fn name(&self) -> &'static str {
        match self {
            IncidentSeverity::Low => "low",
            IncidentSeverity::Medium => "medium",
            IncidentSeverity::High => "high",
            IncidentSeverity::Critical => "critical",
        }
    }

    /// Return `true` if this severity is constitutional (Critical).
    pub fn is_constitutional(&self) -> bool {
        matches!(self, IncidentSeverity::Critical)
    }
}

impl std::fmt::Display for IncidentSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// Canonical taxonomy of all incident kinds in the Lyra system.
///
/// Each variant maps to a fixed [`IncidentSeverity`] and a canonical description.
/// This taxonomy is the authoritative source for incident classification across
/// all Lyra subsystems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IncidentKind {
    // ── Constitutional violations (Critical) ──────────────────────────────
    /// A computation produced different outputs on identical inputs.
    /// Direct violation of the determinism invariant (P0-003).
    DeterminismViolation,
    /// A constitutional check was bypassed or failed to execute.
    /// Violation of the non-bypassability invariant (P0-003).
    ConstitutionalBreach,
    /// A trust root fingerprint failed verification.
    /// Violation of the foundational guarantee invariant (P0-003).
    TrustRootViolation,
    /// A canonical digest did not match the expected value.
    /// Indicates data corruption or tampering at a constitutional boundary.
    DigestMismatch,

    // ── High severity operational failures ────────────────────────────────
    /// A canonical encoding operation failed or produced invalid output.
    EncodingError,
    /// A canonical decoding operation failed or rejected input.
    DecodingError,
    /// A schema version was unknown or incompatible.
    SchemaVersionMismatch,
    /// A state transition was rejected as invalid.
    StateTransitionRejected,

    // ── Medium severity operational anomalies ─────────────────────────────
    /// The entropy pool was exhausted or produced suspicious output.
    EntropyAnomaly,
    /// A virtual time anomaly was detected (e.g., backward reset attempt).
    TimeAnomaly,
    /// Operational drift was detected (empty output where non-empty expected).
    OperationalDrift,
    /// A subsystem boundary contract was violated.
    BoundaryViolation,

    // ── Low severity informational events ─────────────────────────────────
    /// A recoverable error was handled and logged.
    RecoverableError,
    /// An unknown or unclassified incident occurred.
    Unknown,
}

impl IncidentKind {
    /// Return the canonical severity for this incident kind.
    pub fn severity(&self) -> IncidentSeverity {
        match self {
            // Constitutional — Critical
            IncidentKind::DeterminismViolation => IncidentSeverity::Critical,
            IncidentKind::ConstitutionalBreach => IncidentSeverity::Critical,
            IncidentKind::TrustRootViolation => IncidentSeverity::Critical,
            IncidentKind::DigestMismatch => IncidentSeverity::Critical,
            // High
            IncidentKind::EncodingError => IncidentSeverity::High,
            IncidentKind::DecodingError => IncidentSeverity::High,
            IncidentKind::SchemaVersionMismatch => IncidentSeverity::High,
            IncidentKind::StateTransitionRejected => IncidentSeverity::High,
            // Medium
            IncidentKind::EntropyAnomaly => IncidentSeverity::Medium,
            IncidentKind::TimeAnomaly => IncidentSeverity::Medium,
            IncidentKind::OperationalDrift => IncidentSeverity::Medium,
            IncidentKind::BoundaryViolation => IncidentSeverity::Medium,
            // Low
            IncidentKind::RecoverableError => IncidentSeverity::Low,
            IncidentKind::Unknown => IncidentSeverity::Low,
        }
    }

    /// Return the canonical description for this incident kind.
    pub fn description(&self) -> &'static str {
        match self {
            IncidentKind::DeterminismViolation =>
                "A computation produced different outputs on identical inputs (P0-003 violation).",
            IncidentKind::ConstitutionalBreach =>
                "A constitutional check was bypassed or failed to execute (P0-003 non-bypassability violation).",
            IncidentKind::TrustRootViolation =>
                "A trust root fingerprint failed verification (P0-003 foundational guarantee violation).",
            IncidentKind::DigestMismatch =>
                "A canonical digest did not match the expected value (data integrity failure).",
            IncidentKind::EncodingError =>
                "A canonical encoding operation failed or produced invalid output.",
            IncidentKind::DecodingError =>
                "A canonical decoding operation failed or rejected input.",
            IncidentKind::SchemaVersionMismatch =>
                "A schema version was unknown or incompatible with the current decoder.",
            IncidentKind::StateTransitionRejected =>
                "A state transition was rejected as invalid by the admission checker.",
            IncidentKind::EntropyAnomaly =>
                "The entropy pool was exhausted or produced suspicious output.",
            IncidentKind::TimeAnomaly =>
                "A virtual time anomaly was detected (e.g., backward reset attempt).",
            IncidentKind::OperationalDrift =>
                "Operational drift detected: empty output where non-empty was expected.",
            IncidentKind::BoundaryViolation =>
                "A subsystem boundary contract was violated.",
            IncidentKind::RecoverableError =>
                "A recoverable error was handled and logged.",
            IncidentKind::Unknown =>
                "An unknown or unclassified incident occurred.",
        }
    }

    /// Return the canonical short code for this incident kind.
    pub fn code(&self) -> &'static str {
        match self {
            IncidentKind::DeterminismViolation => "INC-001",
            IncidentKind::ConstitutionalBreach => "INC-002",
            IncidentKind::TrustRootViolation => "INC-003",
            IncidentKind::DigestMismatch => "INC-004",
            IncidentKind::EncodingError => "INC-005",
            IncidentKind::DecodingError => "INC-006",
            IncidentKind::SchemaVersionMismatch => "INC-007",
            IncidentKind::StateTransitionRejected => "INC-008",
            IncidentKind::EntropyAnomaly => "INC-009",
            IncidentKind::TimeAnomaly => "INC-010",
            IncidentKind::OperationalDrift => "INC-011",
            IncidentKind::BoundaryViolation => "INC-012",
            IncidentKind::RecoverableError => "INC-013",
            IncidentKind::Unknown => "INC-000",
        }
    }

    /// Return `true` if this incident kind is a constitutional violation.
    pub fn is_constitutional(&self) -> bool {
        self.severity().is_constitutional()
    }

    /// Return all constitutional incident kinds.
    pub fn constitutional_kinds() -> &'static [IncidentKind] {
        &[
            IncidentKind::DeterminismViolation,
            IncidentKind::ConstitutionalBreach,
            IncidentKind::TrustRootViolation,
            IncidentKind::DigestMismatch,
        ]
    }
}

impl std::fmt::Display for IncidentKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.code(), self.severity())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determinism_violation_is_critical() {
        assert_eq!(
            IncidentKind::DeterminismViolation.severity(),
            IncidentSeverity::Critical
        );
        assert!(IncidentKind::DeterminismViolation.is_constitutional());
    }

    #[test]
    fn constitutional_breach_is_critical() {
        assert_eq!(
            IncidentKind::ConstitutionalBreach.severity(),
            IncidentSeverity::Critical
        );
        assert!(IncidentKind::ConstitutionalBreach.is_constitutional());
    }

    #[test]
    fn trust_root_violation_is_critical() {
        assert_eq!(
            IncidentKind::TrustRootViolation.severity(),
            IncidentSeverity::Critical
        );
        assert!(IncidentKind::TrustRootViolation.is_constitutional());
    }

    #[test]
    fn digest_mismatch_is_critical() {
        assert_eq!(
            IncidentKind::DigestMismatch.severity(),
            IncidentSeverity::Critical
        );
        assert!(IncidentKind::DigestMismatch.is_constitutional());
    }

    #[test]
    fn encoding_error_is_high() {
        assert_eq!(
            IncidentKind::EncodingError.severity(),
            IncidentSeverity::High
        );
        assert!(!IncidentKind::EncodingError.is_constitutional());
    }

    #[test]
    fn decoding_error_is_high() {
        assert_eq!(
            IncidentKind::DecodingError.severity(),
            IncidentSeverity::High
        );
    }

    #[test]
    fn schema_version_mismatch_is_high() {
        assert_eq!(
            IncidentKind::SchemaVersionMismatch.severity(),
            IncidentSeverity::High
        );
    }

    #[test]
    fn entropy_anomaly_is_medium() {
        assert_eq!(
            IncidentKind::EntropyAnomaly.severity(),
            IncidentSeverity::Medium
        );
    }

    #[test]
    fn time_anomaly_is_medium() {
        assert_eq!(
            IncidentKind::TimeAnomaly.severity(),
            IncidentSeverity::Medium
        );
    }

    #[test]
    fn operational_drift_is_medium() {
        assert_eq!(
            IncidentKind::OperationalDrift.severity(),
            IncidentSeverity::Medium
        );
    }

    #[test]
    fn recoverable_error_is_low() {
        assert_eq!(
            IncidentKind::RecoverableError.severity(),
            IncidentSeverity::Low
        );
    }

    #[test]
    fn unknown_is_low() {
        assert_eq!(IncidentKind::Unknown.severity(), IncidentSeverity::Low);
    }

    #[test]
    fn severity_ordering() {
        assert!(IncidentSeverity::Critical > IncidentSeverity::High);
        assert!(IncidentSeverity::High > IncidentSeverity::Medium);
        assert!(IncidentSeverity::Medium > IncidentSeverity::Low);
    }

    #[test]
    fn severity_is_constitutional_only_for_critical() {
        assert!(IncidentSeverity::Critical.is_constitutional());
        assert!(!IncidentSeverity::High.is_constitutional());
        assert!(!IncidentSeverity::Medium.is_constitutional());
        assert!(!IncidentSeverity::Low.is_constitutional());
    }

    #[test]
    fn incident_codes_are_unique() {
        let all = [
            IncidentKind::DeterminismViolation,
            IncidentKind::ConstitutionalBreach,
            IncidentKind::TrustRootViolation,
            IncidentKind::DigestMismatch,
            IncidentKind::EncodingError,
            IncidentKind::DecodingError,
            IncidentKind::SchemaVersionMismatch,
            IncidentKind::StateTransitionRejected,
            IncidentKind::EntropyAnomaly,
            IncidentKind::TimeAnomaly,
            IncidentKind::OperationalDrift,
            IncidentKind::BoundaryViolation,
            IncidentKind::RecoverableError,
            IncidentKind::Unknown,
        ];
        let mut codes: Vec<&str> = all.iter().map(|k| k.code()).collect();
        let original_len = codes.len();
        codes.sort_unstable();
        codes.dedup();
        assert_eq!(codes.len(), original_len, "incident codes must be unique");
    }

    #[test]
    fn constitutional_kinds_are_all_critical() {
        for kind in IncidentKind::constitutional_kinds() {
            assert_eq!(kind.severity(), IncidentSeverity::Critical);
        }
    }

    #[test]
    fn constitutional_kinds_count() {
        assert_eq!(IncidentKind::constitutional_kinds().len(), 4);
    }

    #[test]
    fn descriptions_are_non_empty() {
        let all = [
            IncidentKind::DeterminismViolation,
            IncidentKind::ConstitutionalBreach,
            IncidentKind::TrustRootViolation,
            IncidentKind::DigestMismatch,
            IncidentKind::EncodingError,
            IncidentKind::DecodingError,
            IncidentKind::SchemaVersionMismatch,
            IncidentKind::StateTransitionRejected,
            IncidentKind::EntropyAnomaly,
            IncidentKind::TimeAnomaly,
            IncidentKind::OperationalDrift,
            IncidentKind::BoundaryViolation,
            IncidentKind::RecoverableError,
            IncidentKind::Unknown,
        ];
        for kind in &all {
            assert!(!kind.description().is_empty());
        }
    }

    #[test]
    fn display_includes_code_and_severity() {
        let s = format!("{}", IncidentKind::DeterminismViolation);
        assert!(s.contains("INC-001"));
        assert!(s.contains("critical"));
    }
}
