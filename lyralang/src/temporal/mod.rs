
//! Seed linear-temporal-logic checker for LyraLang Stage 0.
//!
//! The executable Stage 0 temporal surface uses conservative builtin call
//! operators over the current parser: `always(expr)`, `eventually(expr)`,
//! `until(lhs, rhs)`, and `since(lhs, rhs)`.

pub mod error;
mod checker;

use serde::{Deserialize, Serialize};

use crate::checker;
use crate::lexer::SourceSpan;
use crate::parser::parse;
use crate::temporal::checker::TemporalAnalyzer;
use crate::temporal::error::{TemporalError, TemporalErrorKind};
use crate::types::Type;

/// A single temporal formula encountered in source order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalFormulaJudgment {
    /// Operator name.
    pub operator: String,
    /// Stable operand summaries.
    pub operands: Vec<String>,
    /// Canonical normalized formula.
    pub normalized_formula: String,
    /// Source span of the operator call.
    pub span: SourceSpan,
}

/// A bound identifier carrying a temporal proposition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalBindingJudgment {
    /// Bound identifier name.
    pub name: String,
    /// Canonical proposition type.
    pub proposition_type: String,
    /// Source span of the binding site.
    pub span: SourceSpan,
}

/// Temporal summary for a checked Stage 0 program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalProgramJudgment {
    /// Optional module name.
    pub module: Option<String>,
    /// Final inferred program type.
    pub program_type: Type,
    /// Temporal bindings encountered in source order.
    pub bindings: Vec<TemporalBindingJudgment>,
    /// Temporal formulas encountered in source order.
    pub formulas: Vec<TemporalFormulaJudgment>,
    /// Source span for the full program.
    pub span: SourceSpan,
}

/// Result bundle returned by the seed temporal checker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalCheckOutput {
    /// Normalized source passed through earlier compiler stages.
    pub normalized_source: String,
    /// Program judgment when temporal checking succeeded.
    pub judgment: Option<TemporalProgramJudgment>,
    /// Diagnostics emitted during checking.
    pub errors: Vec<TemporalError>,
}

/// Deterministic seed temporal checker.
#[derive(Debug, Clone, Default)]
pub struct TemporalChecker;

impl TemporalChecker {
    /// Creates a new temporal checker.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Parses, type-checks, and analyzes source text.
    #[must_use]
    pub fn check_source(&self, source: &str) -> TemporalCheckOutput {
        let type_output = checker::check(source);
        let normalized_source = type_output.normalized_source.clone();

        if !type_output.errors.is_empty() {
            let errors = type_output
                .errors
                .into_iter()
                .map(|error| {
                    TemporalError::new(
                        TemporalErrorKind::TypeError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return TemporalCheckOutput {
                normalized_source,
                judgment: None,
                errors,
            };
        }

        let parse_output = parse(source);
        if !parse_output.errors.is_empty() {
            let errors = parse_output
                .errors
                .into_iter()
                .map(|error| {
                    TemporalError::new(
                        TemporalErrorKind::ParseError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return TemporalCheckOutput {
                normalized_source,
                judgment: None,
                errors,
            };
        }

        let Some(judgment) = type_output.judgment else {
            return TemporalCheckOutput {
                normalized_source,
                judgment: None,
                errors: vec![TemporalError::new(
                    TemporalErrorKind::TypeError,
                    "type checker completed without a program judgment",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        let Some(program) = parse_output.program else {
            return TemporalCheckOutput {
                normalized_source,
                judgment: None,
                errors: vec![TemporalError::new(
                    TemporalErrorKind::ParseError,
                    "parser completed without a program AST",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        match TemporalAnalyzer::from_type_judgment(&judgment).analyze_program(&program, judgment.program_type) {
            Ok(judgment) => TemporalCheckOutput {
                normalized_source,
                judgment: Some(judgment),
                errors: Vec::new(),
            },
            Err(error) => TemporalCheckOutput {
                normalized_source,
                judgment: None,
                errors: vec![error],
            },
        }
    }
}

/// Parses, type-checks, and analyzes source text with the default checker.
#[must_use]
pub fn check(source: &str) -> TemporalCheckOutput {
    TemporalChecker::new().check_source(source)
}
