
//! Internal analyzer for the seed LyraLang structured-concurrency checker.

use std::collections::{BTreeMap, BTreeSet};

use crate::checker::ProgramJudgment;
use crate::concurrency::error::{ConcurrencyError, ConcurrencyErrorKind};
use crate::concurrency::{
    ChannelOperation, ConcurrencyProgramJudgment, JoinSite, SelectSite, SpawnSite,
};
use crate::parser::{CallExpression, Expression, ExpressionKind, Program, Statement};
use crate::types::Type;

/// Internal structured-concurrency analyzer.
#[derive(Debug, Clone)]
pub struct ConcurrencyAnalyzer {
    binding_types: BTreeMap<String, Type>,
    next_task_id: u32,
    spawns: Vec<SpawnSite>,
    joins: Vec<JoinSite>,
    selects: Vec<SelectSite>,
    channel_operations: Vec<ChannelOperation>,
}

impl ConcurrencyAnalyzer {
    /// Creates an analyzer seeded from successful type-checking output.
    #[must_use]
    pub fn from_type_judgment(judgment: &ProgramJudgment) -> Self {
        let binding_types = judgment
            .bindings
            .iter()
            .map(|binding| (binding.name.clone(), binding.scheme.body.clone()))
            .collect();
        Self {
            binding_types,
            next_task_id: 1,
            spawns: Vec::new(),
            joins: Vec::new(),
            selects: Vec::new(),
            channel_operations: Vec::new(),
        }
    }

    /// Analyzes a parsed program and emits structured-concurrency judgments.
    pub fn analyze_program(
        mut self,
        program: &Program,
        program_type: Type,
    ) -> Result<ConcurrencyProgramJudgment, ConcurrencyError> {
        for statement in &program.statements {
            self.analyze_statement(statement)?;
        }
        if let Some(tail_expression) = &program.tail_expression {
            self.analyze_expression(tail_expression)?;
        }
        Ok(ConcurrencyProgramJudgment {
            module: program.module_decl.as_ref().map(|declaration| declaration.name.text.clone()),
            program_type,
            scheduling_policy: "source_order_spawn; lexical_join; leftmost_ready_select".to_string(),
            spawns: self.spawns,
            joins: self.joins,
            selects: self.selects,
            channel_operations: self.channel_operations,
            race_free: true,
            span: program.span,
        })
    }

    fn analyze_statement(&mut self, statement: &Statement) -> Result<(), ConcurrencyError> {
        match statement {
            Statement::Let(statement) => self.analyze_expression(&statement.value),
            Statement::Expr(statement) => self.analyze_expression(&statement.expression),
        }
    }

    fn analyze_expression(&mut self, expression: &Expression) -> Result<(), ConcurrencyError> {
        match &expression.kind {
            ExpressionKind::Identifier(_)
            | ExpressionKind::Integer(_)
            | ExpressionKind::String(_)
            | ExpressionKind::Boolean(_)
            | ExpressionKind::SelfReference(_) => Ok(()),
            ExpressionKind::Group(group) => self.analyze_expression(&group.expression),
            ExpressionKind::Try(try_expression) => self.analyze_expression(&try_expression.operand),
            ExpressionKind::Block(block) => {
                for statement in &block.statements {
                    self.analyze_statement(statement)?;
                }
                if let Some(tail_expression) = &block.tail_expression {
                    self.analyze_expression(tail_expression)?;
                }
                Ok(())
            }
            ExpressionKind::If(if_expression) => {
                self.analyze_expression(&if_expression.condition)?;
                self.analyze_expression(&if_expression.then_branch)?;
                if let Some(else_branch) = &if_expression.else_branch {
                    self.analyze_expression(else_branch)?;
                }
                Ok(())
            }
            ExpressionKind::Match(match_expression) => {
                self.analyze_expression(&match_expression.scrutinee)?;
                for arm in &match_expression.arms {
                    self.analyze_expression(&arm.body)?;
                }
                Ok(())
            }
            ExpressionKind::Prefix(prefix) => self.analyze_expression(&prefix.operand),
            ExpressionKind::Binary { left, right, .. } => {
                self.analyze_expression(left)?;
                self.analyze_expression(right)
            }
            ExpressionKind::Call(call) => self.analyze_call(call, expression.span),
        }
    }

    fn analyze_call(
        &mut self,
        call: &CallExpression,
        span: crate::lexer::SourceSpan,
    ) -> Result<(), ConcurrencyError> {
        self.analyze_expression(&call.callee)?;
        for argument in &call.arguments {
            self.analyze_expression(argument)?;
        }

        let Some(name) = call_name(&call.callee) else {
            return Ok(());
        };

        match name {
            "spawn" => {
                if call.arguments.len() != 1 {
                    return Err(ConcurrencyError::new(
                        ConcurrencyErrorKind::InvalidConcurrencySurface,
                        format!(
                            "concurrency builtin `spawn` expected 1 argument but found {}",
                            call.arguments.len()
                        ),
                        span,
                        false,
                    ));
                }
                let captures = identifiers_in_expression(&call.arguments[0]);
                let mut channel_captures = Vec::new();
                for capture in &captures {
                    if let Some(ty) = self.binding_types.get(capture) {
                        if ty.is_linear() {
                            return Err(ConcurrencyError::new(
                                ConcurrencyErrorKind::LinearCapture,
                                format!(
                                    "spawned expression captures linear resource `{}` of type {}",
                                    capture,
                                    ty.canonical_name()
                                ),
                                span,
                                false,
                            ));
                        }
                        if matches!(ty, Type::Channel(_)) {
                            channel_captures.push(capture.clone());
                        }
                    }
                }
                let task_id = self.next_task_id;
                self.next_task_id += 1;
                self.spawns.push(SpawnSite {
                    task_id,
                    expression: expression_snippet(&call.arguments[0]),
                    captures,
                    channel_captures,
                    span,
                });
            }
            "join" => {
                if call.arguments.len() != 1 {
                    return Err(ConcurrencyError::new(
                        ConcurrencyErrorKind::InvalidConcurrencySurface,
                        format!(
                            "concurrency builtin `join` expected 1 argument but found {}",
                            call.arguments.len()
                        ),
                        span,
                        false,
                    ));
                }
                self.joins.push(JoinSite {
                    task: expression_snippet(&call.arguments[0]),
                    span,
                });
            }
            "select" => {
                if call.arguments.len() != 2 {
                    return Err(ConcurrencyError::new(
                        ConcurrencyErrorKind::InvalidConcurrencySurface,
                        format!(
                            "concurrency builtin `select` expected 2 arguments but found {}",
                            call.arguments.len()
                        ),
                        span,
                        false,
                    ));
                }
                self.selects.push(SelectSite {
                    candidates: call.arguments.iter().map(expression_snippet).collect(),
                    ready_winner_rule: "leftmost_ready_candidate".to_string(),
                    span,
                });
            }
            "channel_int" => {
                self.channel_operations.push(ChannelOperation {
                    operation: "channel_int".to_string(),
                    payload_type: "Int".to_string(),
                    channel: "<fresh>".to_string(),
                    span,
                });
            }
            "send_int" | "recv_int" => {
                let channel = call
                    .arguments
                    .first()
                    .map(expression_snippet)
                    .unwrap_or_else(|| "<missing>".to_string());
                self.channel_operations.push(ChannelOperation {
                    operation: name.to_string(),
                    payload_type: "Int".to_string(),
                    channel,
                    span,
                });
            }
            _ => {}
        }

        Ok(())
    }
}

fn call_name(expression: &Expression) -> Option<&str> {
    match &expression.kind {
        ExpressionKind::Identifier(identifier) => Some(identifier.text.as_str()),
        ExpressionKind::Group(group) => call_name(&group.expression),
        _ => None,
    }
}

fn identifiers_in_expression(expression: &Expression) -> Vec<String> {
    let mut seen = BTreeSet::new();
    collect_identifiers(expression, &mut seen);
    seen.into_iter().collect()
}

fn collect_identifiers(expression: &Expression, seen: &mut BTreeSet<String>) {
    match &expression.kind {
        ExpressionKind::Identifier(identifier) => {
            seen.insert(identifier.text.clone());
        }
        ExpressionKind::Integer(_)
        | ExpressionKind::String(_)
        | ExpressionKind::Boolean(_)
        | ExpressionKind::SelfReference(_) => {}
        ExpressionKind::Group(group) => collect_identifiers(&group.expression, seen),
        ExpressionKind::Try(try_expression) => collect_identifiers(&try_expression.operand, seen),
        ExpressionKind::Block(block) => {
            for statement in &block.statements {
                match statement {
                    Statement::Let(statement) => collect_identifiers(&statement.value, seen),
                    Statement::Expr(statement) => collect_identifiers(&statement.expression, seen),
                }
            }
            if let Some(tail_expression) = &block.tail_expression {
                collect_identifiers(tail_expression, seen);
            }
        }
        ExpressionKind::If(if_expression) => {
            collect_identifiers(&if_expression.condition, seen);
            collect_identifiers(&if_expression.then_branch, seen);
            if let Some(else_branch) = &if_expression.else_branch {
                collect_identifiers(else_branch, seen);
            }
        }
        ExpressionKind::Match(match_expression) => {
            collect_identifiers(&match_expression.scrutinee, seen);
            for arm in &match_expression.arms {
                collect_identifiers(&arm.body, seen);
            }
        }
        ExpressionKind::Prefix(prefix) => collect_identifiers(&prefix.operand, seen),
        ExpressionKind::Binary { left, right, .. } => {
            collect_identifiers(left, seen);
            collect_identifiers(right, seen);
        }
        ExpressionKind::Call(call) => {
            collect_identifiers(&call.callee, seen);
            for argument in &call.arguments {
                collect_identifiers(argument, seen);
            }
        }
    }
}

fn expression_snippet(expression: &Expression) -> String {
    match &expression.kind {
        ExpressionKind::Identifier(identifier) => identifier.text.clone(),
        ExpressionKind::Integer(value) => value.clone(),
        ExpressionKind::String(value) => value.clone(),
        ExpressionKind::Boolean(value) => value.to_string(),
        ExpressionKind::SelfReference(self_reference) => format!("@{}()", self_reference.primitive.as_str()),
        ExpressionKind::Call(call) => {
            let callee = call_name(&call.callee)
                .map(ToString::to_string)
                .unwrap_or_else(|| "<expr>".to_string());
            format!(
                "{}({})",
                callee,
                call.arguments.iter().map(expression_snippet).collect::<Vec<_>>().join(", ")
            )
        }
        ExpressionKind::Try(try_expression) => format!("{}?", expression_snippet(&try_expression.operand)),
        ExpressionKind::Group(group) => expression_snippet(&group.expression),
        ExpressionKind::Block(_) => "{...}".to_string(),
        ExpressionKind::If(_) => "if ...".to_string(),
        ExpressionKind::Match(_) => "match ...".to_string(),
        ExpressionKind::Prefix(_) => "prefix ...".to_string(),
        ExpressionKind::Binary { .. } => "binary ...".to_string(),
    }
}
