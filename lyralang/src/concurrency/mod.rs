
//! Seed structured-concurrency checker for LyraLang Stage 0.
//!
//! This slice exposes a conservative executable concurrency surface using
//! builtin call forms over the existing Stage 0 parser:
//! `spawn(expr)`, `join(task)`, `select(task_a, task_b)`, `channel_int()`,
//! `send_int(channel, value)`, and `recv_int(channel)`.

pub mod error;
mod checker;

use serde::{Deserialize, Serialize};

use crate::checker;
use crate::concurrency::checker::ConcurrencyAnalyzer;
use crate::concurrency::error::{ConcurrencyError, ConcurrencyErrorKind};
use crate::lexer::SourceSpan;
use crate::parser::parse;
use crate::types::Type;

/// A single spawned task discovered in source order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpawnSite {
    /// Deterministic task identifier assigned in source order.
    pub task_id: u32,
    /// Stable source summary of the spawned expression.
    pub expression: String,
    /// Captured identifier names in canonical sorted order.
    pub captures: Vec<String>,
    /// Captured channel-typed identifiers in canonical sorted order.
    pub channel_captures: Vec<String>,
    /// Source span of the spawn call.
    pub span: SourceSpan,
}

/// A single join point discovered in source order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JoinSite {
    /// Stable source summary of the joined task.
    pub task: String,
    /// Source span of the join call.
    pub span: SourceSpan,
}

/// A single deterministic select site discovered in source order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectSite {
    /// Stable source summaries of the candidate tasks.
    pub candidates: Vec<String>,
    /// Deterministic ready-winner rule.
    pub ready_winner_rule: String,
    /// Source span of the select call.
    pub span: SourceSpan,
}

/// A channel operation observed during analysis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelOperation {
    /// Operation name.
    pub operation: String,
    /// Canonical channel payload type.
    pub payload_type: String,
    /// Stable source summary of the channel operand.
    pub channel: String,
    /// Source span of the channel operation.
    pub span: SourceSpan,
}

/// Structured-concurrency summary for a checked Stage 0 program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConcurrencyProgramJudgment {
    /// Optional module name.
    pub module: Option<String>,
    /// Final inferred program type.
    pub program_type: Type,
    /// Deterministic scheduling policy.
    pub scheduling_policy: String,
    /// Spawn sites in source order.
    pub spawns: Vec<SpawnSite>,
    /// Join sites in source order.
    pub joins: Vec<JoinSite>,
    /// Select sites in source order.
    pub selects: Vec<SelectSite>,
    /// Channel operations in source order.
    pub channel_operations: Vec<ChannelOperation>,
    /// Whether the program is race-free under the Stage 0 policy.
    pub race_free: bool,
    /// Source span for the full program.
    pub span: SourceSpan,
}

/// Result bundle returned by the seed structured-concurrency checker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConcurrencyCheckOutput {
    /// Normalized source passed through earlier compiler stages.
    pub normalized_source: String,
    /// Program judgment when checking succeeded.
    pub judgment: Option<ConcurrencyProgramJudgment>,
    /// Diagnostics emitted during checking.
    pub errors: Vec<ConcurrencyError>,
}

/// Deterministic seed structured-concurrency checker.
#[derive(Debug, Clone, Default)]
pub struct ConcurrencyChecker;

impl ConcurrencyChecker {
    /// Creates a new structured-concurrency checker.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Parses, type-checks, and analyzes source text.
    #[must_use]
    pub fn check_source(&self, source: &str) -> ConcurrencyCheckOutput {
        let type_output = checker::check(source);
        let normalized_source = type_output.normalized_source.clone();

        if !type_output.errors.is_empty() {
            let errors = type_output
                .errors
                .into_iter()
                .map(|error| {
                    ConcurrencyError::new(
                        ConcurrencyErrorKind::TypeError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return ConcurrencyCheckOutput {
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
                    ConcurrencyError::new(
                        ConcurrencyErrorKind::ParseError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return ConcurrencyCheckOutput {
                normalized_source,
                judgment: None,
                errors,
            };
        }

        let Some(judgment) = type_output.judgment else {
            return ConcurrencyCheckOutput {
                normalized_source,
                judgment: None,
                errors: vec![ConcurrencyError::new(
                    ConcurrencyErrorKind::TypeError,
                    "type checker completed without a program judgment",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        let Some(program) = parse_output.program else {
            return ConcurrencyCheckOutput {
                normalized_source,
                judgment: None,
                errors: vec![ConcurrencyError::new(
                    ConcurrencyErrorKind::ParseError,
                    "parser completed without a program AST",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        match ConcurrencyAnalyzer::from_type_judgment(&judgment).analyze_program(&program, judgment.program_type) {
            Ok(judgment) => ConcurrencyCheckOutput {
                normalized_source,
                judgment: Some(judgment),
                errors: Vec::new(),
            },
            Err(error) => ConcurrencyCheckOutput {
                normalized_source,
                judgment: None,
                errors: vec![error],
            },
        }
    }
}

/// Parses, type-checks, and analyzes source text with the default checker.
#[must_use]
pub fn check(source: &str) -> ConcurrencyCheckOutput {
    ConcurrencyChecker::new().check_source(source)
}
