
//! Error types for the seed LyraLang temporal-logic checker.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::lexer::SourceSpan;

/// Categories of temporal-logic error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemporalErrorKind {
    /// Parsing failed before temporal checking could proceed.
    ParseError,
    /// Type checking failed before temporal checking could proceed.
    TypeError,
    /// A temporal operator was invoked in an invalid way.
    InvalidTemporalOperator,
}

impl TemporalErrorKind {
    /// Returns a stable machine-readable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::TypeError => "type_error",
            Self::InvalidTemporalOperator => "invalid_temporal_operator",
        }
    }
}

/// A temporal-checking diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{message}")]
pub struct TemporalError {
    /// Error category.
    pub kind: TemporalErrorKind,
    /// Human-readable diagnostic text.
    pub message: String,
    /// Source span associated with the error.
    pub span: SourceSpan,
    /// Whether checking recovered and continued.
    pub recovered: bool,
}

impl TemporalError {
    /// Creates a new temporal-checking diagnostic.
    #[must_use]
    pub fn new(
        kind: TemporalErrorKind,
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
