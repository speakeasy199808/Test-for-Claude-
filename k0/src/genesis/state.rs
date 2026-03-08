//! Genesis State — canonical initial state of the Lyra system.
//!
//! The genesis state is the root of all state derivation. It is
//! deterministically serialized and sealed by the constitutional hash.
//! No field may be non-deterministic (no floats, no ambient randomness,
//! no platform-dependent ordering).

use serde::{Deserialize, Serialize};

/// Current genesis state schema version.
/// Increment this when the genesis state schema changes in a breaking way.
pub const GENESIS_VERSION: u32 = 1;

/// The canonical initial state of the Lyra system.
///
/// All fields are deterministic and canonically ordered.
/// Floating-point types are forbidden per the constitutional math spec (P0-003).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenesisState {
    /// Schema version for this genesis state record.
    pub version: u32,

    /// Human-readable system identifier. ASCII only, no whitespace.
    pub system_id: String,

    /// Monotonic genesis sequence number. Always 0 for the genesis state.
    pub sequence: u64,

    /// Unix timestamp (seconds) at which this genesis state was declared.
    /// Must be a deterministic constant for reproducible builds.
    pub declared_at_unix_secs: u64,

    /// Constitutional invariant set version this genesis state was sealed under.
    /// References the P0-003 constitutional math spec version.
    pub constitutional_version: u32,

    /// Trust root fingerprints included at genesis.
    /// Each entry is a lowercase hex-encoded SHA-3-256 digest.
    pub trust_root_fingerprints: Vec<String>,
}

impl GenesisState {
    /// Construct the canonical Lyra genesis state.
    ///
    /// This is the single authoritative genesis configuration.
    /// All fields are compile-time constants or deterministic derivations.
    pub fn canonical() -> Self {
        Self {
            version: GENESIS_VERSION,
            system_id: "lyra".to_string(),
            sequence: 0,
            // 2024-01-01T00:00:00Z — deterministic genesis timestamp
            declared_at_unix_secs: 1_704_067_200,
            constitutional_version: 1,
            trust_root_fingerprints: vec![],
        }
    }

    /// Serialize this genesis state to canonical JSON bytes.
    ///
    /// The output is deterministic: fields are serialized in declaration order,
    /// no floating-point values, no ambient nondeterminism.
    pub fn to_canonical_bytes(&self) -> Result<Vec<u8>, GenesisError> {
        serde_json::to_vec(self).map_err(GenesisError::Serialization)
    }

    /// Validate structural invariants of this genesis state.
    ///
    /// Returns `Ok(())` if all invariants hold, or a descriptive error.
    pub fn validate(&self) -> Result<(), GenesisError> {
        if self.version == 0 {
            return Err(GenesisError::InvalidVersion(self.version));
        }
        if self.system_id.is_empty() {
            return Err(GenesisError::EmptySystemId);
        }
        if self
            .system_id
            .chars()
            .any(|c| !c.is_ascii() || c.is_whitespace())
        {
            return Err(GenesisError::InvalidSystemId(self.system_id.clone()));
        }
        if self.sequence != 0 {
            return Err(GenesisError::NonZeroGenesisSequence(self.sequence));
        }
        if self.constitutional_version == 0 {
            return Err(GenesisError::InvalidConstitutionalVersion(
                self.constitutional_version,
            ));
        }
        for fp in &self.trust_root_fingerprints {
            if fp.len() != 64 || !fp.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(GenesisError::InvalidTrustRootFingerprint(fp.clone()));
            }
        }
        Ok(())
    }
}

/// Errors arising from genesis state operations.
#[derive(Debug, thiserror::Error)]
pub enum GenesisError {
    /// Version field is zero; must be >= 1.
    #[error("invalid genesis version: {0} (must be >= 1)")]
    InvalidVersion(u32),

    /// `system_id` field is empty.
    #[error("system_id must not be empty")]
    EmptySystemId,

    /// `system_id` contains non-ASCII or whitespace characters.
    #[error("system_id contains non-ASCII or whitespace characters: {0:?}")]
    InvalidSystemId(String),

    /// `sequence` field is non-zero; genesis sequence must be 0.
    #[error("genesis sequence must be 0, got {0}")]
    NonZeroGenesisSequence(u64),

    /// `constitutional_version` is zero; must be >= 1.
    #[error("constitutional_version must be >= 1, got {0}")]
    InvalidConstitutionalVersion(u32),

    /// A trust root fingerprint is not a 64-char lowercase hex string.
    #[error("trust root fingerprint is not a 64-char lowercase hex string: {0:?}")]
    InvalidTrustRootFingerprint(String),

    /// JSON serialization failed.
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_genesis_is_valid() {
        let g = GenesisState::canonical();
        assert!(g.validate().is_ok(), "canonical genesis must be valid");
    }

    #[test]
    fn canonical_genesis_version_is_one() {
        assert_eq!(GenesisState::canonical().version, 1);
    }

    #[test]
    fn canonical_genesis_sequence_is_zero() {
        assert_eq!(GenesisState::canonical().sequence, 0);
    }

    #[test]
    fn canonical_genesis_system_id_is_lyra() {
        assert_eq!(GenesisState::canonical().system_id, "lyra");
    }

    #[test]
    fn canonical_genesis_serializes_deterministically() {
        let g = GenesisState::canonical();
        let b1 = g.to_canonical_bytes().unwrap();
        let b2 = g.to_canonical_bytes().unwrap();
        assert_eq!(b1, b2, "serialization must be deterministic");
    }

    #[test]
    fn nonzero_sequence_is_rejected() {
        let mut g = GenesisState::canonical();
        g.sequence = 1;
        assert!(matches!(
            g.validate(),
            Err(GenesisError::NonZeroGenesisSequence(1))
        ));
    }

    #[test]
    fn empty_system_id_is_rejected() {
        let mut g = GenesisState::canonical();
        g.system_id = String::new();
        assert!(matches!(g.validate(), Err(GenesisError::EmptySystemId)));
    }

    #[test]
    fn whitespace_system_id_is_rejected() {
        let mut g = GenesisState::canonical();
        g.system_id = "lyra os".to_string();
        assert!(matches!(
            g.validate(),
            Err(GenesisError::InvalidSystemId(_))
        ));
    }

    #[test]
    fn invalid_trust_root_fingerprint_is_rejected() {
        let mut g = GenesisState::canonical();
        g.trust_root_fingerprints = vec!["not-a-hash".to_string()];
        assert!(matches!(
            g.validate(),
            Err(GenesisError::InvalidTrustRootFingerprint(_))
        ));
    }
}
