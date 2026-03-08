//! Recovery protocols — structured recovery procedures per incident kind (P0-014).
//!
//! This module defines the canonical recovery protocol for the Lyra system.
//! Every incident classified by [`crate::incident`] has a corresponding
//! recovery policy that determines the sequence of actions to take.
//!
//! # Module Map
//! - [`protocol`] — [`RecoveryAction`], [`RecoveryPolicy`], [`RecoveryOutcome`], [`RecoveryProtocol`]
//!
//! # Recovery Tiers
//! ```text
//! Critical (constitutional) → Log + Isolate + Halt
//! High                      → Log + Rollback + Escalate
//! Medium                    → Log + Retry + Escalate
//! Low                       → Log
//! ```
//!
//! # Usage
//! ```rust
//! use k0::incident::{Incident, IncidentKind};
//! use k0::recovery::{RecoveryProtocol, RecoveryOutcome};
//! use k0::time::VirtualTime;
//!
//! let inc = Incident::new(IncidentKind::EncodingError, "enc", VirtualTime::new(1), "");
//! let outcome = RecoveryProtocol::execute(&inc, VirtualTime::new(2));
//! assert!(outcome.is_escalated());
//! ```

pub mod protocol;

pub use protocol::{RecoveryAction, RecoveryOutcome, RecoveryPolicy, RecoveryProtocol};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::incident::{Incident, IncidentKind};
    use crate::time::VirtualTime;

    fn t(n: u64) -> VirtualTime {
        VirtualTime::new(n)
    }

    fn inc(kind: IncidentKind) -> Incident {
        Incident::new(kind, "integration-test", t(1), "")
    }

    #[test]
    fn recovery_protocol_accessible_from_mod() {
        let i = inc(IncidentKind::RecoverableError);
        let outcome = RecoveryProtocol::execute(&i, t(2));
        assert!(outcome.is_recovered());
    }

    #[test]
    fn recovery_action_accessible_from_mod() {
        let action = RecoveryAction::Halt;
        assert_eq!(action.name(), "halt");
    }

    #[test]
    fn recovery_policy_accessible_from_mod() {
        let p = RecoveryPolicy::for_kind(IncidentKind::DeterminismViolation);
        assert!(p.halts());
    }

    #[test]
    fn recovery_outcome_accessible_from_mod() {
        let i = inc(IncidentKind::EncodingError);
        let outcome = RecoveryProtocol::execute(&i, t(1));
        assert!(matches!(outcome, RecoveryOutcome::Escalated { .. }));
    }

    #[test]
    fn constitutional_incident_halts_system() {
        for kind in IncidentKind::constitutional_kinds() {
            let i = inc(*kind);
            let outcome = RecoveryProtocol::execute(&i, t(0));
            assert!(
                outcome.is_halted(),
                "constitutional kind {kind:?} must halt",
            );
        }
    }

    #[test]
    fn high_severity_incidents_escalate() {
        let high_kinds = [
            IncidentKind::EncodingError,
            IncidentKind::DecodingError,
            IncidentKind::SchemaVersionMismatch,
            IncidentKind::StateTransitionRejected,
        ];
        for kind in &high_kinds {
            let i = inc(*kind);
            let outcome = RecoveryProtocol::execute(&i, t(0));
            assert!(outcome.is_escalated(), "{kind:?} must escalate");
        }
    }

    #[test]
    fn medium_severity_incidents_escalate() {
        let medium_kinds = [
            IncidentKind::EntropyAnomaly,
            IncidentKind::TimeAnomaly,
            IncidentKind::OperationalDrift,
            IncidentKind::BoundaryViolation,
        ];
        for kind in &medium_kinds {
            let i = inc(*kind);
            let outcome = RecoveryProtocol::execute(&i, t(0));
            assert!(outcome.is_escalated(), "{kind:?} must escalate");
        }
    }

    #[test]
    fn low_severity_incidents_recover() {
        let low_kinds = [IncidentKind::RecoverableError, IncidentKind::Unknown];
        for kind in &low_kinds {
            let i = inc(*kind);
            let outcome = RecoveryProtocol::execute(&i, t(0));
            assert!(outcome.is_recovered(), "{kind:?} must recover");
        }
    }
}
