//! Deterministic seed lexer for LyraLang.

use regex::Regex;
use serde::{Deserialize, Serialize};
use unicode_ident::{is_xid_continue, is_xid_start};

use crate::lexer::error::{LexError, LexErrorKind};
use crate::lexer::span::{Position, SourceSpan};
use crate::lexer::token::{Keyword, Token, TokenKind};

/// Result bundle returned by lexical analysis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LexOutput {
    /// Normalized source used by the lexer.
    pub normalized_source: String,
    /// Tokens in deterministic source order.
    pub tokens: Vec<Token>,
    /// Diagnostics captured during recovery.
    pub errors: Vec<LexError>,
}

impl LexOutput {
    /// Returns a borrowed view of tokens excluding trivia and EOF.
    #[must_use]
    pub fn significant_tokens(&self) -> Vec<&Token> {
        self.tokens
            .iter()
            .filter(|token| !token.is_trivia() && token.kind != TokenKind::Eof)
            .collect()
    }
}

/// Deterministic lexer over normalized source text.
pub struct Lexer {
    source: String,
    cursor: usize,
    line: usize,
    column: usize,
    integer_regex: Regex,
    errors: Vec<LexError>,
}

impl Lexer {
    /// Creates a new lexer after normalizing source line endings.
    #[must_use]
    pub fn new(source: &str) -> Self {
        Self {
            source: normalize_source(source),
            cursor: 0,
            line: 1,
            column: 1,
            integer_regex: Regex::new(r"^[0-9](?:_?[0-9])*").expect("valid integer regex"),
            errors: Vec::new(),
        }
    }

    /// Tokenizes the entire input and returns tokens plus diagnostics.
    #[must_use]
    pub fn lex(mut self) -> LexOutput {
        let mut tokens = Vec::new();

        while !self.is_eof() {
            tokens.push(self.next_token());
        }

        let eof_position = self.position();
        tokens.push(Token::new(
            TokenKind::Eof,
            String::new(),
            SourceSpan::new(eof_position, eof_position),
        ));

        LexOutput {
            normalized_source: self.source,
            tokens,
            errors: self.errors,
        }
    }

    fn next_token(&mut self) -> Token {
        let start = self.position();

        if self.starts_with("//") {
            return self.scan_line_comment(start);
        }
        if self.starts_with("/*") {
            return self.scan_block_comment(start);
        }

        match self.peek_char() {
            Some('\n') => {
                self.advance_char();
                Token::new(TokenKind::Newline, "\n", SourceSpan::new(start, self.position()))
            }
            Some(ch) if is_horizontal_whitespace(ch) => self.scan_horizontal_whitespace(start),
            Some('"') => self.scan_string(start),
            Some(ch) if ch.is_ascii_digit() => self.scan_integer(start),
            Some('_') => self.scan_underscore_or_identifier(start),
            Some(ch) if is_xid_start(ch) => self.scan_identifier_or_keyword(start),
            Some(_) => {
                if let Some(token) = self.scan_operator_or_punctuation(start) {
                    token
                } else {
                    self.scan_invalid_character(start)
                }
            }
            None => Token::new(TokenKind::Eof, String::new(), SourceSpan::new(start, start)),
        }
    }

    fn scan_horizontal_whitespace(&mut self, start: Position) -> Token {
        let lexeme = self.take_while(is_horizontal_whitespace);
        Token::new(TokenKind::Whitespace, lexeme, SourceSpan::new(start, self.position()))
    }

    fn scan_integer(&mut self, start: Position) -> Token {
        let lexeme = self
            .integer_regex
            .find(self.remaining())
            .map(|m| m.as_str().to_owned())
            .expect("integer regex must match at digit start");
        self.consume_len(lexeme.len());
        Token::new(TokenKind::Integer, lexeme, SourceSpan::new(start, self.position()))
    }

    fn scan_identifier_or_keyword(&mut self, start: Position) -> Token {
        let lexeme = self.take_while(is_identifier_continue);
        let kind = Keyword::from_lexeme(&lexeme)
            .map(TokenKind::Keyword)
            .unwrap_or(TokenKind::Identifier);
        Token::new(kind, lexeme, SourceSpan::new(start, self.position()))
    }

    fn scan_underscore_or_identifier(&mut self, start: Position) -> Token {
        self.advance_char();
        if matches!(self.peek_char(), Some(next) if is_identifier_continue(next)) {
            let mut lexeme = String::from("_");
            lexeme.push_str(&self.take_while(is_identifier_continue));
            Token::new(TokenKind::Identifier, lexeme, SourceSpan::new(start, self.position()))
        } else {
            Token::new(TokenKind::Underscore, "_", SourceSpan::new(start, self.position()))
        }
    }

    fn scan_line_comment(&mut self, start: Position) -> Token {
        self.consume_exact("//");
        let mut lexeme = String::from("//");
        lexeme.push_str(&self.take_while(|ch| ch != '\n'));
        Token::new(TokenKind::LineComment, lexeme, SourceSpan::new(start, self.position()))
    }

    fn scan_block_comment(&mut self, start: Position) -> Token {
        let mut lexeme = String::new();
        let mut depth = 0usize;

        while !self.is_eof() {
            if self.starts_with("/*") {
                depth += 1;
                lexeme.push_str("/*");
                self.consume_exact("/*");
                continue;
            }

            if self.starts_with("*/") {
                depth = depth.saturating_sub(1);
                lexeme.push_str("*/");
                self.consume_exact("*/");
                if depth == 0 {
                    return Token::new(
                        TokenKind::BlockComment,
                        lexeme,
                        SourceSpan::new(start, self.position()),
                    );
                }
                continue;
            }

            if let Some(ch) = self.advance_char() {
                lexeme.push(ch);
            }
        }

        let span = SourceSpan::new(start, self.position());
        self.errors.push(LexError::new(
            LexErrorKind::UnterminatedBlockComment,
            "unterminated block comment",
            span,
            true,
        ));
        Token::new(TokenKind::Error, lexeme, span)
    }

    fn scan_string(&mut self, start: Position) -> Token {
        let mut lexeme = String::new();
        lexeme.push('"');
        self.advance_char();

        while let Some(ch) = self.peek_char() {
            match ch {
                '"' => {
                    self.advance_char();
                    lexeme.push('"');
                    return Token::new(TokenKind::String, lexeme, SourceSpan::new(start, self.position()));
                }
                '\\' => {
                    lexeme.push(ch);
                    self.advance_char();
                    if let Some(escaped) = self.advance_char() {
                        lexeme.push(escaped);
                    } else {
                        break;
                    }
                }
                '\n' => break,
                _ => {
                    lexeme.push(ch);
                    self.advance_char();
                }
            }
        }

        let span = SourceSpan::new(start, self.position());
        self.errors.push(LexError::new(
            LexErrorKind::UnterminatedString,
            "unterminated string literal",
            span,
            true,
        ));
        Token::new(TokenKind::Error, lexeme, span)
    }

    fn scan_operator_or_punctuation(&mut self, start: Position) -> Option<Token> {
        const DOUBLE_TOKENS: [(&str, TokenKind); 9] = [
            ("==", TokenKind::EqEq),
            ("!=", TokenKind::NotEq),
            ("<=", TokenKind::LtEq),
            (">=", TokenKind::GtEq),
            ("->", TokenKind::Arrow),
            ("=>", TokenKind::FatArrow),
            ("::", TokenKind::DoubleColon),
            ("&&", TokenKind::AndAnd),
            ("||", TokenKind::OrOr),
        ];

        for (lexeme, kind) in DOUBLE_TOKENS {
            if self.starts_with(lexeme) {
                self.consume_exact(lexeme);
                return Some(Token::new(kind, lexeme, SourceSpan::new(start, self.position())));
            }
        }

        let kind = match self.peek_char()? {
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,
            '.' => TokenKind::Dot,
            '@' => TokenKind::At,
            '?' => TokenKind::Question,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            '=' => TokenKind::Assign,
            '<' => TokenKind::Lt,
            '>' => TokenKind::Gt,
            '&' => TokenKind::Ampersand,
            '|' => TokenKind::Pipe,
            _ => return None,
        };

        let ch = self.advance_char()?;
        Some(Token::new(kind, ch.to_string(), SourceSpan::new(start, self.position())))
    }

    fn scan_invalid_character(&mut self, start: Position) -> Token {
        let ch = self.advance_char().expect("invalid character exists");
        let span = SourceSpan::new(start, self.position());
        self.errors.push(LexError::new(
            LexErrorKind::InvalidCharacter,
            format!("invalid character `{ch}`"),
            span,
            true,
        ));
        Token::new(TokenKind::Error, ch.to_string(), span)
    }

    fn remaining(&self) -> &str {
        &self.source[self.cursor..]
    }

    fn starts_with(&self, expected: &str) -> bool {
        self.remaining().starts_with(expected)
    }

    fn peek_char(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    fn position(&self) -> Position {
        Position::new(self.cursor, self.line, self.column)
    }

    fn is_eof(&self) -> bool {
        self.cursor >= self.source.len()
    }

    fn consume_exact(&mut self, expected: &str) {
        debug_assert!(self.starts_with(expected));
        for _ in expected.chars() {
            let _ = self.advance_char();
        }
    }

    fn consume_len(&mut self, len: usize) {
        let target = self.cursor + len;
        while self.cursor < target {
            let _ = self.advance_char();
        }
    }

    fn take_while<F>(&mut self, mut predicate: F) -> String
    where
        F: FnMut(char) -> bool,
    {
        let start = self.cursor;
        while let Some(ch) = self.peek_char() {
            if predicate(ch) {
                let _ = self.advance_char();
            } else {
                break;
            }
        }
        self.source[start..self.cursor].to_owned()
    }

    fn advance_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.cursor += ch.len_utf8();
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }
}

/// Tokenizes a source string using the seed lexer.
#[must_use]
pub fn lex(source: &str) -> LexOutput {
    Lexer::new(source).lex()
}

fn normalize_source(source: &str) -> String {
    source.replace("\r\n", "\n").replace('\r', "\n")
}

fn is_horizontal_whitespace(ch: char) -> bool {
    ch != '\n' && ch.is_whitespace()
}

fn is_identifier_continue(ch: char) -> bool {
    ch == '_' || is_xid_continue(ch) || ch.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::Keyword;

    #[test]
    fn normalizes_line_endings_before_tokenization() {
        let output = lex("let x = 1\r\nlet y = 2\rlet z = 3\n");
        assert_eq!(output.normalized_source.matches('\n').count(), 3);
        assert!(!output.normalized_source.contains('\r'));
    }

    #[test]
    fn accepts_unicode_identifiers() {
        let output = lex("let café = Δelta\n");
        let significant = output.significant_tokens();
        assert_eq!(significant[0].kind, TokenKind::Keyword(Keyword::Let));
        assert_eq!(significant[1].kind, TokenKind::Identifier);
        assert_eq!(significant[1].lexeme, "café");
        assert_eq!(significant[3].kind, TokenKind::Identifier);
        assert_eq!(significant[3].lexeme, "Δelta");
    }

    #[test]
    fn classifies_reserved_words() {
        let output = lex("module demo\nmatch demo { _ => true }\n");
        let significant = output.significant_tokens();
        assert_eq!(significant[0].kind, TokenKind::Keyword(Keyword::Module));
        assert_eq!(significant[2].kind, TokenKind::Keyword(Keyword::Match));
        assert_eq!(significant[5].kind, TokenKind::Underscore);
        assert_eq!(significant[6].kind, TokenKind::FatArrow);
        assert_eq!(significant[7].kind, TokenKind::Keyword(Keyword::True));
    }

    #[test]
    fn preserves_comments_as_trivia() {
        let output = lex("// one\n/* two */\n");
        assert!(output.tokens.iter().any(|token| token.kind == TokenKind::LineComment));
        assert!(output.tokens.iter().any(|token| token.kind == TokenKind::BlockComment));
    }

    #[test]
    fn recovers_from_invalid_characters() {
        let output = lex("let ok = 1\n§bad\n");
        assert_eq!(output.errors.len(), 1);
        assert_eq!(output.errors[0].kind, LexErrorKind::InvalidCharacter);
        assert!(output.significant_tokens().iter().any(|token| token.lexeme == "bad"));
    }

    #[test]
    fn reports_unterminated_block_comments() {
        let output = lex("/* broken");
        assert_eq!(output.errors.len(), 1);
        assert_eq!(output.errors[0].kind, LexErrorKind::UnterminatedBlockComment);
        assert!(output.tokens.iter().any(|token| token.kind == TokenKind::Error));
    }
}
