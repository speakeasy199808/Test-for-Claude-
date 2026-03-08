
//! Seed metaprogramming checker for LyraLang Stage 0 (P1-027).
//!
//! This module analyzes compile-time computations. In Stage 0 the recognized
//! surface forms are:
//!
//! - `meta_eval(expr)` — triggers compile-time evaluation (static folding).
//! - `quasi_quote(expr)` — creates a quoted AST value.
//! - `quasi_unquote(expr)` — splices a value back into a quasi-quoted context.
//!
//! The checker tracks quote-nesting depth; a `quasi_unquote` outside any
//! `quasi_quote` is reported as an `UnbalancedQuasiQuote` error.

use serde::{Deserialize, Serialize};

use crate::lexer::SourceSpan;
use crate::parser::parse;
use crate::parser::{CallExpression, Expression, ExpressionKind, Program, Statement};

// ── Public summary types ──────────────────────────────────────────────────────

/// A compile-time evaluation site discovered in the AST.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompileTimeEval {
    /// Zero-based index of this `meta_eval` call in program order.
    pub index: usize,
    /// Symbolic summary of the argument expression.
    pub input_summary: String,
    /// Result summary: `"static_{type}({val})"` or `"opaque"`.
    pub result_summary: String,
    /// `true` iff the argument was a compile-time constant.
    pub statically_determined: bool,
    /// Source span of the `meta_eval` call.
    pub span: SourceSpan,
}

/// A quasi-quotation site discovered in the AST.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuasiQuote {
    /// Zero-based index of this `quasi_quote` call in program order.
    pub index: usize,
    /// Canonical AST description of the quoted expression.
    pub quoted_form: String,
    /// Descriptions of `quasi_unquote` splice sites within.
    pub unquote_sites: Vec<String>,
    /// Source span of the `quasi_quote` call.
    pub span: SourceSpan,
}

/// An AST manipulation operation recorded during the walk.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstManipulation {
    /// The operation performed.
    pub operation: AstOp,
    /// Symbolic summary of the input expression.
    pub target_summary: String,
    /// Symbolic summary of the result.
    pub result_summary: String,
    /// Source span of the operation.
    pub span: SourceSpan,
}

/// AST manipulation operation kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AstOp {
    /// A `quasi_quote` wrapping operation.
    Quote,
    /// A `quasi_unquote` splice operation.
    Unquote,
    /// A splice-into-surrounding-quote operation.
    Splice,
    /// A `meta_eval` constant-folding operation.
    ConstFold,
}

impl AstOp {
    /// Returns a stable machine-readable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Quote => "quote",
            Self::Unquote => "unquote",
            Self::Splice => "splice",
            Self::ConstFold => "const_fold",
        }
    }
}

/// Metaprogramming judgment for a checked Stage 0 program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetaJudgment {
    /// Optional module name.
    pub module: Option<String>,
    /// All `meta_eval` sites in source order.
    pub compile_time_evals: Vec<CompileTimeEval>,
    /// All `quasi_quote` sites in source order.
    pub quasi_quotes: Vec<QuasiQuote>,
    /// All AST manipulation operations in source order.
    pub ast_manipulations: Vec<AstManipulation>,
    /// `true` iff every `meta_eval` argument was statically determined.
    pub all_static: bool,
    /// Source span for the full program.
    pub span: SourceSpan,
}

/// Categories of metaprogramming error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetaErrorKind {
    /// Parsing failed before metaprogramming analysis could proceed.
    ParseError,
    /// A `meta_eval` argument was not a compile-time constant.
    NonStaticMetaEval,
    /// A `quasi_unquote` appeared outside a surrounding `quasi_quote`.
    UnbalancedQuasiQuote,
    /// A cyclic meta-level dependency was detected.
    CyclicMetaDependency,
}

impl MetaErrorKind {
    /// Returns a stable machine-readable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::NonStaticMetaEval => "non_static_meta_eval",
            Self::UnbalancedQuasiQuote => "unbalanced_quasi_quote",
            Self::CyclicMetaDependency => "cyclic_meta_dependency",
        }
    }
}

/// A metaprogramming diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetaError {
    /// Error category.
    pub kind: MetaErrorKind,
    /// Human-readable diagnostic text.
    pub message: String,
    /// Source span associated with the error.
    pub span: SourceSpan,
    /// Whether analysis recovered and continued.
    pub recovered: bool,
}

impl MetaError {
    /// Creates a new metaprogramming diagnostic.
    #[must_use]
    pub fn new(
        kind: MetaErrorKind,
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

/// Result bundle returned by the seed metaprogramming checker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetaOutput {
    /// Normalized source passed through earlier compiler stages.
    pub normalized_source: String,
    /// Metaprogramming judgment when analysis succeeded.
    pub judgment: Option<MetaJudgment>,
    /// Diagnostics emitted during analysis.
    pub errors: Vec<MetaError>,
}

/// Deterministic seed metaprogramming checker.
#[derive(Debug, Clone, Default)]
pub struct MetaprogrammingChecker;

impl MetaprogrammingChecker {
    /// Creates a new metaprogramming checker.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Parses and analyzes source text for compile-time and quasiquote forms.
    #[must_use]
    pub fn check_source(&self, source: &str) -> MetaOutput {
        let parse_output = parse(source);
        let normalized_source = parse_output.normalized_source.clone();

        if !parse_output.errors.is_empty() {
            let errors = parse_output
                .errors
                .into_iter()
                .map(|error| {
                    MetaError::new(
                        MetaErrorKind::ParseError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return MetaOutput {
                normalized_source,
                judgment: None,
                errors,
            };
        }

        let Some(program) = parse_output.program else {
            return MetaOutput {
                normalized_source,
                judgment: None,
                errors: vec![MetaError::new(
                    MetaErrorKind::ParseError,
                    "parser completed without a program AST",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        let mut analyzer = MetaAnalyzer::new();
        analyzer.analyze_program(&program);

        let all_static = analyzer
            .compile_time_evals
            .iter()
            .all(|e| e.statically_determined);

        let errors = analyzer.errors.clone();
        let judgment = MetaJudgment {
            module: program
                .module_decl
                .as_ref()
                .map(|d| d.name.text.clone()),
            compile_time_evals: analyzer.compile_time_evals,
            quasi_quotes: analyzer.quasi_quotes,
            ast_manipulations: analyzer.ast_manipulations,
            all_static,
            span: program.span,
        };

        MetaOutput {
            normalized_source,
            judgment: Some(judgment),
            errors,
        }
    }
}

/// Parses and analyzes source text with the default checker.
#[must_use]
pub fn check(source: &str) -> MetaOutput {
    MetaprogrammingChecker::new().check_source(source)
}

// ── Internal analyzer ────────────────────────────────────────────────────────

/// Internal AST walker for metaprogramming analysis.
struct MetaAnalyzer {
    /// `meta_eval` sites in source order.
    compile_time_evals: Vec<CompileTimeEval>,
    /// `quasi_quote` sites in source order.
    quasi_quotes: Vec<QuasiQuote>,
    /// AST manipulation records in source order.
    ast_manipulations: Vec<AstManipulation>,
    /// Diagnostics accumulated during the walk.
    errors: Vec<MetaError>,
    /// Zero-based counter for `meta_eval` sites.
    eval_counter: usize,
    /// Zero-based counter for `quasi_quote` sites.
    quote_counter: usize,
    /// Current quasi-quotation nesting depth.
    quote_depth: usize,
    /// Unquote sites collected for the innermost quasi_quote.
    pending_unquote_sites: Vec<String>,
}

impl MetaAnalyzer {
    fn new() -> Self {
        Self {
            compile_time_evals: Vec::new(),
            quasi_quotes: Vec::new(),
            ast_manipulations: Vec::new(),
            errors: Vec::new(),
            eval_counter: 0,
            quote_counter: 0,
            quote_depth: 0,
            pending_unquote_sites: Vec::new(),
        }
    }

    fn analyze_program(&mut self, program: &Program) {
        for statement in &program.statements {
            self.analyze_statement(statement);
        }
        if let Some(tail) = &program.tail_expression {
            self.analyze_expression(tail);
        }
    }

    fn analyze_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Let(s) => self.analyze_expression(&s.value),
            Statement::Expr(s) => self.analyze_expression(&s.expression),
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
            ExpressionKind::Try(try_expr) => self.analyze_expression(&try_expr.operand),
            ExpressionKind::Prefix(prefix) => self.analyze_expression(&prefix.operand),
            ExpressionKind::Binary { left, right, .. } => {
                self.analyze_expression(left);
                self.analyze_expression(right);
            }
            ExpressionKind::Block(block) => {
                for s in &block.statements {
                    self.analyze_statement(s);
                }
                if let Some(tail) = &block.tail_expression {
                    self.analyze_expression(tail);
                }
            }
            ExpressionKind::If(if_expr) => {
                self.analyze_expression(&if_expr.condition);
                self.analyze_expression(&if_expr.then_branch);
                if let Some(else_branch) = &if_expr.else_branch {
                    self.analyze_expression(else_branch);
                }
            }
            ExpressionKind::Match(match_expr) => {
                self.analyze_expression(&match_expr.scrutinee);
                for arm in &match_expr.arms {
                    self.analyze_expression(&arm.body);
                }
            }
            ExpressionKind::Call(call) => self.analyze_call(call, expression.span),
        }
    }

    fn analyze_call(&mut self, call: &CallExpression, span: SourceSpan) {
        let callee_name = call_name(&call.callee);

        match callee_name {
            Some("meta_eval") => self.handle_meta_eval(call, span),
            Some("quasi_quote") => self.handle_quasi_quote(call, span),
            Some("quasi_unquote") => self.handle_quasi_unquote(call, span),
            _ => {
                self.analyze_expression(&call.callee);
                for arg in &call.arguments {
                    self.analyze_expression(arg);
                }
            }
        }
    }

    fn handle_meta_eval(&mut self, call: &CallExpression, span: SourceSpan) {
        let index = self.eval_counter;
        self.eval_counter += 1;

        if call.arguments.is_empty() {
            self.errors.push(MetaError::new(
                MetaErrorKind::NonStaticMetaEval,
                "`meta_eval` expects 1 argument",
                span,
                true,
            ));
            return;
        }

        let arg = &call.arguments[0];
        let input_summary = expression_snippet(arg);

        // Determine if statically evaluable.
        let (statically_determined, result_summary) = static_eval(arg);

        if !statically_determined {
            self.errors.push(MetaError::new(
                MetaErrorKind::NonStaticMetaEval,
                format!(
                    "`meta_eval` argument `{input_summary}` is not a compile-time constant"
                ),
                span,
                true,
            ));
        }

        self.compile_time_evals.push(CompileTimeEval {
            index,
            input_summary: input_summary.clone(),
            result_summary: result_summary.clone(),
            statically_determined,
            span,
        });

        self.ast_manipulations.push(AstManipulation {
            operation: AstOp::ConstFold,
            target_summary: input_summary,
            result_summary,
            span,
        });

        // Recurse into argument.
        for arg in &call.arguments {
            self.analyze_expression(arg);
        }
    }

    fn handle_quasi_quote(&mut self, call: &CallExpression, span: SourceSpan) {
        let index = self.quote_counter;
        self.quote_counter += 1;

        if call.arguments.is_empty() {
            self.errors.push(MetaError::new(
                MetaErrorKind::UnbalancedQuasiQuote,
                "`quasi_quote` expects 1 argument",
                span,
                true,
            ));
            return;
        }

        let arg = &call.arguments[0];
        let quoted_form = expression_snippet(arg);

        // Enter quasi-quote depth.
        self.quote_depth += 1;
        let unquote_start = self.pending_unquote_sites.len();

        self.analyze_expression(arg);

        self.quote_depth -= 1;

        // Collect unquote sites gathered during this quote level.
        let unquote_sites: Vec<String> = self
            .pending_unquote_sites
            .drain(unquote_start..)
            .collect();

        self.quasi_quotes.push(QuasiQuote {
            index,
            quoted_form: quoted_form.clone(),
            unquote_sites,
            span,
        });

        self.ast_manipulations.push(AstManipulation {
            operation: AstOp::Quote,
            target_summary: quoted_form.clone(),
            result_summary: format!("quoted({quoted_form})"),
            span,
        });
    }

    fn handle_quasi_unquote(&mut self, call: &CallExpression, span: SourceSpan) {
        if self.quote_depth == 0 {
            self.errors.push(MetaError::new(
                MetaErrorKind::UnbalancedQuasiQuote,
                "`quasi_unquote` used outside of a `quasi_quote` context",
                span,
                true,
            ));
        }

        if call.arguments.is_empty() {
            self.errors.push(MetaError::new(
                MetaErrorKind::UnbalancedQuasiQuote,
                "`quasi_unquote` expects 1 argument",
                span,
                true,
            ));
            return;
        }

        let arg = &call.arguments[0];
        let site_summary = expression_snippet(arg);

        self.pending_unquote_sites.push(site_summary.clone());

        self.ast_manipulations.push(AstManipulation {
            operation: AstOp::Unquote,
            target_summary: site_summary.clone(),
            result_summary: format!("unquoted({site_summary})"),
            span,
        });

        for arg in &call.arguments {
            self.analyze_expression(arg);
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn call_name(expression: &Expression) -> Option<&str> {
    match &expression.kind {
        ExpressionKind::Identifier(identifier) => Some(identifier.text.as_str()),
        ExpressionKind::Group(group) => call_name(&group.expression),
        _ => None,
    }
}

/// Returns a compact symbolic snippet for an expression.
fn expression_snippet(expression: &Expression) -> String {
    match &expression.kind {
        ExpressionKind::Identifier(identifier) => identifier.text.clone(),
        ExpressionKind::Integer(value) => value.clone(),
        ExpressionKind::String(value) => format!("{value:?}"),
        ExpressionKind::Boolean(value) => value.to_string(),
        ExpressionKind::SelfReference(sr) => format!("@{}()", sr.primitive.as_str()),
        ExpressionKind::Call(call) => {
            let callee = call_name(&call.callee)
                .map(ToString::to_string)
                .unwrap_or_else(|| "<expr>".to_string());
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
        ExpressionKind::Try(try_expr) => format!("{}?", expression_snippet(&try_expr.operand)),
        ExpressionKind::Group(group) => expression_snippet(&group.expression),
        ExpressionKind::Block(_) => "{...}".to_string(),
        ExpressionKind::If(_) => "if ...".to_string(),
        ExpressionKind::Match(_) => "match ...".to_string(),
        ExpressionKind::Prefix(_) => "prefix ...".to_string(),
        ExpressionKind::Binary { .. } => "binary ...".to_string(),
    }
}

/// Attempt to statically evaluate an expression.
///
/// Returns `(statically_determined, result_summary)`.
fn static_eval(expression: &Expression) -> (bool, String) {
    match &expression.kind {
        ExpressionKind::Integer(value) => (true, format!("static_int({value})")),
        ExpressionKind::Boolean(value) => (true, format!("static_bool({value})")),
        ExpressionKind::String(value) => (true, format!("static_string({value:?})")),
        ExpressionKind::Group(group) => static_eval(&group.expression),
        _ => (false, "opaque".to_string()),
    }
}
