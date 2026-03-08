//! Diagnostics for the seed LyraLang bytecode emitter.

use serde::{Deserialize, Serialize};

use crate::lexer::SourceSpan;

/// Classification for a bytecode emission diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BytecodeErrorKind {
    /// Upstream code-generation failure.
    CodegenError,
    /// Malformed canonical IR text.
    InvalidIr,
    /// Failure during canonical encoding.
    EncodingError,
}

impl BytecodeErrorKind {
    /// Returns the canonical machine label for the diagnostic kind.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::CodegenError => "codegen_error",
            Self::InvalidIr => "invalid_ir",
            Self::EncodingError => "encoding_error",
        }
    }
}

/// A deterministic bytecode emission diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BytecodeError {
    /// Diagnostic classification.
    pub kind: BytecodeErrorKind,
    /// Human-readable message.
    pub message: String,
    /// Source span associated with the failure.
    pub span: SourceSpan,
    /// Whether the emitter recovered from the failure.
    pub recovered: bool,
}

impl BytecodeError {
    /// Creates a new bytecode emission diagnostic.
    #[must_use]
    pub fn new(
        kind: BytecodeErrorKind,
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
