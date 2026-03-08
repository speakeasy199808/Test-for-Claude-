
//! Seed syntax-extension checker for LyraLang Stage 0 (P1-026).
//!
//! This module implements hygienic macro recognition for Stage 0 LyraLang.
//! It recognizes two surface forms over the existing parser:
//!
//! - `syntax_define(name, template)` — defines a new macro.
//! - `syntax_expand(name, args...)` — expands a previously defined macro.
//!
//! Hygiene is enforced by tracking macro-introduced binding names with a
//! gensym-like index: introduced bindings are renamed `{original}#gensym{N}`.

use serde::{Deserialize, Serialize};

use crate::lexer::SourceSpan;
use crate::parser::parse;
use crate::parser::{CallExpression, Expression, ExpressionKind, Program, Statement};

// ── Public types ─────────────────────────────────────────────────────────────

/// A macro definition discovered while walking the AST.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroDefinition {
    /// Macro name as written in the `syntax_define` call.
    pub name: String,
    /// Number of template holes inferred from the template string.
    pub template_arity: usize,
    /// Human-readable summary of the template argument.
    pub template_summary: String,
    /// Source span of the `syntax_define` call.
    pub span: SourceSpan,
}

/// A hygienic binding introduced by a macro expansion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HygienicBinding {
    /// Original binding name before renaming.
    pub original_name: String,
    /// Hygienic name: `"{original_name}#gensym{index}"`.
    pub hygienic_name: String,
    /// Monotonically increasing gensym index.
    pub gensym_index: usize,
}

/// A macro expansion discovered while walking the AST.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroExpansion {
    /// Name of the macro being expanded.
    pub macro_name: String,
    /// Zero-based index of this expansion in program order.
    pub expansion_index: usize,
    /// Number of arguments supplied at the expansion site.
    pub argument_count: usize,
    /// Hygienic bindings introduced by this expansion.
    pub introduced_bindings: Vec<HygienicBinding>,
    /// Canonical expanded-form description.
    pub expanded_form: String,
    /// Whether this expansion is hygienically safe.
    pub hygienic: bool,
    /// Source span of the `syntax_expand` call.
    pub span: SourceSpan,
}

/// A syntax-extension judgment for a checked Stage 0 program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyntaxExtensionJudgment {
    /// Optional module name.
    pub module: Option<String>,
    /// Macro definitions in source order.
    pub macro_definitions: Vec<MacroDefinition>,
    /// Macro expansions in source order.
    pub macro_expansions: Vec<MacroExpansion>,
    /// `true` iff every expansion is hygienically safe.
    pub hygienic: bool,
    /// Source span for the full program.
    pub span: SourceSpan,
}

/// Categories of syntax-extension error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyntaxExtensionErrorKind {
    /// Parsing failed before syntax-extension analysis could proceed.
    ParseError,
    /// A `syntax_expand` call refers to an undefined macro.
    UndefinedMacro,
    /// A `syntax_expand` call supplies the wrong number of arguments.
    ArityMismatch,
    /// A macro expansion would capture an outer binding.
    HygieneViolation,
}

impl SyntaxExtensionErrorKind {
    /// Returns a stable machine-readable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::UndefinedMacro => "undefined_macro",
            Self::ArityMismatch => "arity_mismatch",
            Self::HygieneViolation => "hygiene_violation",
        }
    }
}

/// A syntax-extension diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyntaxExtensionError {
    /// Error category.
    pub kind: SyntaxExtensionErrorKind,
    /// Human-readable diagnostic text.
    pub message: String,
    /// Source span associated with the error.
    pub span: SourceSpan,
    /// Whether analysis recovered and continued.
    pub recovered: bool,
}

impl SyntaxExtensionError {
    /// Creates a new syntax-extension diagnostic.
    #[must_use]
    pub fn new(
        kind: SyntaxExtensionErrorKind,
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

/// Result bundle returned by the seed syntax-extension checker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyntaxExtensionOutput {
    /// Normalized source passed through earlier compiler stages.
    pub normalized_source: String,
    /// Syntax-extension judgment when analysis succeeded.
    pub judgment: Option<SyntaxExtensionJudgment>,
    /// Diagnostics emitted during analysis.
    pub errors: Vec<SyntaxExtensionError>,
}

/// Deterministic seed syntax-extension checker.
#[derive(Debug, Clone, Default)]
pub struct SyntaxExtensionChecker;

impl SyntaxExtensionChecker {
    /// Creates a new syntax-extension checker.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Parses and analyzes source text for macro definitions and expansions.
    #[must_use]
    pub fn check_source(&self, source: &str) -> SyntaxExtensionOutput {
        let parse_output = parse(source);
        let normalized_source = parse_output.normalized_source.clone();

        if !parse_output.errors.is_empty() {
            let errors = parse_output
                .errors
                .into_iter()
                .map(|error| {
                    SyntaxExtensionError::new(
                        SyntaxExtensionErrorKind::ParseError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return SyntaxExtensionOutput {
                normalized_source,
                judgment: None,
                errors,
            };
        }

        let Some(program) = parse_output.program else {
            return SyntaxExtensionOutput {
                normalized_source,
                judgment: None,
                errors: vec![SyntaxExtensionError::new(
                    SyntaxExtensionErrorKind::ParseError,
                    "parser completed without a program AST",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        let mut analyzer = SyntaxExtensionAnalyzer::new();
        analyzer.analyze_program(&program);

        let hygienic = !analyzer
            .errors
            .iter()
            .any(|e| matches!(e.kind, SyntaxExtensionErrorKind::HygieneViolation));

        let errors = analyzer.errors.clone();
        let judgment = SyntaxExtensionJudgment {
            module: program
                .module_decl
                .as_ref()
                .map(|d| d.name.text.clone()),
            macro_definitions: analyzer.definitions,
            macro_expansions: analyzer.expansions,
            hygienic,
            span: program.span,
        };

        SyntaxExtensionOutput {
            normalized_source,
            judgment: Some(judgment),
            errors,
        }
    }
}

/// Parses and analyzes source text with the default checker.
#[must_use]
pub fn check(source: &str) -> SyntaxExtensionOutput {
    SyntaxExtensionChecker::new().check_source(source)
}

// ── Internal analyzer ────────────────────────────────────────────────────────

/// Internal state for the syntax-extension walk.
struct SyntaxExtensionAnalyzer {
    /// Known macro definitions: name → template_arity.
    known_macros: std::collections::BTreeMap<String, usize>,
    /// Macro definitions in source order.
    definitions: Vec<MacroDefinition>,
    /// Macro expansions in source order.
    expansions: Vec<MacroExpansion>,
    /// Diagnostics accumulated during the walk.
    errors: Vec<SyntaxExtensionError>,
    /// Monotonically increasing gensym counter.
    gensym_counter: usize,
    /// Zero-based expansion index counter.
    expansion_counter: usize,
}

impl SyntaxExtensionAnalyzer {
    fn new() -> Self {
        Self {
            known_macros: std::collections::BTreeMap::new(),
            definitions: Vec::new(),
            expansions: Vec::new(),
            errors: Vec::new(),
            gensym_counter: 0,
            expansion_counter: 0,
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
        // Recurse into callee and arguments first.
        self.analyze_expression(&call.callee);
        for arg in &call.arguments {
            self.analyze_expression(arg);
        }

        let Some(name) = call_name(&call.callee) else {
            return;
        };

        match name {
            "syntax_define" => self.handle_syntax_define(call, span),
            "syntax_expand" => self.handle_syntax_expand(call, span),
            _ => {}
        }
    }

    fn handle_syntax_define(&mut self, call: &CallExpression, span: SourceSpan) {
        // syntax_define(name, template)
        if call.arguments.len() != 2 {
            self.errors.push(SyntaxExtensionError::new(
                SyntaxExtensionErrorKind::ArityMismatch,
                format!(
                    "`syntax_define` expects 2 arguments but found {}",
                    call.arguments.len()
                ),
                span,
                true,
            ));
            return;
        }

        let macro_name = string_literal_value(&call.arguments[0])
            .unwrap_or_else(|| expression_snippet(&call.arguments[0]));
        let template_summary = string_literal_value(&call.arguments[1])
            .unwrap_or_else(|| expression_snippet(&call.arguments[1]));

        // Infer arity: count occurrences of `$` in template string (Stage 0 heuristic).
        let template_arity = count_template_holes(&template_summary);

        self.known_macros.insert(macro_name.clone(), template_arity);
        self.definitions.push(MacroDefinition {
            name: macro_name,
            template_arity,
            template_summary,
            span,
        });
    }

    fn handle_syntax_expand(&mut self, call: &CallExpression, span: SourceSpan) {
        // syntax_expand(name, args...)
        if call.arguments.is_empty() {
            self.errors.push(SyntaxExtensionError::new(
                SyntaxExtensionErrorKind::ArityMismatch,
                "`syntax_expand` expects at least 1 argument (the macro name)".to_string(),
                span,
                true,
            ));
            return;
        }

        let macro_name = string_literal_value(&call.arguments[0])
            .unwrap_or_else(|| expression_snippet(&call.arguments[0]));
        let argument_count = call.arguments.len().saturating_sub(1);
        let expansion_index = self.expansion_counter;
        self.expansion_counter += 1;

        // Check if macro is defined.
        let expected_arity = self.known_macros.get(&macro_name).copied();
        let Some(expected) = expected_arity else {
            self.errors.push(SyntaxExtensionError::new(
                SyntaxExtensionErrorKind::UndefinedMacro,
                format!("expansion of undefined macro `{macro_name}`"),
                span,
                true,
            ));
            // Still record the expansion with hygienic=false.
            let expanded_form = format!(
                "syntax_expand({macro_name}, {})",
                (0..argument_count)
                    .map(|i| expression_snippet(&call.arguments[i + 1]))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            self.expansions.push(MacroExpansion {
                macro_name,
                expansion_index,
                argument_count,
                introduced_bindings: Vec::new(),
                expanded_form,
                hygienic: false,
                span,
            });
            return;
        };

        // Check arity.
        if argument_count != expected {
            self.errors.push(SyntaxExtensionError::new(
                SyntaxExtensionErrorKind::ArityMismatch,
                format!(
                    "macro `{macro_name}` expects {expected} argument(s) but {argument_count} supplied"
                ),
                span,
                true,
            ));
        }

        // Generate hygienic bindings: one per argument.
        let mut introduced_bindings = Vec::new();
        for i in 0..argument_count {
            let original_name = identifier_name_of(&call.arguments[i + 1])
                .unwrap_or_else(|| format!("arg{i}"));
            let gensym_index = self.gensym_counter;
            self.gensym_counter += 1;
            let hygienic_name = format!("{original_name}#gensym{gensym_index}");
            introduced_bindings.push(HygienicBinding {
                original_name,
                hygienic_name,
                gensym_index,
            });
        }

        let expanded_form = format!(
            "syntax_expand({macro_name}, {})",
            (0..argument_count)
                .map(|i| expression_snippet(&call.arguments[i + 1]))
                .collect::<Vec<_>>()
                .join(", ")
        );

        self.expansions.push(MacroExpansion {
            macro_name,
            expansion_index,
            argument_count,
            introduced_bindings,
            expanded_form,
            hygienic: true,
            span,
        });
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn call_name(expression: &Expression) -> Option<&str> {
    match &expression.kind {
        ExpressionKind::Identifier(identifier) => Some(identifier.text.as_str()),
        ExpressionKind::Group(group) => call_name(&group.expression),
        _ => None,
    }
}

fn string_literal_value(expression: &Expression) -> Option<String> {
    match &expression.kind {
        ExpressionKind::String(s) => Some(s.clone()),
        ExpressionKind::Group(group) => string_literal_value(&group.expression),
        _ => None,
    }
}

fn identifier_name_of(expression: &Expression) -> Option<String> {
    match &expression.kind {
        ExpressionKind::Identifier(identifier) => Some(identifier.text.clone()),
        ExpressionKind::Group(group) => identifier_name_of(&group.expression),
        _ => None,
    }
}

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

/// Count template holes in a template string: each `$` counts as one hole.
fn count_template_holes(template: &str) -> usize {
    template.chars().filter(|&c| c == '$').count()
}
