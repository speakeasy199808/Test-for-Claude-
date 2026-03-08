//! Diagnostics for the seed LyraLang formal semantics evaluator.

use serde::{Deserialize, Serialize};

use crate::lexer::SourceSpan;

/// Classification for a semantics diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SemanticsErrorKind {
    /// Upstream parse failure.
    ParseError,
    /// Upstream type-check failure.
    TypeError,
    /// Unsupported construct during semantic execution.
    UnsupportedConstruct,
    /// Unknown identifier during evaluation.
    UnknownIdentifier,
    /// Invalid builtin invocation or argument shape.
    InvalidBuiltin,
}

impl SemanticsErrorKind {
    /// Returns the canonical machine label for the diagnostic kind.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::TypeError => "type_error",
            Self::UnsupportedConstruct => "unsupported_construct",
            Self::UnknownIdentifier => "unknown_identifier",
            Self::InvalidBuiltin => "invalid_builtin",
        }
    }
}

/// A deterministic semantics diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticsError {
    /// Diagnostic classification.
    pub kind: SemanticsErrorKind,
    /// Human-readable message.
    pub message: String,
    /// Source span associated with the failure.
    pub span: SourceSpan,
    /// Whether the evaluator recovered from the failure.
    pub recovered: bool,
}

impl SemanticsError {
    /// Creates a new semantics diagnostic.
    #[must_use]
    pub fn new(
        kind: SemanticsErrorKind,
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
