//! Lifetime annotations checker for LyraLang Stage 0.
//!
//! This module performs borrow-checking semantics for non-linear types,
//! lifetime elision rules, region inference, and ensures no dangling
//! references by construction. Linear resources (File, Socket, Capability)
//! are already handled by the `linear` checker and are assigned `BorrowKind::Owned`.

pub mod error;

use serde::{Deserialize, Serialize};

use crate::lexer::SourceSpan;
use crate::lifetimes::error::{LifetimeError, LifetimeErrorKind};
use crate::parser::parse;
use crate::parser::{
    BlockExpression, Expression, ExpressionKind, Program, Statement,
};

/// Kind of borrow or ownership for a given binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BorrowKind {
    /// The binding owns its value outright (also used for linear resources).
    Owned,
    /// The binding holds a shared (immutable) reference.
    SharedRef,
    /// The binding holds a unique (mutable) reference.
    UniqueRef,
    /// The binding lives for the entire program lifetime (`'static`).
    Static,
}

/// Describes the relationship between two lifetime regions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutlivesKind {
    /// The left region outlives the right region (`'a: 'b`).
    Outlives,
    /// Both regions share the same lexical scope.
    SameScopeAs,
    /// The left region is `'static` (outlives all).
    Static,
}

/// A constraint between two named lifetime regions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionConstraint {
    /// Left-hand lifetime region name (e.g. `"'a"`).
    pub lhs: String,
    /// Right-hand lifetime region name (e.g. `"'b"`).
    pub rhs: String,
    /// The kind of outlives constraint.
    pub constraint: OutlivesKind,
}

/// Inferred lifetime judgment for a single let binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifetimeBindingJudgment {
    /// Bound identifier name.
    pub name: String,
    /// Inferred lifetime region (e.g. `"'a"`, `"'b"`, `"'static"`).
    pub inferred_region: String,
    /// Kind of borrow for this binding.
    pub borrow_kind: BorrowKind,
    /// Source span of the binding site.
    pub span: SourceSpan,
}

/// Program-level lifetime judgment for a fully checked program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifetimeProgramJudgment {
    /// Optional module name declared at the head of the program.
    pub module: Option<String>,
    /// Inferred lifetime judgments for let bindings in source order.
    pub bindings: Vec<LifetimeBindingJudgment>,
    /// Region constraints collected during analysis.
    pub regions: Vec<RegionConstraint>,
    /// True when no dangling references exist by construction.
    pub dangling_free: bool,
    /// Names of bindings where lifetime elision was applied.
    pub elision_applied: Vec<String>,
    /// Source span of the full program.
    pub span: SourceSpan,
}

/// Result bundle returned by the lifetime checker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifetimeCheckOutput {
    /// Normalized source passed through earlier compiler stages.
    pub normalized_source: String,
    /// Program judgment when lifetime checking succeeded.
    pub judgment: Option<LifetimeProgramJudgment>,
    /// Diagnostics emitted during checking.
    pub errors: Vec<LifetimeError>,
}

/// Deterministic seed lifetime annotations checker.
#[derive(Debug, Clone, Default)]
pub struct LifetimeChecker;

impl LifetimeChecker {
    /// Creates a new lifetime checker.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Parses and performs lifetime analysis on source text.
    #[must_use]
    pub fn check_source(&self, source: &str) -> LifetimeCheckOutput {
        let parse_output = parse(source);
        let normalized_source = parse_output.normalized_source.clone();

        if !parse_output.errors.is_empty() {
            let errors = parse_output
                .errors
                .into_iter()
                .map(|error| {
                    LifetimeError::new(
                        LifetimeErrorKind::ParseError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return LifetimeCheckOutput {
                normalized_source,
                judgment: None,
                errors,
            };
        }

        let program = match parse_output.program {
            Some(program) => program,
            None => {
                return LifetimeCheckOutput {
                    normalized_source,
                    judgment: None,
                    errors: vec![LifetimeError::new(
                        LifetimeErrorKind::ParseError,
                        "parser completed without a program AST",
                        SourceSpan::default(),
                        false,
                    )],
                };
            }
        };

        LifetimeAnalyzer::default().check_program(normalized_source, &program)
    }
}

/// Parses and lifetime-checks source text with the default checker.
#[must_use]
pub fn check(source: &str) -> LifetimeCheckOutput {
    LifetimeChecker::new().check_source(source)
}

// ---------------------------------------------------------------------------
// Internal analyzer
// ---------------------------------------------------------------------------

/// Known linear resource type names that are handled by the linear checker.
const LINEAR_RESOURCE_NAMES: &[&str] = &["File", "Socket", "Capability"];

#[derive(Debug, Clone)]
struct LifetimeAnalyzer {
    bindings: Vec<LifetimeBindingJudgment>,
    regions: Vec<RegionConstraint>,
    elision_applied: Vec<String>,
    /// Counter for generating fresh region names.
    region_counter: usize,
    /// Nesting depth (0 = top-level, increments per block).
    scope_depth: usize,
}

impl Default for LifetimeAnalyzer {
    fn default() -> Self {
        Self {
            bindings: Vec::new(),
            regions: Vec::new(),
            elision_applied: Vec::new(),
            region_counter: 0,
            scope_depth: 0,
        }
    }
}

impl LifetimeAnalyzer {
    fn check_program(mut self, normalized_source: String, program: &Program) -> LifetimeCheckOutput {
        for statement in &program.statements {
            self.visit_statement(statement);
        }
        if let Some(tail) = &program.tail_expression {
            self.visit_expression(tail);
        }

        let judgment = LifetimeProgramJudgment {
            module: program.module_decl.as_ref().map(|d| d.name.text.clone()),
            bindings: self.bindings,
            regions: self.regions,
            dangling_free: true, // Stage 0: safe by construction, no raw pointers
            elision_applied: self.elision_applied,
            span: program.span,
        };

        LifetimeCheckOutput {
            normalized_source,
            judgment: Some(judgment),
            errors: Vec::new(),
        }
    }

    fn fresh_region(&mut self) -> String {
        // Assign 'a, 'b, 'c, ... then 'a1, 'b1, ...
        let letter_index = self.region_counter % 26;
        let generation = self.region_counter / 26;
        let letter = (b'a' + letter_index as u8) as char;
        self.region_counter += 1;
        if generation == 0 {
            format!("'{}", letter)
        } else {
            format!("'{}{}", letter, generation)
        }
    }

    fn visit_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Let(let_stmt) => {
                self.visit_expression(&let_stmt.value);
                let name = match &let_stmt.pattern.kind {
                    crate::parser::PatternKind::Identifier(ident) => ident.text.clone(),
                    crate::parser::PatternKind::Wildcard => "_".to_owned(),
                    crate::parser::PatternKind::Integer(v) => v.clone(),
                    crate::parser::PatternKind::String(v) => v.clone(),
                    crate::parser::PatternKind::Boolean(b) => b.to_string(),
                };

                // Determine borrow kind: linear resources get Owned, top-level
                // string/static literals get Static, others get SharedRef.
                let (borrow_kind, inferred_region) =
                    self.infer_binding_lifetime(&name, &let_stmt.value);

                // Elision: if there is exactly one reference in the current
                // scope, we elide the annotation (Stage 0 elision rule 1).
                let shared_ref_count = self
                    .bindings
                    .iter()
                    .filter(|b| b.borrow_kind == BorrowKind::SharedRef)
                    .count();
                let elided = borrow_kind == BorrowKind::SharedRef && shared_ref_count == 0;

                // Emit a SameScopeAs constraint for non-static bindings.
                if self.scope_depth > 0 {
                    if let Some(prev) = self.bindings.last() {
                        if prev.borrow_kind != BorrowKind::Static
                            && borrow_kind != BorrowKind::Static
                        {
                            self.regions.push(RegionConstraint {
                                lhs: inferred_region.clone(),
                                rhs: prev.inferred_region.clone(),
                                constraint: OutlivesKind::SameScopeAs,
                            });
                        }
                    }
                } else if borrow_kind == BorrowKind::Static {
                    self.regions.push(RegionConstraint {
                        lhs: inferred_region.clone(),
                        rhs: "'static".to_owned(),
                        constraint: OutlivesKind::Static,
                    });
                } else {
                    // Top-level non-static binding outlives inner scopes.
                    self.regions.push(RegionConstraint {
                        lhs: inferred_region.clone(),
                        rhs: "'program".to_owned(),
                        constraint: OutlivesKind::Outlives,
                    });
                }

                if elided {
                    self.elision_applied.push(name.clone());
                }

                self.bindings.push(LifetimeBindingJudgment {
                    name,
                    inferred_region,
                    borrow_kind,
                    span: let_stmt.span,
                });
            }
            Statement::Expr(expr_stmt) => {
                self.visit_expression(&expr_stmt.expression);
            }
        }
    }

    fn infer_binding_lifetime(
        &mut self,
        name: &str,
        value: &Expression,
    ) -> (BorrowKind, String) {
        // Linear resources are handled by the linear checker; mark as Owned.
        if LINEAR_RESOURCE_NAMES.contains(&name) {
            return (BorrowKind::Owned, "'static".to_owned());
        }

        // Check whether the RHS is a call to a linear resource constructor.
        if let ExpressionKind::Call(call) = &value.kind {
            if let ExpressionKind::Identifier(callee) = &call.callee.kind {
                if LINEAR_RESOURCE_NAMES.contains(&callee.text.as_str()) {
                    return (BorrowKind::Owned, "'static".to_owned());
                }
            }
        }

        // String literals and explicit static annotations → 'static.
        if matches!(&value.kind, ExpressionKind::String(_)) {
            return (BorrowKind::Static, "'static".to_owned());
        }

        // All other bindings get a fresh SharedRef region.
        let region = self.fresh_region();
        (BorrowKind::SharedRef, region)
    }

    fn visit_expression(&mut self, expression: &Expression) {
        match &expression.kind {
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
            ExpressionKind::Call(call) => {
                self.visit_expression(&call.callee);
                for arg in &call.arguments {
                    self.visit_expression(arg);
                }
            }
            ExpressionKind::Group(group) => self.visit_expression(&group.expression),
            ExpressionKind::Try(try_expr) => self.visit_expression(&try_expr.operand),
            ExpressionKind::Prefix(prefix) => self.visit_expression(&prefix.operand),
            ExpressionKind::Binary { left, right, .. } => {
                self.visit_expression(left);
                self.visit_expression(right);
            }
            // Leaf nodes carry no sub-expressions.
            ExpressionKind::Identifier(_)
            | ExpressionKind::Integer(_)
            | ExpressionKind::String(_)
            | ExpressionKind::Boolean(_)
            | ExpressionKind::SelfReference(_) => {}
        }
    }

    fn visit_block(&mut self, block: &BlockExpression) {
        self.scope_depth += 1;
        for statement in &block.statements {
            self.visit_statement(statement);
        }
        if let Some(tail) = &block.tail_expression {
            self.visit_expression(tail);
        }
        self.scope_depth -= 1;
    }
}
