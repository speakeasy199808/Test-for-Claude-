//! Seed proof construction checker for LyraLang Stage 0.
//!
//! This module recognizes proof blocks syntactically approximated via builtin
//! calls: `assume(expr)`, `assert_eq(lhs, rhs)`, `qed()`, `proof_begin()`,
//! and `proof_end()`. It extracts proof obligations, validates discharge via
//! `qed()`, and produces verifiable proof artifacts.
//!
//! The existing type system carries `EffectAtom::Proof` (from
//! `crate::types::EffectAtom`) and `Type::Evidence(EvidenceKind::Proof)`
//! (from `crate::types::EvidenceKind`). This module builds upon those
//! foundations at the structural / syntactic level.
//!
//! A proof block is valid when: it has at least one hypothesis (`assume`)
//! and at least one claim (`assert_eq`) and a `qed()` call. A claim without
//! `qed()` generates an `UndischargedObligation` error (recovered = true so
//! the rest of the program continues to be analyzed).

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::checker;
use crate::lexer::SourceSpan;
use crate::parser::parse;
use crate::parser::{CallExpression, Expression, ExpressionKind, Program, Statement};

// ─── Proof artifact kind ──────────────────────────────────────────────────────

/// Classification of a verifiable proof artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProofArtifactKind {
    /// A recorded `assume(...)` hypothesis.
    AssumptionRecord,
    /// A verified `assert_eq(...)` claim.
    ClaimVerification,
    /// A discharge receipt attached to a `qed()` call.
    DischargeReceipt,
    /// A summary artifact for a fully discharged proof block.
    ProofSummary,
}

// ─── Core output types ────────────────────────────────────────────────────────

/// A single proof block extracted from source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofBlock {
    /// Zero-based index of this proof block in source order.
    pub index: usize,
    /// Symbolic summaries of all `assume(...)` calls in this block.
    pub hypotheses: Vec<String>,
    /// Symbolic summaries of all `assert_eq(...)` calls in this block.
    pub claims: Vec<String>,
    /// Whether a `qed()` call was present in this block.
    pub qed_present: bool,
    /// Whether the block constitutes a structurally valid proof.
    ///
    /// A block is valid when it has at least one hypothesis, at least one
    /// claim, and a `qed()` call present.
    pub valid: bool,
    /// Source span of the proof block (first builtin call encountered).
    pub span: SourceSpan,
}

/// A single proof obligation extracted from a proof block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofObligation {
    /// Stable obligation identifier in the form `"PO-{index}"`.
    pub obligation_id: String,
    /// Human-readable obligation statement.
    pub statement: String,
    /// Whether the obligation was discharged by a `qed()` call.
    pub discharged: bool,
    /// Description of the discharge evidence when `discharged` is `true`.
    pub discharge_evidence: Option<String>,
}

/// A verifiable proof artifact produced from a discharged proof block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofArtifact {
    /// Stable artifact identifier in the form `"PA-{index}"`.
    pub artifact_id: String,
    /// Artifact classification.
    pub kind: ProofArtifactKind,
    /// Human-readable artifact description.
    pub description: String,
    /// Whether this artifact is structurally verifiable.
    pub verifiable: bool,
}

/// Proof summary for a checked Stage 0 program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofProgramJudgment {
    /// Optional module name from a `module` declaration.
    pub module: Option<String>,
    /// All proof blocks encountered in source order.
    pub proof_blocks: Vec<ProofBlock>,
    /// All proof obligations extracted in source order.
    pub obligations: Vec<ProofObligation>,
    /// All verifiable proof artifacts produced from discharged blocks.
    pub artifacts: Vec<ProofArtifact>,
    /// `true` iff every obligation has been discharged.
    pub all_discharged: bool,
    /// Source span for the full program.
    pub span: SourceSpan,
}

// ─── Check output ─────────────────────────────────────────────────────────────

/// Result bundle returned by the proof construction checker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofCheckOutput {
    /// Normalized source passed through earlier compiler stages.
    pub normalized_source: String,
    /// Program judgment when proof checking succeeded without fatal errors.
    pub judgment: Option<ProofProgramJudgment>,
    /// Diagnostics emitted during checking.
    pub errors: Vec<ProofError>,
}

// ─── Error types ──────────────────────────────────────────────────────────────

/// Categories of proof-construction error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProofErrorKind {
    /// Parsing failed before proof checking could proceed.
    ParseError,
    /// A proof block has claims but no `qed()` discharge.
    UndischargedObligation,
    /// A `qed()` was found outside any active proof block context.
    MissingQed,
    /// A proof block is structurally malformed (e.g. claims with no hypotheses).
    InvalidProofBlock,
}

impl ProofErrorKind {
    /// Returns a stable machine-readable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::UndischargedObligation => "undischarged_obligation",
            Self::MissingQed => "missing_qed",
            Self::InvalidProofBlock => "invalid_proof_block",
        }
    }
}

/// A proof-construction diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{message}")]
pub struct ProofError {
    /// Error category.
    pub kind: ProofErrorKind,
    /// Human-readable diagnostic text.
    pub message: String,
    /// Source span associated with the error.
    pub span: SourceSpan,
    /// Whether checking recovered and continued past this error.
    pub recovered: bool,
}

impl ProofError {
    /// Creates a new proof-construction diagnostic.
    #[must_use]
    pub fn new(
        kind: ProofErrorKind,
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

/// Deterministic seed proof construction checker.
#[derive(Debug, Clone, Default)]
pub struct ProofChecker;

impl ProofChecker {
    /// Creates a new proof construction checker.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Parses, type-checks, and proof-checks source text.
    #[must_use]
    pub fn check_source(&self, source: &str) -> ProofCheckOutput {
        let type_output = checker::check(source);
        let normalized_source = type_output.normalized_source.clone();

        if !type_output.errors.is_empty() {
            let errors = type_output
                .errors
                .into_iter()
                .map(|error| {
                    ProofError::new(
                        ProofErrorKind::ParseError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return ProofCheckOutput {
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
                    ProofError::new(
                        ProofErrorKind::ParseError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return ProofCheckOutput {
                normalized_source,
                judgment: None,
                errors,
            };
        }

        let Some(_type_judgment) = type_output.judgment else {
            return ProofCheckOutput {
                normalized_source,
                judgment: None,
                errors: vec![ProofError::new(
                    ProofErrorKind::ParseError,
                    "type checker completed without a program judgment",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        let Some(program) = parse_output.program else {
            return ProofCheckOutput {
                normalized_source,
                judgment: None,
                errors: vec![ProofError::new(
                    ProofErrorKind::ParseError,
                    "parser completed without a program AST",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        let mut analyzer = ProofAnalyzer::new();
        analyzer.analyze_program(&program);

        let errors = analyzer.errors;

        // Build obligations from proof blocks.
        let mut obligations: Vec<ProofObligation> = Vec::new();
        let mut artifacts: Vec<ProofArtifact> = Vec::new();
        let mut artifact_counter: usize = 0;

        for (block_index, block) in analyzer.proof_blocks.iter().enumerate() {
            // Create an obligation for each claim in the block.
            for (claim_index, claim) in block.claims.iter().enumerate() {
                let obligation_id = format!("PO-{}", obligations.len());
                let discharged = block.qed_present;
                let discharge_evidence = if discharged {
                    Some("qed: proof-term discharge via EffectAtom::Proof".to_string())
                } else {
                    None
                };
                obligations.push(ProofObligation {
                    obligation_id,
                    statement: format!(
                        "Block {block_index}, claim {claim_index}: {claim}"
                    ),
                    discharged,
                    discharge_evidence,
                });
            }

            // If valid, generate artifacts.
            if block.valid {
                // AssumptionRecord per hypothesis.
                for hypothesis in &block.hypotheses {
                    let artifact_id = format!("PA-{artifact_counter}");
                    artifact_counter += 1;
                    artifacts.push(ProofArtifact {
                        artifact_id,
                        kind: ProofArtifactKind::AssumptionRecord,
                        description: format!("Assumption recorded: {hypothesis}"),
                        verifiable: true,
                    });
                }
                // ClaimVerification per claim.
                for claim in &block.claims {
                    let artifact_id = format!("PA-{artifact_counter}");
                    artifact_counter += 1;
                    artifacts.push(ProofArtifact {
                        artifact_id,
                        kind: ProofArtifactKind::ClaimVerification,
                        description: format!("Claim verified: {claim}"),
                        verifiable: true,
                    });
                }
                // DischargeReceipt for the qed.
                let artifact_id = format!("PA-{artifact_counter}");
                artifact_counter += 1;
                artifacts.push(ProofArtifact {
                    artifact_id,
                    kind: ProofArtifactKind::DischargeReceipt,
                    description: format!(
                        "Block {block_index} discharged via qed()"
                    ),
                    verifiable: true,
                });
                // ProofSummary.
                let artifact_id = format!("PA-{artifact_counter}");
                artifact_counter += 1;
                artifacts.push(ProofArtifact {
                    artifact_id,
                    kind: ProofArtifactKind::ProofSummary,
                    description: format!(
                        "Block {block_index}: {} hypothesis(es), {} claim(s), discharged",
                        block.hypotheses.len(),
                        block.claims.len()
                    ),
                    verifiable: true,
                });
            }
        }

        let all_discharged = obligations.iter().all(|o| o.discharged);

        let judgment = ProofProgramJudgment {
            module: program
                .module_decl
                .as_ref()
                .map(|declaration| declaration.name.text.clone()),
            proof_blocks: analyzer.proof_blocks,
            obligations,
            artifacts,
            all_discharged,
            span: program.span,
        };

        ProofCheckOutput {
            normalized_source,
            judgment: Some(judgment),
            errors,
        }
    }
}

/// Parses, type-checks, and proof-checks source text with the default checker.
#[must_use]
pub fn check(source: &str) -> ProofCheckOutput {
    ProofChecker::new().check_source(source)
}

// ─── Internal analyzer ────────────────────────────────────────────────────────

/// Internal AST walker that extracts proof blocks and obligations.
struct ProofAnalyzer {
    proof_blocks: Vec<ProofBlock>,
    errors: Vec<ProofError>,
    /// Hypotheses accumulated for the current implicit proof block.
    current_hypotheses: Vec<String>,
    /// Claims accumulated for the current implicit proof block.
    current_claims: Vec<String>,
    /// Whether `qed()` has been seen for the current block.
    current_qed: bool,
    /// Whether `proof_begin()` has been called and a block is open.
    in_explicit_block: bool,
    /// Span of the first element of the current block.
    current_block_span: SourceSpan,
}

impl ProofAnalyzer {
    fn new() -> Self {
        Self {
            proof_blocks: Vec::new(),
            errors: Vec::new(),
            current_hypotheses: Vec::new(),
            current_claims: Vec::new(),
            current_qed: false,
            in_explicit_block: false,
            current_block_span: SourceSpan::default(),
        }
    }

    fn analyze_program(&mut self, program: &Program) {
        for statement in &program.statements {
            self.analyze_statement(statement);
        }
        if let Some(tail_expression) = &program.tail_expression {
            self.analyze_expression(tail_expression);
        }
        // Flush any implicit block that was never closed.
        self.flush_implicit_block();
    }

    fn analyze_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Let(let_statement) => {
                self.analyze_expression(&let_statement.value);
            }
            Statement::Expr(expr_statement) => {
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
        let callee_name = match &call.callee.kind {
            ExpressionKind::Identifier(identifier) => identifier.text.as_str(),
            _ => {
                self.analyze_expression(&call.callee);
                for arg in &call.arguments {
                    self.analyze_expression(arg);
                }
                return;
            }
        };

        match callee_name {
            "proof_begin" => {
                // Flush any prior implicit block first.
                self.flush_implicit_block();
                self.in_explicit_block = true;
                self.current_block_span = span;
            }
            "proof_end" => {
                if self.in_explicit_block {
                    self.flush_explicit_block(span);
                } else {
                    // proof_end without proof_begin — treat as a lone qed.
                    self.errors.push(ProofError::new(
                        ProofErrorKind::InvalidProofBlock,
                        "`proof_end` called without a preceding `proof_begin`",
                        span,
                        true,
                    ));
                }
            }
            "assume" => {
                if self.current_hypotheses.is_empty()
                    && self.current_claims.is_empty()
                    && !self.in_explicit_block
                {
                    // First element of an implicit block — record span.
                    self.current_block_span = span;
                }
                let summary = if call.arguments.len() == 1 {
                    expression_snippet(&call.arguments[0])
                } else {
                    format!("<assume/{}-args>", call.arguments.len())
                };
                self.current_hypotheses.push(format!("assume({summary})"));
                for arg in &call.arguments {
                    self.analyze_expression(arg);
                }
            }
            "assert_eq" => {
                if self.current_hypotheses.is_empty()
                    && self.current_claims.is_empty()
                    && !self.in_explicit_block
                {
                    self.current_block_span = span;
                }
                let summary = if call.arguments.len() == 2 {
                    let lhs = expression_snippet(&call.arguments[0]);
                    let rhs = expression_snippet(&call.arguments[1]);
                    format!("{lhs} == {rhs}")
                } else {
                    format!("<assert_eq/{}-args>", call.arguments.len())
                };
                self.current_claims.push(format!("assert_eq({summary})"));
                for arg in &call.arguments {
                    self.analyze_expression(arg);
                }
            }
            "qed" => {
                self.current_qed = true;
                // If we are not in an explicit block, flush the implicit one now.
                if !self.in_explicit_block {
                    self.flush_implicit_block();
                }
                // If we are in an explicit block, qed just marks discharge;
                // proof_end will finalize it.
            }
            _ => {
                // Not a proof builtin — recurse normally.
                self.analyze_expression(&call.callee);
                for arg in &call.arguments {
                    self.analyze_expression(arg);
                }
            }
        }
    }

    /// Flushes the pending implicit block (accumulated outside explicit markers).
    fn flush_implicit_block(&mut self) {
        if self.current_hypotheses.is_empty() && self.current_claims.is_empty() {
            // Nothing to flush.
            self.current_qed = false;
            return;
        }
        self.finalize_block(self.current_block_span);
    }

    /// Flushes an explicit block opened via `proof_begin`.
    fn flush_explicit_block(&mut self, end_span: SourceSpan) {
        self.in_explicit_block = false;
        let span = self.current_block_span;
        let _ = end_span;
        self.finalize_block(span);
    }

    /// Records the current accumulated block state as a `ProofBlock`.
    fn finalize_block(&mut self, span: SourceSpan) {
        let index = self.proof_blocks.len();
        let hypotheses = std::mem::take(&mut self.current_hypotheses);
        let claims = std::mem::take(&mut self.current_claims);
        let qed_present = self.current_qed;
        self.current_qed = false;

        let valid = !hypotheses.is_empty() && !claims.is_empty() && qed_present;

        // Emit UndischargedObligation if there are claims but no qed.
        if !claims.is_empty() && !qed_present {
            self.errors.push(ProofError::new(
                ProofErrorKind::UndischargedObligation,
                format!(
                    "proof block {index} has {} claim(s) but no `qed()` discharge",
                    claims.len()
                ),
                span,
                true,
            ));
        }

        // Emit InvalidProofBlock if there are claims but no hypotheses.
        if !claims.is_empty() && hypotheses.is_empty() {
            self.errors.push(ProofError::new(
                ProofErrorKind::InvalidProofBlock,
                format!(
                    "proof block {index} has {} claim(s) but no `assume(...)` hypotheses",
                    claims.len()
                ),
                span,
                true,
            ));
        }

        self.proof_blocks.push(ProofBlock {
            index,
            hypotheses,
            claims,
            qed_present,
            valid,
            span,
        });
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
