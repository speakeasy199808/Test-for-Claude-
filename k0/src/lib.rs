//! k0 — Lyra Trusted Deterministic Foundation
//!
//! This crate is the constitutional substrate of the Lyra system.
//! It owns: canonicalization, verification, foundational runtime law,
//! security-critical substrate, and irreducible deterministic machinery.
//!
//! # Ownership Contract
//! All foundational determinism, trust, verification, and core substrate
//! behavior belongs in this crate. No ambient nondeterministic sources
//! are permitted in any module within k0.
//!
//! # Module Map
//! - `genesis`     — initial repository state, trust roots, constitutional hash (P0-001)
//! - `self_verify` — runtime code integrity verification loop (P0-005)
//! - `codec`       — canonical LyraCodec encoder/decoder (P0-007)
//! - `digest`      — SHA-3-256 primary, BLAKE3 secondary hash routing (P0-008)
//! - `time`        — monotonic virtual time, no wall clock (P0-009)
//! - `entropy`     — seeded deterministic randomness, hash-chained pool (P0-010)
//! - `drift`       — runtime nondeterminism detection (P0-012)
//! - `incident`    — incident taxonomy and recovery protocols (P0-013, P0-014)
//! - `errors`      — globally unique error code system (P0-018)
//! - `logging`     — structured deterministic logging (P0-019)

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]

pub mod codec;
pub mod digest;
pub mod drift;
pub mod entropy;
pub mod errors;
pub mod genesis;
pub mod incident;
pub mod logging;
pub mod recovery;
pub mod self_verify;
pub mod time;
pub mod verifier;
