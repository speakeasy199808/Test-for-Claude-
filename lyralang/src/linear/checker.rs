//! Internal ownership analyzer for Stage 0 linear resources.

use std::collections::{BTreeMap, BTreeSet};

use crate::builtins::{
    builtin_linear_environment, LinearBuiltinBehavior, LinearCallableSignature,
};
use crate::lexer::SourceSpan;
use crate::linear::error::{LinearError, LinearErrorKind};
use crate::linear::{LinearBindingJudgment, LinearCheckOutput, LinearProgramJudgment};
use crate::parser::{
    BinaryOperator, BlockExpression, Expression, ExpressionKind, IfExpression, MatchExpression,
    Pattern, PatternKind, PrefixOperator, Program, Statement,
};
use crate::types::ResourceType;

#[derive(Debug, Clone, PartialEq, Eq)]
enum BindingDef {
    NonLinear,
    LinearOutstanding { resource: ResourceType, span: SourceSpan },
    LinearConsumed { resource: ResourceType, span: SourceSpan },
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ScopeFrame {
    bindings: BTreeMap<String, BindingDef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Outcome {
    resource: Option<ResourceType>,
}

impl Outcome {
    const fn none() -> Self {
        Self { resource: None }
    }

    const fn resource(resource: ResourceType) -> Self {
        Self {
            resource: Some(resource),
        }
    }
}

/// Internal checker implementation.
#[derive(Debug, Clone)]
pub struct LinearAnalyzer {
    scopes: Vec<ScopeFrame>,
    bindings: Vec<LinearBindingJudgment>,
    builtin_signatures: BTreeMap<String, LinearCallableSignature>,
}

impl Default for LinearAnalyzer {
    fn default() -> Self {
        Self {
            scopes: vec![ScopeFrame::default()],
            bindings: Vec::new(),
            builtin_signatures: builtin_linear_environment(),
        }
    }
}

impl LinearAnalyzer {
    /// Checks a parsed program and returns a deterministic ownership summary.
    pub fn check_program(
        mut self,
        normalized_source: String,
        program: &Program,
    ) -> LinearCheckOutput {
        match self.analyze_program(program) {
            Ok(judgment) => LinearCheckOutput {
                normalized_source,
                judgment: Some(judgment),
                errors: Vec::new(),
            },
            Err(error) => LinearCheckOutput {
                normalized_source,
                judgment: None,
                errors: vec![error],
            },
        }
    }

    fn analyze_program(&mut self, program: &Program) -> Result<LinearProgramJudgment, LinearError> {
        for statement in &program.statements {
            self.analyze_statement(statement)?;
        }

        let tail = if let Some(tail_expression) = &program.tail_expression {
            self.analyze_expression(tail_expression)?
        } else {
            Outcome::none()
        };

        if let Some(resource) = tail.resource {
            return Err(LinearError::new(
                LinearErrorKind::LeakedResource,
                format!(
                    "top-level program result leaves linear resource `{}` undischarged",
                    resource.as_str()
                ),
                program.span,
                false,
            ));
        }

        self.ensure_current_scope_closed(program.span)?;

        Ok(LinearProgramJudgment {
            module: program.module_decl.as_ref().map(|declaration| declaration.name.text.clone()),
            bindings: self.bindings.clone(),
            span: program.span,
        })
    }

    fn analyze_statement(&mut self, statement: &Statement) -> Result<(), LinearError> {
        match statement {
            Statement::Let(statement) => {
                let value = self.analyze_expression(&statement.value)?;
                self.bind_pattern(&statement.pattern, value.resource, statement.span)
            }
            Statement::Expr(statement) => {
                let outcome = self.analyze_expression(&statement.expression)?;
                if statement.terminated {
                    self.require_no_resource(
                        outcome.resource,
                        statement.span,
                        "terminated expression statement cannot drop a linear temporary",
                    )?;
                }
                Ok(())
            }
        }
    }

    fn analyze_expression(&mut self, expression: &Expression) -> Result<Outcome, LinearError> {
        match &expression.kind {
            ExpressionKind::Identifier(identifier) => self.consume_identifier(&identifier.text, identifier.span),
            ExpressionKind::Integer(_)
            | ExpressionKind::Boolean(_)
            | ExpressionKind::String(_)
            | ExpressionKind::SelfReference(_) => Ok(Outcome::none()),
            ExpressionKind::Group(group) => self.analyze_expression(&group.expression),
            ExpressionKind::Try(try_expression) => self.analyze_expression(&try_expression.operand),
            ExpressionKind::Block(block) => self.analyze_block(block),
            ExpressionKind::If(if_expression) => self.analyze_if(if_expression),
            ExpressionKind::Match(match_expression) => self.analyze_match(match_expression),
            ExpressionKind::Prefix(prefix) => self.analyze_prefix(prefix.operator, &prefix.operand, prefix.span),
            ExpressionKind::Binary { left, operator, right } => {
                self.analyze_binary(left, *operator, right, expression.span)
            }
            ExpressionKind::Call(call) => self.analyze_call(&call.callee, &call.arguments, call.span),
        }
    }

    fn analyze_block(&mut self, block: &BlockExpression) -> Result<Outcome, LinearError> {
        self.push_scope();
        for statement in &block.statements {
            self.analyze_statement(statement)?;
        }
        let outcome = if let Some(tail_expression) = &block.tail_expression {
            self.analyze_expression(tail_expression)?
        } else {
            Outcome::none()
        };
        self.ensure_current_scope_closed(block.span)?;
        self.pop_scope();
        Ok(outcome)
    }

    fn analyze_if(&mut self, if_expression: &IfExpression) -> Result<Outcome, LinearError> {
        let condition = self.analyze_expression(&if_expression.condition)?;
        self.require_no_resource(
            condition.resource,
            if_expression.condition.span,
            "if condition cannot leave a linear temporary",
        )?;

        let checkpoint = self.bindings.len();
        let mut then_checker = self.clone();
        let then_outcome = then_checker.analyze_expression(&if_expression.then_branch)?;

        let mut else_checker = self.clone();
        let else_outcome = if let Some(else_branch) = &if_expression.else_branch {
            else_checker.analyze_expression(else_branch)?
        } else {
            Outcome::none()
        };

        self.merge_branch_state(
            checkpoint,
            &then_checker,
            &else_checker,
            then_outcome.resource,
            else_outcome.resource,
            if_expression.span,
        )
    }

    fn analyze_match(&mut self, match_expression: &MatchExpression) -> Result<Outcome, LinearError> {
        let scrutinee = self.analyze_expression(&match_expression.scrutinee)?;
        self.require_no_resource(
            scrutinee.resource,
            match_expression.scrutinee.span,
            "match scrutinee cannot leave a linear temporary",
        )?;

        let checkpoint = self.bindings.len();
        let mut arm_checkers = Vec::with_capacity(match_expression.arms.len());
        let mut arm_resources = Vec::with_capacity(match_expression.arms.len());

        for arm in &match_expression.arms {
            let mut arm_checker = self.clone();
            arm_checker.push_scope();
            arm_checker.bind_pattern(&arm.pattern, None, arm.pattern.span)?;
            let outcome = arm_checker.analyze_expression(&arm.body)?;
            arm_checker.ensure_current_scope_closed(arm.span)?;
            arm_checker.pop_scope();
            arm_resources.push(outcome.resource);
            arm_checkers.push(arm_checker);
        }

        if arm_checkers.is_empty() {
            return Ok(Outcome::none());
        }

        let reference = arm_checkers.remove(0);
        let reference_resource = arm_resources.remove(0);
        self.scopes = reference.scopes.clone();
        self.bindings.truncate(checkpoint);
        self.merge_binding_segments(&reference.bindings[checkpoint..]);

        for (checker, resource) in arm_checkers.iter().zip(arm_resources.iter()) {
            if checker.scopes != self.scopes || *resource != reference_resource {
                return Err(LinearError::new(
                    LinearErrorKind::BranchMismatch,
                    "match arms must leave identical ownership state and resource result",
                    match_expression.span,
                    false,
                ));
            }
            self.merge_binding_segments(&checker.bindings[checkpoint..]);
        }

        Ok(Outcome {
            resource: reference_resource,
        })
    }

    fn analyze_prefix(
        &mut self,
        _operator: PrefixOperator,
        operand: &Expression,
        span: SourceSpan,
    ) -> Result<Outcome, LinearError> {
        let operand = self.analyze_expression(operand)?;
        self.require_no_resource(
            operand.resource,
            span,
            "prefix operators cannot consume or propagate linear resources in Stage 0",
        )?;
        Ok(Outcome::none())
    }

    fn analyze_binary(
        &mut self,
        left: &Expression,
        _operator: BinaryOperator,
        right: &Expression,
        span: SourceSpan,
    ) -> Result<Outcome, LinearError> {
        let left = self.analyze_expression(left)?;
        self.require_no_resource(
            left.resource,
            span,
            "binary operators cannot consume or propagate linear resources in Stage 0",
        )?;
        let right = self.analyze_expression(right)?;
        self.require_no_resource(
            right.resource,
            span,
            "binary operators cannot consume or propagate linear resources in Stage 0",
        )?;
        Ok(Outcome::none())
    }

    fn analyze_call(
        &mut self,
        callee: &Expression,
        arguments: &[Expression],
        span: SourceSpan,
    ) -> Result<Outcome, LinearError> {
        let builtin_name = match &callee.kind {
            ExpressionKind::Identifier(identifier) => Some(identifier.text.as_str()),
            _ => None,
        };

        let callee_resource = if builtin_name
            .and_then(|name| self.builtin_signatures.get(name))
            .is_none()
        {
            self.analyze_expression(callee)?.resource
        } else {
            None
        };
        self.require_no_resource(
            callee_resource,
            callee.span,
            "call target cannot be a moved linear resource",
        )?;

        let mut argument_resources = Vec::with_capacity(arguments.len());
        for argument in arguments {
            argument_resources.push(self.analyze_expression(argument)?.resource);
        }

        let Some(name) = builtin_name else {
            if argument_resources.iter().any(Option::is_some) {
                return Err(LinearError::new(
                    LinearErrorKind::UnsupportedLinearFlow,
                    "linear resources may only flow through explicit Stage 0 builtin contracts",
                    span,
                    false,
                ));
            }
            return Ok(Outcome::none());
        };

        let Some(signature) = self.builtin_signatures.get(name).copied() else {
            if argument_resources.iter().any(Option::is_some) {
                return Err(LinearError::new(
                    LinearErrorKind::UnsupportedLinearFlow,
                    format!(
                        "call to `{}` with a linear resource is not supported without an ownership contract",
                        name
                    ),
                    span,
                    false,
                ));
            }
            return Ok(Outcome::none());
        };

        if signature.arity != arguments.len() {
            return Err(LinearError::new(
                LinearErrorKind::InvalidLinearCall,
                format!(
                    "call to `{}` expected {} arguments but found {}",
                    name,
                    signature.arity,
                    arguments.len()
                ),
                span,
                false,
            ));
        }

        match signature.behavior {
            LinearBuiltinBehavior::None => {
                if argument_resources.iter().any(Option::is_some) {
                    return Err(LinearError::new(
                        LinearErrorKind::InvalidLinearCall,
                        format!(
                            "call to `{}` cannot accept linear resources in Stage 0",
                            name
                        ),
                        span,
                        false,
                    ));
                }
                Ok(Outcome::none())
            }
            LinearBuiltinBehavior::Produce(resource) => {
                if argument_resources.iter().any(Option::is_some) {
                    return Err(LinearError::new(
                        LinearErrorKind::InvalidLinearCall,
                        format!(
                            "constructor `{}` cannot consume an existing linear resource",
                            name
                        ),
                        span,
                        false,
                    ));
                }
                Ok(Outcome::resource(resource))
            }
            LinearBuiltinBehavior::Consume { index, resource } => {
                for (position, argument_resource) in argument_resources.iter().enumerate() {
                    if position == index {
                        if *argument_resource != Some(resource) {
                            return Err(LinearError::new(
                                LinearErrorKind::InvalidLinearCall,
                                format!(
                                    "call to `{}` must consume `{}` at argument {}",
                                    name,
                                    resource.as_str(),
                                    index
                                ),
                                span,
                                false,
                            ));
                        }
                    } else if argument_resource.is_some() {
                        return Err(LinearError::new(
                            LinearErrorKind::InvalidLinearCall,
                            format!(
                                "call to `{}` cannot accept extra linear resources outside the consuming position",
                                name
                            ),
                            span,
                            false,
                        ));
                    }
                }
                Ok(Outcome::none())
            }
            LinearBuiltinBehavior::Forward { index } => {
                for (position, argument_resource) in argument_resources.iter().enumerate() {
                    if position != index && argument_resource.is_some() {
                        return Err(LinearError::new(
                            LinearErrorKind::InvalidLinearCall,
                            format!(
                                "call to `{}` cannot accept extra linear resources outside the forwarded position",
                                name
                            ),
                            span,
                            false,
                        ));
                    }
                }
                Ok(Outcome {
                    resource: argument_resources.get(index).copied().flatten(),
                })
            }
        }
    }

    fn bind_pattern(
        &mut self,
        pattern: &Pattern,
        resource: Option<ResourceType>,
        span: SourceSpan,
    ) -> Result<(), LinearError> {
        match &pattern.kind {
            PatternKind::Wildcard => {
                if let Some(resource) = resource {
                    return Err(LinearError::new(
                        LinearErrorKind::LeakedResource,
                        format!(
                            "wildcard pattern cannot discard linear resource `{}`",
                            resource.as_str()
                        ),
                        span,
                        false,
                    ));
                }
                Ok(())
            }
            PatternKind::Identifier(identifier) => {
                let definition = if let Some(resource) = resource {
                    self.bindings.push(LinearBindingJudgment {
                        name: identifier.text.clone(),
                        resource,
                        span: identifier.span,
                    });
                    BindingDef::LinearOutstanding {
                        resource,
                        span: identifier.span,
                    }
                } else {
                    BindingDef::NonLinear
                };
                self.insert_current_binding(identifier.text.clone(), definition, identifier.span)
            }
            PatternKind::Integer(_) | PatternKind::String(_) | PatternKind::Boolean(_) => {
                if let Some(resource) = resource {
                    return Err(LinearError::new(
                        LinearErrorKind::UnsupportedLinearFlow,
                        format!(
                            "literal patterns cannot bind linear resource `{}` in Stage 0",
                            resource.as_str()
                        ),
                        span,
                        false,
                    ));
                }
                Ok(())
            }
        }
    }

    fn insert_current_binding(
        &mut self,
        name: String,
        definition: BindingDef,
        span: SourceSpan,
    ) -> Result<(), LinearError> {
        let current_scope = self.scopes.last_mut().expect("scope stack non-empty");
        if let Some(existing) = current_scope.bindings.get(&name) {
            if let BindingDef::LinearOutstanding { resource, .. } = existing {
                return Err(LinearError::new(
                    LinearErrorKind::LeakedResource,
                    format!(
                        "binding `{}` would shadow outstanding linear resource `{}` before discharge",
                        name,
                        resource.as_str()
                    ),
                    span,
                    false,
                ));
            }
        }
        current_scope.bindings.insert(name, definition);
        Ok(())
    }

    fn consume_identifier(&mut self, name: &str, span: SourceSpan) -> Result<Outcome, LinearError> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(definition) = scope.bindings.get_mut(name) {
                return match definition {
                    BindingDef::NonLinear => Ok(Outcome::none()),
                    BindingDef::LinearOutstanding { resource, span: binding_span } => {
                        let resource = *resource;
                        let binding_span = *binding_span;
                        *definition = BindingDef::LinearConsumed {
                            resource,
                            span: binding_span,
                        };
                        Ok(Outcome::resource(resource))
                    }
                    BindingDef::LinearConsumed { resource, .. } => Err(LinearError::new(
                        LinearErrorKind::DuplicateUse,
                        format!(
                            "linear resource `{}` bound to `{}` was already moved or consumed",
                            resource.as_str(),
                            name
                        ),
                        span,
                        false,
                    )),
                };
            }
        }
        Ok(Outcome::none())
    }

    fn ensure_current_scope_closed(&self, span: SourceSpan) -> Result<(), LinearError> {
        let current_scope = self.scopes.last().expect("scope stack non-empty");
        for (name, definition) in &current_scope.bindings {
            if let BindingDef::LinearOutstanding { resource, .. } = definition {
                return Err(LinearError::new(
                    LinearErrorKind::LeakedResource,
                    format!(
                        "linear resource `{}` bound to `{}` was not consumed exactly once",
                        resource.as_str(),
                        name
                    ),
                    span,
                    false,
                ));
            }
        }
        Ok(())
    }

    fn require_no_resource(
        &self,
        resource: Option<ResourceType>,
        span: SourceSpan,
        message: &'static str,
    ) -> Result<(), LinearError> {
        if let Some(resource) = resource {
            return Err(LinearError::new(
                LinearErrorKind::UnsupportedLinearFlow,
                format!("{}: found `{}`", message, resource.as_str()),
                span,
                false,
            ));
        }
        Ok(())
    }

    fn merge_branch_state(
        &mut self,
        checkpoint: usize,
        then_checker: &LinearAnalyzer,
        else_checker: &LinearAnalyzer,
        then_resource: Option<ResourceType>,
        else_resource: Option<ResourceType>,
        span: SourceSpan,
    ) -> Result<Outcome, LinearError> {
        if then_checker.scopes != else_checker.scopes || then_resource != else_resource {
            return Err(LinearError::new(
                LinearErrorKind::BranchMismatch,
                "branches must leave identical ownership state and resource result",
                span,
                false,
            ));
        }

        self.scopes = then_checker.scopes.clone();
        self.bindings.truncate(checkpoint);
        self.merge_binding_segments(&then_checker.bindings[checkpoint..]);
        self.merge_binding_segments(&else_checker.bindings[checkpoint..]);

        Ok(Outcome {
            resource: then_resource,
        })
    }

    fn merge_binding_segments(&mut self, segment: &[LinearBindingJudgment]) {
        let mut seen: BTreeSet<(usize, String, ResourceType)> = self
            .bindings
            .iter()
            .map(|binding| (binding.span.start.offset, binding.name.clone(), binding.resource))
            .collect();
        for binding in segment {
            let key = (binding.span.start.offset, binding.name.clone(), binding.resource);
            if seen.insert(key) {
                self.bindings.push(binding.clone());
            }
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(ScopeFrame::default());
    }

    fn pop_scope(&mut self) {
        let _ = self.scopes.pop();
    }
}
