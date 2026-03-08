//! Error types for the seed LyraLang parser.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::lexer::span::SourceSpan;
use crate::parser::ast::Program;

/// Categories of parser error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParseErrorKind {
    /// The parser expected a specific token or token class.
    ExpectedToken,
    /// The parser encountered an unexpected token.
    UnexpectedToken,
    /// The token stream ended before the construct completed.
    UnexpectedEof,
    /// Lexical analysis failed before parsing could proceed.
    LexicalError,
}

impl ParseErrorKind {
    /// Returns a stable machine-readable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ExpectedToken => "expected_token",
            Self::UnexpectedToken => "unexpected_token",
            Self::UnexpectedEof => "unexpected_eof",
            Self::LexicalError => "lexical_error",
        }
    }
}

/// A parsing diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{message}")]
pub struct ParseError {
    /// Error category.
    pub kind: ParseErrorKind,
    /// Human-readable diagnostic text.
    pub message: String,
    /// Source span associated with the error.
    pub span: SourceSpan,
    /// Whether the parser recovered and continued.
    pub recovered: bool,
}

impl ParseError {
    /// Creates a new parser diagnostic.
    #[must_use]
    pub fn new(
        kind: ParseErrorKind,
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

/// Result bundle returned by syntactic analysis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParseOutput {
    /// Normalized source passed through lexical analysis.
    pub normalized_source: String,
    /// Parsed program, when construction succeeded.
    pub program: Option<Program>,
    /// Diagnostics captured during parsing.
    pub errors: Vec<ParseError>,
}
