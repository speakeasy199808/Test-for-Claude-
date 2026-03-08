//! Error types for the seed LyraLang modal checker.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::lexer::SourceSpan;

/// Categories of modal-checking error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModalErrorKind {
    /// Parsing failed before modal checking could proceed.
    ParseError,
    /// Type checking failed before modal checking could proceed.
    TypeError,
    /// A modal promotion was invoked in an invalid way.
    InvalidModalPromotion,
    /// The current Stage 0 surface does not support the encountered modal flow.
    UnsupportedModalFlow,
}

impl ModalErrorKind {
    /// Returns a stable machine-readable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::TypeError => "type_error",
            Self::InvalidModalPromotion => "invalid_modal_promotion",
            Self::UnsupportedModalFlow => "unsupported_modal_flow",
        }
    }
}

/// A modal-checking diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{message}")]
pub struct ModalError {
    /// Error category.
    pub kind: ModalErrorKind,
    /// Human-readable diagnostic text.
    pub message: String,
    /// Source span associated with the error.
    pub span: SourceSpan,
    /// Whether checking recovered and continued.
    pub recovered: bool,
}

impl ModalError {
    /// Creates a new modal-checking diagnostic.
    #[must_use]
    pub fn new(
        kind: ModalErrorKind,
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
