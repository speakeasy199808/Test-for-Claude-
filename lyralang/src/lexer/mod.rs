//! Seed lexer and lexical structures for LyraLang.
//!
//! This module implements the first executable Phase 1 compiler surface.
//! It follows the lexical specification in `docs/lyralang/GRAMMAR.md` and
//! provides deterministic tokenization over normalized source text.

pub mod error;
pub mod lexer;
pub mod span;
pub mod token;

pub use error::{LexError, LexErrorKind};
pub use lexer::{lex, LexOutput, Lexer};
pub use span::{Position, SourceSpan};
pub use token::{Keyword, Token, TokenKind};
