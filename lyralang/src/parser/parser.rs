//! Deterministic seed parser for LyraLang.

use crate::lexer::{lex, Keyword, LexOutput, SourceSpan, Token, TokenKind};
use crate::parser::ast::{
    BinaryOperator, BlockExpression, CallExpression, Expression, ExpressionKind,
    ExpressionStatement, GroupExpression, Identifier, IfExpression, LetStatement, MatchArm,
    MatchExpression, ModuleDecl, Pattern, PatternKind, PrefixExpression, PrefixOperator,
    Program, SelfReferenceExpression, SelfReferencePrimitive, Statement,
};
use crate::parser::error::{ParseError, ParseErrorKind, ParseOutput};

/// Parses Lyra source text into a Stage 0 AST.
#[must_use]
pub fn parse(source: &str) -> ParseOutput {
    let lex_output = lex(source);
    Parser::from_lex_output(lex_output).parse()
}

/// Deterministic recursive descent parser with Pratt expression parsing.
pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
    errors: Vec<ParseError>,
    normalized_source: String,
}

impl Parser {
    /// Creates a parser from lexical output.
    #[must_use]
    pub fn from_lex_output(lex_output: LexOutput) -> Self {
        let mut errors = Vec::new();

        for error in &lex_output.errors {
            errors.push(ParseError::new(
                ParseErrorKind::LexicalError,
                error.message.clone(),
                error.span,
                error.recovered,
            ));
        }

        let tokens = lex_output
            .tokens
            .into_iter()
            .filter(|token| !matches!(&token.kind, TokenKind::Whitespace | TokenKind::LineComment | TokenKind::BlockComment))
            .collect();

        Self {
            tokens,
            cursor: 0,
            errors,
            normalized_source: lex_output.normalized_source,
        }
    }

    /// Parses the full token stream into a program.
    #[must_use]
    pub fn parse(mut self) -> ParseOutput {
        if self
            .errors
            .iter()
            .any(|error| error.kind == ParseErrorKind::LexicalError)
        {
            return ParseOutput {
                normalized_source: self.normalized_source,
                program: None,
                errors: self.errors,
            };
        }

        let program = self.parse_program();
        ParseOutput {
            normalized_source: self.normalized_source,
            program,
            errors: self.errors,
        }
    }

    fn parse_program(&mut self) -> Option<Program> {
        let start = self.current().span.start;
        self.consume_separators();

        let module_decl = if self.check_keyword(Keyword::Module) {
            let declaration = self.parse_module_decl();
            self.consume_separators();
            declaration
        } else {
            None
        };

        let mut statements = Vec::new();
        let mut tail_expression = None;

        while !self.is_eof() {
            self.consume_separators();
            if self.is_eof() {
                break;
            }

            if self.matches_kind(&TokenKind::RBrace) {
                let token = self.advance().clone();
                self.push_error(
                    ParseErrorKind::UnexpectedToken,
                    "unexpected `}` at program scope",
                    token.span,
                    true,
                );
                continue;
            }

            if self.check_keyword(Keyword::Let) {
                if let Some(statement) = self.parse_let_statement() {
                    statements.push(Statement::Let(statement));
                }
                self.recover_statement_boundary();
                self.consume_separators();
                continue;
            }

            let expression = match self.parse_expression(0) {
                Some(expression) => expression,
                None => {
                    self.recover_statement_boundary();
                    self.consume_separators();
                    continue;
                }
            };

            if self.is_eof() {
                tail_expression = Some(expression);
                break;
            }

            if self.is_separator() {
                let span = expression.span;
                statements.push(Statement::Expr(ExpressionStatement {
                    expression,
                    terminated: true,
                    span,
                }));
                self.consume_separators();
                continue;
            }

            if self.matches_kind(&TokenKind::RBrace) {
                tail_expression = Some(expression);
                break;
            }

            let span = self.current().span;
            self.push_error(
                ParseErrorKind::ExpectedToken,
                "expected newline, `;`, or end of container after expression",
                span,
                true,
            );
            let expression_span = expression.span;
            statements.push(Statement::Expr(ExpressionStatement {
                expression,
                terminated: false,
                span: expression_span,
            }));
            self.recover_statement_boundary();
            self.consume_separators();
        }

        let end = self.current().span.end;
        Some(Program {
            module_decl,
            statements,
            tail_expression,
            span: SourceSpan::new(start, end),
        })
    }

    fn parse_module_decl(&mut self) -> Option<ModuleDecl> {
        let start = self.current().span.start;
        self.advance();
        let name = self.parse_identifier()?;
        let span = SourceSpan::new(start, name.span.end);
        Some(ModuleDecl { name, span })
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let start = self.current().span.start;
        self.advance();
        let pattern = self.parse_pattern()?;
        self.expect_kind(TokenKind::Assign, "`=` after let pattern")?;
        let value = self.parse_expression(0)?;
        let span = SourceSpan::new(start, value.span.end);
        Some(LetStatement { pattern, value, span })
    }

    fn parse_pattern(&mut self) -> Option<Pattern> {
        let token = self.current().clone();
        let kind = match token.kind.clone() {
            TokenKind::Underscore => {
                self.advance();
                PatternKind::Wildcard
            }
            TokenKind::Identifier => {
                self.advance();
                PatternKind::Identifier(Identifier {
                    text: token.lexeme,
                    span: token.span,
                })
            }
            TokenKind::Integer => {
                self.advance();
                PatternKind::Integer(token.lexeme)
            }
            TokenKind::String => {
                self.advance();
                PatternKind::String(token.lexeme)
            }
            TokenKind::Keyword(Keyword::True) => {
                self.advance();
                PatternKind::Boolean(true)
            }
            TokenKind::Keyword(Keyword::False) => {
                self.advance();
                PatternKind::Boolean(false)
            }
            _ => {
                self.push_error(
                    ParseErrorKind::ExpectedToken,
                    "expected pattern",
                    token.span,
                    false,
                );
                return None;
            }
        };

        Some(Pattern {
            kind,
            span: token.span,
        })
    }

    fn parse_expression(&mut self, min_binding_power: u8) -> Option<Expression> {
        let mut left = self.parse_prefix_expression()?;

        loop {
            if self.matches_kind(&TokenKind::LParen) {
                let call_binding_power = 15;
                if call_binding_power < min_binding_power {
                    break;
                }
                left = self.parse_call_expression(left)?;
                continue;
            }

            if self.matches_kind(&TokenKind::Question) {
                let try_binding_power = 15;
                if try_binding_power < min_binding_power {
                    break;
                }
                left = self.parse_try_expression(left)?;
                continue;
            }

            let (operator, left_binding_power, right_binding_power) = match self.current_binary_operator() {
                Some(data) => data,
                None => break,
            };

            if left_binding_power < min_binding_power {
                break;
            }

            let start = left.span.start;
            self.advance();
            let right = self.parse_expression(right_binding_power)?;
            let span = SourceSpan::new(start, right.span.end);
            left = Expression {
                kind: ExpressionKind::Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                },
                span,
            };
        }

        Some(left)
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        if self.matches_kind(&TokenKind::Minus) {
            let start = self.current().span.start;
            self.advance();
            let operand = self.parse_expression(13)?;
            let span = SourceSpan::new(start, operand.span.end);
            return Some(Expression {
                kind: ExpressionKind::Prefix(Box::new(PrefixExpression {
                    operator: PrefixOperator::Negate,
                    operand,
                    span,
                })),
                span,
            });
        }

        self.parse_primary_expression()
    }

    fn parse_primary_expression(&mut self) -> Option<Expression> {
        let token = self.current().clone();
        match token.kind.clone() {
            TokenKind::Identifier => {
                self.advance();
                let identifier = Identifier {
                    text: token.lexeme,
                    span: token.span,
                };
                Some(Expression {
                    kind: ExpressionKind::Identifier(identifier),
                    span: token.span,
                })
            }
            TokenKind::Integer => {
                self.advance();
                Some(Expression {
                    kind: ExpressionKind::Integer(token.lexeme),
                    span: token.span,
                })
            }
            TokenKind::String => {
                self.advance();
                Some(Expression {
                    kind: ExpressionKind::String(token.lexeme),
                    span: token.span,
                })
            }
            TokenKind::Keyword(Keyword::True) => {
                self.advance();
                Some(Expression {
                    kind: ExpressionKind::Boolean(true),
                    span: token.span,
                })
            }
            TokenKind::Keyword(Keyword::False) => {
                self.advance();
                Some(Expression {
                    kind: ExpressionKind::Boolean(false),
                    span: token.span,
                })
            }
            TokenKind::At => self.parse_self_reference_expression(),
            TokenKind::LParen => self.parse_group_expression(),
            TokenKind::LBrace => self.parse_block_expression(),
            TokenKind::Keyword(Keyword::If) => self.parse_if_expression(),
            TokenKind::Keyword(Keyword::Match) => self.parse_match_expression(),
            _ => {
                self.push_error(
                    ParseErrorKind::UnexpectedToken,
                    "expected expression",
                    token.span,
                    false,
                );
                None
            }
        }
    }


    fn parse_self_reference_expression(&mut self) -> Option<Expression> {
        let start = self.current().span.start;
        self.expect_kind(TokenKind::At, "`@` to begin self reference")?;
        let identifier = self.parse_identifier()?;
        let primitive = match identifier.text.as_str() {
            "current_program" => SelfReferencePrimitive::CurrentProgram,
            "current_receipt" => SelfReferencePrimitive::CurrentReceipt,
            "ledger_state" => SelfReferencePrimitive::LedgerState,
            other => {
                self.push_error(
                    ParseErrorKind::UnexpectedToken,
                    format!("unknown self reference primitive `@{other}`"),
                    identifier.span,
                    false,
                );
                return None;
            }
        };
        self.expect_kind(TokenKind::LParen, "`(` after self reference primitive")?;
        let end = self.expect_kind(TokenKind::RParen, "`)` after self reference primitive")?.span.end;
        let span = SourceSpan::new(start, end);
        Some(Expression {
            kind: ExpressionKind::SelfReference(Box::new(SelfReferenceExpression {
                primitive,
                span,
            })),
            span,
        })
    }

    fn parse_group_expression(&mut self) -> Option<Expression> {
        let start = self.current().span.start;
        self.expect_kind(TokenKind::LParen, "`(` to begin grouped expression")?;
        let expression = self.parse_expression(0)?;
        let end = self.expect_kind(TokenKind::RParen, "`)` to close grouped expression")?.span.end;
        let span = SourceSpan::new(start, end);
        Some(Expression {
            kind: ExpressionKind::Group(Box::new(GroupExpression { expression, span })),
            span,
        })
    }

    fn parse_block_expression(&mut self) -> Option<Expression> {
        let start = self.current().span.start;
        self.expect_kind(TokenKind::LBrace, "`{` to begin block")?;
        self.consume_separators();

        let mut statements = Vec::new();
        let mut tail_expression = None;

        while !self.is_eof() && !self.matches_kind(&TokenKind::RBrace) {
            if self.check_keyword(Keyword::Let) {
                if let Some(statement) = self.parse_let_statement() {
                    statements.push(Statement::Let(statement));
                }
                self.recover_statement_boundary();
                self.consume_separators();
                continue;
            }

            let expression = match self.parse_expression(0) {
                Some(expression) => expression,
                None => {
                    self.recover_statement_boundary();
                    self.consume_separators();
                    continue;
                }
            };

            if self.matches_kind(&TokenKind::RBrace) {
                tail_expression = Some(expression);
                break;
            }

            if self.is_separator() {
                let span = expression.span;
                statements.push(Statement::Expr(ExpressionStatement {
                    expression,
                    terminated: true,
                    span,
                }));
                self.consume_separators();
                continue;
            }

            let span = self.current().span;
            self.push_error(
                ParseErrorKind::ExpectedToken,
                "expected newline, `;`, or `}` after block expression",
                span,
                true,
            );
            let expression_span = expression.span;
            statements.push(Statement::Expr(ExpressionStatement {
                expression,
                terminated: false,
                span: expression_span,
            }));
            self.recover_statement_boundary();
            self.consume_separators();
        }

        let end = self.expect_kind(TokenKind::RBrace, "`}` to close block")?.span.end;
        let span = SourceSpan::new(start, end);
        Some(Expression {
            kind: ExpressionKind::Block(Box::new(BlockExpression {
                statements,
                tail_expression,
                span,
            })),
            span,
        })
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        let start = self.current().span.start;
        self.advance();
        let condition = self.parse_expression(0)?;
        let then_branch = self.parse_expression(0)?;
        let else_branch = if self.check_keyword(Keyword::Else) {
            self.advance();
            Some(self.parse_expression(0)?)
        } else {
            None
        };
        let end = else_branch
            .as_ref()
            .map_or(then_branch.span.end, |expression| expression.span.end);
        let span = SourceSpan::new(start, end);
        Some(Expression {
            kind: ExpressionKind::If(Box::new(IfExpression {
                condition,
                then_branch,
                else_branch,
                span,
            })),
            span,
        })
    }

    fn parse_match_expression(&mut self) -> Option<Expression> {
        let start = self.current().span.start;
        self.advance();
        let scrutinee = self.parse_expression(0)?;
        self.expect_kind(TokenKind::LBrace, "`{` to begin match arms")?;
        self.consume_separators();

        let mut arms = Vec::new();
        while !self.is_eof() && !self.matches_kind(&TokenKind::RBrace) {
            if let Some(arm) = self.parse_match_arm() {
                arms.push(arm);
            } else {
                self.recover_match_arm_boundary();
            }

            if self.matches_kind(&TokenKind::Comma) {
                self.advance();
            }
            self.consume_separators();
        }

        let end = self.expect_kind(TokenKind::RBrace, "`}` to close match expression")?.span.end;
        let span = SourceSpan::new(start, end);
        Some(Expression {
            kind: ExpressionKind::Match(Box::new(MatchExpression {
                scrutinee,
                arms,
                span,
            })),
            span,
        })
    }

    fn parse_match_arm(&mut self) -> Option<MatchArm> {
        let start = self.current().span.start;
        let pattern = self.parse_pattern()?;
        self.expect_kind(TokenKind::FatArrow, "`=>` after match pattern")?;
        let body = self.parse_expression(0)?;
        let span = SourceSpan::new(start, body.span.end);
        Some(MatchArm { pattern, body, span })
    }

    fn parse_call_expression(&mut self, callee: Expression) -> Option<Expression> {
        let start = callee.span.start;
        self.expect_kind(TokenKind::LParen, "`(` to begin call arguments")?;
        let mut arguments = Vec::new();

        if !self.matches_kind(&TokenKind::RParen) {
            loop {
                arguments.push(self.parse_expression(0)?);
                if self.matches_kind(&TokenKind::Comma) {
                    self.advance();
                    continue;
                }
                break;
            }
        }

        let end = self.expect_kind(TokenKind::RParen, "`)` to close call arguments")?.span.end;
        let span = SourceSpan::new(start, end);
        Some(Expression {
            kind: ExpressionKind::Call(Box::new(CallExpression {
                callee,
                arguments,
                span,
            })),
            span,
        })
    }

    fn parse_try_expression(&mut self, operand: Expression) -> Option<Expression> {
        let start = operand.span.start;
        let end = self.expect_kind(TokenKind::Question, "`?` to propagate Option/Result")?.span.end;
        let span = SourceSpan::new(start, end);
        Some(Expression {
            kind: ExpressionKind::Try(Box::new(TryExpression { operand, span })),
            span,
        })
    }

    fn parse_identifier(&mut self) -> Option<Identifier> {
        let token = self.current().clone();
        if !matches!(token.kind, TokenKind::Identifier) {
            self.push_error(
                ParseErrorKind::ExpectedToken,
                "expected identifier",
                token.span,
                false,
            );
            return None;
        }

        self.advance();
        Some(Identifier {
            text: token.lexeme,
            span: token.span,
        })
    }

    fn current_binary_operator(&self) -> Option<(BinaryOperator, u8, u8)> {
        match &self.current().kind {
            TokenKind::OrOr => Some((BinaryOperator::LogicalOr, 1, 2)),
            TokenKind::AndAnd => Some((BinaryOperator::LogicalAnd, 3, 4)),
            TokenKind::EqEq => Some((BinaryOperator::Equal, 5, 6)),
            TokenKind::NotEq => Some((BinaryOperator::NotEqual, 5, 6)),
            TokenKind::Lt => Some((BinaryOperator::Less, 7, 8)),
            TokenKind::LtEq => Some((BinaryOperator::LessEqual, 7, 8)),
            TokenKind::Gt => Some((BinaryOperator::Greater, 7, 8)),
            TokenKind::GtEq => Some((BinaryOperator::GreaterEqual, 7, 8)),
            TokenKind::Plus => Some((BinaryOperator::Add, 9, 10)),
            TokenKind::Minus => Some((BinaryOperator::Subtract, 9, 10)),
            TokenKind::Star => Some((BinaryOperator::Multiply, 11, 12)),
            TokenKind::Slash => Some((BinaryOperator::Divide, 11, 12)),
            TokenKind::Percent => Some((BinaryOperator::Modulo, 11, 12)),
            _ => None,
        }
    }

    fn expect_kind(&mut self, expected: TokenKind, context: &str) -> Option<Token> {
        let token = self.current().clone();
        if token.kind == expected {
            self.advance();
            return Some(token);
        }

        let kind = if token.kind == TokenKind::Eof {
            ParseErrorKind::UnexpectedEof
        } else {
            ParseErrorKind::ExpectedToken
        };
        self.push_error(
            kind,
            format!("expected {context}"),
            token.span,
            false,
        );
        None
    }

    fn push_error(
        &mut self,
        kind: ParseErrorKind,
        message: impl Into<String>,
        span: SourceSpan,
        recovered: bool,
    ) {
        self.errors.push(ParseError::new(kind, message, span, recovered));
    }

    fn consume_separators(&mut self) {
        while self.is_separator() {
            self.advance();
        }
    }

    fn recover_statement_boundary(&mut self) {
        while !self.is_eof() && !self.is_separator() && !self.matches_kind(&TokenKind::RBrace) {
            self.advance();
        }
    }

    fn recover_match_arm_boundary(&mut self) {
        while !self.is_eof()
            && !self.is_separator()
            && !self.matches_kind(&TokenKind::Comma)
            && !self.matches_kind(&TokenKind::RBrace)
        {
            self.advance();
        }
    }

    fn check_keyword(&self, keyword: Keyword) -> bool {
        matches!(&self.current().kind, TokenKind::Keyword(found) if *found == keyword)
    }

    fn is_separator(&self) -> bool {
        self.matches_kind(&TokenKind::Newline) || self.matches_kind(&TokenKind::Semicolon)
    }

    fn matches_kind(&self, expected: &TokenKind) -> bool {
        &self.current().kind == expected
    }

    fn advance(&mut self) -> &Token {
        let index = self.cursor;
        if !self.is_eof() {
            self.cursor += 1;
        }
        &self.tokens[index]
    }

    fn current(&self) -> &Token {
        let index = self.cursor.min(self.tokens.len().saturating_sub(1));
        &self.tokens[index]
    }

    fn is_eof(&self) -> bool {
        self.matches_kind(&TokenKind::Eof)
    }
}
