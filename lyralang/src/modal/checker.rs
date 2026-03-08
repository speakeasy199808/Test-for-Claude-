//! Internal analyzer for the seed LyraLang modal checker.

use std::collections::BTreeMap;

use crate::builtins::{builtin_modal_environment, ModalBuiltinBehavior, ModalCallableSignature};
use crate::checker::ProgramJudgment;
use crate::lexer::SourceSpan;
use crate::modal::error::{ModalError, ModalErrorKind};
use crate::modal::{ModalBindingJudgment, ModalProgramJudgment, ModalPromotionJudgment};
use crate::parser::{CallExpression, Expression, ExpressionKind, Program, Statement};
use crate::types::Type;

/// Internal modal analyzer.
#[derive(Debug, Clone)]
pub struct ModalAnalyzer {
    builtin_signatures: BTreeMap<String, ModalCallableSignature>,
    binding_types: BTreeMap<String, Type>,
    modal_bindings: Vec<ModalBindingJudgment>,
    promotions: Vec<ModalPromotionJudgment>,
}

impl ModalAnalyzer {
    /// Creates an analyzer seeded from successful type-checking output.
    #[must_use]
    pub fn from_type_judgment(judgment: &ProgramJudgment) -> Self {
        let binding_types = judgment
            .bindings
            .iter()
            .map(|binding| (binding.name.clone(), binding.scheme.body.clone()))
            .collect();
        Self {
            builtin_signatures: builtin_modal_environment(),
            binding_types,
            modal_bindings: Vec::new(),
            promotions: Vec::new(),
        }
    }

    /// Analyzes a parsed program and emits modal judgments.
    pub fn analyze_program(mut self, program: &Program, program_type: Type) -> Result<ModalProgramJudgment, ModalError> {
        for statement in &program.statements {
            self.analyze_statement(statement)?;
        }
        if let Some(tail_expression) = &program.tail_expression {
            self.analyze_expression(tail_expression)?;
        }
        Ok(ModalProgramJudgment {
            module: program.module_decl.as_ref().map(|declaration| declaration.name.text.clone()),
            program_type,
            bindings: self.modal_bindings,
            promotions: self.promotions,
            span: program.span,
        })
    }

    fn analyze_statement(&mut self, statement: &Statement) -> Result<(), ModalError> {
        match statement {
            Statement::Let(statement) => {
                self.analyze_expression(&statement.value)?;
                if let Some(identifier) = statement.pattern.identifier_text() {
                    if let Some(ty) = self.binding_types.get(identifier) {
                        if let Type::Modal(modal) = ty {
                            self.modal_bindings.push(ModalBindingJudgment {
                                name: identifier.to_string(),
                                modality: modal.modality,
                                payload_type: (*modal.body).clone(),
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

    fn analyze_expression(&mut self, expression: &Expression) -> Result<(), ModalError> {
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

    fn analyze_call(&mut self, call: &CallExpression, span: SourceSpan) -> Result<(), ModalError> {
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
            return Err(ModalError::new(
                ModalErrorKind::InvalidModalPromotion,
                format!(
                    "modal builtin `{}` expected {} arguments but found {}",
                    builtin_name,
                    signature.arity,
                    call.arguments.len()
                ),
                span,
                false,
            ));
        }

        if let ModalBuiltinBehavior::Promote { from, to, evidence } = signature.behavior {
            self.promotions.push(ModalPromotionJudgment {
                name: builtin_name.to_string(),
                from,
                to,
                evidence,
                span,
            });
        }

        Ok(())
    }
}
