//! Error types for the LyraLang FFI specification checker.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::lexer::SourceSpan;

/// Categories of FFI-checking error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FfiErrorKind {
    /// Parsing failed before FFI checking could proceed.
    ParseError,
    /// An FFI call was made without a required `Capability` binding in scope.
    MissingCapability,
    /// A type used in an FFI call cannot be safely marshalled.
    UnsafeTypeMarshalling,
    /// The FFI target language could not be determined.
    UnknownFfiTarget,
}

impl FfiErrorKind {
    /// Returns a stable machine-readable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::MissingCapability => "missing_capability",
            Self::UnsafeTypeMarshalling => "unsafe_type_marshalling",
            Self::UnknownFfiTarget => "unknown_ffi_target",
        }
    }
}

/// An FFI-checking diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{message}")]
pub struct FfiError {
    /// Error category.
    pub kind: FfiErrorKind,
    /// Human-readable diagnostic text.
    pub message: String,
    /// Source span associated with the error.
    pub span: SourceSpan,
    /// Whether checking recovered and continued.
    pub recovered: bool,
}

impl FfiError {
    /// Creates a new FFI-checking diagnostic.
    #[must_use]
    pub fn new(
        kind: FfiErrorKind,
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
