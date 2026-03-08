//! Trait/typeclass registry for the executable Stage 0 LyraLang surface.
//!
//! Full source-level trait declarations are deferred to a later grammar task.
//! This module provides the deterministic internal registry and validation
//! machinery required for coherence-safe ad-hoc polymorphism now.

pub mod error;
mod registry;

pub use error::{TraitError, TraitErrorKind};
pub use registry::{
    seed_registry, validate_seed_registry, DerivedExpansion, TraitCheckOutput,
    TraitDefinition, TraitImplementationStyle, TraitInstance, TraitMethodSignature,
    TraitRegistry, TraitResolution,
};
