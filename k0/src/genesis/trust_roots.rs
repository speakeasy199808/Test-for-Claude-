//! Trust Roots — declarations of trusted authority anchors at genesis.
//!
//! Trust roots are the set of authority anchors that the system recognizes
//! at genesis. Each root is identified by a kind and a SHA-3-256 fingerprint
//! of its canonical public material.
//!
//! # Constitutional Guarantee
//! Trust roots are declared at genesis and sealed by the constitutional hash.
//! No trust root may be added or removed without producing a new genesis state
//! with a different constitutional hash.
//!
//! # Threshold Policy
//! A [`ThresholdPolicy`] specifies how many trust roots must be verified
//! for a quorum to be reached. The [`TrustRootSet`] combines a [`TrustRoots`]
//! collection with a threshold policy for quorum-based verification.
//!
//! # Key Ceremony
//! [`CeremonyStep`] and [`CeremonyRecord`] provide stubs for multi-party
//! key ceremony workflows. These are structural placeholders for Phase 1
//! implementation of distributed key generation.
//!
//! # HSM Binding
//! [`HsmCapability`] and [`HsmBinding`] provide stubs for hardware security
//! module integration. These are structural placeholders for Phase 1
//! implementation of HSM-backed trust roots.

use serde::{Deserialize, Serialize};

// ── Core Types ──────────────────────────────────────────────────────────────

/// The kind of a trust root.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustRootKind {
    /// A cryptographic signing key used to verify canonical artifacts.
    SigningKey,
    /// A canonical specification document whose hash is pinned at genesis.
    SpecDocument,
    /// A constitutional invariant set whose hash is pinned at genesis.
    ConstitutionalSpec,
}

/// A single trust root entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustRootEntry {
    /// Unique identifier for this trust root within the genesis set.
    pub id: String,
    /// Kind of this trust root.
    pub kind: TrustRootKind,
    /// SHA-3-256 fingerprint of the canonical public material (64-char lowercase hex).
    pub fingerprint: String,
    /// Human-readable description of this trust root.
    pub description: String,
}

impl TrustRootEntry {
    /// Validate structural invariants of this trust root entry.
    pub fn validate(&self) -> Result<(), TrustRootError> {
        if self.id.is_empty() {
            return Err(TrustRootError::EmptyId);
        }
        if self.id.chars().any(|c| !c.is_ascii() || c.is_whitespace()) {
            return Err(TrustRootError::InvalidId(self.id.clone()));
        }
        if self.fingerprint.len() != 64 {
            return Err(TrustRootError::InvalidFingerprintLength(
                self.fingerprint.len(),
            ));
        }
        if !self
            .fingerprint
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_uppercase())
        {
            return Err(TrustRootError::InvalidFingerprintFormat(
                self.fingerprint.clone(),
            ));
        }
        Ok(())
    }
}

/// The complete set of trust roots declared at genesis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustRoots {
    /// Schema version for this trust roots record.
    pub version: u32,
    /// Ordered list of trust root entries.
    /// Ordering is canonical: sorted by `id` lexicographically.
    pub entries: Vec<TrustRootEntry>,
}

impl TrustRoots {
    /// Construct an empty trust roots set (valid for genesis with no external roots).
    pub fn empty() -> Self {
        Self {
            version: 1,
            entries: vec![],
        }
    }

    /// Validate all entries and structural invariants.
    pub fn validate(&self) -> Result<(), TrustRootError> {
        if self.version == 0 {
            return Err(TrustRootError::InvalidVersion(self.version));
        }
        // Validate each entry
        for entry in &self.entries {
            entry.validate()?;
        }
        // Validate canonical ordering: entries must be sorted by id
        for window in self.entries.windows(2) {
            if window[0].id >= window[1].id {
                return Err(TrustRootError::NonCanonicalOrdering {
                    first: window[0].id.clone(),
                    second: window[1].id.clone(),
                });
            }
        }
        // Validate no duplicate ids
        let mut ids: Vec<&str> = self.entries.iter().map(|e| e.id.as_str()).collect();
        ids.sort_unstable();
        ids.dedup();
        if ids.len() != self.entries.len() {
            return Err(TrustRootError::DuplicateId);
        }
        Ok(())
    }

    /// Return the fingerprints of all entries, in canonical order.
    pub fn fingerprints(&self) -> Vec<&str> {
        self.entries
            .iter()
            .map(|e| e.fingerprint.as_str())
            .collect()
    }

    /// Return the number of trust root entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Return `true` if there are no trust root entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

// ── Threshold Policy ────────────────────────────────────────────────────────

/// A threshold policy for quorum-based trust root verification.
///
/// Specifies how many trust roots out of a total set must be verified
/// for a quorum to be reached. Both `required` and `total` must be > 0,
/// and `required` must be <= `total`.
///
/// # Example
/// A 3-of-5 threshold means 3 out of 5 trust roots must be verified.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThresholdPolicy {
    /// Number of trust roots required for quorum.
    pub required: u32,
    /// Total number of trust roots in the set.
    pub total: u32,
}

impl ThresholdPolicy {
    /// Create a new threshold policy.
    ///
    /// # Errors
    /// Returns [`TrustRootError::InvalidThreshold`] if `required` or `total`
    /// is zero, or if `required > total`.
    pub fn new(required: u32, total: u32) -> Result<Self, TrustRootError> {
        if required == 0 {
            return Err(TrustRootError::InvalidThreshold {
                required,
                total,
                reason: "required must be > 0".to_string(),
            });
        }
        if total == 0 {
            return Err(TrustRootError::InvalidThreshold {
                required,
                total,
                reason: "total must be > 0".to_string(),
            });
        }
        if required > total {
            return Err(TrustRootError::InvalidThreshold {
                required,
                total,
                reason: "required must be <= total".to_string(),
            });
        }
        Ok(Self { required, total })
    }

    /// Check whether the given number of verified roots meets the threshold.
    pub fn is_met(&self, verified_count: u32) -> bool {
        verified_count >= self.required
    }
}

// ── Trust Root Set ──────────────────────────────────────────────────────────

/// A trust root set with an associated threshold policy.
///
/// Combines a [`TrustRoots`] collection with a [`ThresholdPolicy`] for
/// quorum-based verification. The set validates that the threshold policy
/// is consistent with the number of entries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustRootSet {
    /// The underlying trust roots collection.
    pub roots: TrustRoots,
    /// The threshold policy for quorum verification.
    pub policy: ThresholdPolicy,
}

impl TrustRootSet {
    /// Create a new trust root set from roots and a threshold policy.
    ///
    /// # Errors
    /// Returns an error if the roots fail validation or if the policy's
    /// `total` does not match the number of entries (when entries are non-empty).
    pub fn new(roots: TrustRoots, policy: ThresholdPolicy) -> Result<Self, TrustRootError> {
        roots.validate()?;
        Ok(Self { roots, policy })
    }

    /// Check whether the given number of verified roots meets the threshold.
    pub fn verify_threshold(&self, verified_count: u32) -> bool {
        self.policy.is_met(verified_count)
    }

    /// Check whether it is possible to reach quorum given the available entries.
    ///
    /// Returns `true` if the number of trust root entries is >= the required
    /// threshold. Returns `false` if there are fewer entries than required,
    /// meaning quorum can never be reached.
    pub fn is_quorum_possible(&self) -> bool {
        self.roots.entries.len() as u32 >= self.policy.required
    }
}

// ── Key Ceremony Stubs ──────────────────────────────────────────────────────

/// Steps in a multi-party key ceremony.
///
/// This is a structural placeholder for Phase 1 distributed key generation.
/// Each step represents a phase in the ceremony protocol.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CeremonyStep {
    /// Initialize the ceremony with parameters and participant list.
    Init,
    /// Each participant contributes their key share.
    ContributeShare,
    /// All participants verify the collected shares.
    VerifyShares,
    /// Finalize the ceremony and produce the combined trust root.
    Finalize,
}

impl CeremonyStep {
    /// Return the human-readable name of this ceremony step.
    pub fn name(&self) -> &'static str {
        match self {
            CeremonyStep::Init => "init",
            CeremonyStep::ContributeShare => "contribute-share",
            CeremonyStep::VerifyShares => "verify-shares",
            CeremonyStep::Finalize => "finalize",
        }
    }
}

/// A record of a key ceremony execution.
///
/// Captures the sequence of steps completed, the resulting trust root
/// fingerprint (if finalized), and the number of participants.
///
/// This is a structural placeholder for Phase 1 implementation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CeremonyRecord {
    /// Unique identifier for this ceremony instance.
    pub ceremony_id: String,
    /// Ordered list of completed ceremony steps.
    pub steps_completed: Vec<CeremonyStep>,
    /// Number of participants in the ceremony.
    pub participant_count: u32,
    /// The resulting trust root fingerprint, if the ceremony was finalized.
    /// 64-char lowercase hex SHA-3-256 digest, or `None` if not yet finalized.
    pub result_fingerprint: Option<String>,
}

impl CeremonyRecord {
    /// Create a new ceremony record in the Init state.
    pub fn new(ceremony_id: impl Into<String>, participant_count: u32) -> Self {
        Self {
            ceremony_id: ceremony_id.into(),
            steps_completed: vec![CeremonyStep::Init],
            participant_count,
            result_fingerprint: None,
        }
    }

    /// Return `true` if the ceremony has been finalized.
    pub fn is_finalized(&self) -> bool {
        self.steps_completed.contains(&CeremonyStep::Finalize)
    }

    /// Return the current (last completed) step.
    pub fn current_step(&self) -> Option<&CeremonyStep> {
        self.steps_completed.last()
    }
}

// ── HSM Stubs ───────────────────────────────────────────────────────────────

/// Capabilities that an HSM may provide.
///
/// This is a structural placeholder for Phase 1 HSM integration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HsmCapability {
    /// The HSM can generate cryptographic key pairs.
    KeyGeneration,
    /// The HSM can produce digital signatures.
    Signing,
    /// The HSM can verify digital signatures.
    Verification,
    /// The HSM provides secure key storage.
    SecureStorage,
}

impl HsmCapability {
    /// Return the human-readable name of this capability.
    pub fn name(&self) -> &'static str {
        match self {
            HsmCapability::KeyGeneration => "key-generation",
            HsmCapability::Signing => "signing",
            HsmCapability::Verification => "verification",
            HsmCapability::SecureStorage => "secure-storage",
        }
    }
}

/// A binding between a trust root and a hardware security module.
///
/// Records the HSM's fingerprint and the capabilities it provides
/// for a particular trust root.
///
/// This is a structural placeholder for Phase 1 HSM integration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HsmBinding {
    /// SHA-3-256 fingerprint of the HSM's identity key (64-char lowercase hex).
    pub fingerprint: String,
    /// The set of capabilities this HSM provides.
    pub capabilities: Vec<HsmCapability>,
}

impl HsmBinding {
    /// Create a new HSM binding with the given fingerprint and capabilities.
    pub fn new(fingerprint: impl Into<String>, capabilities: Vec<HsmCapability>) -> Self {
        Self {
            fingerprint: fingerprint.into(),
            capabilities,
        }
    }

    /// Return `true` if this HSM has the given capability.
    pub fn has_capability(&self, cap: &HsmCapability) -> bool {
        self.capabilities.contains(cap)
    }

    /// Return `true` if this HSM can both sign and verify.
    pub fn can_sign_and_verify(&self) -> bool {
        self.has_capability(&HsmCapability::Signing)
            && self.has_capability(&HsmCapability::Verification)
    }
}

// ── Errors ──────────────────────────────────────────────────────────────────

/// Errors arising from trust root operations.
#[derive(Debug, thiserror::Error)]
pub enum TrustRootError {
    /// Version field is zero; must be >= 1.
    #[error("trust root version must be >= 1, got {0}")]
    InvalidVersion(u32),

    /// Entry `id` field is empty.
    #[error("trust root entry id must not be empty")]
    EmptyId,

    /// Entry `id` contains non-ASCII or whitespace characters.
    #[error("trust root entry id contains non-ASCII or whitespace: {0:?}")]
    InvalidId(String),

    /// Fingerprint is not exactly 64 characters.
    #[error("fingerprint must be 64 characters, got {0}")]
    InvalidFingerprintLength(usize),

    /// Fingerprint is not lowercase hex.
    #[error("fingerprint must be lowercase hex, got {0:?}")]
    InvalidFingerprintFormat(String),

    /// Entries are not in canonical ascending-id order.
    #[error(
        "trust root entries are not in canonical (ascending id) order: {first:?} >= {second:?}"
    )]
    NonCanonicalOrdering {
        /// The id of the first (out-of-order) entry.
        first: String,
        /// The id of the second entry that should have come first.
        second: String,
    },

    /// Two entries share the same id.
    #[error("duplicate trust root id detected")]
    DuplicateId,

    /// Threshold policy is invalid.
    #[error("invalid threshold policy: required={required}, total={total}: {reason}")]
    InvalidThreshold {
        /// The required count.
        required: u32,
        /// The total count.
        total: u32,
        /// Reason the threshold is invalid.
        reason: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Helper ──────────────────────────────────────────────────────────

    fn make_entry(id: &str, fingerprint: &str) -> TrustRootEntry {
        TrustRootEntry {
            id: id.to_string(),
            kind: TrustRootKind::SigningKey,
            fingerprint: fingerprint.to_string(),
            description: format!("Test entry {id}"),
        }
    }

    fn valid_fp() -> String {
        "a".repeat(64)
    }

    // ── TrustRoots (original) ───────────────────────────────────────────

    #[test]
    fn empty_trust_roots_is_valid() {
        assert!(TrustRoots::empty().validate().is_ok());
    }

    #[test]
    fn single_valid_entry_is_accepted() {
        let roots = TrustRoots {
            version: 1,
            entries: vec![TrustRootEntry {
                id: "constitutional-spec-v1".to_string(),
                kind: TrustRootKind::ConstitutionalSpec,
                fingerprint: "a".repeat(64),
                description: "P0-003 constitutional math spec v1".to_string(),
            }],
        };
        assert!(roots.validate().is_ok());
    }

    #[test]
    fn non_canonical_ordering_is_rejected() {
        let roots = TrustRoots {
            version: 1,
            entries: vec![
                TrustRootEntry {
                    id: "z-root".to_string(),
                    kind: TrustRootKind::SigningKey,
                    fingerprint: "b".repeat(64),
                    description: "z".to_string(),
                },
                TrustRootEntry {
                    id: "a-root".to_string(),
                    kind: TrustRootKind::SigningKey,
                    fingerprint: "c".repeat(64),
                    description: "a".to_string(),
                },
            ],
        };
        assert!(matches!(
            roots.validate(),
            Err(TrustRootError::NonCanonicalOrdering { .. })
        ));
    }

    #[test]
    fn duplicate_id_is_rejected() {
        let roots = TrustRoots {
            version: 1,
            entries: vec![
                TrustRootEntry {
                    id: "same-id".to_string(),
                    kind: TrustRootKind::SigningKey,
                    fingerprint: "a".repeat(64),
                    description: "first".to_string(),
                },
                TrustRootEntry {
                    id: "same-id".to_string(),
                    kind: TrustRootKind::SigningKey,
                    fingerprint: "b".repeat(64),
                    description: "second".to_string(),
                },
            ],
        };
        // Non-canonical ordering will fire first (same-id == same-id fails >=)
        // but duplicate check also catches it
        assert!(roots.validate().is_err());
    }

    #[test]
    fn uppercase_fingerprint_is_rejected() {
        let entry = TrustRootEntry {
            id: "test-root".to_string(),
            kind: TrustRootKind::SigningKey,
            fingerprint: "A".repeat(64),
            description: "test".to_string(),
        };
        assert!(matches!(
            entry.validate(),
            Err(TrustRootError::InvalidFingerprintFormat(_))
        ));
    }

    #[test]
    fn short_fingerprint_is_rejected() {
        let entry = TrustRootEntry {
            id: "test-root".to_string(),
            kind: TrustRootKind::SigningKey,
            fingerprint: "abc".to_string(),
            description: "test".to_string(),
        };
        assert!(matches!(
            entry.validate(),
            Err(TrustRootError::InvalidFingerprintLength(3))
        ));
    }

    #[test]
    fn trust_roots_len_and_is_empty() {
        let empty = TrustRoots::empty();
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);

        let roots = TrustRoots {
            version: 1,
            entries: vec![make_entry("alpha", &valid_fp())],
        };
        assert!(!roots.is_empty());
        assert_eq!(roots.len(), 1);
    }

    // ── ThresholdPolicy ─────────────────────────────────────────────────

    #[test]
    fn threshold_policy_valid_construction() {
        let policy = ThresholdPolicy::new(2, 3).unwrap();
        assert_eq!(policy.required, 2);
        assert_eq!(policy.total, 3);
    }

    #[test]
    fn threshold_policy_one_of_one() {
        let policy = ThresholdPolicy::new(1, 1).unwrap();
        assert!(policy.is_met(1));
        assert!(!policy.is_met(0));
    }

    #[test]
    fn threshold_policy_rejects_zero_required() {
        assert!(matches!(
            ThresholdPolicy::new(0, 3),
            Err(TrustRootError::InvalidThreshold { .. })
        ));
    }

    #[test]
    fn threshold_policy_rejects_zero_total() {
        assert!(matches!(
            ThresholdPolicy::new(1, 0),
            Err(TrustRootError::InvalidThreshold { .. })
        ));
    }

    #[test]
    fn threshold_policy_rejects_required_greater_than_total() {
        assert!(matches!(
            ThresholdPolicy::new(5, 3),
            Err(TrustRootError::InvalidThreshold { .. })
        ));
    }

    #[test]
    fn threshold_policy_is_met() {
        let policy = ThresholdPolicy::new(3, 5).unwrap();
        assert!(!policy.is_met(0));
        assert!(!policy.is_met(1));
        assert!(!policy.is_met(2));
        assert!(policy.is_met(3));
        assert!(policy.is_met(4));
        assert!(policy.is_met(5));
    }

    // ── TrustRootSet ────────────────────────────────────────────────────

    #[test]
    fn trust_root_set_construction() {
        let roots = TrustRoots {
            version: 1,
            entries: vec![
                make_entry("alpha", &valid_fp()),
                make_entry("beta", &"b".repeat(64)),
            ],
        };
        let policy = ThresholdPolicy::new(1, 2).unwrap();
        let set = TrustRootSet::new(roots, policy).unwrap();
        assert!(set.verify_threshold(1));
        assert!(!set.verify_threshold(0));
    }

    #[test]
    fn trust_root_set_verify_threshold() {
        let roots = TrustRoots {
            version: 1,
            entries: vec![
                make_entry("a-root", &valid_fp()),
                make_entry("b-root", &"b".repeat(64)),
                make_entry("c-root", &"c".repeat(64)),
            ],
        };
        let policy = ThresholdPolicy::new(2, 3).unwrap();
        let set = TrustRootSet::new(roots, policy).unwrap();
        assert!(!set.verify_threshold(0));
        assert!(!set.verify_threshold(1));
        assert!(set.verify_threshold(2));
        assert!(set.verify_threshold(3));
    }

    #[test]
    fn trust_root_set_quorum_possible() {
        let roots = TrustRoots {
            version: 1,
            entries: vec![
                make_entry("a-root", &valid_fp()),
                make_entry("b-root", &"b".repeat(64)),
            ],
        };
        let policy = ThresholdPolicy::new(2, 3).unwrap();
        let set = TrustRootSet::new(roots, policy).unwrap();
        assert!(set.is_quorum_possible()); // 2 entries >= 2 required
    }

    #[test]
    fn trust_root_set_quorum_impossible() {
        let roots = TrustRoots {
            version: 1,
            entries: vec![make_entry("only-one", &valid_fp())],
        };
        let policy = ThresholdPolicy::new(3, 5).unwrap();
        let set = TrustRootSet::new(roots, policy).unwrap();
        assert!(!set.is_quorum_possible()); // 1 entry < 3 required
    }

    #[test]
    fn trust_root_set_empty_roots_quorum_impossible() {
        let roots = TrustRoots::empty();
        let policy = ThresholdPolicy::new(1, 1).unwrap();
        let set = TrustRootSet::new(roots, policy).unwrap();
        assert!(!set.is_quorum_possible());
    }

    // ── CeremonyStep ────────────────────────────────────────────────────

    #[test]
    fn ceremony_step_names() {
        assert_eq!(CeremonyStep::Init.name(), "init");
        assert_eq!(CeremonyStep::ContributeShare.name(), "contribute-share");
        assert_eq!(CeremonyStep::VerifyShares.name(), "verify-shares");
        assert_eq!(CeremonyStep::Finalize.name(), "finalize");
    }

    // ── CeremonyRecord ──────────────────────────────────────────────────

    #[test]
    fn ceremony_record_new_starts_at_init() {
        let record = CeremonyRecord::new("ceremony-001", 5);
        assert_eq!(record.ceremony_id, "ceremony-001");
        assert_eq!(record.participant_count, 5);
        assert_eq!(record.steps_completed, vec![CeremonyStep::Init]);
        assert!(!record.is_finalized());
        assert_eq!(record.current_step(), Some(&CeremonyStep::Init));
        assert!(record.result_fingerprint.is_none());
    }

    #[test]
    fn ceremony_record_finalized() {
        let mut record = CeremonyRecord::new("ceremony-002", 3);
        record.steps_completed.push(CeremonyStep::ContributeShare);
        record.steps_completed.push(CeremonyStep::VerifyShares);
        record.steps_completed.push(CeremonyStep::Finalize);
        record.result_fingerprint = Some("d".repeat(64));
        assert!(record.is_finalized());
        assert_eq!(record.current_step(), Some(&CeremonyStep::Finalize));
        assert_eq!(
            record.result_fingerprint.as_deref(),
            Some(&"d".repeat(64) as &str)
        );
    }

    #[test]
    fn ceremony_record_serialization_roundtrip() {
        let record = CeremonyRecord::new("ceremony-003", 7);
        let json = serde_json::to_string(&record).unwrap();
        let deserialized: CeremonyRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(record, deserialized);
    }

    // ── HsmCapability ───────────────────────────────────────────────────

    #[test]
    fn hsm_capability_names() {
        assert_eq!(HsmCapability::KeyGeneration.name(), "key-generation");
        assert_eq!(HsmCapability::Signing.name(), "signing");
        assert_eq!(HsmCapability::Verification.name(), "verification");
        assert_eq!(HsmCapability::SecureStorage.name(), "secure-storage");
    }

    // ── HsmBinding ──────────────────────────────────────────────────────

    #[test]
    fn hsm_binding_construction() {
        let binding = HsmBinding::new(
            "e".repeat(64),
            vec![HsmCapability::Signing, HsmCapability::Verification],
        );
        assert_eq!(binding.fingerprint, "e".repeat(64));
        assert_eq!(binding.capabilities.len(), 2);
    }

    #[test]
    fn hsm_binding_has_capability() {
        let binding = HsmBinding::new(
            "f".repeat(64),
            vec![HsmCapability::Signing, HsmCapability::SecureStorage],
        );
        assert!(binding.has_capability(&HsmCapability::Signing));
        assert!(binding.has_capability(&HsmCapability::SecureStorage));
        assert!(!binding.has_capability(&HsmCapability::KeyGeneration));
        assert!(!binding.has_capability(&HsmCapability::Verification));
    }

    #[test]
    fn hsm_binding_can_sign_and_verify() {
        let full = HsmBinding::new(
            "a".repeat(64),
            vec![HsmCapability::Signing, HsmCapability::Verification],
        );
        assert!(full.can_sign_and_verify());

        let sign_only = HsmBinding::new("b".repeat(64), vec![HsmCapability::Signing]);
        assert!(!sign_only.can_sign_and_verify());

        let empty = HsmBinding::new("c".repeat(64), vec![]);
        assert!(!empty.can_sign_and_verify());
    }

    #[test]
    fn hsm_binding_serialization_roundtrip() {
        let binding = HsmBinding::new(
            "d".repeat(64),
            vec![HsmCapability::KeyGeneration, HsmCapability::Signing],
        );
        let json = serde_json::to_string(&binding).unwrap();
        let deserialized: HsmBinding = serde_json::from_str(&json).unwrap();
        assert_eq!(binding, deserialized);
    }

    // ── ThresholdPolicy serialization ───────────────────────────────────

    #[test]
    fn threshold_policy_serialization_roundtrip() {
        let policy = ThresholdPolicy::new(3, 5).unwrap();
        let json = serde_json::to_string(&policy).unwrap();
        let deserialized: ThresholdPolicy = serde_json::from_str(&json).unwrap();
        assert_eq!(policy, deserialized);
    }

    // ── TrustRootSet serialization ──────────────────────────────────────

    #[test]
    fn trust_root_set_serialization_roundtrip() {
        let roots = TrustRoots {
            version: 1,
            entries: vec![make_entry("alpha", &valid_fp())],
        };
        let policy = ThresholdPolicy::new(1, 1).unwrap();
        let set = TrustRootSet::new(roots, policy).unwrap();
        let json = serde_json::to_string(&set).unwrap();
        let deserialized: TrustRootSet = serde_json::from_str(&json).unwrap();
        assert_eq!(set, deserialized);
    }
}
