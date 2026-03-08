//! Seed modal checker for LyraLang Stage 0.
//!
//! This module owns the first executable epistemic typing surface for modal
//! values such as `Fact[T]`, `Hypothesis[T]`, `Unknown[T]`, `Necessary[T]`,
//! and `Possible[T]`. Promotion is intentionally explicit and must flow
//! through evidence-bearing builtins.

pub mod error;
mod checker;

use serde::{Deserialize, Serialize};

use crate::checker;
use crate::lexer::SourceSpan;
use crate::modal::checker::ModalAnalyzer;
use crate::modal::error::{ModalError, ModalErrorKind};
use crate::parser::parse;
use crate::types::{EvidenceKind, ModalKind, Type};

/// A bound identifier carrying a modal type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModalBindingJudgment {
    /// Bound identifier name.
    pub name: String,
    /// Assigned modality.
    pub modality: ModalKind,
    /// Underlying payload type.
    pub payload_type: Type,
    /// Source span of the binding site.
    pub span: SourceSpan,
}

/// A witnessed promotion through an evidence-bearing builtin.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModalPromotionJudgment {
    /// Builtin name used for promotion.
    pub name: String,
    /// Source modality.
    pub from: ModalKind,
    /// Target modality.
    pub to: ModalKind,
    /// Evidence required by the promotion.
    pub evidence: EvidenceKind,
    /// Source span for the promotion call.
    pub span: SourceSpan,
}

/// Modal summary for a checked Stage 0 program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModalProgramJudgment {
    /// Optional module name.
    pub module: Option<String>,
    /// Final program type inferred by the seed checker.
    pub program_type: Type,
    /// Modal bindings encountered in source order.
    pub bindings: Vec<ModalBindingJudgment>,
    /// Explicit promotions encountered in source order.
    pub promotions: Vec<ModalPromotionJudgment>,
    /// Source span for the full program.
    pub span: SourceSpan,
}

/// Result bundle returned by the seed modal checker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModalCheckOutput {
    /// Normalized source passed through earlier compiler stages.
    pub normalized_source: String,
    /// Program judgment when modal checking succeeded.
    pub judgment: Option<ModalProgramJudgment>,
    /// Diagnostics emitted during checking.
    pub errors: Vec<ModalError>,
}

/// Deterministic seed modal checker.
#[derive(Debug, Clone, Default)]
pub struct ModalChecker;

impl ModalChecker {
    /// Creates a new seed modal checker.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Parses and modal-checks source text.
    #[must_use]
    pub fn check_source(&self, source: &str) -> ModalCheckOutput {
        let type_output = checker::check(source);
        let normalized_source = type_output.normalized_source.clone();

        if !type_output.errors.is_empty() {
            let errors = type_output
                .errors
                .into_iter()
                .map(|error| {
                    ModalError::new(
                        ModalErrorKind::TypeError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return ModalCheckOutput {
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
                    ModalError::new(
                        ModalErrorKind::ParseError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return ModalCheckOutput {
                normalized_source,
                judgment: None,
                errors,
            };
        }

        let Some(judgment) = type_output.judgment else {
            return ModalCheckOutput {
                normalized_source,
                judgment: None,
                errors: vec![ModalError::new(
                    ModalErrorKind::TypeError,
                    "type checker completed without a program judgment",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        let Some(program) = parse_output.program else {
            return ModalCheckOutput {
                normalized_source,
                judgment: None,
                errors: vec![ModalError::new(
                    ModalErrorKind::ParseError,
                    "parser completed without a program AST",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        match ModalAnalyzer::from_type_judgment(&judgment).analyze_program(&program, judgment.program_type) {
            Ok(judgment) => ModalCheckOutput {
                normalized_source,
                judgment: Some(judgment),
                errors: Vec::new(),
            },
            Err(error) => ModalCheckOutput {
                normalized_source,
                judgment: None,
                errors: vec![error],
            },
        }
    }
}

/// Parses and modal-checks source text with the default seed checker.
#[must_use]
pub fn check(source: &str) -> ModalCheckOutput {
    ModalChecker::new().check_source(source)
}
