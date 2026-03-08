//! k1 — Lyra Higher Symbolic Systems
//!
//! This crate owns higher symbolic domain services: memory, reasoning,
//! planning, retrieval, coordination, and domain engines.
//!
//! # Dependency Contract
//! k1 depends on k0 for all foundational deterministic substrate.
//! k1 MUST NOT introduce ambient nondeterminism; all randomness and
//! time must be sourced from k0 primitives.
//!
//! # Module Map (future phases)
//! - `memory`       — episodic and semantic memory engines
//! - `reasoning`    — symbolic reasoning and inference
//! - `planning`     — goal decomposition and plan execution
//! - `retrieval`    — knowledge retrieval and indexing
//! - `coordination` — multi-agent coordination protocols
//! - `domain`       — domain-specific engines and adapters

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
