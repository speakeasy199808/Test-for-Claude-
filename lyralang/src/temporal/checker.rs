
//! Internal analyzer for the seed LyraLang temporal checker.

use std::collections::BTreeMap;

use crate::builtins::{
    builtin_temporal_environment, TemporalBuiltinBehavior, TemporalCallableSignature,
};
use crate::checker::ProgramJudgment;
use crate::temporal::error::{TemporalError, TemporalErrorKind};
use crate::temporal::{
    TemporalBindingJudgment, TemporalFormulaJudgment, TemporalProgramJudgment,
};
use crate::parser::{CallExpression, Expression, ExpressionKind, Program, Statement};
use crate::types::Type;

/// Internal temporal analyzer.
#[derive(Debug, Clone)]
pub struct TemporalAnalyzer {
    builtin_signatures: BTreeMap<String, TemporalCallableSignature>,
    binding_types: BTreeMap<String, Type>,
    bindings: Vec<TemporalBindingJudgment>,
    formulas: Vec<TemporalFormulaJudgment>,
}

impl TemporalAnalyzer {
    /// Creates an analyzer seeded from successful type-checking output.
    #[must_use]
    pub fn from_type_judgment(judgment: &ProgramJudgment) -> Self {
        let binding_types = judgment
            .bindings
            .iter()
            .map(|binding| (binding.name.clone(), binding.scheme.body.clone()))
            .collect();
        Self {
            builtin_signatures: builtin_temporal_environment(),
            binding_types,
            bindings: Vec::new(),
            formulas: Vec::new(),
        }
    }

    /// Analyzes a parsed program and emits temporal judgments.
    pub fn analyze_program(
        mut self,
        program: &Program,
        program_type: Type,
    ) -> Result<TemporalProgramJudgment, TemporalError> {
        for statement in &program.statements {
            self.analyze_statement(statement)?;
        }
        if let Some(tail_expression) = &program.tail_expression {
            self.analyze_expression(tail_expression)?;
        }
        Ok(TemporalProgramJudgment {
            module: program.module_decl.as_ref().map(|declaration| declaration.name.text.clone()),
            program_type,
            bindings: self.bindings,
            formulas: self.formulas,
            span: program.span,
        })
    }

    fn analyze_statement(&mut self, statement: &Statement) -> Result<(), TemporalError> {
        match statement {
            Statement::Let(statement) => {
                self.analyze_expression(&statement.value)?;
                if let Some(identifier) = statement.pattern.identifier_text() {
                    if let Some(ty) = self.binding_types.get(identifier) {
                        if matches!(ty, Type::Temporal(_)) {
                            self.bindings.push(TemporalBindingJudgment {
                                name: identifier.to_string(),
                                proposition_type: ty.canonical_name(),
                                span: statement.pattern.span,
                            });
                        }
                    }
                }
                Ok(())
            }
            Statement::Expr(statement) => self.analyze_expression(&statement.expression),
        }
    }

    fn analyze_expression(&mut self, expression: &Expression) -> Result<(), TemporalError> {
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
    ) -> Result<(), TemporalError> {
        self.analyze_expression(&call.callee)?;
        for argument in &call.arguments {
            self.analyze_expression(argument)?;
        }

        let builtin_name = match &call.callee.kind {
            ExpressionKind::Identifier(identifier) => identifier.text.as_str(),
            _ => return Ok(()),
        };
        let Some(signature) = self.builtin_signatures.get(builtin_name).copied() else {
            return Ok(());
        };

        if signature.arity != call.arguments.len() {
            return Err(TemporalError::new(
                TemporalErrorKind::InvalidTemporalOperator,
                format!(
                    "temporal operator `{}` expected {} arguments but found {}",
                    builtin_name,
                    signature.arity,
                    call.arguments.len()
                ),
                span,
                false,
            ));
        }

        let operands: Vec<String> = call.arguments.iter().map(expression_snippet).collect();
        let normalized_formula = match signature.behavior {
            TemporalBuiltinBehavior::Always => format!("always({})", operands[0]),
            TemporalBuiltinBehavior::Eventually => format!("eventually({})", operands[0]),
            TemporalBuiltinBehavior::Until => format!("until({}, {})", operands[0], operands[1]),
            TemporalBuiltinBehavior::Since => format!("since({}, {})", operands[0], operands[1]),
        };

        self.formulas.push(TemporalFormulaJudgment {
            operator: builtin_name.to_string(),
            operands,
            normalized_formula,
            span,
        });
        Ok(())
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
            let callee = match &call.callee.kind {
                ExpressionKind::Identifier(identifier) => identifier.text.clone(),
                _ => "<expr>".to_string(),
            };
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
