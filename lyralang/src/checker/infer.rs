//! Internal inference engine for the seed LyraLang type checker.

use std::collections::{BTreeMap, BTreeSet};

use crate::builtins::{builtin_type_environment, self_reference_type};
use crate::checker::error::{TypeError, TypeErrorKind};
use crate::checker::{BindingJudgment, ProgramJudgment, TypeCheckOutput};
use crate::lexer::SourceSpan;
use crate::parser::{
    BinaryOperator, BlockExpression, Expression, ExpressionKind, IfExpression, MatchExpression,
    Pattern, PatternKind, PrefixOperator, Program, Statement,
};
use crate::types::{EffectSet, ErrorType, Type, TypeScheme, TypeVariableId};

/// Internal equality constraint.
#[derive(Debug, Clone)]
struct TypeConstraint {
    expected: Type,
    actual: Type,
    span: SourceSpan,
    reason: &'static str,
}

/// Substitution state accumulated during unification.
#[derive(Debug, Clone, Default)]
struct Substitution {
    types: BTreeMap<TypeVariableId, Type>,
}

impl Substitution {
    fn apply_type(&self, ty: &Type) -> Type {
        ty.substitute(&self.types)
    }
}

/// Deterministic type environment.
#[derive(Debug, Clone, Default)]
struct TypeEnvironment {
    values: BTreeMap<String, TypeScheme>,
}

impl TypeEnvironment {
    fn from_values(values: BTreeMap<String, TypeScheme>) -> Self {
        Self { values }
    }

    fn insert(&mut self, name: String, scheme: TypeScheme) {
        self.values.insert(name, scheme);
    }

    fn get(&self, name: &str) -> Option<&TypeScheme> {
        self.values.get(name)
    }

    fn free_type_variables(&self) -> BTreeSet<TypeVariableId> {
        self.values
            .values()
            .flat_map(TypeScheme::free_type_variables)
            .collect()
    }

    fn substitute(&self, substitutions: &Substitution) -> Self {
        let values = self
            .values
            .iter()
            .map(|(name, scheme)| (name.clone(), scheme.substitute(&substitutions.types)))
            .collect();
        Self { values }
    }
}

/// Inferred type-and-effect pair.
#[derive(Debug, Clone)]
struct Inference {
    ty: Type,
    effects: EffectSet,
}

impl Inference {
    fn pure(ty: Type) -> Self {
        Self {
            ty,
            effects: EffectSet::pure(),
        }
    }
}

/// Error-propagation mode active for a program or block scope.
#[derive(Debug, Clone)]
enum PropagationMode {
    Result(ErrorType),
    Option,
}

/// Scope-local propagation state.
#[derive(Debug, Clone, Default)]
struct PropagationContext {
    mode: Option<PropagationMode>,
}

/// Internal checker implementation.
#[derive(Debug, Clone)]
pub struct InferenceEngine {
    next_type_variable: u32,
    substitution: Substitution,
    constraints: Vec<TypeConstraint>,
    environment: TypeEnvironment,
    bindings: Vec<BindingJudgment>,
    propagation_stack: Vec<PropagationContext>,
}

impl Default for InferenceEngine {
    fn default() -> Self {
        Self {
            next_type_variable: 0,
            substitution: Substitution::default(),
            constraints: Vec::new(),
            environment: TypeEnvironment::from_values(builtin_type_environment()),
            bindings: Vec::new(),
            propagation_stack: Vec::new(),
        }
    }
}

impl InferenceEngine {
    /// Type-checks a parsed program.
    pub fn check_program(mut self, normalized_source: String, program: &Program) -> TypeCheckOutput {
        match self.infer_program(program) {
            Ok(judgment) => TypeCheckOutput {
                normalized_source,
                judgment: Some(judgment),
                errors: Vec::new(),
            },
            Err(error) => TypeCheckOutput {
                normalized_source,
                judgment: None,
                errors: vec![error],
            },
        }
    }

    fn infer_program(&mut self, program: &Program) -> Result<ProgramJudgment, TypeError> {
        let mut program_effects = EffectSet::pure();
        let mut program_type = Type::unit();

        self.propagation_stack.push(PropagationContext::default());

        for statement in &program.statements {
            let effects = self.infer_statement(statement)?;
            program_effects = program_effects.union(&effects);
        }

        if let Some(tail_expression) = &program.tail_expression {
            let tail = self.infer_expression(tail_expression)?;
            program_effects = program_effects.union(&tail.effects);
            program_type = self.substitution.apply_type(&tail.ty);
        }

        let context = self.propagation_stack.pop().expect("program propagation context present");
        program_type = self.apply_propagation_context(program_type, context);

        Ok(ProgramJudgment {
            module: program.module_decl.as_ref().map(|declaration| declaration.name.text.clone()),
            program_type,
            program_effects,
            bindings: self.bindings.clone(),
            span: program.span,
        })
    }

    fn infer_statement(&mut self, statement: &Statement) -> Result<EffectSet, TypeError> {
        match statement {
            Statement::Let(statement) => {
                let value = self.infer_expression(&statement.value)?;
                self.bind_pattern(&statement.pattern, &value.ty)?;
                Ok(value.effects)
            }
            Statement::Expr(statement) => Ok(self.infer_expression(&statement.expression)?.effects),
        }
    }

    fn bind_pattern(&mut self, pattern: &Pattern, value_type: &Type) -> Result<(), TypeError> {
        match &pattern.kind {
            PatternKind::Wildcard => Ok(()),
            PatternKind::Identifier(identifier) => {
                let environment = self.environment.substitute(&self.substitution);
                let generalized = self.generalize(&environment, self.substitution.apply_type(value_type));
                self.environment
                    .insert(identifier.text.clone(), generalized.clone());
                self.bindings.push(BindingJudgment {
                    name: identifier.text.clone(),
                    scheme: generalized,
                    span: identifier.span,
                });
                Ok(())
            }
            PatternKind::Integer(_) => self.constrain(
                Type::int(),
                value_type.clone(),
                pattern.span,
                "integer pattern must match Int",
            ),
            PatternKind::Boolean(_) => self.constrain(
                Type::bool(),
                value_type.clone(),
                pattern.span,
                "boolean pattern must match Bool",
            ),
            PatternKind::String(_) => Err(TypeError::new(
                TypeErrorKind::UnsupportedConstruct,
                "string pattern typing is deferred in Stage 0",
                pattern.span,
                false,
            )),
        }
    }

    fn bind_match_pattern(
        &mut self,
        pattern: &Pattern,
        scrutinee_type: &Type,
    ) -> Result<(), TypeError> {
        match &pattern.kind {
            PatternKind::Wildcard => Ok(()),
            PatternKind::Identifier(identifier) => {
                self.environment.insert(
                    identifier.text.clone(),
                    TypeScheme::mono(self.substitution.apply_type(scrutinee_type)),
                );
                Ok(())
            }
            PatternKind::Integer(_) => self.constrain(
                Type::int(),
                scrutinee_type.clone(),
                pattern.span,
                "integer pattern must match Int",
            ),
            PatternKind::Boolean(_) => self.constrain(
                Type::bool(),
                scrutinee_type.clone(),
                pattern.span,
                "boolean pattern must match Bool",
            ),
            PatternKind::String(_) => Err(TypeError::new(
                TypeErrorKind::UnsupportedConstruct,
                "string pattern typing is deferred in Stage 0",
                pattern.span,
                false,
            )),
        }
    }

    fn infer_expression(&mut self, expression: &Expression) -> Result<Inference, TypeError> {
        match &expression.kind {
            ExpressionKind::Identifier(identifier) => {
                let scheme = self.environment.get(&identifier.text).cloned().ok_or_else(|| {
                    TypeError::new(
                        TypeErrorKind::UnknownIdentifier,
                        format!("unknown identifier `{}`", identifier.text),
                        identifier.span,
                        false,
                    )
                })?;
                Ok(Inference::pure(self.instantiate(&scheme)))
            }
            ExpressionKind::Integer(_) => Ok(Inference::pure(Type::int())),
            ExpressionKind::SelfReference(self_reference) => {
                let primitive = self_reference.primitive.as_str();
                let ty = self_reference_type(primitive).ok_or_else(|| {
                    TypeError::new(
                        TypeErrorKind::UnsupportedConstruct,
                        format!("unsupported self reference primitive `@{primitive}`"),
                        self_reference.span,
                        false,
                    )
                })?;
                Ok(Inference::pure(ty))
            }
            ExpressionKind::String(_) => Err(TypeError::new(
                TypeErrorKind::UnsupportedConstruct,
                "string literal typing is deferred in Stage 0",
                expression.span,
                false,
            )),
            ExpressionKind::Boolean(_) => Ok(Inference::pure(Type::bool())),
            ExpressionKind::Group(group) => self.infer_expression(&group.expression),
            ExpressionKind::Try(try_expression) => self.infer_try_expression(&try_expression.operand, try_expression.span),
            ExpressionKind::Block(block) => self.infer_block(block),
            ExpressionKind::If(if_expression) => self.infer_if(if_expression),
            ExpressionKind::Match(match_expression) => self.infer_match(match_expression),
            ExpressionKind::Prefix(prefix) => {
                let operand = self.infer_expression(&prefix.operand)?;
                match prefix.operator {
                    PrefixOperator::Negate => {
                        self.constrain(
                            Type::int(),
                            operand.ty,
                            prefix.span,
                            "prefix `-` requires Int",
                        )?;
                        Ok(Inference {
                            ty: Type::int(),
                            effects: operand.effects,
                        })
                    }
                }
            }
            ExpressionKind::Binary { left, operator, right } => {
                self.infer_binary_expression(left, *operator, right, expression.span)
            }
            ExpressionKind::Call(call) => self.infer_call_expression(call.span, &call.callee, &call.arguments),
        }
    }

    fn infer_block(&mut self, block: &BlockExpression) -> Result<Inference, TypeError> {
        let saved_environment = self.environment.clone();
        let mut block_effects = EffectSet::pure();
        self.propagation_stack.push(PropagationContext::default());

        for statement in &block.statements {
            let effects = self.infer_statement(statement)?;
            block_effects = block_effects.union(&effects);
        }

        let mut inference = if let Some(tail_expression) = &block.tail_expression {
            let tail = self.infer_expression(tail_expression)?;
            block_effects = block_effects.union(&tail.effects);
            Inference {
                ty: tail.ty,
                effects: block_effects,
            }
        } else {
            Inference {
                ty: Type::unit(),
                effects: block_effects,
            }
        };

        let context = self.propagation_stack.pop().expect("block propagation context present");
        inference.ty = self.apply_propagation_context(self.substitution.apply_type(&inference.ty), context);

        self.environment = saved_environment;
        Ok(inference)
    }

    fn infer_try_expression(
        &mut self,
        operand: &Expression,
        span: SourceSpan,
    ) -> Result<Inference, TypeError> {
        let operand = self.infer_expression(operand)?;
        let operand_type = self.substitution.apply_type(&operand.ty);

        match operand_type {
            Type::Result(result) => {
                let error_type = self.expect_error_type(result.err.as_ref(), span)?;
                self.register_result_propagation(error_type, span)?;
                Ok(Inference {
                    ty: self.substitution.apply_type(result.ok.as_ref()),
                    effects: operand.effects,
                })
            }
            Type::Option(body) => {
                self.register_option_propagation(span)?;
                Ok(Inference {
                    ty: self.substitution.apply_type(body.as_ref()),
                    effects: operand.effects,
                })
            }
            other => Err(TypeError::new(
                TypeErrorKind::TypeMismatch,
                format!(
                    "postfix `?` requires Option[T] or Result[T, Error[..]]: found {}",
                    other.canonical_name()
                ),
                span,
                false,
            )),
        }
    }

    fn register_result_propagation(
        &mut self,
        error_type: ErrorType,
        span: SourceSpan,
    ) -> Result<(), TypeError> {
        let Some(context) = self.propagation_stack.last_mut() else {
            return Err(TypeError::new(
                TypeErrorKind::UnsupportedConstruct,
                "`?` requires an enclosing propagation scope",
                span,
                false,
            ));
        };

        match &mut context.mode {
            None => context.mode = Some(PropagationMode::Result(error_type.with_trace())),
            Some(PropagationMode::Result(existing)) => {
                *existing = existing.compose(&error_type.with_trace());
            }
            Some(PropagationMode::Option) => {
                return Err(TypeError::new(
                    TypeErrorKind::UnsupportedConstruct,
                    "Stage 0 cannot mix Option and Result propagation in the same scope",
                    span,
                    false,
                ));
            }
        }

        Ok(())
    }

    fn register_option_propagation(&mut self, span: SourceSpan) -> Result<(), TypeError> {
        let Some(context) = self.propagation_stack.last_mut() else {
            return Err(TypeError::new(
                TypeErrorKind::UnsupportedConstruct,
                "`?` requires an enclosing propagation scope",
                span,
                false,
            ));
        };

        match context.mode.clone() {
            None | Some(PropagationMode::Option) => {
                context.mode = Some(PropagationMode::Option);
                Ok(())
            }
            Some(PropagationMode::Result(_)) => Err(TypeError::new(
                TypeErrorKind::UnsupportedConstruct,
                "Stage 0 cannot mix Option and Result propagation in the same scope",
                span,
                false,
            )),
        }
    }

    fn expect_error_type(&self, ty: &Type, span: SourceSpan) -> Result<ErrorType, TypeError> {
        match ty {
            Type::Error(error) => Ok((**error).clone()),
            other => Err(TypeError::new(
                TypeErrorKind::TypeMismatch,
                format!(
                    "Result propagation requires an Error payload: found {}",
                    other.canonical_name()
                ),
                span,
                false,
            )),
        }
    }

    fn apply_propagation_context(&self, body_type: Type, context: PropagationContext) -> Type {
        match context.mode {
            None => body_type,
            Some(PropagationMode::Option) => Type::option(body_type),
            Some(PropagationMode::Result(error)) => Type::result(body_type, Type::Error(Box::new(error))),
        }
    }

    fn infer_if(&mut self, if_expression: &IfExpression) -> Result<Inference, TypeError> {
        let condition = self.infer_expression(&if_expression.condition)?;
        self.constrain(
            Type::bool(),
            condition.ty,
            if_expression.condition.span,
            "if condition must have type Bool",
        )?;

        let then_branch = self.infer_expression(&if_expression.then_branch)?;
        let else_branch = if let Some(else_branch) = &if_expression.else_branch {
            self.infer_expression(else_branch)?
        } else {
            self.constrain(
                Type::unit(),
                then_branch.ty.clone(),
                if_expression.then_branch.span,
                "if without else must produce Unit",
            )?;
            Inference::pure(Type::unit())
        };

        self.constrain(
            then_branch.ty.clone(),
            else_branch.ty.clone(),
            if_expression.span,
            "if branches must have the same type",
        )?;

        Ok(Inference {
            ty: self.substitution.apply_type(&then_branch.ty),
            effects: condition
                .effects
                .union(&then_branch.effects)
                .union(&else_branch.effects),
        })
    }

    fn infer_match(&mut self, match_expression: &MatchExpression) -> Result<Inference, TypeError> {
        let scrutinee = self.infer_expression(&match_expression.scrutinee)?;
        let result_type = self.fresh_type();
        let mut total_effects = scrutinee.effects.clone();
        let base_environment = self.environment.clone();

        for arm in &match_expression.arms {
            self.environment = base_environment.clone();
            self.bind_match_pattern(&arm.pattern, &scrutinee.ty)?;
            let body = self.infer_expression(&arm.body)?;
            self.constrain(
                result_type.clone(),
                body.ty,
                arm.span,
                "match arms must have the same type",
            )?;
            total_effects = total_effects.union(&body.effects);
        }

        self.environment = base_environment;
        Ok(Inference {
            ty: self.substitution.apply_type(&result_type),
            effects: total_effects,
        })
    }

    fn infer_binary_expression(
        &mut self,
        left: &Expression,
        operator: BinaryOperator,
        right: &Expression,
        span: SourceSpan,
    ) -> Result<Inference, TypeError> {
        let left = self.infer_expression(left)?;
        let right = self.infer_expression(right)?;
        let effects = left.effects.union(&right.effects);

        let ty = match operator {
            BinaryOperator::LogicalOr | BinaryOperator::LogicalAnd => {
                self.constrain(Type::bool(), left.ty, span, "logical operands must be Bool")?;
                self.constrain(Type::bool(), right.ty, span, "logical operands must be Bool")?;
                Type::bool()
            }
            BinaryOperator::Equal | BinaryOperator::NotEqual => {
                self.constrain(left.ty.clone(), right.ty.clone(), span, "equality operands must agree")?;
                Type::bool()
            }
            BinaryOperator::Less
            | BinaryOperator::LessEqual
            | BinaryOperator::Greater
            | BinaryOperator::GreaterEqual => {
                self.constrain(Type::int(), left.ty, span, "comparison operands must be Int")?;
                self.constrain(Type::int(), right.ty, span, "comparison operands must be Int")?;
                Type::bool()
            }
            BinaryOperator::Add
            | BinaryOperator::Subtract
            | BinaryOperator::Multiply
            | BinaryOperator::Divide
            | BinaryOperator::Modulo => {
                self.constrain(Type::int(), left.ty, span, "arithmetic operands must be Int")?;
                self.constrain(Type::int(), right.ty, span, "arithmetic operands must be Int")?;
                Type::int()
            }
        };

        Ok(Inference { ty, effects })
    }

    fn infer_call_expression(
        &mut self,
        span: SourceSpan,
        callee: &Expression,
        arguments: &[Expression],
    ) -> Result<Inference, TypeError> {
        let callee = self.infer_expression(callee)?;
        let mut argument_types = Vec::with_capacity(arguments.len());
        let mut effects = callee.effects.clone();

        for argument in arguments {
            let inferred = self.infer_expression(argument)?;
            effects = effects.union(&inferred.effects);
            argument_types.push(inferred.ty);
        }

        let callee_type = self.substitution.apply_type(&callee.ty);
        let function = match callee_type {
            Type::Function(function) => function,
            other => {
                return Err(TypeError::new(
                    TypeErrorKind::TypeMismatch,
                    format!(
                        "call target must be a function with matching arity: found {}",
                        other.canonical_name()
                    ),
                    span,
                    false,
                ));
            }
        };

        if function.parameters.len() != argument_types.len() {
            return Err(TypeError::new(
                TypeErrorKind::ArityMismatch,
                format!(
                    "call target must be a function with matching arity: expected {} arguments but found {}",
                    function.parameters.len(),
                    argument_types.len()
                ),
                span,
                false,
            ));
        }

        for (expected_parameter, actual_argument) in function.parameters.iter().zip(argument_types.iter()) {
            self.unify(
                expected_parameter.clone(),
                actual_argument.clone(),
                span,
                "call argument type must match parameter type",
            )?;
        }

        effects = effects.union(&function.effects);

        Ok(Inference {
            ty: self.substitution.apply_type(function.result.as_ref()),
            effects,
        })
    }

    fn fresh_type(&mut self) -> Type {
        let variable = TypeVariableId(self.next_type_variable);
        self.next_type_variable += 1;
        Type::Variable(variable)
    }

    fn instantiate(&mut self, scheme: &TypeScheme) -> Type {
        let substitutions: BTreeMap<_, _> = scheme
            .variables
            .iter()
            .map(|variable| (*variable, self.fresh_type()))
            .collect();
        scheme.body.substitute(&substitutions)
    }

    fn generalize(&self, environment: &TypeEnvironment, ty: Type) -> TypeScheme {
        let mut variables: Vec<_> = ty
            .free_type_variables()
            .difference(&environment.free_type_variables())
            .copied()
            .collect();
        variables.sort();
        TypeScheme { variables, body: ty }
    }

    fn constrain(
        &mut self,
        expected: Type,
        actual: Type,
        span: SourceSpan,
        reason: &'static str,
    ) -> Result<(), TypeError> {
        let constraint = TypeConstraint {
            expected,
            actual,
            span,
            reason,
        };
        self.constraints.push(constraint.clone());
        self.solve_constraint(constraint)
    }

    fn solve_constraint(&mut self, constraint: TypeConstraint) -> Result<(), TypeError> {
        self.unify(constraint.expected, constraint.actual, constraint.span, constraint.reason)
    }

    fn unify(
        &mut self,
        expected: Type,
        actual: Type,
        span: SourceSpan,
        reason: &'static str,
    ) -> Result<(), TypeError> {
        let expected = self.substitution.apply_type(&expected);
        let actual = self.substitution.apply_type(&actual);

        if expected == actual {
            return Ok(());
        }

        match (expected, actual) {
            (Type::Variable(variable), ty) | (ty, Type::Variable(variable)) => {
                self.bind_variable(variable, ty, span, reason)
            }
            (Type::Primitive(left), Type::Primitive(right)) => Err(TypeError::new(
                TypeErrorKind::TypeMismatch,
                format!(
                    "{}: expected {} but found {}",
                    reason,
                    left.as_str(),
                    right.as_str()
                ),
                span,
                false,
            )),
            (Type::Resource(left), Type::Resource(right)) => Err(TypeError::new(
                TypeErrorKind::TypeMismatch,
                format!(
                    "{}: expected {} but found {}",
                    reason,
                    left.as_str(),
                    right.as_str()
                ),
                span,
                false,
            )),
            (Type::Meta(left), Type::Meta(right)) => Err(TypeError::new(
                TypeErrorKind::TypeMismatch,
                format!(
                    "{}: expected {} but found {}",
                    reason,
                    left.as_str(),
                    right.as_str()
                ),
                span,
                false,
            )),
            (Type::Error(left), Type::Error(right)) => Err(TypeError::new(
                TypeErrorKind::TypeMismatch,
                format!(
                    "{}: expected {} but found {}",
                    reason,
                    left.canonical_name(),
                    right.canonical_name()
                ),
                span,
                false,
            )),
            (Type::Option(left), Type::Option(right)) => self.unify(*left, *right, span, reason),
            (Type::Task(left), Type::Task(right)) => self.unify(*left, *right, span, reason),
            (Type::Channel(left), Type::Channel(right)) => self.unify(*left, *right, span, reason),
            (Type::Result(left), Type::Result(right)) => {
                self.unify(*left.ok, *right.ok, span, reason)?;
                self.unify(*left.err, *right.err, span, reason)
            }
            (Type::Evidence(left), Type::Evidence(right)) => Err(TypeError::new(
                TypeErrorKind::TypeMismatch,
                format!(
                    "{}: expected {} but found {}",
                    reason,
                    Type::Evidence(left).canonical_name(),
                    Type::Evidence(right).canonical_name()
                ),
                span,
                false,
            )),
            (Type::Modal(left), Type::Modal(right)) => {
                if left.modality != right.modality {
                    return Err(TypeError::new(
                        TypeErrorKind::TypeMismatch,
                        format!(
                            "{}: expected {} but found {}",
                            reason,
                            Type::Modal(left).canonical_name(),
                            Type::Modal(right).canonical_name()
                        ),
                        span,
                        false,
                    ));
                }
                self.unify(*left.body, *right.body, span, reason)
            }
            (Type::Temporal(left), Type::Temporal(right)) => self.unify(*left.body, *right.body, span, reason),
            (Type::Product(left), Type::Product(right)) | (Type::Sum(left), Type::Sum(right)) => {
                if left.len() != right.len() {
                    return Err(TypeError::new(
                        TypeErrorKind::ArityMismatch,
                        format!(
                            "{}: expected {} members but found {}",
                            reason,
                            left.len(),
                            right.len()
                        ),
                        span,
                        false,
                    ));
                }

                for (left_member, right_member) in left.into_iter().zip(right) {
                    self.unify(left_member, right_member, span, reason)?;
                }
                Ok(())
            }
            (Type::Function(left), Type::Function(right)) => {
                if left.parameters.len() != right.parameters.len() {
                    return Err(TypeError::new(
                        TypeErrorKind::ArityMismatch,
                        format!(
                            "{}: expected {} arguments but found {}",
                            reason,
                            left.parameters.len(),
                            right.parameters.len()
                        ),
                        span,
                        false,
                    ));
                }
                if left.effects != right.effects {
                    return Err(TypeError::new(
                        TypeErrorKind::TypeMismatch,
                        format!(
                            "{}: expected effects {} but found {}",
                            reason,
                            left.effects.canonical_name(),
                            right.effects.canonical_name()
                        ),
                        span,
                        false,
                    ));
                }

                for (left_parameter, right_parameter) in left.parameters.into_iter().zip(right.parameters) {
                    self.unify(left_parameter, right_parameter, span, reason)?;
                }
                self.unify(*left.result, *right.result, span, reason)
            }
            (expected, actual) => Err(TypeError::new(
                TypeErrorKind::TypeMismatch,
                format!(
                    "{}: expected {} but found {}",
                    reason,
                    expected.canonical_name(),
                    actual.canonical_name()
                ),
                span,
                false,
            )),
        }
    }

    fn bind_variable(
        &mut self,
        variable: TypeVariableId,
        ty: Type,
        span: SourceSpan,
        reason: &'static str,
    ) -> Result<(), TypeError> {
        if ty.contains_variable(variable) {
            return Err(TypeError::new(
                TypeErrorKind::OccursCheckFailed,
                format!(
                    "{}: occurs check failed while binding {} to {}",
                    reason,
                    variable.canonical_name(),
                    ty.canonical_name()
                ),
                span,
                false,
            ));
        }
        self.substitution.types.insert(variable, ty);
        Ok(())
    }
}
