//! Pattern matching exhaustiveness checker for LyraLang Stage 0.
//!
//! This module statically verifies that every `match` expression in a program
//! covers all reachable cases. It supports wildcard patterns, boolean literals,
//! integer literals, string literals, and identifier catch-all patterns.

pub mod error;

use serde::{Deserialize, Serialize};

use crate::lexer::SourceSpan;
use crate::parser::parse;
use crate::parser::{
    BlockExpression, Expression, ExpressionKind, MatchExpression, PatternKind, Program, Statement,
};
use crate::patterns::error::{PatternError, PatternErrorKind};

/// Per-match exhaustiveness report produced by the pattern checker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchExhaustivenessReport {
    /// Zero-based index of this match expression in program source order.
    pub match_index: usize,
    /// Number of arms in this match expression.
    pub arm_count: usize,
    /// Whether any arm uses a wildcard (`_`) or identifier catch-all pattern.
    pub has_wildcard: bool,
    /// Canonical summary string for each covered pattern.
    pub covered_patterns: Vec<String>,
    /// Whether this match expression is exhaustive.
    pub exhaustive: bool,
    /// Human-readable descriptions of any missing cases.
    pub missing_cases: Vec<String>,
    /// Source span of the match expression.
    pub span: SourceSpan,
}

/// Program-level exhaustiveness judgment for a fully checked program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatternProgramJudgment {
    /// Optional module name declared at the head of the program.
    pub module: Option<String>,
    /// One exhaustiveness report per match expression encountered in source order.
    pub match_expressions: Vec<MatchExhaustivenessReport>,
    /// True only when every match expression in the program is exhaustive.
    pub exhaustive: bool,
    /// Source span of the full program.
    pub span: SourceSpan,
}

/// Result bundle returned by the pattern checker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatternCheckOutput {
    /// Normalized source passed through earlier compiler stages.
    pub normalized_source: String,
    /// Program judgment when pattern checking succeeded.
    pub judgment: Option<PatternProgramJudgment>,
    /// Diagnostics emitted during checking.
    pub errors: Vec<PatternError>,
}

/// Deterministic pattern matching exhaustiveness checker.
#[derive(Debug, Clone, Default)]
pub struct PatternChecker;

impl PatternChecker {
    /// Creates a new pattern checker.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Parses and exhaustiveness-checks all match expressions in source text.
    #[must_use]
    pub fn check_source(&self, source: &str) -> PatternCheckOutput {
        let parse_output = parse(source);
        let normalized_source = parse_output.normalized_source.clone();

        if !parse_output.errors.is_empty() {
            let errors = parse_output
                .errors
                .into_iter()
                .map(|error| {
                    PatternError::new(
                        PatternErrorKind::ParseError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return PatternCheckOutput {
                normalized_source,
                judgment: None,
                errors,
            };
        }

        let program = match parse_output.program {
            Some(program) => program,
            None => {
                return PatternCheckOutput {
                    normalized_source,
                    judgment: None,
                    errors: vec![PatternError::new(
                        PatternErrorKind::ParseError,
                        "parser completed without a program AST",
                        SourceSpan::default(),
                        false,
                    )],
                };
            }
        };

        PatternAnalyzer::default().check_program(normalized_source, &program)
    }
}

/// Parses and exhaustiveness-checks source text with the default checker.
#[must_use]
pub fn check(source: &str) -> PatternCheckOutput {
    PatternChecker::new().check_source(source)
}

// ---------------------------------------------------------------------------
// Internal analyzer
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
struct PatternAnalyzer {
    reports: Vec<MatchExhaustivenessReport>,
    errors: Vec<PatternError>,
    match_counter: usize,
}

impl PatternAnalyzer {
    fn check_program(mut self, normalized_source: String, program: &Program) -> PatternCheckOutput {
        for statement in &program.statements {
            self.visit_statement(statement);
        }
        if let Some(tail) = &program.tail_expression {
            self.visit_expression(tail);
        }

        let all_exhaustive = self.reports.iter().all(|r| r.exhaustive);
        let judgment = PatternProgramJudgment {
            module: program.module_decl.as_ref().map(|d| d.name.text.clone()),
            match_expressions: self.reports,
            exhaustive: all_exhaustive,
            span: program.span,
        };

        PatternCheckOutput {
            normalized_source,
            judgment: Some(judgment),
            errors: self.errors,
        }
    }

    fn visit_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Let(let_stmt) => self.visit_expression(&let_stmt.value),
            Statement::Expr(expr_stmt) => self.visit_expression(&expr_stmt.expression),
        }
    }

    fn visit_expression(&mut self, expression: &Expression) {
        match &expression.kind {
            ExpressionKind::Match(match_expr) => {
                self.visit_expression(&match_expr.scrutinee);
                self.analyze_match(match_expr);
            }
            ExpressionKind::Block(block) => self.visit_block(block),
            ExpressionKind::If(if_expr) => {
                self.visit_expression(&if_expr.condition);
                self.visit_expression(&if_expr.then_branch);
                if let Some(else_branch) = &if_expr.else_branch {
                    self.visit_expression(else_branch);
                }
            }
            ExpressionKind::Group(group) => self.visit_expression(&group.expression),
            ExpressionKind::Try(try_expr) => self.visit_expression(&try_expr.operand),
            ExpressionKind::Prefix(prefix) => self.visit_expression(&prefix.operand),
            ExpressionKind::Binary { left, right, .. } => {
                self.visit_expression(left);
                self.visit_expression(right);
            }
            ExpressionKind::Call(call) => {
                self.visit_expression(&call.callee);
                for arg in &call.arguments {
                    self.visit_expression(arg);
                }
            }
            // Leaf expressions carry no sub-expressions to visit.
            ExpressionKind::Identifier(_)
            | ExpressionKind::Integer(_)
            | ExpressionKind::String(_)
            | ExpressionKind::Boolean(_)
            | ExpressionKind::SelfReference(_) => {}
        }
    }

    fn visit_block(&mut self, block: &BlockExpression) {
        for statement in &block.statements {
            self.visit_statement(statement);
        }
        if let Some(tail) = &block.tail_expression {
            self.visit_expression(tail);
        }
    }

    fn analyze_match(&mut self, match_expr: &MatchExpression) {
        let match_index = self.match_counter;
        self.match_counter += 1;

        let mut has_wildcard = false;
        let mut covered_bool_true = false;
        let mut covered_bool_false = false;
        let mut covered_patterns: Vec<String> = Vec::new();
        let mut seen_literals: Vec<String> = Vec::new();
        let mut duplicate_errors: Vec<PatternError> = Vec::new();
        let mut unreachable_errors: Vec<PatternError> = Vec::new();

        for arm in &match_expr.arms {
            // Visit the arm body for nested matches.
            self.visit_expression(&arm.body);

            let canonical = canonical_pattern(&arm.pattern.kind);

            // Check for wildcard / catch-all.
            let is_catch_all = matches!(
                &arm.pattern.kind,
                PatternKind::Wildcard | PatternKind::Identifier(_)
            );

            if has_wildcard {
                // Any arm after a wildcard is unreachable.
                unreachable_errors.push(PatternError::new(
                    PatternErrorKind::UnreachablePattern,
                    format!("pattern `{}` is unreachable after a catch-all arm", canonical),
                    arm.span,
                    true,
                ));
                continue;
            }

            // Detect duplicate literal patterns.
            if !is_catch_all {
                if seen_literals.contains(&canonical) {
                    duplicate_errors.push(PatternError::new(
                        PatternErrorKind::DuplicatePattern,
                        format!("duplicate pattern `{}` in match expression", canonical),
                        arm.span,
                        true,
                    ));
                    continue;
                }
                seen_literals.push(canonical.clone());
            }

            // Track bool coverage.
            if let PatternKind::Boolean(value) = &arm.pattern.kind {
                if *value {
                    covered_bool_true = true;
                } else {
                    covered_bool_false = true;
                }
            }

            if is_catch_all {
                has_wildcard = true;
            }

            covered_patterns.push(canonical);
        }

        self.errors.extend(duplicate_errors);
        self.errors.extend(unreachable_errors);

        // Determine exhaustiveness.
        let (exhaustive, missing_cases) =
            is_exhaustive(match_expr, has_wildcard, covered_bool_true, covered_bool_false);

        if !exhaustive {
            for missing in &missing_cases {
                self.errors.push(PatternError::new(
                    PatternErrorKind::NonExhaustiveMatch,
                    format!(
                        "non-exhaustive match expression: missing case `{}`",
                        missing
                    ),
                    match_expr.span,
                    false,
                ));
            }
        }

        self.reports.push(MatchExhaustivenessReport {
            match_index,
            arm_count: match_expr.arms.len(),
            has_wildcard,
            covered_patterns,
            exhaustive,
            missing_cases,
            span: match_expr.span,
        });
    }
}

/// Returns the canonical string representation of a pattern kind.
fn canonical_pattern(kind: &PatternKind) -> String {
    match kind {
        PatternKind::Wildcard => "_".to_owned(),
        PatternKind::Identifier(identifier) => identifier.text.clone(),
        PatternKind::Integer(value) => value.clone(),
        PatternKind::String(value) => format!("\"{}\"", value),
        PatternKind::Boolean(true) => "true".to_owned(),
        PatternKind::Boolean(false) => "false".to_owned(),
    }
}

/// Determines whether a match expression is exhaustive and returns any missing cases.
///
/// Logic:
/// - Wildcard / identifier catch-all → always exhaustive.
/// - Boolean scrutinee without catch-all → exhaustive iff both `true` and `false` are covered.
/// - Integer / string literal patterns without catch-all → never exhaustive (infinite domain).
/// - Identifier scrutinee → always exhaustive (treated as opaque value).
fn is_exhaustive(
    match_expr: &MatchExpression,
    has_wildcard: bool,
    covered_bool_true: bool,
    covered_bool_false: bool,
) -> (bool, Vec<String>) {
    if has_wildcard {
        return (true, Vec::new());
    }

    // Determine scrutinee class from the arm patterns.
    let has_bool_arm = match_expr
        .arms
        .iter()
        .any(|arm| matches!(&arm.pattern.kind, PatternKind::Boolean(_)));
    let has_int_arm = match_expr
        .arms
        .iter()
        .any(|arm| matches!(&arm.pattern.kind, PatternKind::Integer(_)));
    let has_string_arm = match_expr
        .arms
        .iter()
        .any(|arm| matches!(&arm.pattern.kind, PatternKind::String(_)));

    // Scrutinee expressed as a boolean: need true + false.
    if has_bool_arm && !has_int_arm && !has_string_arm {
        let mut missing = Vec::new();
        if !covered_bool_true {
            missing.push("true".to_owned());
        }
        if !covered_bool_false {
            missing.push("false".to_owned());
        }
        return (missing.is_empty(), missing);
    }

    // Integer or string domains are infinite — only exhaustive with a wildcard.
    if has_int_arm || has_string_arm {
        return (false, vec!["_".to_owned()]);
    }

    // Empty match or scrutinee is an opaque identifier → exhaustive by convention.
    (true, Vec::new())
}
