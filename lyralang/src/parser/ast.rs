//! Abstract syntax tree structures for the seed LyraLang parser.

use serde::{Deserialize, Serialize};

use crate::lexer::span::SourceSpan;

/// A parsed Stage 0 program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Program {
    /// Optional module declaration at the head of the file.
    pub module_decl: Option<ModuleDecl>,
    /// All terminated statements preceding the tail expression.
    pub statements: Vec<Statement>,
    /// Optional trailing expression for the program.
    pub tail_expression: Option<Expression>,
    /// Span of the full program.
    pub span: SourceSpan,
}

/// A `module` declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleDecl {
    /// Declared module name.
    pub name: Identifier,
    /// Span of the full declaration.
    pub span: SourceSpan,
}

/// A Unicode-aware identifier node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identifier {
    /// Identifier text as written in normalized source.
    pub text: String,
    /// Source span for the identifier token.
    pub span: SourceSpan,
}

/// A Stage 0 statement node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Statement {
    /// A `let` binding statement.
    Let(LetStatement),
    /// A terminated expression statement.
    Expr(ExpressionStatement),
}

impl Statement {
    /// Returns the statement span.
    #[must_use]
    pub const fn span(&self) -> SourceSpan {
        match self {
            Self::Let(statement) => statement.span,
            Self::Expr(statement) => statement.span,
        }
    }
}

/// A `let` binding statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LetStatement {
    /// The bound pattern.
    pub pattern: Pattern,
    /// The initializer expression.
    pub value: Expression,
    /// Span of the full statement.
    pub span: SourceSpan,
}

/// A terminated expression statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpressionStatement {
    /// The expression payload.
    pub expression: Expression,
    /// Whether an explicit or structural terminator ended the statement.
    pub terminated: bool,
    /// Span of the statement.
    pub span: SourceSpan,
}

/// A Stage 0 pattern node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pattern {
    /// Pattern classification.
    pub kind: PatternKind,
    /// Span of the pattern.
    pub span: SourceSpan,
}

impl Pattern {
    /// Returns the bound identifier text when the pattern is an identifier.
    #[must_use]
    pub fn identifier_text(&self) -> Option<&str> {
        match &self.kind {
            PatternKind::Identifier(identifier) => Some(identifier.text.as_str()),
            _ => None,
        }
    }
}

/// Supported Stage 0 pattern forms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternKind {
    /// The wildcard `_` pattern.
    Wildcard,
    /// An identifier pattern.
    Identifier(Identifier),
    /// A decimal integer literal pattern.
    Integer(String),
    /// A string literal pattern.
    String(String),
    /// A boolean literal pattern.
    Boolean(bool),
}

/// A Stage 0 expression node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Expression {
    /// Expression classification.
    pub kind: ExpressionKind,
    /// Span of the expression.
    pub span: SourceSpan,
}

/// Supported Stage 0 expression forms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpressionKind {
    /// An identifier reference.
    Identifier(Identifier),
    /// A decimal integer literal.
    Integer(String),
    /// A string literal.
    String(String),
    /// A boolean literal.
    Boolean(bool),
    /// A self-reference primitive invocation.
    SelfReference(Box<SelfReferenceExpression>),
    /// A grouped expression.
    Group(Box<GroupExpression>),
    /// A block expression.
    Block(Box<BlockExpression>),
    /// An `if` expression.
    If(Box<IfExpression>),
    /// A `match` expression.
    Match(Box<MatchExpression>),
    /// A prefix expression.
    Prefix(Box<PrefixExpression>),
    /// An infix expression.
    Binary {
        /// Left-hand operand.
        left: Box<Expression>,
        /// Binary operator.
        operator: BinaryOperator,
        /// Right-hand operand.
        right: Box<Expression>,
    },
    /// A function or callable invocation.
    Call(Box<CallExpression>),
    /// Postfix propagation operator.
    Try(Box<TryExpression>),
}


/// Supported built-in self-reference primitives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelfReferencePrimitive {
    /// `@current_program()`
    CurrentProgram,
    /// `@current_receipt()`
    CurrentReceipt,
    /// `@ledger_state()`
    LedgerState,
}

impl SelfReferencePrimitive {
    /// Returns the canonical source spelling of the primitive name without `@`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentProgram => "current_program",
            Self::CurrentReceipt => "current_receipt",
            Self::LedgerState => "ledger_state",
        }
    }
}

/// A self-reference primitive expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelfReferenceExpression {
    /// The referenced primitive kind.
    pub primitive: SelfReferencePrimitive,
    /// Span of the full expression.
    pub span: SourceSpan,
}

/// A grouped expression, preserving source grouping.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupExpression {
    /// Inner grouped expression.
    pub expression: Expression,
    /// Span of the full grouped construct.
    pub span: SourceSpan,
}

/// A block expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockExpression {
    /// Statements inside the block.
    pub statements: Vec<Statement>,
    /// Optional trailing expression in the block.
    pub tail_expression: Option<Expression>,
    /// Span of the full block.
    pub span: SourceSpan,
}

/// An `if` expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IfExpression {
    /// Conditional expression.
    pub condition: Expression,
    /// Then branch expression.
    pub then_branch: Expression,
    /// Optional else branch expression.
    pub else_branch: Option<Expression>,
    /// Span of the full expression.
    pub span: SourceSpan,
}

/// A `match` expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchExpression {
    /// Scrutinee being matched.
    pub scrutinee: Expression,
    /// Ordered arms.
    pub arms: Vec<MatchArm>,
    /// Span of the full expression.
    pub span: SourceSpan,
}

/// A single `match` arm.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchArm {
    /// Pattern for this arm.
    pub pattern: Pattern,
    /// Expression body for this arm.
    pub body: Expression,
    /// Span of the full arm.
    pub span: SourceSpan,
}

/// A prefix expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrefixExpression {
    /// Prefix operator.
    pub operator: PrefixOperator,
    /// Operand expression.
    pub operand: Expression,
    /// Span of the full expression.
    pub span: SourceSpan,
}

/// Prefix operators supported in Stage 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrefixOperator {
    /// Unary numeric negation.
    Negate,
}

/// Infix operators supported in Stage 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOperator {
    /// Logical disjunction.
    LogicalOr,
    /// Logical conjunction.
    LogicalAnd,
    /// Equality comparison.
    Equal,
    /// Inequality comparison.
    NotEqual,
    /// Less-than comparison.
    Less,
    /// Less-than-or-equal comparison.
    LessEqual,
    /// Greater-than comparison.
    Greater,
    /// Greater-than-or-equal comparison.
    GreaterEqual,
    /// Addition.
    Add,
    /// Subtraction.
    Subtract,
    /// Multiplication.
    Multiply,
    /// Division.
    Divide,
    /// Modulo.
    Modulo,
}

/// A function or callable invocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallExpression {
    /// Expression being called.
    pub callee: Expression,
    /// Positional call arguments.
    pub arguments: Vec<Expression>,
    /// Span of the full call.
    pub span: SourceSpan,
}

/// A postfix propagation expression using `?`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TryExpression {
    /// Operand being propagated.
    pub operand: Expression,
    /// Span of the full expression.
    pub span: SourceSpan,
}
