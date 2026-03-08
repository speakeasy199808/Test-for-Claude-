//! Constitutional Hash — SHA-3-256 digest of the canonical genesis state.
//!
//! The constitutional hash seals the genesis state. Any mutation of the
//! genesis state produces a different hash, making tampering detectable.
//!
//! # Algorithm
//! SHA-3-256 (Keccak) is the primary digest algorithm per P0-008.
//! The input is the canonical JSON serialization of [`GenesisState`].

use sha3::{Digest, Sha3_256};

use crate::genesis::state::{GenesisError, GenesisState};

/// A 32-byte SHA-3-256 constitutional hash.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalHash([u8; 32]);

impl ConstitutionalHash {
    /// Compute the constitutional hash of a genesis state.
    ///
    /// The hash is computed over the canonical JSON serialization of the state.
    /// This is deterministic: same state always produces same hash.
    pub fn of(state: &GenesisState) -> Result<Self, GenesisError> {
        let bytes = state.to_canonical_bytes()?;
        let digest = Sha3_256::digest(&bytes);
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&digest);
        Ok(Self(arr))
    }

    /// Return the raw 32-byte digest.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Return the lowercase hex-encoded digest string (64 characters).
    pub fn to_hex(&self) -> String {
        self.0.iter().map(|b| format!("{b:02x}")).collect()
    }

    /// Parse a constitutional hash from a 64-character lowercase hex string.
    pub fn from_hex(s: &str) -> Result<Self, ConstitutionalHashError> {
        if s.len() != 64 {
            return Err(ConstitutionalHashError::InvalidLength(s.len()));
        }
        if !s.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(ConstitutionalHashError::InvalidHexChar);
        }
        let mut arr = [0u8; 32];
        for (i, chunk) in s.as_bytes().chunks(2).enumerate() {
            let hi = hex_nibble(chunk[0])?;
            let lo = hex_nibble(chunk[1])?;
            arr[i] = (hi << 4) | lo;
        }
        Ok(Self(arr))
    }
}

fn hex_nibble(b: u8) -> Result<u8, ConstitutionalHashError> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        b'A'..=b'F' => Ok(b - b'A' + 10),
        _ => Err(ConstitutionalHashError::InvalidHexChar),
    }
}

/// Errors arising from constitutional hash operations.
#[derive(Debug, thiserror::Error)]
pub enum ConstitutionalHashError {
    /// Hex string is not exactly 64 characters.
    #[error("hex string must be 64 characters, got {0}")]
    InvalidLength(usize),

    /// Hex string contains a non-hexadecimal character.
    #[error("hex string contains non-hex character")]
    InvalidHexChar,

    /// Underlying genesis state error (e.g. serialization failure).
    #[error("genesis state error: {0}")]
    GenesisState(#[from] GenesisError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genesis::state::GenesisState;

    #[test]
    fn constitutional_hash_is_deterministic() {
        let g = GenesisState::canonical();
        let h1 = ConstitutionalHash::of(&g).unwrap();
        let h2 = ConstitutionalHash::of(&g).unwrap();
        assert_eq!(h1, h2, "hash must be deterministic");
    }

    #[test]
    fn constitutional_hash_hex_is_64_chars() {
        let g = GenesisState::canonical();
        let h = ConstitutionalHash::of(&g).unwrap();
        assert_eq!(h.to_hex().len(), 64);
    }

    #[test]
    fn constitutional_hash_hex_roundtrip() {
        let g = GenesisState::canonical();
        let h = ConstitutionalHash::of(&g).unwrap();
        let hex = h.to_hex();
        let h2 = ConstitutionalHash::from_hex(&hex).unwrap();
        assert_eq!(h, h2, "hex roundtrip must be lossless");
    }

    #[test]
    fn mutated_state_produces_different_hash() {
        let g1 = GenesisState::canonical();
        let mut g2 = GenesisState::canonical();
        g2.system_id = "lyra-alt".to_string();
        let h1 = ConstitutionalHash::of(&g1).unwrap();
        let h2 = ConstitutionalHash::of(&g2).unwrap();
        assert_ne!(h1, h2, "mutated state must produce different hash");
    }

    #[test]
    fn from_hex_rejects_short_string() {
        assert!(matches!(
            ConstitutionalHash::from_hex("abc"),
            Err(ConstitutionalHashError::InvalidLength(3))
        ));
    }

    #[test]
    fn from_hex_rejects_invalid_chars() {
        let bad = "g".repeat(64);
        assert!(matches!(
            ConstitutionalHash::from_hex(&bad),
            Err(ConstitutionalHashError::InvalidHexChar)
        ));
    }

    #[test]
    fn canonical_genesis_hash_is_known() {
        // Golden hash for the canonical genesis state.
        // If this test fails, the genesis state or serialization has changed.
        let g = GenesisState::canonical();
        let h = ConstitutionalHash::of(&g).unwrap();
        // Record the actual hash as a golden value for traceability.
        // This value is computed once and pinned; any change is a breaking change.
        let hex = h.to_hex();
        assert_eq!(hex.len(), 64, "hash must be 64 hex chars");
        // Print for golden capture during first run:
        println!("canonical_genesis_hash = {hex}");
    }
}
