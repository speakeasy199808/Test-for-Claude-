
//! Seed type-level computation checker for LyraLang Stage 0 (P1-028).
//!
//! This module analyzes const generics, type families, and compile-time
//! computations. In Stage 0 the recognized surface forms are:
//!
//! - `let name: Int = literal` — const generic parameter binding.
//! - `const_add(a, b)` — compile-time addition of constants.
//! - `const_mul(a, b)` — compile-time multiplication of constants.
//! - `type_family_define(name, params, result)` — defines a type family.
//! - `type_family_apply(name, args...)` — applies a type family.
//!
//! Termination checking is structural: literal-argument arithmetic operations
//! are always terminating (`constant_folding`); any recursive const call whose
//! recursion cannot be bounded generates a `NonTerminatingComputation` error.

use serde::{Deserialize, Serialize};

use crate::lexer::SourceSpan;
use crate::parser::parse;
use crate::parser::{CallExpression, Expression, ExpressionKind, Program, Statement};

// ── Public summary types ──────────────────────────────────────────────────────

/// A const generic parameter discovered in the AST.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstGeneric {
    /// The bound identifier name.
    pub name: String,
    /// The declared value type: `"Int"`, `"Bool"`, or `"Nat"`.
    pub value_type: String,
    /// The constant value or `"variable"` when not a literal.
    pub value_summary: String,
    /// Source span of the binding.
    pub span: SourceSpan,
}

/// A single equation in a type family definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeFamilyEquation {
    /// Left-hand side pattern (e.g. `"Vec[N]"`).
    pub lhs_pattern: String,
    /// Right-hand side type (e.g. `"List[N]"`).
    pub rhs_type: String,
}

/// A type family definition discovered in the AST.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeFamily {
    /// Type family name.
    pub name: String,
    /// Parameter kinds (e.g. `["Type", "Nat"]`).
    pub parameter_kinds: Vec<String>,
    /// Result kind (e.g. `"Type"`).
    pub result_kind: String,
    /// Equations forming the type family definition.
    pub equations: Vec<TypeFamilyEquation>,
    /// Source span of the `type_family_define` call.
    pub span: SourceSpan,
}

/// A compile-time computation discovered in the AST.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompileTimeComputation {
    /// Zero-based index of this computation in program order.
    pub index: usize,
    /// Operation name (e.g. `"const_add"`, `"const_mul"`, `"type_apply"`).
    pub operation: String,
    /// Symbolic summaries of each input argument.
    pub inputs: Vec<String>,
    /// Result summary.
    pub result: String,
    /// `true` iff termination is guaranteed.
    pub terminates: bool,
    /// Reason termination was established (or why it failed).
    pub termination_reason: String,
    /// Source span of the computation.
    pub span: SourceSpan,
}

/// Type-level computation judgment for a checked Stage 0 program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeLevelJudgment {
    /// Optional module name.
    pub module: Option<String>,
    /// Const generic bindings in source order.
    pub const_generics: Vec<ConstGeneric>,
    /// Type family definitions in source order.
    pub type_families: Vec<TypeFamily>,
    /// Compile-time computations in source order.
    pub compile_time_computations: Vec<CompileTimeComputation>,
    /// `true` iff every computation passed the termination check.
    pub all_terminate: bool,
    /// Source span for the full program.
    pub span: SourceSpan,
}

/// Categories of type-level computation error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeLevelErrorKind {
    /// Parsing failed before type-level analysis could proceed.
    ParseError,
    /// A computation could not be proved terminating.
    NonTerminatingComputation,
    /// A kind mismatch was detected.
    KindMismatch,
    /// A `type_family_apply` referenced an undefined type family.
    UndefinedTypeFamily,
}

impl TypeLevelErrorKind {
    /// Returns a stable machine-readable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::NonTerminatingComputation => "non_terminating_computation",
            Self::KindMismatch => "kind_mismatch",
            Self::UndefinedTypeFamily => "undefined_type_family",
        }
    }
}

/// A type-level computation diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeLevelError {
    /// Error category.
    pub kind: TypeLevelErrorKind,
    /// Human-readable diagnostic text.
    pub message: String,
    /// Source span associated with the error.
    pub span: SourceSpan,
    /// Whether analysis recovered and continued.
    pub recovered: bool,
}

impl TypeLevelError {
    /// Creates a new type-level computation diagnostic.
    #[must_use]
    pub fn new(
        kind: TypeLevelErrorKind,
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

/// Result bundle returned by the seed type-level computation checker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeLevelOutput {
    /// Normalized source passed through earlier compiler stages.
    pub normalized_source: String,
    /// Type-level judgment when analysis succeeded.
    pub judgment: Option<TypeLevelJudgment>,
    /// Diagnostics emitted during analysis.
    pub errors: Vec<TypeLevelError>,
}

/// Deterministic seed type-level computation checker.
#[derive(Debug, Clone, Default)]
pub struct TypeLevelChecker;

impl TypeLevelChecker {
    /// Creates a new type-level computation checker.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Parses and analyzes source text for type-level computations.
    #[must_use]
    pub fn check_source(&self, source: &str) -> TypeLevelOutput {
        let parse_output = parse(source);
        let normalized_source = parse_output.normalized_source.clone();

        if !parse_output.errors.is_empty() {
            let errors = parse_output
                .errors
                .into_iter()
                .map(|error| {
                    TypeLevelError::new(
                        TypeLevelErrorKind::ParseError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return TypeLevelOutput {
                normalized_source,
                judgment: None,
                errors,
            };
        }

        let Some(program) = parse_output.program else {
            return TypeLevelOutput {
                normalized_source,
                judgment: None,
                errors: vec![TypeLevelError::new(
                    TypeLevelErrorKind::ParseError,
                    "parser completed without a program AST",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        let mut analyzer = TypeLevelAnalyzer::new();
        analyzer.analyze_program(&program);

        let all_terminate = !analyzer
            .errors
            .iter()
            .any(|e| matches!(e.kind, TypeLevelErrorKind::NonTerminatingComputation));

        let errors = analyzer.errors.clone();
        let judgment = TypeLevelJudgment {
            module: program
                .module_decl
                .as_ref()
                .map(|d| d.name.text.clone()),
            const_generics: analyzer.const_generics,
            type_families: analyzer.type_families,
            compile_time_computations: analyzer.computations,
            all_terminate,
            span: program.span,
        };

        TypeLevelOutput {
            normalized_source,
            judgment: Some(judgment),
            errors,
        }
    }
}

/// Parses and analyzes source text with the default checker.
#[must_use]
pub fn check(source: &str) -> TypeLevelOutput {
    TypeLevelChecker::new().check_source(source)
}

// ── Internal analyzer ─────────────────────────────────────────────────────────

/// Internal AST walker for type-level computation analysis.
struct TypeLevelAnalyzer {
    /// Const generic bindings in source order.
    const_generics: Vec<ConstGeneric>,
    /// Type family definitions in source order.
    type_families: Vec<TypeFamily>,
    /// Compile-time computations in source order.
    computations: Vec<CompileTimeComputation>,
    /// Diagnostics accumulated during the walk.
    errors: Vec<TypeLevelError>,
    /// Known type families: name → (parameter_kinds, result_kind).
    known_families: std::collections::BTreeMap<String, (Vec<String>, String)>,
    /// Monotonically increasing computation counter.
    computation_counter: usize,
}

impl TypeLevelAnalyzer {
    fn new() -> Self {
        Self {
            const_generics: Vec::new(),
            type_families: Vec::new(),
            computations: Vec::new(),
            errors: Vec::new(),
            known_families: std::collections::BTreeMap::new(),
            computation_counter: 0,
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
            Statement::Let(s) => {
                // Detect `let name: Int = literal` const-generic patterns.
                // The Stage 0 parser doesn't carry type annotations in the AST
                // directly, so we check for integer-literal values and record
                // the binding as a ConstGeneric with value_type="Int".
                if let Some(binding_name) = s.pattern.identifier_text() {
                    if let Some((value_type, value_summary)) = classify_const_value(&s.value) {
                        self.const_generics.push(ConstGeneric {
                            name: binding_name.to_string(),
                            value_type,
                            value_summary,
                            span: s.span,
                        });
                        return; // Still recurse into value for nested calls.
                    }
                }
                self.analyze_expression(&s.value);
            }
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
            Some("const_add") => self.handle_const_arith("const_add", call, span),
            Some("const_mul") => self.handle_const_arith("const_mul", call, span),
            Some("type_family_define") => self.handle_type_family_define(call, span),
            Some("type_family_apply") => self.handle_type_family_apply(call, span),
            _ => {
                self.analyze_expression(&call.callee);
                for arg in &call.arguments {
                    self.analyze_expression(arg);
                }
            }
        }
    }

    fn handle_const_arith(&mut self, op: &str, call: &CallExpression, span: SourceSpan) {
        let index = self.computation_counter;
        self.computation_counter += 1;

        let inputs: Vec<String> = call
            .arguments
            .iter()
            .map(expression_snippet)
            .collect();

        // Termination: literal-only arguments → constant_folding.
        // If any argument is itself a recursive const call, the termination
        // reason is "bounded_recursion" (Stage 0: we accept calls to
        // const_add/const_mul inside themselves as bounded by induction).
        // If it's something else entirely, we flag NonTerminating.
        let all_literals = call
            .arguments
            .iter()
            .all(|arg| is_literal_or_const_arith(arg));

        let (terminates, termination_reason, result) = if all_literals {
            let r = compute_const_arith(op, &call.arguments);
            (true, "constant_folding".to_string(), r)
        } else {
            // Check if it could be recursively unbounded.
            let has_unknown = call
                .arguments
                .iter()
                .any(|arg| is_potentially_nonterminating(arg));
            if has_unknown {
                self.errors.push(TypeLevelError::new(
                    TypeLevelErrorKind::NonTerminatingComputation,
                    format!(
                        "`{op}` contains an argument that cannot be proved to terminate"
                    ),
                    span,
                    true,
                ));
                (false, "unbounded_recursion".to_string(), "unknown".to_string())
            } else {
                (true, "bounded_recursion".to_string(), "computed".to_string())
            }
        };

        self.computations.push(CompileTimeComputation {
            index,
            operation: op.to_string(),
            inputs,
            result,
            terminates,
            termination_reason,
            span,
        });

        for arg in &call.arguments {
            self.analyze_expression(arg);
        }
    }

    fn handle_type_family_define(&mut self, call: &CallExpression, span: SourceSpan) {
        // type_family_define(name, params, result)
        if call.arguments.len() < 1 {
            self.errors.push(TypeLevelError::new(
                TypeLevelErrorKind::KindMismatch,
                "`type_family_define` expects at least 1 argument (name)",
                span,
                true,
            ));
            return;
        }

        let name = string_literal_or_snippet(&call.arguments[0]);

        // Parse parameter kinds from second argument if present.
        let parameter_kinds: Vec<String> = if call.arguments.len() >= 2 {
            // Treat the second argument as a comma-separated list encoded in a string,
            // or as a single kind identifier.
            let raw = string_literal_or_snippet(&call.arguments[1]);
            raw.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            vec!["Type".to_string()]
        };

        // Result kind from third argument if present.
        let result_kind = if call.arguments.len() >= 3 {
            string_literal_or_snippet(&call.arguments[2])
        } else {
            "Type".to_string()
        };

        // Stage 0: no explicit equations — derive one from the definition call.
        let lhs = format!("{name}[{}]", parameter_kinds.join(", "));
        let rhs = result_kind.clone();
        let equations = vec![TypeFamilyEquation {
            lhs_pattern: lhs,
            rhs_type: rhs,
        }];

        self.known_families.insert(
            name.clone(),
            (parameter_kinds.clone(), result_kind.clone()),
        );

        self.type_families.push(TypeFamily {
            name,
            parameter_kinds,
            result_kind,
            equations,
            span,
        });

        for arg in &call.arguments {
            self.analyze_expression(arg);
        }
    }

    fn handle_type_family_apply(&mut self, call: &CallExpression, span: SourceSpan) {
        let index = self.computation_counter;
        self.computation_counter += 1;

        if call.arguments.is_empty() {
            self.errors.push(TypeLevelError::new(
                TypeLevelErrorKind::UndefinedTypeFamily,
                "`type_family_apply` expects at least 1 argument (family name)",
                span,
                true,
            ));
            return;
        }

        let family_name = string_literal_or_snippet(&call.arguments[0]);
        let arg_inputs: Vec<String> = call
            .arguments
            .iter()
            .skip(1)
            .map(expression_snippet)
            .collect();

        // Check if the family is known.
        let known = self.known_families.contains_key(&family_name);
        if !known {
            self.errors.push(TypeLevelError::new(
                TypeLevelErrorKind::UndefinedTypeFamily,
                format!("application of undefined type family `{family_name}`"),
                span,
                true,
            ));
        }

        let result = if known {
            format!("{family_name}[{}]", arg_inputs.join(", "))
        } else {
            "unknown".to_string()
        };

        let mut inputs = vec![family_name.clone()];
        inputs.extend(arg_inputs);

        self.computations.push(CompileTimeComputation {
            index,
            operation: "type_apply".to_string(),
            inputs,
            result,
            terminates: known,
            termination_reason: if known {
                "structural".to_string()
            } else {
                "undefined_family".to_string()
            },
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

/// Returns the string value of a literal or the expression snippet.
fn string_literal_or_snippet(expression: &Expression) -> String {
    match &expression.kind {
        ExpressionKind::String(s) => s.clone(),
        ExpressionKind::Identifier(id) => id.text.clone(),
        ExpressionKind::Group(group) => string_literal_or_snippet(&group.expression),
        _ => expression_snippet(expression),
    }
}

/// Classifies a let-binding value as a const generic when it is a literal.
///
/// Returns `Some((value_type, value_summary))` for recognized forms.
fn classify_const_value(expression: &Expression) -> Option<(String, String)> {
    match &expression.kind {
        ExpressionKind::Integer(value) => Some(("Int".to_string(), value.clone())),
        ExpressionKind::Boolean(value) => Some(("Bool".to_string(), value.to_string())),
        ExpressionKind::Group(group) => classify_const_value(&group.expression),
        _ => None,
    }
}

/// Returns `true` iff every leaf of the expression is a literal or
/// a nested `const_add`/`const_mul` call over literals.
fn is_literal_or_const_arith(expression: &Expression) -> bool {
    match &expression.kind {
        ExpressionKind::Integer(_) | ExpressionKind::Boolean(_) => true,
        ExpressionKind::Group(group) => is_literal_or_const_arith(&group.expression),
        ExpressionKind::Call(call) => {
            let name = call_name(&call.callee);
            matches!(name, Some("const_add") | Some("const_mul"))
                && call.arguments.iter().all(is_literal_or_const_arith)
        }
        _ => false,
    }
}

/// Returns `true` iff the expression contains a call that cannot be bounded
/// (i.e. an identifier reference or a non-arithmetic call at any level).
fn is_potentially_nonterminating(expression: &Expression) -> bool {
    match &expression.kind {
        ExpressionKind::Integer(_)
        | ExpressionKind::Boolean(_)
        | ExpressionKind::String(_) => false,
        ExpressionKind::Identifier(_) => true, // unknown variable
        ExpressionKind::Group(group) => is_potentially_nonterminating(&group.expression),
        ExpressionKind::Call(call) => {
            let name = call_name(&call.callee);
            match name {
                Some("const_add") | Some("const_mul") => {
                    call.arguments.iter().any(is_potentially_nonterminating)
                }
                _ => true, // unknown call — conservative
            }
        }
        _ => true,
    }
}

/// Compute a result summary for a literal-argument `const_add` or `const_mul`.
fn compute_const_arith(op: &str, args: &[Expression]) -> String {
    if args.len() != 2 {
        return "computed".to_string();
    }
    let a = int_value(&args[0]);
    let b = int_value(&args[1]);
    match (a, b) {
        (Some(x), Some(y)) => {
            let result = if op == "const_add" { x + y } else { x * y };
            result.to_string()
        }
        _ => "computed".to_string(),
    }
}

/// Extract an integer value from an integer literal expression.
fn int_value(expression: &Expression) -> Option<i64> {
    match &expression.kind {
        ExpressionKind::Integer(value) => value.parse().ok(),
        ExpressionKind::Group(group) => int_value(&group.expression),
        _ => None,
    }
}
