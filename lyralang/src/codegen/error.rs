//! Diagnostics for the seed LyraLang code generator.

use serde::{Deserialize, Serialize};

use crate::lexer::SourceSpan;

/// Classification for a code-generation diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodegenErrorKind {
    /// Upstream parse failure.
    ParseError,
    /// Upstream type-check failure.
    TypeError,
    /// Unsupported Stage 0 construct during lowering.
    UnsupportedConstruct,
    /// Unsupported call shape during lowering.
    UnsupportedCallTarget,
}

impl CodegenErrorKind {
    /// Returns the canonical machine label for the diagnostic kind.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::TypeError => "type_error",
            Self::UnsupportedConstruct => "unsupported_construct",
            Self::UnsupportedCallTarget => "unsupported_call_target",
        }
    }
}

/// A deterministic code-generation diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodegenError {
    /// Diagnostic classification.
    pub kind: CodegenErrorKind,
    /// Human-readable message.
    pub message: String,
    /// Source span associated with the failure.
    pub span: SourceSpan,
    /// Whether the compiler recovered from the failure.
    pub recovered: bool,
}

impl CodegenError {
    /// Creates a new code-generation diagnostic.
    #[must_use]
    pub fn new(
        kind: CodegenErrorKind,
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
