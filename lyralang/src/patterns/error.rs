//! Error types for the LyraLang pattern matching exhaustiveness checker.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::lexer::SourceSpan;

/// Categories of pattern-checking error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternErrorKind {
    /// Parsing failed before pattern checking could proceed.
    ParseError,
    /// A match expression does not cover all possible cases.
    NonExhaustiveMatch,
    /// A pattern arm can never be reached given earlier arms.
    UnreachablePattern,
    /// The same literal pattern appears more than once in a match.
    DuplicatePattern,
}

impl PatternErrorKind {
    /// Returns a stable machine-readable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::NonExhaustiveMatch => "non_exhaustive_match",
            Self::UnreachablePattern => "unreachable_pattern",
            Self::DuplicatePattern => "duplicate_pattern",
        }
    }
}

/// A pattern-checking diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{message}")]
pub struct PatternError {
    /// Error category.
    pub kind: PatternErrorKind,
    /// Human-readable diagnostic text.
    pub message: String,
    /// Source span associated with the error.
    pub span: SourceSpan,
    /// Whether checking recovered and continued.
    pub recovered: bool,
}

impl PatternError {
    /// Creates a new pattern-checking diagnostic.
    #[must_use]
    pub fn new(
        kind: PatternErrorKind,
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
