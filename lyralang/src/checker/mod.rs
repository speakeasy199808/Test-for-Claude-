//! Seed type checker for LyraLang Stage 0.
//!
//! This module owns the first executable Hindley-Milner inference surface for
//! the currently parsed Stage 0 language. It consumes the parser AST and the
//! canonical type kernel from [`crate::types`].

pub mod error;
mod infer;

use serde::{Deserialize, Serialize};

use crate::checker::error::{TypeError, TypeErrorKind};
use crate::checker::infer::InferenceEngine;
use crate::lexer::SourceSpan;
use crate::parser::parse;
use crate::types::{EffectSet, Type, TypeScheme};

/// A binding and its inferred type scheme.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BindingJudgment {
    /// Bound identifier name.
    pub name: String,
    /// Generalized type scheme.
    pub scheme: TypeScheme,
    /// Source span of the binding site.
    pub span: SourceSpan,
}

/// The inferred judgment for a fully checked program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgramJudgment {
    /// Optional module name.
    pub module: Option<String>,
    /// Inferred program result type.
    pub program_type: Type,
    /// Aggregate program effects.
    pub program_effects: EffectSet,
    /// User-defined bindings encountered in source order.
    pub bindings: Vec<BindingJudgment>,
    /// Source span for the full program.
    pub span: SourceSpan,
}

/// Result bundle returned by the seed type checker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeCheckOutput {
    /// Normalized source passed through earlier compiler stages.
    pub normalized_source: String,
    /// Program judgment when inference succeeded.
    pub judgment: Option<ProgramJudgment>,
    /// Diagnostics emitted during checking.
    pub errors: Vec<TypeError>,
}

/// Deterministic seed type checker.
#[derive(Debug, Clone, Default)]
pub struct TypeChecker;

impl TypeChecker {
    /// Creates a new seed type checker.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Parses and type-checks source text.
    #[must_use]
    pub fn check_source(&self, source: &str) -> TypeCheckOutput {
        let parse_output = parse(source);
        let normalized_source = parse_output.normalized_source.clone();

        if !parse_output.errors.is_empty() {
            let errors = parse_output
                .errors
                .into_iter()
                .map(|error| {
                    TypeError::new(
                        TypeErrorKind::ParseError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return TypeCheckOutput {
                normalized_source,
                judgment: None,
                errors,
            };
        }

        let program = match parse_output.program {
            Some(program) => program,
            None => {
                return TypeCheckOutput {
                    normalized_source,
                    judgment: None,
                    errors: vec![TypeError::new(
                        TypeErrorKind::ParseError,
                        "parser completed without a program AST",
                        SourceSpan::default(),
                        false,
                    )],
                };
            }
        };

        InferenceEngine::default().check_program(normalized_source, &program)
    }
}

/// Parses and type-checks source text with the default seed checker.
#[must_use]
pub fn check(source: &str) -> TypeCheckOutput {
    TypeChecker::new().check_source(source)
}
