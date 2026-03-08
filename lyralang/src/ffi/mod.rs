//! FFI specification checker for LyraLang Stage 0.
//!
//! This module analyzes FFI calls to Rust and C, enforces safety boundary
//! specifications, verifies that a `Capability` resource is in scope for all
//! FFI call sites, and documents marshalling rules between Lyra and foreign types.
//!
//! All Stage 0 FFI is safe-by-construction: no unsafe blocks are permitted
//! (the crate forbids unsafe code). Foreign calls are identified by callee
//! names beginning with `"ffi_"`.

pub mod error;

use serde::{Deserialize, Serialize};

use crate::ffi::error::{FfiError, FfiErrorKind};
use crate::lexer::SourceSpan;
use crate::parser::parse;
use crate::parser::{
    BlockExpression, Expression, ExpressionKind, Program, Statement,
};

/// The target language of an FFI call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FfiTarget {
    /// Call targets the Rust ABI (`ffi_rust_*`).
    Rust,
    /// Call targets the C ABI (`ffi_c_*`).
    C,
    /// Target language could not be determined.
    Unknown,
}

/// Direction of type marshalling across an FFI boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarshalDirection {
    /// Value flows from Lyra into the foreign function.
    LyraToForeign,
    /// Value flows from the foreign function back into Lyra.
    ForeignToLyra,
    /// Conversion is defined in both directions.
    Bidirectional,
}

/// A canonical marshalling rule between a Lyra type and a foreign type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarshallingRule {
    /// Lyra-side type name.
    pub lyra_type: String,
    /// Foreign-side type name.
    pub foreign_type: String,
    /// Direction of the marshalling conversion.
    pub direction: MarshalDirection,
    /// Human-readable description of the conversion strategy.
    pub conversion: String,
}

/// Safety boundary summary for the analyzed program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafetyBoundary {
    /// True when every FFI call site is guarded by a `Capability` binding.
    pub all_calls_gated: bool,
    /// True when any unsafe blocks appear in the program (always false at Stage 0).
    pub unsafe_blocks_present: bool,
    /// Human-readable description of the boundary policy.
    pub boundary_description: String,
}

/// Per-call summary produced by the FFI checker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FfiCallSummary {
    /// Callee name as written in source (e.g. `"ffi_rust_read"`).
    pub callee: String,
    /// Detected target language.
    pub target_language: FfiTarget,
    /// Capability kind required to perform this call.
    pub required_capability: String,
    /// Canonical marshalled representations of each argument.
    pub marshalled_params: Vec<String>,
    /// Canonical marshalled representation of the return value.
    pub return_marshalling: String,
    /// Source span of the call expression.
    pub span: SourceSpan,
}

/// Program-level FFI judgment for a fully checked program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FfiProgramJudgment {
    /// Optional module name declared at the head of the program.
    pub module: Option<String>,
    /// FFI call summaries in source order.
    pub ffi_calls: Vec<FfiCallSummary>,
    /// Safety boundary classification for the program.
    pub safety_boundary: SafetyBoundary,
    /// Capabilities required to execute all FFI calls in this program.
    pub required_capabilities: Vec<String>,
    /// Canonical marshalling rules referenced by the analyzed calls.
    pub marshalling_rules: Vec<MarshallingRule>,
    /// Source span of the full program.
    pub span: SourceSpan,
}

/// Result bundle returned by the FFI checker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FfiCheckOutput {
    /// Normalized source passed through earlier compiler stages.
    pub normalized_source: String,
    /// Program judgment when FFI checking succeeded.
    pub judgment: Option<FfiProgramJudgment>,
    /// Diagnostics emitted during checking.
    pub errors: Vec<FfiError>,
}

/// Deterministic seed FFI specification checker.
#[derive(Debug, Clone, Default)]
pub struct FfiChecker;

impl FfiChecker {
    /// Creates a new FFI checker.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Parses and FFI-checks all foreign call sites in source text.
    #[must_use]
    pub fn check_source(&self, source: &str) -> FfiCheckOutput {
        let parse_output = parse(source);
        let normalized_source = parse_output.normalized_source.clone();

        if !parse_output.errors.is_empty() {
            let errors = parse_output
                .errors
                .into_iter()
                .map(|error| {
                    FfiError::new(
                        FfiErrorKind::ParseError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return FfiCheckOutput {
                normalized_source,
                judgment: None,
                errors,
            };
        }

        let program = match parse_output.program {
            Some(program) => program,
            None => {
                return FfiCheckOutput {
                    normalized_source,
                    judgment: None,
                    errors: vec![FfiError::new(
                        FfiErrorKind::ParseError,
                        "parser completed without a program AST",
                        SourceSpan::default(),
                        false,
                    )],
                };
            }
        };

        FfiAnalyzer::default().check_program(normalized_source, &program)
    }
}

/// Parses and FFI-checks source text with the default checker.
#[must_use]
pub fn check(source: &str) -> FfiCheckOutput {
    FfiChecker::new().check_source(source)
}

// ---------------------------------------------------------------------------
// Internal analyzer
// ---------------------------------------------------------------------------

/// Canonical Stage 0 marshalling rules (Lyra → foreign).
fn canonical_marshalling_rules() -> Vec<MarshallingRule> {
    vec![
        MarshallingRule {
            lyra_type: "Int".to_owned(),
            foreign_type: "i64".to_owned(),
            direction: MarshalDirection::Bidirectional,
            conversion: "Lyra Int is represented as a 64-bit signed integer in foreign ABIs".to_owned(),
        },
        MarshallingRule {
            lyra_type: "Bool".to_owned(),
            foreign_type: "bool".to_owned(),
            direction: MarshalDirection::Bidirectional,
            conversion: "Lyra Bool maps to the C99/Rust bool type (0 = false, 1 = true)".to_owned(),
        },
        MarshallingRule {
            lyra_type: "String".to_owned(),
            foreign_type: "*const u8".to_owned(),
            direction: MarshalDirection::LyraToForeign,
            conversion: "Lyra String is passed as a null-terminated UTF-8 pointer to foreign functions".to_owned(),
        },
        MarshallingRule {
            lyra_type: "Unit".to_owned(),
            foreign_type: "void".to_owned(),
            direction: MarshalDirection::Bidirectional,
            conversion: "Lyra Unit corresponds to void in C and () in Rust".to_owned(),
        },
    ]
}

/// Determines the FFI target from a callee name.
fn ffi_target_from_name(name: &str) -> FfiTarget {
    if name.starts_with("ffi_rust_") || name == "ffi_rust" {
        FfiTarget::Rust
    } else if name.starts_with("ffi_c_") || name == "ffi_c" {
        FfiTarget::C
    } else {
        FfiTarget::Unknown
    }
}

/// Returns a canonical marshalled-type label for an argument expression.
fn marshal_arg(expr: &Expression) -> String {
    match &expr.kind {
        ExpressionKind::Integer(_) => "i64".to_owned(),
        ExpressionKind::Boolean(_) => "bool".to_owned(),
        ExpressionKind::String(_) => "*const u8".to_owned(),
        ExpressionKind::Identifier(ident) => format!("marshal({})", ident.text),
        _ => "opaque".to_owned(),
    }
}

#[derive(Debug, Clone, Default)]
struct FfiAnalyzer {
    ffi_calls: Vec<FfiCallSummary>,
    errors: Vec<FfiError>,
    /// Whether a `Capability` binding is currently in scope.
    capability_in_scope: bool,
    /// Whether any FFI call was ungated.
    any_ungated: bool,
}

impl FfiAnalyzer {
    fn check_program(mut self, normalized_source: String, program: &Program) -> FfiCheckOutput {
        // Pre-scan: check whether a Capability binding exists at the top level.
        self.capability_in_scope = program_has_capability_binding(program);

        for statement in &program.statements {
            self.visit_statement(statement);
        }
        if let Some(tail) = &program.tail_expression {
            self.visit_expression(tail);
        }

        let all_calls_gated = !self.any_ungated;

        // Collect required capabilities (deduplicated).
        let mut required_capabilities: Vec<String> = self
            .ffi_calls
            .iter()
            .map(|c| c.required_capability.clone())
            .collect();
        required_capabilities.sort();
        required_capabilities.dedup();

        let safety_boundary = SafetyBoundary {
            all_calls_gated,
            unsafe_blocks_present: false,
            boundary_description: if all_calls_gated {
                "All FFI call sites are gated by a Capability binding in scope. \
                 No unsafe blocks present (forbidden by crate policy)."
                    .to_owned()
            } else {
                "One or more FFI call sites lack a Capability binding in scope. \
                 No unsafe blocks present (forbidden by crate policy)."
                    .to_owned()
            },
        };

        // Include marshalling rules only when FFI calls are present.
        let marshalling_rules = if self.ffi_calls.is_empty() {
            Vec::new()
        } else {
            canonical_marshalling_rules()
        };

        let judgment = FfiProgramJudgment {
            module: program.module_decl.as_ref().map(|d| d.name.text.clone()),
            ffi_calls: self.ffi_calls,
            safety_boundary,
            required_capabilities,
            marshalling_rules,
            span: program.span,
        };

        FfiCheckOutput {
            normalized_source,
            judgment: Some(judgment),
            errors: self.errors,
        }
    }

    fn visit_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Let(let_stmt) => {
                // A `let capability = Capability(...)` binding gates subsequent FFI.
                if let Some(name) = let_stmt.pattern.identifier_text() {
                    if name == "capability" || name == "cap" {
                        // Check if the value is a Capability constructor call.
                        if is_capability_expr(&let_stmt.value) {
                            self.capability_in_scope = true;
                        }
                    }
                }
                self.visit_expression(&let_stmt.value);
            }
            Statement::Expr(expr_stmt) => {
                self.visit_expression(&expr_stmt.expression);
            }
        }
    }

    fn visit_expression(&mut self, expression: &Expression) {
        match &expression.kind {
            ExpressionKind::Call(call) => {
                // Check whether the callee is an FFI entry point.
                if let ExpressionKind::Identifier(callee_ident) = &call.callee.kind {
                    let name = &callee_ident.text;
                    if name.starts_with("ffi_") {
                        let target = ffi_target_from_name(name);
                        let gated = self.capability_in_scope;

                        if !gated {
                            self.any_ungated = true;
                            self.errors.push(FfiError::new(
                                FfiErrorKind::MissingCapability,
                                format!(
                                    "FFI call to `{}` requires a `Capability` binding in scope",
                                    name
                                ),
                                call.span,
                                false,
                            ));
                        }

                        let marshalled_params: Vec<String> =
                            call.arguments.iter().map(marshal_arg).collect();

                        self.ffi_calls.push(FfiCallSummary {
                            callee: name.clone(),
                            target_language: target,
                            required_capability: "Capability".to_owned(),
                            marshalled_params,
                            return_marshalling: "i64".to_owned(), // conservative default
                            span: call.span,
                        });
                    }
                }
                // Recurse into callee and arguments.
                self.visit_expression(&call.callee);
                for arg in &call.arguments {
                    self.visit_expression(arg);
                }
            }
            ExpressionKind::Block(block) => self.visit_block(block),
            ExpressionKind::If(if_expr) => {
                self.visit_expression(&if_expr.condition);
                self.visit_expression(&if_expr.then_branch);
                if let Some(else_branch) = &if_expr.else_branch {
                    self.visit_expression(else_branch);
                }
            }
            ExpressionKind::Match(match_expr) => {
                self.visit_expression(&match_expr.scrutinee);
                for arm in &match_expr.arms {
                    self.visit_expression(&arm.body);
                }
            }
            ExpressionKind::Group(group) => self.visit_expression(&group.expression),
            ExpressionKind::Try(try_expr) => self.visit_expression(&try_expr.operand),
            ExpressionKind::Prefix(prefix) => self.visit_expression(&prefix.operand),
            ExpressionKind::Binary { left, right, .. } => {
                self.visit_expression(left);
                self.visit_expression(right);
            }
            // Leaf nodes.
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
}

/// Returns true if any top-level let statement binds a `Capability` value.
fn program_has_capability_binding(program: &Program) -> bool {
    for statement in &program.statements {
        if let Statement::Let(let_stmt) = statement {
            if let Some(name) = let_stmt.pattern.identifier_text() {
                if (name == "capability" || name == "cap") && is_capability_expr(&let_stmt.value) {
                    return true;
                }
            }
            // Also accept any identifier pattern whose type constructor is Capability.
            if is_capability_expr(&let_stmt.value) {
                return true;
            }
        }
    }
    false
}

/// Returns true when an expression is a `Capability(...)` constructor call.
fn is_capability_expr(expr: &Expression) -> bool {
    if let ExpressionKind::Call(call) = &expr.kind {
        if let ExpressionKind::Identifier(callee) = &call.callee.kind {
            return callee.text == "Capability";
        }
    }
    // Also accept a bare identifier named `Capability`.
    if let ExpressionKind::Identifier(ident) = &expr.kind {
        return ident.text == "Capability";
    }
    false
}
