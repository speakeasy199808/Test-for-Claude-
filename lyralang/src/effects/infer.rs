//! Internal inference engine for the seed LyraLang effect checker.

use std::collections::BTreeMap;

use crate::builtins::{builtin_callable_environment, CallableSignature};
use crate::effects::error::{EffectError, EffectErrorKind};
use crate::effects::{
    EffectBindingJudgment, EffectCheckOutput, EffectPolicy, EffectProgramJudgment,
};
use crate::parser::{
    BinaryOperator, BlockExpression, Expression, ExpressionKind, IfExpression, MatchExpression,
    Pattern, PatternKind, PrefixOperator, Program, Statement,
};
use crate::types::EffectSet;

/// Bound names visible during effect inference.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ValueEntry {
    /// A non-callable value binding.
    Value,
    /// A callable builtin surface with latent effects.
    Callable(CallableSignature),
}

/// Deterministic effect environment.
#[derive(Debug, Clone, Default)]
struct EffectEnvironment {
    values: BTreeMap<String, ValueEntry>,
}

impl EffectEnvironment {
    fn with_builtins() -> Self {
        let values = builtin_callable_environment()
            .into_iter()
            .map(|(name, signature)| (name, ValueEntry::Callable(signature)))
            .collect();
        Self { values }
    }

    fn insert_value(&mut self, name: String) {
        self.values.insert(name, ValueEntry::Value);
    }

    fn get(&self, name: &str) -> Option<&ValueEntry> {
        self.values.get(name)
    }
}

/// Internal checker implementation.
#[derive(Debug, Clone)]
pub struct EffectInferenceEngine {
    policy: Option<EffectPolicy>,
    environment: EffectEnvironment,
    bindings: Vec<EffectBindingJudgment>,
}

impl EffectInferenceEngine {
    /// Creates a new effect inference engine.
    #[must_use]
    pub fn new(policy: Option<EffectPolicy>) -> Self {
        Self {
            policy,
            environment: EffectEnvironment::with_builtins(),
            bindings: Vec::new(),
        }
    }

    /// Effect-checks a parsed program.
    pub fn check_program(mut self, normalized_source: String, program: &Program) -> EffectCheckOutput {
        match self.infer_program(program) {
            Ok(judgment) => EffectCheckOutput {
                normalized_source,
                judgment: Some(judgment),
                errors: Vec::new(),
                policy: self.policy.map(|policy| policy.allowed_effects),
            },
            Err(error) => EffectCheckOutput {
                normalized_source,
                judgment: None,
                errors: vec![error],
                policy: self.policy.map(|policy| policy.allowed_effects),
            },
        }
    }

    fn infer_program(&mut self, program: &Program) -> Result<EffectProgramJudgment, EffectError> {
        let mut program_effects = EffectSet::pure();

        for statement in &program.statements {
            let effects = self.infer_statement(statement)?;
            program_effects = program_effects.union(&effects);
        }

        if let Some(tail_expression) = &program.tail_expression {
            let tail_effects = self.infer_expression(tail_expression)?;
            program_effects = program_effects.union(&tail_effects);
        }

        if let Some(policy) = &self.policy {
            if !program_effects.is_sub_effect_of(&policy.allowed_effects) {
                let missing = program_effects.missing_from(&policy.allowed_effects);
                return Err(EffectError::new(
                    EffectErrorKind::EffectViolation,
                    format!(
                        "program requires effects {} but policy allows {}; missing {}",
                        program_effects.canonical_name(),
                        policy.allowed_effects.canonical_name(),
                        missing.canonical_name(),
                    ),
                    program.span,
                    false,
                ));
            }
        }

        Ok(EffectProgramJudgment {
            module: program.module_decl.as_ref().map(|declaration| declaration.name.text.clone()),
            program_effects,
            bindings: self.bindings.clone(),
            span: program.span,
        })
    }

    fn infer_statement(&mut self, statement: &Statement) -> Result<EffectSet, EffectError> {
        match statement {
            Statement::Let(statement) => {
                let initializer_effects = self.infer_expression(&statement.value)?;
                self.bind_pattern(&statement.pattern, &initializer_effects)?;
                Ok(initializer_effects)
            }
            Statement::Expr(statement) => self.infer_expression(&statement.expression),
        }
    }

    fn bind_pattern(
        &mut self,
        pattern: &Pattern,
        initializer_effects: &EffectSet,
    ) -> Result<(), EffectError> {
        match &pattern.kind {
            PatternKind::Wildcard | PatternKind::Integer(_) | PatternKind::String(_) | PatternKind::Boolean(_) => Ok(()),
            PatternKind::Identifier(identifier) => {
                self.environment.insert_value(identifier.text.clone());
                self.bindings.push(EffectBindingJudgment {
                    name: identifier.text.clone(),
                    initializer_effects: initializer_effects.clone(),
                    span: identifier.span,
                });
                Ok(())
            }
        }
    }

    fn bind_match_pattern(&mut self, pattern: &Pattern) {
        if let PatternKind::Identifier(identifier) = &pattern.kind {
            self.environment.insert_value(identifier.text.clone());
        }
    }

    fn infer_expression(&mut self, expression: &Expression) -> Result<EffectSet, EffectError> {
        match &expression.kind {
            ExpressionKind::Identifier(identifier) => {
                if self.environment.get(&identifier.text).is_none() {
                    return Err(EffectError::new(
                        EffectErrorKind::UnknownIdentifier,
                        format!("unknown identifier `{}`", identifier.text),
                        identifier.span,
                        false,
                    ));
                }
                Ok(EffectSet::pure())
            }
            ExpressionKind::Integer(_)
            | ExpressionKind::String(_)
            | ExpressionKind::Boolean(_)
            | ExpressionKind::SelfReference(_) => Ok(EffectSet::pure()),
            ExpressionKind::Group(group) => self.infer_expression(&group.expression),
            ExpressionKind::Try(try_expression) => self.infer_expression(&try_expression.operand),
            ExpressionKind::Block(block) => self.infer_block(block),
            ExpressionKind::If(if_expression) => self.infer_if(if_expression),
            ExpressionKind::Match(match_expression) => self.infer_match(match_expression),
            ExpressionKind::Prefix(prefix) => self.infer_prefix(prefix.operator, &prefix.operand),
            ExpressionKind::Binary { left, operator, right } => {
                self.infer_binary(*operator, left, right)
            }
            ExpressionKind::Call(call) => self.infer_call_expression(call.span, &call.callee, &call.arguments),
        }
    }

    fn infer_block(&mut self, block: &BlockExpression) -> Result<EffectSet, EffectError> {
        let saved_environment = self.environment.clone();
        let mut total_effects = EffectSet::pure();

        for statement in &block.statements {
            let effects = self.infer_statement(statement)?;
            total_effects = total_effects.union(&effects);
        }

        if let Some(tail_expression) = &block.tail_expression {
            total_effects = total_effects.union(&self.infer_expression(tail_expression)?);
        }

        self.environment = saved_environment;
        Ok(total_effects)
    }

    fn infer_if(&mut self, if_expression: &IfExpression) -> Result<EffectSet, EffectError> {
        Ok(self
            .infer_expression(&if_expression.condition)?
            .union(&self.infer_expression(&if_expression.then_branch)?)
            .union(&match &if_expression.else_branch {
                Some(else_branch) => self.infer_expression(else_branch)?,
                None => EffectSet::pure(),
            }))
    }

    fn infer_match(&mut self, match_expression: &MatchExpression) -> Result<EffectSet, EffectError> {
        let scrutinee_effects = self.infer_expression(&match_expression.scrutinee)?;
        let base_environment = self.environment.clone();
        let mut total_effects = scrutinee_effects;

        for arm in &match_expression.arms {
            self.environment = base_environment.clone();
            self.bind_match_pattern(&arm.pattern);
            total_effects = total_effects.union(&self.infer_expression(&arm.body)?);
        }

        self.environment = base_environment;
        Ok(total_effects)
    }

    fn infer_prefix(
        &mut self,
        _operator: PrefixOperator,
        operand: &Expression,
    ) -> Result<EffectSet, EffectError> {
        self.infer_expression(operand)
    }

    fn infer_binary(
        &mut self,
        _operator: BinaryOperator,
        left: &Expression,
        right: &Expression,
    ) -> Result<EffectSet, EffectError> {
        Ok(self
            .infer_expression(left)?
            .union(&self.infer_expression(right)?))
    }

    fn infer_call_expression(
        &mut self,
        span: crate::lexer::SourceSpan,
        callee: &Expression,
        arguments: &[Expression],
    ) -> Result<EffectSet, EffectError> {
        let callee_effects = self.infer_expression(callee)?;
        let mut total_effects = callee_effects;

        for argument in arguments {
            total_effects = total_effects.union(&self.infer_expression(argument)?);
        }

        let (call_name, signature) = self.resolve_callable(callee)?;

        if signature.arity != arguments.len() {
            return Err(EffectError::new(
                EffectErrorKind::ArityMismatch,
                format!(
                    "call to `{}` expected {} arguments but found {}",
                    call_name,
                    signature.arity,
                    arguments.len(),
                ),
                span,
                false,
            ));
        }

        self.require_allowed(&signature.effects, span, &call_name)?;
        Ok(total_effects.union(&signature.effects))
    }

    fn resolve_callable(
        &self,
        callee: &Expression,
    ) -> Result<(String, CallableSignature), EffectError> {
        match &callee.kind {
            ExpressionKind::Identifier(identifier) => match self.environment.get(&identifier.text) {
                Some(ValueEntry::Callable(signature)) => Ok((identifier.text.clone(), signature.clone())),
                Some(ValueEntry::Value) => Err(EffectError::new(
                    EffectErrorKind::NotCallable,
                    format!("identifier `{}` is not callable in Stage 0", identifier.text),
                    identifier.span,
                    false,
                )),
                None => Err(EffectError::new(
                    EffectErrorKind::UnknownIdentifier,
                    format!("unknown identifier `{}`", identifier.text),
                    identifier.span,
                    false,
                )),
            },
            ExpressionKind::Group(group) => self.resolve_callable(&group.expression),
            _ => Err(EffectError::new(
                EffectErrorKind::NotCallable,
                "call target is not a callable Stage 0 identifier",
                callee.span,
                false,
            )),
        }
    }

    fn require_allowed(
        &self,
        required: &EffectSet,
        span: crate::lexer::SourceSpan,
        callable_name: &str,
    ) -> Result<(), EffectError> {
        let Some(policy) = &self.policy else {
            return Ok(());
        };

        if required.is_sub_effect_of(&policy.allowed_effects) {
            return Ok(());
        }

        let missing = required.missing_from(&policy.allowed_effects);
        Err(EffectError::new(
            EffectErrorKind::EffectViolation,
            format!(
                "call to `{}` requires effects {} but policy allows {}; missing {}",
                callable_name,
                required.canonical_name(),
                policy.allowed_effects.canonical_name(),
                missing.canonical_name(),
            ),
            span,
            false,
        ))
    }
}
