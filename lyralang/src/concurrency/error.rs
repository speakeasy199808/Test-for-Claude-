
//! Error types for the seed LyraLang structured-concurrency checker.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::lexer::SourceSpan;

/// Categories of structured-concurrency error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConcurrencyErrorKind {
    /// Parsing failed before concurrency analysis could proceed.
    ParseError,
    /// Type checking failed before concurrency analysis could proceed.
    TypeError,
    /// A spawned expression captured a forbidden linear resource.
    LinearCapture,
    /// A concurrency builtin was invoked in an invalid way.
    InvalidConcurrencySurface,
}

impl ConcurrencyErrorKind {
    /// Returns a stable machine-readable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::TypeError => "type_error",
            Self::LinearCapture => "linear_capture",
            Self::InvalidConcurrencySurface => "invalid_concurrency_surface",
        }
    }
}

/// A structured-concurrency diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{message}")]
pub struct ConcurrencyError {
    /// Error category.
    pub kind: ConcurrencyErrorKind,
    /// Human-readable diagnostic text.
    pub message: String,
    /// Source span associated with the error.
    pub span: SourceSpan,
    /// Whether checking recovered and continued.
    pub recovered: bool,
}

impl ConcurrencyError {
    /// Creates a new structured-concurrency diagnostic.
    #[must_use]
    pub fn new(
        kind: ConcurrencyErrorKind,
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
