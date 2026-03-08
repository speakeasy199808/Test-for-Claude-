//! Seed linear-resource checker for LyraLang Stage 0.
//!
//! This module owns exact-once discharge checking for the initial linear
//! resource surface: `File`, `Socket`, and `Capability`.

pub mod error;
mod checker;

use serde::{Deserialize, Serialize};

use crate::checker;
use crate::checker::error::TypeErrorKind;
use crate::lexer::SourceSpan;
use crate::linear::checker::LinearAnalyzer;
use crate::linear::error::{LinearError, LinearErrorKind};
use crate::parser::parse;
use crate::types::ResourceType;

/// A discharged linear binding encountered during checking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinearBindingJudgment {
    /// Bound identifier name.
    pub name: String,
    /// Linear resource kind.
    pub resource: ResourceType,
    /// Source span of the binding site.
    pub span: SourceSpan,
}

/// Ownership summary for a fully checked program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinearProgramJudgment {
    /// Optional module name.
    pub module: Option<String>,
    /// Discharged linear bindings encountered in source order.
    pub bindings: Vec<LinearBindingJudgment>,
    /// Source span for the full program.
    pub span: SourceSpan,
}

/// Result bundle returned by the seed linear checker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinearCheckOutput {
    /// Normalized source passed through earlier compiler stages.
    pub normalized_source: String,
    /// Program judgment when checking succeeded.
    pub judgment: Option<LinearProgramJudgment>,
    /// Diagnostics emitted during checking.
    pub errors: Vec<LinearError>,
}

/// Deterministic seed linear checker.
#[derive(Debug, Clone, Default)]
pub struct LinearChecker;

impl LinearChecker {
    /// Creates a new seed linear checker.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Parses, type-checks, and linear-checks source text.
    #[must_use]
    pub fn check_source(&self, source: &str) -> LinearCheckOutput {
        let type_output = checker::check(source);
        let normalized_source = type_output.normalized_source.clone();

        if !type_output.errors.is_empty() {
            let errors = type_output
                .errors
                .into_iter()
                .map(|error| {
                    let kind = match error.kind {
                        TypeErrorKind::ParseError => LinearErrorKind::ParseError,
                        _ => LinearErrorKind::TypeError,
                    };
                    LinearError::new(kind, error.message, error.span, error.recovered)
                })
                .collect();
            return LinearCheckOutput {
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
                    LinearError::new(
                        LinearErrorKind::ParseError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return LinearCheckOutput {
                normalized_source,
                judgment: None,
                errors,
            };
        }

        let program = match parse_output.program {
            Some(program) => program,
            None => {
                return LinearCheckOutput {
                    normalized_source,
                    judgment: None,
                    errors: vec![LinearError::new(
                        LinearErrorKind::ParseError,
                        "parser completed without a program AST",
                        SourceSpan::default(),
                        false,
                    )],
                };
            }
        };

        LinearAnalyzer::default().check_program(normalized_source, &program)
    }
}

/// Parses, type-checks, and linear-checks source text with the default checker.
#[must_use]
pub fn check(source: &str) -> LinearCheckOutput {
    LinearChecker::new().check_source(source)
}
