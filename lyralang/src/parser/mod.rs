//! Seed parser and syntactic structures for LyraLang.
//!
//! This module implements the first executable Phase 1 syntax surface.
//! It follows the grammatical specification in `docs/lyralang/GRAMMAR.md`
//! and provides a deterministic, span-carrying AST.

pub mod ast;
pub mod error;
pub mod parser;

pub use ast::{
    BinaryOperator, BlockExpression, CallExpression, Expression, ExpressionKind,
    ExpressionStatement, GroupExpression, Identifier, IfExpression, LetStatement, MatchArm,
    MatchExpression, ModuleDecl, Pattern, PatternKind, PrefixExpression, PrefixOperator,
    Program, SelfReferenceExpression, SelfReferencePrimitive, Statement, TryExpression,
};
pub use error::{ParseError, ParseErrorKind, ParseOutput};
pub use parser::{parse, Parser};
