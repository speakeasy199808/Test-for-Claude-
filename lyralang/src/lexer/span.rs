//! Source position and span types for LyraLang lexical artifacts.

use serde::{Deserialize, Serialize};

/// A normalized source position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Position {
    /// Zero-based byte offset in the normalized source.
    pub offset: usize,
    /// One-based line number in the normalized source.
    pub line: usize,
    /// One-based column number in the normalized source.
    pub column: usize,
}

impl Position {
    /// Creates a new position.
    #[must_use]
    pub const fn new(offset: usize, line: usize, column: usize) -> Self {
        Self { offset, line, column }
    }
}

/// A half-open source span over normalized source text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SourceSpan {
    /// Inclusive start position.
    pub start: Position,
    /// Exclusive end position.
    pub end: Position,
}

impl SourceSpan {
    /// Creates a new source span.
    #[must_use]
    pub const fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}
