//! Error types for the seed LyraLang trait system.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Categories of trait-system error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraitErrorKind {
    /// An instance violated coherence by overlapping an existing instance.
    CoherenceViolation,
    /// An instance violated the orphan rule.
    OrphanInstance,
    /// The registry referenced a trait that does not exist.
    UnknownTrait,
    /// A method could not be resolved for the requested argument types.
    ResolutionFailure,
}

impl TraitErrorKind {
    /// Returns a stable machine-readable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::CoherenceViolation => "coherence_violation",
            Self::OrphanInstance => "orphan_instance",
            Self::UnknownTrait => "unknown_trait",
            Self::ResolutionFailure => "resolution_failure",
        }
    }
}

/// A deterministic trait-system diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{message}")]
pub struct TraitError {
    /// Error category.
    pub kind: TraitErrorKind,
    /// Human-readable diagnostic text.
    pub message: String,
}

impl TraitError {
    /// Creates a new trait diagnostic.
    #[must_use]
    pub fn new(kind: TraitErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}
