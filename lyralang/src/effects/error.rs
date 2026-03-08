//! Error types for the seed LyraLang effect checker.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::lexer::SourceSpan;

/// Categories of effect-checking error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectErrorKind {
    /// Parsing failed before effect checking could proceed.
    ParseError,
    /// A referenced identifier was not bound.
    UnknownIdentifier,
    /// A non-callable expression was invoked.
    NotCallable,
    /// A callable was invoked with the wrong arity.
    ArityMismatch,
    /// Required effects exceed the allowed policy ceiling.
    EffectViolation,
}

impl EffectErrorKind {
    /// Returns a stable machine-readable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::UnknownIdentifier => "unknown_identifier",
            Self::NotCallable => "not_callable",
            Self::ArityMismatch => "arity_mismatch",
            Self::EffectViolation => "effect_violation",
        }
    }
}

/// An effect-checking diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{message}")]
pub struct EffectError {
    /// Error category.
    pub kind: EffectErrorKind,
    /// Human-readable diagnostic text.
    pub message: String,
    /// Source span associated with the error.
    pub span: SourceSpan,
    /// Whether inference recovered and continued.
    pub recovered: bool,
}

impl EffectError {
    /// Creates a new effect-checking diagnostic.
    #[must_use]
    pub fn new(
        kind: EffectErrorKind,
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
