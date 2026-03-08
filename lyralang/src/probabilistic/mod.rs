//! Seed probabilistic symbolic checker for LyraLang Stage 0.
//!
//! This module introduces the `Dist[T]` type concept for symbolic probability
//! distributions. It recognizes builtin calls `dist_uniform`, `dist_bernoulli`,
//! and `dist_bayesian_update`, analyzing them symbolically without any numeric
//! evaluation or sampling. All analysis is structural: the checker records
//! distribution shapes and Bayesian update flows, never executing them.
//!
//! Sampling is actively forbidden: any call to `dist_sample` is a hard error.
//!
//! The `EffectAtom::Entropy` atom (from `crate::types::EffectAtom`) tracks
//! entropy in the broader type system; this module operates purely symbolically
//! and does not require entropy effects.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::checker;
use crate::lexer::SourceSpan;
use crate::parser::parse;
use crate::parser::{CallExpression, Expression, ExpressionKind, Program, Statement};

// ─── Distribution kind ────────────────────────────────────────────────────────

/// Recognized symbolic distribution families.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistributionKind {
    /// A `dist_uniform(lo, hi)` distribution over a continuous interval.
    Uniform,
    /// A `dist_bernoulli(p)` distribution where `p` is a symbolic probability.
    Bernoulli,
    /// The posterior produced by a `dist_bayesian_update(prior, likelihood)` call.
    Derived,
    /// An unrecognized distribution form.
    Unknown,
}

// ─── Summary types ────────────────────────────────────────────────────────────

/// A symbolic summary for a single recognized distribution binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DistributionSummary {
    /// Binding name that received the distribution value.
    pub name: String,
    /// Recognized distribution family.
    pub distribution_kind: DistributionKind,
    /// Symbolic summaries of each parameter expression.
    pub parameter_summaries: Vec<String>,
    /// Canonical symbolic PDF description.
    pub pdf_description: String,
    /// Source span of the distribution constructor call.
    pub span: SourceSpan,
}

/// A symbolic summary for a single Bayesian update encountered in source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BayesianUpdateSummary {
    /// Binding name of the prior distribution argument.
    pub prior_name: String,
    /// Binding name of the likelihood distribution argument.
    pub likelihood_name: String,
    /// Binding name that receives the posterior.
    pub posterior_name: String,
    /// Canonical symbolic description of the update.
    pub update_description: String,
    /// Source span of the `dist_bayesian_update` call.
    pub span: SourceSpan,
}

// ─── Program judgment ─────────────────────────────────────────────────────────

/// Probabilistic summary for a checked Stage 0 program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProbabilisticProgramJudgment {
    /// Optional module name from a `module` declaration.
    pub module: Option<String>,
    /// Canonical type name of the whole program.
    pub program_type: String,
    /// All distribution bindings encountered in source order.
    pub distributions: Vec<DistributionSummary>,
    /// All Bayesian update sites encountered in source order.
    pub bayesian_updates: Vec<BayesianUpdateSummary>,
    /// `true` iff no `dist_sample` call was detected.
    pub symbolic_only: bool,
    /// Source span for the full program.
    pub span: SourceSpan,
}

// ─── Check output ─────────────────────────────────────────────────────────────

/// Result bundle returned by the probabilistic symbolic checker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProbabilisticCheckOutput {
    /// Normalized source passed through earlier compiler stages.
    pub normalized_source: String,
    /// Program judgment when probabilistic checking succeeded without fatal errors.
    pub judgment: Option<ProbabilisticProgramJudgment>,
    /// Diagnostics emitted during checking.
    pub errors: Vec<ProbabilisticError>,
}

// ─── Error types ──────────────────────────────────────────────────────────────

/// Categories of probabilistic-checking error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProbabilisticErrorKind {
    /// Parsing failed before probabilistic checking could proceed.
    ParseError,
    /// A numeric sampling call was detected — sampling is forbidden in symbolic mode.
    SamplingDetected,
    /// A distribution constructor was malformed (wrong arity or unrecognized form).
    InvalidDistribution,
    /// A `dist_bayesian_update` call was malformed (wrong arity or non-identifier arguments).
    MalformedBayesianUpdate,
}

impl ProbabilisticErrorKind {
    /// Returns a stable machine-readable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::SamplingDetected => "sampling_detected",
            Self::InvalidDistribution => "invalid_distribution",
            Self::MalformedBayesianUpdate => "malformed_bayesian_update",
        }
    }
}

/// A probabilistic-checking diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{message}")]
pub struct ProbabilisticError {
    /// Error category.
    pub kind: ProbabilisticErrorKind,
    /// Human-readable diagnostic text.
    pub message: String,
    /// Source span associated with the error.
    pub span: SourceSpan,
    /// Whether checking recovered and continued past this error.
    pub recovered: bool,
}

impl ProbabilisticError {
    /// Creates a new probabilistic-checking diagnostic.
    #[must_use]
    pub fn new(
        kind: ProbabilisticErrorKind,
        message: impl Into<String>,
        span: SourceSpan,
        recovered: bool,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            span,
            recovered,
        }
    }
}

// ─── Checker ──────────────────────────────────────────────────────────────────

/// Deterministic seed probabilistic symbolic checker.
#[derive(Debug, Clone, Default)]
pub struct ProbabilisticChecker;

impl ProbabilisticChecker {
    /// Creates a new probabilistic symbolic checker.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Parses, type-checks, and symbolically analyzes source text.
    #[must_use]
    pub fn check_source(&self, source: &str) -> ProbabilisticCheckOutput {
        let type_output = checker::check(source);
        let normalized_source = type_output.normalized_source.clone();

        if !type_output.errors.is_empty() {
            let errors = type_output
                .errors
                .into_iter()
                .map(|error| {
                    ProbabilisticError::new(
                        ProbabilisticErrorKind::ParseError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return ProbabilisticCheckOutput {
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
                    ProbabilisticError::new(
                        ProbabilisticErrorKind::ParseError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return ProbabilisticCheckOutput {
                normalized_source,
                judgment: None,
                errors,
            };
        }

        let Some(type_judgment) = type_output.judgment else {
            return ProbabilisticCheckOutput {
                normalized_source,
                judgment: None,
                errors: vec![ProbabilisticError::new(
                    ProbabilisticErrorKind::ParseError,
                    "type checker completed without a program judgment",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        let Some(program) = parse_output.program else {
            return ProbabilisticCheckOutput {
                normalized_source,
                judgment: None,
                errors: vec![ProbabilisticError::new(
                    ProbabilisticErrorKind::ParseError,
                    "parser completed without a program AST",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        let mut analyzer = ProbabilisticAnalyzer::new();
        analyzer.analyze_program(&program);

        let symbolic_only = !analyzer
            .errors
            .iter()
            .any(|e| matches!(e.kind, ProbabilisticErrorKind::SamplingDetected));

        let errors = analyzer.errors;

        let judgment = ProbabilisticProgramJudgment {
            module: program
                .module_decl
                .as_ref()
                .map(|declaration| declaration.name.text.clone()),
            program_type: type_judgment.program_type.canonical_name(),
            distributions: analyzer.distributions,
            bayesian_updates: analyzer.bayesian_updates,
            symbolic_only,
            span: program.span,
        };

        ProbabilisticCheckOutput {
            normalized_source,
            judgment: Some(judgment),
            errors,
        }
    }
}

/// Parses, type-checks, and symbolically analyzes source text with the default checker.
#[must_use]
pub fn check(source: &str) -> ProbabilisticCheckOutput {
    ProbabilisticChecker::new().check_source(source)
}

// ─── Internal analyzer ────────────────────────────────────────────────────────

/// Internal AST walker that extracts distribution and Bayesian update summaries.
struct ProbabilisticAnalyzer {
    distributions: Vec<DistributionSummary>,
    bayesian_updates: Vec<BayesianUpdateSummary>,
    errors: Vec<ProbabilisticError>,
    /// Pending binding name for the next distribution found.
    pending_binding: Option<String>,
}

impl ProbabilisticAnalyzer {
    fn new() -> Self {
        Self {
            distributions: Vec::new(),
            bayesian_updates: Vec::new(),
            errors: Vec::new(),
            pending_binding: None,
        }
    }

    fn analyze_program(&mut self, program: &Program) {
        for statement in &program.statements {
            self.analyze_statement(statement);
        }
        if let Some(tail_expression) = &program.tail_expression {
            self.pending_binding = None;
            self.analyze_expression(tail_expression);
        }
    }

    fn analyze_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Let(let_statement) => {
                // Record the binding name so that when we visit the value
                // expression we can attach it to any distribution found.
                self.pending_binding = let_statement
                    .pattern
                    .identifier_text()
                    .map(|s| s.to_string());
                self.analyze_expression(&let_statement.value);
                self.pending_binding = None;
            }
            Statement::Expr(expr_statement) => {
                self.pending_binding = None;
                self.analyze_expression(&expr_statement.expression);
            }
        }
    }

    fn analyze_expression(&mut self, expression: &Expression) {
        match &expression.kind {
            ExpressionKind::Identifier(_)
            | ExpressionKind::Integer(_)
            | ExpressionKind::String(_)
            | ExpressionKind::Boolean(_)
            | ExpressionKind::SelfReference(_) => {}
            ExpressionKind::Group(group) => self.analyze_expression(&group.expression),
            ExpressionKind::Try(try_expression) => {
                self.analyze_expression(&try_expression.operand);
            }
            ExpressionKind::Block(block) => {
                for statement in &block.statements {
                    self.analyze_statement(statement);
                }
                if let Some(tail) = &block.tail_expression {
                    self.analyze_expression(tail);
                }
            }
            ExpressionKind::If(if_expression) => {
                self.analyze_expression(&if_expression.condition);
                self.analyze_expression(&if_expression.then_branch);
                if let Some(else_branch) = &if_expression.else_branch {
                    self.analyze_expression(else_branch);
                }
            }
            ExpressionKind::Match(match_expression) => {
                self.analyze_expression(&match_expression.scrutinee);
                for arm in &match_expression.arms {
                    self.analyze_expression(&arm.body);
                }
            }
            ExpressionKind::Prefix(prefix) => self.analyze_expression(&prefix.operand),
            ExpressionKind::Binary { left, right, .. } => {
                self.analyze_expression(left);
                self.analyze_expression(right);
            }
            ExpressionKind::Call(call) => self.analyze_call(call, expression.span),
        }
    }

    fn analyze_call(&mut self, call: &CallExpression, span: SourceSpan) {
        // Recurse into the callee first (handles higher-order cases).
        // We do not recurse into arguments before deciding the kind, so that
        // the argument snippets can be extracted directly.

        let callee_name = match &call.callee.kind {
            ExpressionKind::Identifier(identifier) => identifier.text.as_str(),
            _ => {
                // Not a named call — recurse and leave.
                self.analyze_expression(&call.callee);
                for arg in &call.arguments {
                    self.analyze_expression(arg);
                }
                return;
            }
        };

        match callee_name {
            "dist_uniform" => {
                let binding_name = self
                    .pending_binding
                    .take()
                    .unwrap_or_else(|| "<anonymous>".to_string());
                if call.arguments.len() == 2 {
                    let lo = expression_snippet(&call.arguments[0]);
                    let hi = expression_snippet(&call.arguments[1]);
                    self.distributions.push(DistributionSummary {
                        name: binding_name,
                        distribution_kind: DistributionKind::Uniform,
                        parameter_summaries: vec![lo.clone(), hi.clone()],
                        pdf_description: format!(
                            "Uniform[lo={lo}, hi={hi}]: pdf(x) = 1/(hi-lo) for x in [lo, hi]"
                        ),
                        span,
                    });
                } else {
                    self.errors.push(ProbabilisticError::new(
                        ProbabilisticErrorKind::InvalidDistribution,
                        format!(
                            "`dist_uniform` expects 2 arguments (lo, hi), found {}",
                            call.arguments.len()
                        ),
                        span,
                        true,
                    ));
                }
                for arg in &call.arguments {
                    self.analyze_expression(arg);
                }
            }
            "dist_bernoulli" => {
                let binding_name = self
                    .pending_binding
                    .take()
                    .unwrap_or_else(|| "<anonymous>".to_string());
                if call.arguments.len() == 1 {
                    let p = expression_snippet(&call.arguments[0]);
                    self.distributions.push(DistributionSummary {
                        name: binding_name,
                        distribution_kind: DistributionKind::Bernoulli,
                        parameter_summaries: vec![p.clone()],
                        pdf_description: format!(
                            "Bernoulli[p={p}]: P(1)=p, P(0)=1-p (symbolic)"
                        ),
                        span,
                    });
                } else {
                    self.errors.push(ProbabilisticError::new(
                        ProbabilisticErrorKind::InvalidDistribution,
                        format!(
                            "`dist_bernoulli` expects 1 argument (p), found {}",
                            call.arguments.len()
                        ),
                        span,
                        true,
                    ));
                }
                for arg in &call.arguments {
                    self.analyze_expression(arg);
                }
            }
            "dist_bayesian_update" => {
                let posterior_name = self
                    .pending_binding
                    .take()
                    .unwrap_or_else(|| "<anonymous>".to_string());
                if call.arguments.len() == 2 {
                    let prior_name = expression_snippet(&call.arguments[0]);
                    let likelihood_name = expression_snippet(&call.arguments[1]);
                    let update_description = format!(
                        "P({posterior_name}) ∝ P({likelihood_name}|{prior_name})"
                    );
                    self.bayesian_updates.push(BayesianUpdateSummary {
                        prior_name: prior_name.clone(),
                        likelihood_name: likelihood_name.clone(),
                        posterior_name: posterior_name.clone(),
                        update_description,
                        span,
                    });
                    // Also register the posterior as a Derived distribution.
                    self.distributions.push(DistributionSummary {
                        name: posterior_name,
                        distribution_kind: DistributionKind::Derived,
                        parameter_summaries: vec![
                            format!("prior={prior_name}"),
                            format!("likelihood={likelihood_name}"),
                        ],
                        pdf_description: format!(
                            "Derived[prior={prior_name}, likelihood={likelihood_name}]: posterior via Bayes' theorem"
                        ),
                        span,
                    });
                } else {
                    self.errors.push(ProbabilisticError::new(
                        ProbabilisticErrorKind::MalformedBayesianUpdate,
                        format!(
                            "`dist_bayesian_update` expects 2 arguments (prior, likelihood), found {}",
                            call.arguments.len()
                        ),
                        span,
                        true,
                    ));
                }
                for arg in &call.arguments {
                    self.analyze_expression(arg);
                }
            }
            "dist_sample" => {
                // Sampling is forbidden in symbolic mode.
                self.errors.push(ProbabilisticError::new(
                    ProbabilisticErrorKind::SamplingDetected,
                    "`dist_sample` is forbidden in symbolic probabilistic mode — use symbolic distribution operations only",
                    span,
                    false,
                ));
                for arg in &call.arguments {
                    self.analyze_expression(arg);
                }
            }
            _ => {
                // Not a probabilistic builtin; recurse normally.
                self.analyze_expression(&call.callee);
                for arg in &call.arguments {
                    self.analyze_expression(arg);
                }
            }
        }
    }
}

/// Returns a compact symbolic snippet for an expression.
fn expression_snippet(expression: &Expression) -> String {
    match &expression.kind {
        ExpressionKind::Identifier(identifier) => identifier.text.clone(),
        ExpressionKind::Integer(value) => value.clone(),
        ExpressionKind::String(value) => format!("\"{value}\""),
        ExpressionKind::Boolean(value) => value.to_string(),
        ExpressionKind::SelfReference(self_reference) => {
            format!("@{}()", self_reference.primitive.as_str())
        }
        ExpressionKind::Call(call) => {
            let callee = match &call.callee.kind {
                ExpressionKind::Identifier(identifier) => identifier.text.clone(),
                _ => "<expr>".to_string(),
            };
            format!(
                "{}({})",
                callee,
                call.arguments
                    .iter()
                    .map(expression_snippet)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
        ExpressionKind::Try(try_expression) => {
            format!("{}?", expression_snippet(&try_expression.operand))
        }
        ExpressionKind::Group(group) => expression_snippet(&group.expression),
        ExpressionKind::Block(_) => "{...}".to_string(),
        ExpressionKind::If(_) => "if ...".to_string(),
        ExpressionKind::Match(_) => "match ...".to_string(),
        ExpressionKind::Prefix(_) => "prefix ...".to_string(),
        ExpressionKind::Binary { .. } => "binary ...".to_string(),
    }
}
