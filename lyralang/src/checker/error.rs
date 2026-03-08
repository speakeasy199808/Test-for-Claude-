//! Error types for the seed LyraLang type checker.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::lexer::span::SourceSpan;

/// Categories of type-checking error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeErrorKind {
    /// Parsing failed before type checking could proceed.
    ParseError,
    /// A referenced identifier was not bound.
    UnknownIdentifier,
    /// Two types could not be unified.
    TypeMismatch,
    /// A callable was invoked with the wrong arity.
    ArityMismatch,
    /// A cyclic type would have been created.
    OccursCheckFailed,
    /// The current kernel does not yet type the construct.
    UnsupportedConstruct,
}

impl TypeErrorKind {
    /// Returns a stable machine-readable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::UnknownIdentifier => "unknown_identifier",
            Self::TypeMismatch => "type_mismatch",
            Self::ArityMismatch => "arity_mismatch",
            Self::OccursCheckFailed => "occurs_check_failed",
            Self::UnsupportedConstruct => "unsupported_construct",
        }
    }
}

/// A type-checking diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{message}")]
pub struct TypeError {
    /// Error category.
    pub kind: TypeErrorKind,
    /// Human-readable diagnostic text.
    pub message: String,
    /// Source span associated with the error.
    pub span: SourceSpan,
    /// Whether inference recovered and continued.
    pub recovered: bool,
}

impl TypeError {
    /// Creates a new type-checking diagnostic.
    #[must_use]
    pub fn new(
        kind: TypeErrorKind,
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
