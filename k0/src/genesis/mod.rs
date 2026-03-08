//! genesis — Initial Repository State and Constitutional Foundation
//!
//! This module owns the genesis state of the Lyra system: the initial
//! canonical state from which all subsequent states are derived by
//! deterministic transition. It also owns trust roots and the
//! constitutional hash that seals the genesis configuration.
//!
//! # Constitutional Guarantee
//! The genesis state is immutable once sealed. All downstream state
//! transitions are derived from it via deterministic application of
//! the constitutional gate pipeline defined in P0-003.
//!
//! # Submodules
//! - [`state`]       — Genesis state definition and canonical serialization
//! - [`hash`]        — Constitutional hash: SHA-3-256 digest of genesis state
//! - [`trust_roots`] — Trust root declarations and verification stubs

pub mod hash;
pub mod state;
pub mod trust_roots;

pub use hash::ConstitutionalHash;
pub use state::{GenesisState, GENESIS_VERSION};
pub use trust_roots::{
    CeremonyRecord, CeremonyStep, HsmBinding, HsmCapability, ThresholdPolicy, TrustRootEntry,
    TrustRootError, TrustRootKind, TrustRootSet, TrustRoots,
};
