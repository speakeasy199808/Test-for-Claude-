//! Stage 0 error-handling analysis for LyraLang.
//!
//! This slice owns Result/Option propagation summaries, panic-free subset
//! enforcement, error-type composition reporting, and stack-trace integration.

pub mod error;
mod analyzer;

use serde::{Deserialize, Serialize};

use crate::checker;
use crate::checker::error::TypeErrorKind;
use crate::errors::analyzer::ErrorAnalyzer;
use crate::errors::error::{ErrorAnalysis, ErrorAnalysisKind};
use crate::lexer::SourceSpan;
use crate::parser::parse;
use crate::types::Type;

pub use error::{ErrorAnalysis as ErrorHandlingError, ErrorAnalysisKind as ErrorHandlingErrorKind};

/// A binding summary for error propagation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorBindingJudgment {
    /// Bound identifier name.
    pub name: String,
    /// Whether the initializer used postfix propagation.
    pub uses_try: bool,
    /// Source span of the binding site.
    pub span: SourceSpan,
}

/// A single propagated stack-trace frame.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StackTraceFrame {
    /// One-based line.
    pub line: usize,
    /// One-based column.
    pub column: usize,
    /// Stable operand summary.
    pub snippet: String,
}

/// A prohibited panic-style call discovered in source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PanicRestriction {
    /// Forbidden callable name.
    pub name: String,
    /// Source span for the violation.
    pub span: SourceSpan,
}

/// Error-handling summary for a checked program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorProgramJudgment {
    /// Optional module name.
    pub module: Option<String>,
    /// Final inferred program type.
    pub program_type: Type,
    /// Propagation mode for the enclosing program scope.
    pub propagation_mode: String,
    /// Composed propagated error type when result propagation is active.
    pub propagated_error_type: Option<String>,
    /// Collected propagated stack-trace frames.
    pub stack_trace: Vec<StackTraceFrame>,
    /// Whether the program stayed inside the panic-free subset.
    pub panic_free: bool,
    /// Binding-level propagation summary.
    pub bindings: Vec<ErrorBindingJudgment>,
    /// Forbidden panic-style calls, if any.
    pub panic_restrictions: Vec<PanicRestriction>,
    /// Source span for the full program.
    pub span: SourceSpan,
}

/// Result bundle returned by the seed error-handling analyzer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorCheckOutput {
    /// Normalized source passed through earlier compiler stages.
    pub normalized_source: String,
    /// Program judgment when checking succeeded.
    pub judgment: Option<ErrorProgramJudgment>,
    /// Diagnostics emitted during checking.
    pub errors: Vec<ErrorAnalysis>,
}

/// Deterministic Stage 0 error-handling analyzer.
#[derive(Debug, Clone, Default)]
pub struct ErrorChecker;

impl ErrorChecker {
    /// Creates a new seed error-handling analyzer.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Parses, type-checks, and analyzes source text.
    #[must_use]
    pub fn check_source(&self, source: &str) -> ErrorCheckOutput {
        let type_output = checker::check(source);
        let normalized_source = type_output.normalized_source.clone();

        if !type_output.errors.is_empty() {
            let errors = type_output
                .errors
                .into_iter()
                .map(|error| {
                    let kind = match error.kind {
                        TypeErrorKind::ParseError => ErrorAnalysisKind::ParseError,
                        _ => ErrorAnalysisKind::TypeError,
                    };
                    ErrorAnalysis::new(kind, error.message, error.span, error.recovered)
                })
                .collect();
            return ErrorCheckOutput {
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
                    ErrorAnalysis::new(
                        ErrorAnalysisKind::ParseError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return ErrorCheckOutput {
                normalized_source,
                judgment: None,
                errors,
            };
        }

        let Some(judgment) = type_output.judgment else {
            return ErrorCheckOutput {
                normalized_source,
                judgment: None,
                errors: vec![ErrorAnalysis::new(
                    ErrorAnalysisKind::TypeError,
                    "type checker completed without a program judgment",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        let Some(program) = parse_output.program else {
            return ErrorCheckOutput {
                normalized_source,
                judgment: None,
                errors: vec![ErrorAnalysis::new(
                    ErrorAnalysisKind::ParseError,
                    "parser completed without a program AST",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        ErrorAnalyzer::default().analyze_program(normalized_source, &program, judgment.program_type)
    }
}

/// Parses, type-checks, and analyzes source text with the default error checker.
#[must_use]
pub fn check(source: &str) -> ErrorCheckOutput {
    ErrorChecker::new().check_source(source)
}
