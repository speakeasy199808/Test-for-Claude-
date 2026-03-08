//! Error types for the seed LyraLang lexer.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::lexer::span::SourceSpan;

/// Categories of lexical error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LexErrorKind {
    /// An unexpected or unsupported character.
    InvalidCharacter,
    /// A string literal was not closed.
    UnterminatedString,
    /// A block comment was not closed.
    UnterminatedBlockComment,
}

impl LexErrorKind {
    /// Returns a stable machine-readable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::InvalidCharacter => "invalid_character",
            Self::UnterminatedString => "unterminated_string",
            Self::UnterminatedBlockComment => "unterminated_block_comment",
        }
    }
}

/// A lexical diagnostic emitted during tokenization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{message}")]
pub struct LexError {
    /// Error category.
    pub kind: LexErrorKind,
    /// Human-readable diagnostic text.
    pub message: String,
    /// Error span in normalized coordinates.
    pub span: SourceSpan,
    /// Whether the lexer recovered and continued scanning.
    pub recovered: bool,
}

impl LexError {
    /// Creates a new lexical diagnostic.
    #[must_use]
    pub fn new(
        kind: LexErrorKind,
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
