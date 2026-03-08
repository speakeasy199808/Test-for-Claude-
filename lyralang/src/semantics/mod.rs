//! Formal semantics for the executable LyraLang Stage 0 subset.

pub mod error;
mod evaluator;

use serde::{Deserialize, Serialize};

use crate::checker;
use crate::lexer::SourceSpan;
use crate::parser::parse;

pub use error::{SemanticsError, SemanticsErrorKind};
use evaluator::SemanticsEvaluator;

/// A canonical semantic value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SemanticValue {
    /// The unit semantic value.
    Unit,
    /// A boolean semantic value.
    Bool(bool),
    /// An integer semantic value.
    Int(i64),
    /// A rational semantic value.
    Rational {
        /// Numerator.
        numerator: i64,
        /// Denominator.
        denominator: i64,
    },
    /// A linear resource token.
    Resource {
        /// Canonical resource kind.
        kind: String,
        /// Deterministic resource identifier.
        id: u32,
    },
    /// A self-reference metadata descriptor.
    Meta(String),
    /// An evidence token.
    Evidence(String),
    /// A modal wrapper over another semantic value.
    Modal {
        /// Canonical modality name.
        modality: String,
        /// Wrapped semantic value.
        body: Box<SemanticValue>,
    },
}

/// A binding denotation recorded during semantic execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BindingDenotation {
    /// Bound name.
    pub name: String,
    /// Canonical semantic value.
    pub value: SemanticValue,
    /// Human-readable canonical rendering.
    pub rendered: String,
    /// Binding span.
    pub span: SourceSpan,
}

/// A single operational semantic step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationalStep {
    /// Stable step index.
    pub index: u32,
    /// Canonical rule name.
    pub rule: String,
    /// Deterministic step detail.
    pub detail: String,
    /// Source span associated with the step.
    pub span: SourceSpan,
}

/// Semantic judgment for a successfully executed Stage 0 program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticJudgment {
    /// Optional module name.
    pub module: Option<String>,
    /// Canonical result type from the type checker.
    pub program_type: String,
    /// Denotational result.
    pub denotation: SemanticValue,
    /// Canonical textual rendering of the denotation.
    pub denotation_rendered: String,
    /// Source-ordered binding denotations.
    pub bindings: Vec<BindingDenotation>,
    /// Operational evaluation trace.
    pub steps: Vec<OperationalStep>,
    /// Stage 0 soundness statement carried with the judgment.
    pub soundness_statement: String,
    /// Program span.
    pub span: SourceSpan,
}

/// Result bundle returned by the seed formal semantics pass.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticsOutput {
    /// Normalized source passed through earlier compiler stages.
    pub normalized_source: String,
    /// Semantic judgment when execution succeeded.
    pub judgment: Option<SemanticJudgment>,
    /// Diagnostics emitted during semantic analysis.
    pub errors: Vec<SemanticsError>,
}

/// Deterministic Stage 0 formal semantics pass.
#[derive(Debug, Clone, Default)]
pub struct FormalSemantics;

impl FormalSemantics {
    /// Creates a new semantics pass.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Parses, type-checks, and semantically evaluates source text.
    #[must_use]
    pub fn analyze_source(&self, source: &str) -> SemanticsOutput {
        let type_output = checker::check(source);
        let normalized_source = type_output.normalized_source.clone();

        if !type_output.errors.is_empty() {
            let errors = type_output
                .errors
                .into_iter()
                .map(|error| {
                    SemanticsError::new(
                        SemanticsErrorKind::TypeError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return SemanticsOutput {
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
                    SemanticsError::new(
                        SemanticsErrorKind::ParseError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return SemanticsOutput {
                normalized_source,
                judgment: None,
                errors,
            };
        }

        let Some(type_judgment) = type_output.judgment else {
            return SemanticsOutput {
                normalized_source,
                judgment: None,
                errors: vec![SemanticsError::new(
                    SemanticsErrorKind::TypeError,
                    "type checker completed without a program judgment",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        let Some(program) = parse_output.program else {
            return SemanticsOutput {
                normalized_source,
                judgment: None,
                errors: vec![SemanticsError::new(
                    SemanticsErrorKind::ParseError,
                    "parser completed without a program AST",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        match SemanticsEvaluator::default().evaluate_program(normalized_source.clone(), &program, &type_judgment) {
            Ok(output) => output,
            Err(error) => SemanticsOutput {
                normalized_source,
                judgment: None,
                errors: vec![error],
            },
        }
    }
}

/// Parses, type-checks, and semantically evaluates source text with the default semantics pass.
#[must_use]
pub fn analyze(source: &str) -> SemanticsOutput {
    FormalSemantics::new().analyze_source(source)
}
