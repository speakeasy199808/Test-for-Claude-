//! Token kinds for the seed LyraLang lexer.

use serde::{Deserialize, Serialize};

use crate::lexer::span::SourceSpan;

/// Reserved words recognized by the Stage 0 lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Keyword {
    /// `as`
    As,
    /// `break`
    Break,
    /// `continue`
    Continue,
    /// `effect`
    Effect,
    /// `else`
    Else,
    /// `false`
    False,
    /// `fn`
    Fn,
    /// `for`
    For,
    /// `if`
    If,
    /// `impl`
    Impl,
    /// `import`
    Import,
    /// `in`
    In,
    /// `let`
    Let,
    /// `loop`
    Loop,
    /// `match`
    Match,
    /// `module`
    Module,
    /// `proof`
    Proof,
    /// `return`
    Return,
    /// `trait`
    Trait,
    /// `true`
    True,
    /// `type`
    Type,
    /// `use`
    Use,
    /// `where`
    Where,
    /// `while`
    While,
    /// `with`
    With,
}

impl Keyword {
    /// Resolves a source lexeme to a keyword if it is reserved.
    #[must_use]
    pub fn from_lexeme(lexeme: &str) -> Option<Self> {
        match lexeme {
            "as" => Some(Self::As),
            "break" => Some(Self::Break),
            "continue" => Some(Self::Continue),
            "effect" => Some(Self::Effect),
            "else" => Some(Self::Else),
            "false" => Some(Self::False),
            "fn" => Some(Self::Fn),
            "for" => Some(Self::For),
            "if" => Some(Self::If),
            "impl" => Some(Self::Impl),
            "import" => Some(Self::Import),
            "in" => Some(Self::In),
            "let" => Some(Self::Let),
            "loop" => Some(Self::Loop),
            "match" => Some(Self::Match),
            "module" => Some(Self::Module),
            "proof" => Some(Self::Proof),
            "return" => Some(Self::Return),
            "trait" => Some(Self::Trait),
            "true" => Some(Self::True),
            "type" => Some(Self::Type),
            "use" => Some(Self::Use),
            "where" => Some(Self::Where),
            "while" => Some(Self::While),
            "with" => Some(Self::With),
            _ => None,
        }
    }

    /// Returns the canonical textual spelling of the keyword.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::As => "as",
            Self::Break => "break",
            Self::Continue => "continue",
            Self::Effect => "effect",
            Self::Else => "else",
            Self::False => "false",
            Self::Fn => "fn",
            Self::For => "for",
            Self::If => "if",
            Self::Impl => "impl",
            Self::Import => "import",
            Self::In => "in",
            Self::Let => "let",
            Self::Loop => "loop",
            Self::Match => "match",
            Self::Module => "module",
            Self::Proof => "proof",
            Self::Return => "return",
            Self::Trait => "trait",
            Self::True => "true",
            Self::Type => "type",
            Self::Use => "use",
            Self::Where => "where",
            Self::While => "while",
            Self::With => "with",
        }
    }
}

/// Token kinds emitted by the seed lexer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenKind {
    /// A Unicode-aware identifier.
    Identifier,
    /// A reserved word.
    Keyword(Keyword),
    /// A wildcard underscore.
    Underscore,
    /// A decimal integer literal.
    Integer,
    /// A double-quoted string literal.
    String,
    /// Horizontal whitespace trivia.
    Whitespace,
    /// A normalized newline token.
    Newline,
    /// A `//` line comment.
    LineComment,
    /// A `/* ... */` block comment.
    BlockComment,
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `[`
    LBracket,
    /// `]`
    RBracket,
    /// `,`
    Comma,
    /// `:`
    Colon,
    /// `;`
    Semicolon,
    /// `.`
    Dot,
    /// `@`
    At,
    /// `?`
    Question,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `=`
    Assign,
    /// `==`
    EqEq,
    /// `!=`
    NotEq,
    /// `<`
    Lt,
    /// `<=`
    LtEq,
    /// `>`
    Gt,
    /// `>=`
    GtEq,
    /// `->`
    Arrow,
    /// `=>`
    FatArrow,
    /// `::`
    DoubleColon,
    /// `&`
    Ampersand,
    /// `&&`
    AndAnd,
    /// `|`
    Pipe,
    /// `||`
    OrOr,
    /// A lexer recovery token.
    Error,
    /// End-of-file marker.
    Eof,
}

impl TokenKind {
    /// Returns `true` when the token is lexical trivia.
    #[must_use]
    pub const fn is_trivia(&self) -> bool {
        matches!(
            self,
            Self::Whitespace | Self::Newline | Self::LineComment | Self::BlockComment
        )
    }

    /// Returns a stable diagnostic label for the token kind.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Keyword(_) => "keyword",
            Self::Underscore => "underscore",
            Self::Integer => "integer",
            Self::String => "string",
            Self::Whitespace => "whitespace",
            Self::Newline => "newline",
            Self::LineComment => "line_comment",
            Self::BlockComment => "block_comment",
            Self::LParen => "l_paren",
            Self::RParen => "r_paren",
            Self::LBrace => "l_brace",
            Self::RBrace => "r_brace",
            Self::LBracket => "l_bracket",
            Self::RBracket => "r_bracket",
            Self::Comma => "comma",
            Self::Colon => "colon",
            Self::Semicolon => "semicolon",
            Self::Dot => "dot",
            Self::At => "at",
            Self::Question => "question",
            Self::Plus => "plus",
            Self::Minus => "minus",
            Self::Star => "star",
            Self::Slash => "slash",
            Self::Percent => "percent",
            Self::Assign => "assign",
            Self::EqEq => "eq_eq",
            Self::NotEq => "not_eq",
            Self::Lt => "lt",
            Self::LtEq => "lt_eq",
            Self::Gt => "gt",
            Self::GtEq => "gt_eq",
            Self::Arrow => "arrow",
            Self::FatArrow => "fat_arrow",
            Self::DoubleColon => "double_colon",
            Self::Ampersand => "ampersand",
            Self::AndAnd => "and_and",
            Self::Pipe => "pipe",
            Self::OrOr => "or_or",
            Self::Error => "error",
            Self::Eof => "eof",
        }
    }
}

/// A single token emitted by the lexer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Token {
    /// Token classification.
    pub kind: TokenKind,
    /// Exact lexeme taken from the normalized source.
    pub lexeme: String,
    /// Source span in normalized coordinates.
    pub span: SourceSpan,
}

impl Token {
    /// Creates a new token.
    #[must_use]
    pub fn new(kind: TokenKind, lexeme: impl Into<String>, span: SourceSpan) -> Self {
        Self {
            kind,
            lexeme: lexeme.into(),
            span,
        }
    }

    /// Returns `true` when the token is trivia.
    #[must_use]
    pub const fn is_trivia(&self) -> bool {
        self.kind.is_trivia()
    }
}
