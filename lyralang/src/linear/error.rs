//! Error types for the seed LyraLang linear-resource checker.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::lexer::SourceSpan;

/// Categories of linear-checking error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinearErrorKind {
    /// Parsing failed before linear checking could proceed.
    ParseError,
    /// Type checking failed before ownership analysis could proceed.
    TypeError,
    /// A linear binding was used more than once.
    DuplicateUse,
    /// A linear resource escaped or was dropped without discharge.
    LeakedResource,
    /// A callable consumed or produced the wrong linear resource shape.
    InvalidLinearCall,
    /// Branches disagree about ownership state or returned linear resources.
    BranchMismatch,
    /// The current Stage 0 surface does not support the encountered flow.
    UnsupportedLinearFlow,
}

impl LinearErrorKind {
    /// Returns a stable machine-readable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::TypeError => "type_error",
            Self::DuplicateUse => "duplicate_use",
            Self::LeakedResource => "leaked_resource",
            Self::InvalidLinearCall => "invalid_linear_call",
            Self::BranchMismatch => "branch_mismatch",
            Self::UnsupportedLinearFlow => "unsupported_linear_flow",
        }
    }
}

/// A linear-checking diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{message}")]
pub struct LinearError {
    /// Error category.
    pub kind: LinearErrorKind,
    /// Human-readable diagnostic text.
    pub message: String,
    /// Source span associated with the error.
    pub span: SourceSpan,
    /// Whether checking recovered and continued.
    pub recovered: bool,
}

impl LinearError {
    /// Creates a new linear-checking diagnostic.
    #[must_use]
    pub fn new(
        kind: LinearErrorKind,
        message: impl Into<String>,
        span: SourceSpan,
        recovered: bool,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            span,
            recovered,
        }
    }
}
