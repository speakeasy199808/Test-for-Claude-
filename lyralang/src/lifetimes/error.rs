//! Error types for the LyraLang lifetime annotations checker.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::lexer::SourceSpan;

/// Categories of lifetime-checking error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifetimeErrorKind {
    /// Parsing failed before lifetime checking could proceed.
    ParseError,
    /// A reference would outlive its referent, creating a dangling reference.
    DanglingReference,
    /// A binding's inferred lifetime is shorter than required by its usage.
    LifetimeTooShort,
    /// Conflicting borrows prevent safe region assignment.
    ConflictingBorrows,
}

impl LifetimeErrorKind {
    /// Returns a stable machine-readable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::DanglingReference => "dangling_reference",
            Self::LifetimeTooShort => "lifetime_too_short",
            Self::ConflictingBorrows => "conflicting_borrows",
        }
    }
}

/// A lifetime-checking diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{message}")]
pub struct LifetimeError {
    /// Error category.
    pub kind: LifetimeErrorKind,
    /// Human-readable diagnostic text.
    pub message: String,
    /// Source span associated with the error.
    pub span: SourceSpan,
    /// Whether checking recovered and continued.
    pub recovered: bool,
}

impl LifetimeError {
    /// Creates a new lifetime-checking diagnostic.
    #[must_use]
    pub fn new(
        kind: LifetimeErrorKind,
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
