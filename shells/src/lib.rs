//! shells — Lyra Interaction Shells and Orchestration Surfaces
//!
//! This crate owns interaction shells and environment-facing orchestration
//! surfaces. Shells are the outermost layer of the Lyra system, providing
//! user-facing and agent-facing interfaces.
//!
//! # Dependency Contract
//! Shells depend on k0 and k1 for all substrate and symbolic services.
//! Platform-specific shell behavior belongs here or in android/web roots.
//!
//! # Module Map (future phases)
//! - `cli`      — command-line interface shell
//! - `agent`    — agent orchestration surface
//! - `repl`     — interactive shell surface
//! - `api`      — programmatic API surface

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
