//! Internal analyzer for the Stage 0 error-handling slice.

use crate::errors::{
    ErrorAnalysis, ErrorAnalysisKind, ErrorBindingJudgment, ErrorCheckOutput,
    ErrorProgramJudgment, PanicRestriction, StackTraceFrame,
};
use crate::parser::{
    CallExpression, Expression, ExpressionKind, Program, Statement,
};
use crate::types::Type;

const FORBIDDEN_CALLS: &[&str] = &["panic", "unwrap", "unwrap_result", "unwrap_option", "expect"];

/// Internal Stage 0 error-handling analyzer.
#[derive(Debug, Clone, Default)]
pub(crate) struct ErrorAnalyzer {
    trace_frames: Vec<StackTraceFrame>,
    panic_restrictions: Vec<PanicRestriction>,
    bindings: Vec<ErrorBindingJudgment>,
}

impl ErrorAnalyzer {
    /// Analyzes a parsed and typed program for Stage 0 error-handling invariants.
    pub(crate) fn analyze_program(
        mut self,
        normalized_source: String,
        program: &Program,
        program_type: Type,
    ) -> ErrorCheckOutput {
        for statement in &program.statements {
            self.analyze_statement(statement);
        }
        if let Some(tail_expression) = &program.tail_expression {
            self.analyze_expression(tail_expression);
        }

        if !self.panic_restrictions.is_empty() {
            let errors = self
                .panic_restrictions
                .iter()
                .map(|restriction| {
                    ErrorAnalysis::new(
                        ErrorAnalysisKind::PanicForbidden,
                        format!(
                            "panic-free Stage 0 forbids `{}`; use Option/Result propagation instead",
                            restriction.name
                        ),
                        restriction.span,
                        false,
                    )
                })
                .collect();
            return ErrorCheckOutput {
                normalized_source,
                judgment: None,
                errors,
            };
        }

        let (propagation_mode, propagated_error_type) = propagation_summary(&program_type);

        ErrorCheckOutput {
            normalized_source,
            judgment: Some(ErrorProgramJudgment {
                module: program.module_decl.as_ref().map(|declaration| declaration.name.text.clone()),
                program_type,
                propagation_mode,
                propagated_error_type,
                stack_trace: self.trace_frames,
                panic_free: true,
                bindings: self.bindings,
                panic_restrictions: Vec::new(),
                span: program.span,
            }),
            errors: Vec::new(),
        }
    }

    fn analyze_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Let(statement) => {
                let uses_try = contains_try(&statement.value);
                self.bindings.push(ErrorBindingJudgment {
                    name: statement
                        .pattern
                        .identifier_text()
                        .unwrap_or("_")
                        .to_string(),
                    uses_try,
                    span: statement.span,
                });
                self.analyze_expression(&statement.value);
            }
            Statement::Expr(statement) => self.analyze_expression(&statement.expression),
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
                self.trace_frames.push(StackTraceFrame {
                    line: try_expression.span.start.line,
                    column: try_expression.span.start.column,
                    snippet: expression_snippet(&try_expression.operand),
                });
                self.analyze_expression(&try_expression.operand);
            }
            ExpressionKind::Block(block) => {
                for statement in &block.statements {
                    self.analyze_statement(statement);
                }
                if let Some(tail_expression) = &block.tail_expression {
                    self.analyze_expression(tail_expression);
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
            ExpressionKind::Call(call) => self.analyze_call(call),
        }
    }

    fn analyze_call(&mut self, call: &CallExpression) {
        if let Some(name) = call_name(&call.callee) {
            if FORBIDDEN_CALLS.contains(&name) {
                self.panic_restrictions.push(PanicRestriction {
                    name: name.to_string(),
                    span: call.span,
                });
            }
        }

        self.analyze_expression(&call.callee);
        for argument in &call.arguments {
            self.analyze_expression(argument);
        }
    }
}

fn propagation_summary(program_type: &Type) -> (String, Option<String>) {
    match program_type {
        Type::Option(_) => ("option".to_string(), None),
        Type::Result(result) => {
            let err = match result.err.as_ref() {
                Type::Error(error) => Some(error.canonical_name()),
                other => Some(other.canonical_name()),
            };
            ("result".to_string(), err)
        }
        _ => ("none".to_string(), None),
    }
}

fn contains_try(expression: &Expression) -> bool {
    match &expression.kind {
        ExpressionKind::Try(_) => true,
        ExpressionKind::Group(group) => contains_try(&group.expression),
        ExpressionKind::Block(block) => {
            block.statements.iter().any(|statement| match statement {
                Statement::Let(statement) => contains_try(&statement.value),
                Statement::Expr(statement) => contains_try(&statement.expression),
            }) || block.tail_expression.as_ref().is_some_and(contains_try)
        }
        ExpressionKind::If(if_expression) => {
            contains_try(&if_expression.condition)
                || contains_try(&if_expression.then_branch)
                || if_expression.else_branch.as_ref().is_some_and(contains_try)
        }
        ExpressionKind::Match(match_expression) => {
            contains_try(&match_expression.scrutinee)
                || match_expression.arms.iter().any(|arm| contains_try(&arm.body))
        }
        ExpressionKind::Prefix(prefix) => contains_try(&prefix.operand),
        ExpressionKind::Binary { left, right, .. } => contains_try(left) || contains_try(right),
        ExpressionKind::Call(call) => {
            contains_try(&call.callee) || call.arguments.iter().any(contains_try)
        }
        ExpressionKind::Identifier(_)
        | ExpressionKind::Integer(_)
        | ExpressionKind::String(_)
        | ExpressionKind::Boolean(_)
        | ExpressionKind::SelfReference(_) => false,
    }
}

fn call_name(expression: &Expression) -> Option<&str> {
    match &expression.kind {
        ExpressionKind::Identifier(identifier) => Some(identifier.text.as_str()),
        ExpressionKind::Group(group) => call_name(&group.expression),
        _ => None,
    }
}

fn expression_snippet(expression: &Expression) -> String {
    match &expression.kind {
        ExpressionKind::Identifier(identifier) => identifier.text.clone(),
        ExpressionKind::Integer(value) => value.clone(),
        ExpressionKind::String(value) => value.clone(),
        ExpressionKind::Boolean(value) => value.to_string(),
        ExpressionKind::SelfReference(self_reference) => format!("@{}()", self_reference.primitive.as_str()),
        ExpressionKind::Call(call) => call_name(&call.callee)
            .map(|name| format!("{}(..)", name))
            .unwrap_or_else(|| "call(..)".to_string()),
        ExpressionKind::Try(try_expression) => format!("{}?", expression_snippet(&try_expression.operand)),
        ExpressionKind::Group(group) => expression_snippet(&group.expression),
        ExpressionKind::Block(_) => "{...}".to_string(),
        ExpressionKind::If(_) => "if ...".to_string(),
        ExpressionKind::Match(_) => "match ...".to_string(),
        ExpressionKind::Prefix(_) => "prefix ...".to_string(),
        ExpressionKind::Binary { .. } => "binary ...".to_string(),
    }
}
