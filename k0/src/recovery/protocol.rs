//! Recovery protocols — structured recovery procedures per incident kind (P0-014).

use crate::incident::{Incident, IncidentKind, IncidentSeverity};
use crate::time::VirtualTime;

/// A single recovery action step.
///
/// Actions are executed in sequence as part of a [`RecoveryPolicy`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryAction {
    /// Log the incident for audit and observability.
    Log,
    /// Retry the failed operation once.
    Retry,
    /// Roll back to the last known-good state.
    Rollback,
    /// Isolate the affected subsystem from the rest of the system.
    Isolate,
    /// Escalate to the next severity tier or operator.
    Escalate,
    /// Halt the system — used for constitutional violations.
    Halt,
}

impl RecoveryAction {
    /// Return the human-readable name of this action.
    pub fn name(&self) -> &'static str {
        match self {
            RecoveryAction::Log => "log",
            RecoveryAction::Retry => "retry",
            RecoveryAction::Rollback => "rollback",
            RecoveryAction::Isolate => "isolate",
            RecoveryAction::Escalate => "escalate",
            RecoveryAction::Halt => "halt",
        }
    }
}

impl std::fmt::Display for RecoveryAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// The canonical recovery policy for an incident kind.
///
/// A policy is an ordered sequence of [`RecoveryAction`]s to execute.
/// Policies are derived from the incident kind's severity and constitutional status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPolicy {
    /// The incident kind this policy applies to.
    pub kind: IncidentKind,
    /// The ordered sequence of recovery actions.
    pub actions: &'static [RecoveryAction],
}

// Static action sequences for each severity tier.
static ACTIONS_CONSTITUTIONAL: &[RecoveryAction] = &[
    RecoveryAction::Log,
    RecoveryAction::Isolate,
    RecoveryAction::Halt,
];

static ACTIONS_HIGH: &[RecoveryAction] = &[
    RecoveryAction::Log,
    RecoveryAction::Rollback,
    RecoveryAction::Escalate,
];

static ACTIONS_MEDIUM: &[RecoveryAction] = &[
    RecoveryAction::Log,
    RecoveryAction::Retry,
    RecoveryAction::Escalate,
];

static ACTIONS_LOW: &[RecoveryAction] = &[RecoveryAction::Log];

impl RecoveryPolicy {
    /// Return the canonical recovery policy for the given incident kind.
    ///
    /// Policy is determined by severity:
    /// - `Critical` (constitutional) → Log + Isolate + Halt
    /// - `High`                      → Log + Rollback + Escalate
    /// - `Medium`                    → Log + Retry + Escalate
    /// - `Low`                       → Log
    pub fn for_kind(kind: IncidentKind) -> Self {
        let actions = match kind.severity() {
            IncidentSeverity::Critical => ACTIONS_CONSTITUTIONAL,
            IncidentSeverity::High => ACTIONS_HIGH,
            IncidentSeverity::Medium => ACTIONS_MEDIUM,
            IncidentSeverity::Low => ACTIONS_LOW,
        };
        RecoveryPolicy { kind, actions }
    }

    /// Return `true` if this policy includes a `Halt` action.
    pub fn halts(&self) -> bool {
        self.actions.contains(&RecoveryAction::Halt)
    }

    /// Return `true` if this policy includes an `Escalate` action.
    pub fn escalates(&self) -> bool {
        self.actions.contains(&RecoveryAction::Escalate)
    }

    /// Return `true` if this policy includes a `Rollback` action.
    pub fn rolls_back(&self) -> bool {
        self.actions.contains(&RecoveryAction::Rollback)
    }
}

/// The outcome of executing a recovery protocol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryOutcome {
    /// The incident was handled and the system can continue.
    Recovered {
        /// The incident that was handled.
        incident_code: &'static str,
        /// The actions that were executed.
        actions_taken: &'static [RecoveryAction],
        /// The virtual timestamp of recovery.
        timestamp: VirtualTime,
    },
    /// The incident was escalated to a higher tier.
    Escalated {
        /// The incident that was escalated.
        incident_code: &'static str,
        /// The actions that were executed before escalation.
        actions_taken: &'static [RecoveryAction],
        /// The virtual timestamp of escalation.
        timestamp: VirtualTime,
    },
    /// The system was halted due to a constitutional violation.
    Halted {
        /// The incident that caused the halt.
        incident_code: &'static str,
        /// The actions that were executed before halt.
        actions_taken: &'static [RecoveryAction],
        /// The virtual timestamp of halt.
        timestamp: VirtualTime,
    },
}

impl RecoveryOutcome {
    /// Return `true` if the system was halted.
    pub fn is_halted(&self) -> bool {
        matches!(self, RecoveryOutcome::Halted { .. })
    }

    /// Return `true` if the incident was escalated.
    pub fn is_escalated(&self) -> bool {
        matches!(self, RecoveryOutcome::Escalated { .. })
    }

    /// Return `true` if the incident was recovered without halt or escalation.
    pub fn is_recovered(&self) -> bool {
        matches!(self, RecoveryOutcome::Recovered { .. })
    }

    /// Return the incident code for this outcome.
    pub fn incident_code(&self) -> &'static str {
        match self {
            RecoveryOutcome::Recovered { incident_code, .. } => incident_code,
            RecoveryOutcome::Escalated { incident_code, .. } => incident_code,
            RecoveryOutcome::Halted { incident_code, .. } => incident_code,
        }
    }
}

impl std::fmt::Display for RecoveryOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecoveryOutcome::Recovered {
                incident_code,
                timestamp,
                ..
            } => write!(f, "Recovered({incident_code} at {timestamp})"),
            RecoveryOutcome::Escalated {
                incident_code,
                timestamp,
                ..
            } => write!(f, "Escalated({incident_code} at {timestamp})"),
            RecoveryOutcome::Halted {
                incident_code,
                timestamp,
                ..
            } => write!(f, "Halted({incident_code} at {timestamp})"),
        }
    }
}

/// Executes the canonical recovery protocol for an incident.
///
/// `RecoveryProtocol` is a stateless executor: given an [`Incident`] and a
/// virtual timestamp, it looks up the canonical [`RecoveryPolicy`] for the
/// incident kind and returns the appropriate [`RecoveryOutcome`].
pub struct RecoveryProtocol;

impl RecoveryProtocol {
    /// Execute the canonical recovery protocol for the given incident.
    ///
    /// Returns:
    /// - `Halted` for constitutional (Critical) incidents
    /// - `Escalated` for High and Medium incidents
    /// - `Recovered` for Low incidents
    pub fn execute(incident: &Incident, timestamp: VirtualTime) -> RecoveryOutcome {
        let policy = RecoveryPolicy::for_kind(incident.kind);
        let code = incident.kind.code();

        if policy.halts() {
            RecoveryOutcome::Halted {
                incident_code: code,
                actions_taken: policy.actions,
                timestamp,
            }
        } else if policy.escalates() {
            RecoveryOutcome::Escalated {
                incident_code: code,
                actions_taken: policy.actions,
                timestamp,
            }
        } else {
            RecoveryOutcome::Recovered {
                incident_code: code,
                actions_taken: policy.actions,
                timestamp,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::VirtualTime;

    fn t(n: u64) -> VirtualTime {
        VirtualTime::new(n)
    }

    fn incident(kind: IncidentKind) -> Incident {
        Incident::new(kind, "test", t(1), "")
    }

    // ── RecoveryAction ────────────────────────────────────────────────────

    #[test]
    fn action_names_are_correct() {
        assert_eq!(RecoveryAction::Log.name(), "log");
        assert_eq!(RecoveryAction::Retry.name(), "retry");
        assert_eq!(RecoveryAction::Rollback.name(), "rollback");
        assert_eq!(RecoveryAction::Isolate.name(), "isolate");
        assert_eq!(RecoveryAction::Escalate.name(), "escalate");
        assert_eq!(RecoveryAction::Halt.name(), "halt");
    }

    #[test]
    fn action_display_matches_name() {
        assert_eq!(format!("{}", RecoveryAction::Halt), "halt");
        assert_eq!(format!("{}", RecoveryAction::Log), "log");
    }

    // ── RecoveryPolicy ────────────────────────────────────────────────────

    #[test]
    fn constitutional_policy_halts() {
        let p = RecoveryPolicy::for_kind(IncidentKind::DeterminismViolation);
        assert!(p.halts());
        assert!(!p.escalates());
    }

    #[test]
    fn constitutional_policy_includes_isolate() {
        let p = RecoveryPolicy::for_kind(IncidentKind::ConstitutionalBreach);
        assert!(p.actions.contains(&RecoveryAction::Isolate));
    }

    #[test]
    fn high_policy_escalates_and_rolls_back() {
        let p = RecoveryPolicy::for_kind(IncidentKind::EncodingError);
        assert!(p.escalates());
        assert!(p.rolls_back());
        assert!(!p.halts());
    }

    #[test]
    fn medium_policy_escalates_and_retries() {
        let p = RecoveryPolicy::for_kind(IncidentKind::EntropyAnomaly);
        assert!(p.escalates());
        assert!(p.actions.contains(&RecoveryAction::Retry));
        assert!(!p.halts());
        assert!(!p.rolls_back());
    }

    #[test]
    fn low_policy_only_logs() {
        let p = RecoveryPolicy::for_kind(IncidentKind::RecoverableError);
        assert_eq!(p.actions, &[RecoveryAction::Log]);
        assert!(!p.halts());
        assert!(!p.escalates());
    }

    #[test]
    fn all_policies_include_log() {
        let all = [
            IncidentKind::DeterminismViolation,
            IncidentKind::EncodingError,
            IncidentKind::EntropyAnomaly,
            IncidentKind::RecoverableError,
        ];
        for kind in &all {
            let p = RecoveryPolicy::for_kind(*kind);
            assert!(
                p.actions.contains(&RecoveryAction::Log),
                "{kind:?} policy must include Log"
            );
        }
    }

    #[test]
    fn policy_kind_is_stored() {
        let p = RecoveryPolicy::for_kind(IncidentKind::DigestMismatch);
        assert_eq!(p.kind, IncidentKind::DigestMismatch);
    }

    // ── RecoveryOutcome ───────────────────────────────────────────────────

    #[test]
    fn constitutional_incident_produces_halted_outcome() {
        let inc = incident(IncidentKind::DeterminismViolation);
        let outcome = RecoveryProtocol::execute(&inc, t(5));
        assert!(outcome.is_halted());
        assert!(!outcome.is_escalated());
        assert!(!outcome.is_recovered());
    }

    #[test]
    fn high_incident_produces_escalated_outcome() {
        let inc = incident(IncidentKind::EncodingError);
        let outcome = RecoveryProtocol::execute(&inc, t(5));
        assert!(outcome.is_escalated());
        assert!(!outcome.is_halted());
    }

    #[test]
    fn medium_incident_produces_escalated_outcome() {
        let inc = incident(IncidentKind::TimeAnomaly);
        let outcome = RecoveryProtocol::execute(&inc, t(5));
        assert!(outcome.is_escalated());
    }

    #[test]
    fn low_incident_produces_recovered_outcome() {
        let inc = incident(IncidentKind::RecoverableError);
        let outcome = RecoveryProtocol::execute(&inc, t(5));
        assert!(outcome.is_recovered());
        assert!(!outcome.is_halted());
        assert!(!outcome.is_escalated());
    }

    #[test]
    fn outcome_incident_code_matches() {
        let inc = incident(IncidentKind::DigestMismatch);
        let outcome = RecoveryProtocol::execute(&inc, t(1));
        assert_eq!(outcome.incident_code(), "INC-004");
    }

    #[test]
    fn outcome_timestamp_is_stored() {
        let inc = incident(IncidentKind::RecoverableError);
        let outcome = RecoveryProtocol::execute(&inc, t(42));
        match outcome {
            RecoveryOutcome::Recovered { timestamp, .. } => assert_eq!(timestamp, t(42)),
            _ => panic!("expected Recovered"),
        }
    }

    #[test]
    fn halted_outcome_display_includes_code_and_timestamp() {
        let inc = incident(IncidentKind::DeterminismViolation);
        let outcome = RecoveryProtocol::execute(&inc, t(7));
        let s = format!("{outcome}");
        assert!(s.contains("Halted"));
        assert!(s.contains("INC-001"));
        assert!(s.contains("t:7"));
    }

    #[test]
    fn escalated_outcome_display_includes_code() {
        let inc = incident(IncidentKind::EncodingError);
        let outcome = RecoveryProtocol::execute(&inc, t(3));
        let s = format!("{outcome}");
        assert!(s.contains("Escalated"));
        assert!(s.contains("INC-005"));
    }

    #[test]
    fn all_constitutional_kinds_produce_halted() {
        for kind in IncidentKind::constitutional_kinds() {
            let inc = incident(*kind);
            let outcome = RecoveryProtocol::execute(&inc, t(0));
            assert!(outcome.is_halted(), "{kind:?} must produce Halted");
        }
    }

    #[test]
    fn trust_root_violation_halts() {
        let inc = incident(IncidentKind::TrustRootViolation);
        let outcome = RecoveryProtocol::execute(&inc, t(1));
        assert!(outcome.is_halted());
    }

    #[test]
    fn schema_version_mismatch_escalates() {
        let inc = incident(IncidentKind::SchemaVersionMismatch);
        let outcome = RecoveryProtocol::execute(&inc, t(1));
        assert!(outcome.is_escalated());
    }

    #[test]
    fn operational_drift_escalates() {
        let inc = incident(IncidentKind::OperationalDrift);
        let outcome = RecoveryProtocol::execute(&inc, t(1));
        assert!(outcome.is_escalated());
    }

    #[test]
    fn unknown_incident_recovers() {
        let inc = incident(IncidentKind::Unknown);
        let outcome = RecoveryProtocol::execute(&inc, t(1));
        assert!(outcome.is_recovered());
    }
}
