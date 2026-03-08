//! Diagnostics for the seed LyraLang error-handling analyzer.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::lexer::SourceSpan;

/// Classification for an error-handling diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorAnalysisKind {
    /// Upstream parse failure.
    ParseError,
    /// Upstream type-check failure.
    TypeError,
    /// Panic-style operations are forbidden in the Stage 0 subset.
    PanicForbidden,
}

impl ErrorAnalysisKind {
    /// Returns the canonical machine label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::TypeError => "type_error",
            Self::PanicForbidden => "panic_forbidden",
        }
    }
}

/// A deterministic error-handling diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{message}")]
pub struct ErrorAnalysis {
    /// Diagnostic classification.
    pub kind: ErrorAnalysisKind,
    /// Human-readable message.
    pub message: String,
    /// Source span associated with the failure.
    pub span: SourceSpan,
    /// Whether the analyzer recovered from the failure.
    pub recovered: bool,
}

impl ErrorAnalysis {
    /// Creates a new diagnostic.
    #[must_use]
    pub fn new(
        kind: ErrorAnalysisKind,
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
