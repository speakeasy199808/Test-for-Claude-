//! slices — Lyra Vertical Compositions and End-to-End Feature Slices
//!
//! This crate owns vertical compositions and end-to-end feature slices
//! that span subsystem roots. A slice wires together k0, k1, lyralang,
//! and shell components into a coherent end-to-end capability.
//!
//! # Dependency Contract
//! Slices depend on k0 and k1. They are integration-level compositions
//! and must not introduce new foundational logic — they wire existing
//! subsystem outputs together.
//!
//! # Module Map (future phases)
//! - `foundation` — Phase 0 foundation integration slice (P0-023)
//! - `language`   — Phase 1 language integration slice (P1-032)
//! - `bootstrap`  — Phase 2 bootstrap integration slice (P2-020)

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
