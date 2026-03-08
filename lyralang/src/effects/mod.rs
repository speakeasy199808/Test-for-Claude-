//! Seed effect checker for LyraLang Stage 0.
//!
//! This module owns deterministic effect inference and effect-policy validation
//! for the currently parsed Stage 0 language surface.

pub mod error;
mod infer;

use serde::{Deserialize, Serialize};

use crate::effects::error::{EffectError, EffectErrorKind};
use crate::effects::infer::EffectInferenceEngine;
use crate::lexer::SourceSpan;
use crate::parser::parse;
use crate::types::EffectSet;

/// A bound identifier and the effects required to initialize it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectBindingJudgment {
    /// Bound identifier name.
    pub name: String,
    /// Effects required by the initializer expression.
    pub initializer_effects: EffectSet,
    /// Source span of the binding site.
    pub span: SourceSpan,
}

/// Inferred effect summary for a checked program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectProgramJudgment {
    /// Optional module name.
    pub module: Option<String>,
    /// Aggregate program effects.
    pub program_effects: EffectSet,
    /// Effect-bearing let-bindings encountered in source order.
    pub bindings: Vec<EffectBindingJudgment>,
    /// Source span for the full program.
    pub span: SourceSpan,
}

/// Allowed effect ceiling for validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectPolicy {
    /// Allowed effect ceiling.
    pub allowed_effects: EffectSet,
}

impl EffectPolicy {
    /// Creates a policy from an explicit effect ceiling.
    #[must_use]
    pub fn new(allowed_effects: EffectSet) -> Self {
        Self { allowed_effects }
    }

    /// Returns the pure policy.
    #[must_use]
    pub fn pure() -> Self {
        Self::new(EffectSet::pure())
    }
}

/// Result bundle returned by the seed effect checker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectCheckOutput {
    /// Normalized source passed through earlier compiler stages.
    pub normalized_source: String,
    /// Program judgment when effect inference succeeded.
    pub judgment: Option<EffectProgramJudgment>,
    /// Diagnostics emitted during checking.
    pub errors: Vec<EffectError>,
    /// Optional policy ceiling applied during checking.
    pub policy: Option<EffectSet>,
}

/// Deterministic seed effect checker.
#[derive(Debug, Clone, Default)]
pub struct EffectChecker {
    policy: Option<EffectPolicy>,
}

impl EffectChecker {
    /// Creates an unrestricted effect checker.
    #[must_use]
    pub fn new() -> Self {
        Self { policy: None }
    }

    /// Creates an effect checker with a policy ceiling.
    #[must_use]
    pub fn with_policy(policy: EffectPolicy) -> Self {
        Self { policy: Some(policy) }
    }

    /// Parses and effect-checks source text.
    #[must_use]
    pub fn check_source(&self, source: &str) -> EffectCheckOutput {
        let parse_output = parse(source);
        let normalized_source = parse_output.normalized_source.clone();

        if !parse_output.errors.is_empty() {
            let errors = parse_output
                .errors
                .into_iter()
                .map(|error| {
                    EffectError::new(
                        EffectErrorKind::ParseError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return EffectCheckOutput {
                normalized_source,
                judgment: None,
                errors,
                policy: self.policy.as_ref().map(|policy| policy.allowed_effects.clone()),
            };
        }

        let program = match parse_output.program {
            Some(program) => program,
            None => {
                return EffectCheckOutput {
                    normalized_source,
                    judgment: None,
                    errors: vec![EffectError::new(
                        EffectErrorKind::ParseError,
                        "parser completed without a program AST",
                        SourceSpan::default(),
                        false,
                    )],
                    policy: self.policy.as_ref().map(|policy| policy.allowed_effects.clone()),
                };
            }
        };

        EffectInferenceEngine::new(self.policy.clone()).check_program(normalized_source, &program)
    }
}

/// Parses and effect-checks source text without a policy ceiling.
#[must_use]
pub fn check(source: &str) -> EffectCheckOutput {
    EffectChecker::new().check_source(source)
}

/// Parses and effect-checks source text against an explicit policy ceiling.
#[must_use]
pub fn check_with_policy(source: &str, policy: EffectPolicy) -> EffectCheckOutput {
    EffectChecker::with_policy(policy).check_source(source)
}
