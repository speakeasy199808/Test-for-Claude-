//! Internal evaluator for the seed LyraLang formal semantics pass.

use std::collections::BTreeMap;

use crate::checker::ProgramJudgment;
use crate::lexer::SourceSpan;
use crate::parser::{
    BinaryOperator, BlockExpression, Expression, ExpressionKind, IfExpression, MatchExpression,
    Pattern, PatternKind, PrefixOperator, Program, Statement,
};
use crate::semantics::error::{SemanticsError, SemanticsErrorKind};
use crate::semantics::{
    BindingDenotation, OperationalStep, SemanticJudgment, SemanticValue, SemanticsOutput,
};

#[derive(Debug, Clone)]
enum RuntimeValue {
    Unit,
    Bool(bool),
    Int(i64),
    Rational { numerator: i64, denominator: i64 },
    Resource { kind: &'static str, id: u32 },
    Meta(&'static str),
    Evidence(&'static str),
    Modal { modality: &'static str, body: Box<RuntimeValue> },
}

impl RuntimeValue {
    fn to_semantic_value(&self) -> SemanticValue {
        match self {
            Self::Unit => SemanticValue::Unit,
            Self::Bool(value) => SemanticValue::Bool(*value),
            Self::Int(value) => SemanticValue::Int(*value),
            Self::Rational { numerator, denominator } => SemanticValue::Rational {
                numerator: *numerator,
                denominator: *denominator,
            },
            Self::Resource { kind, id } => SemanticValue::Resource {
                kind: (*kind).to_string(),
                id: *id,
            },
            Self::Meta(name) => SemanticValue::Meta((*name).to_string()),
            Self::Evidence(kind) => SemanticValue::Evidence((*kind).to_string()),
            Self::Modal { modality, body } => SemanticValue::Modal {
                modality: (*modality).to_string(),
                body: Box::new(body.to_semantic_value()),
            },
        }
    }

    fn render(&self) -> String {
        match self {
            Self::Unit => "Unit".to_string(),
            Self::Bool(value) => format!("Bool({value})"),
            Self::Int(value) => format!("Int({value})"),
            Self::Rational { numerator, denominator } => {
                format!("Rational({numerator}/{denominator})")
            }
            Self::Resource { kind, id } => format!("Resource({kind}#{id})"),
            Self::Meta(name) => format!("Meta({name})"),
            Self::Evidence(kind) => format!("Evidence({kind})"),
            Self::Modal { modality, body } => {
                format!("{modality}[{}]", body.render())
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
struct RuntimeEnvironment {
    values: BTreeMap<String, RuntimeValue>,
}

/// Internal semantics evaluator.
#[derive(Debug, Clone, Default)]
pub struct SemanticsEvaluator {
    next_step_index: u32,
    next_resource_id: u32,
    environment: RuntimeEnvironment,
    bindings: Vec<BindingDenotation>,
    steps: Vec<OperationalStep>,
}

impl SemanticsEvaluator {
    /// Evaluates a typed program and returns the executable semantics judgment.
    pub fn evaluate_program(
        mut self,
        normalized_source: String,
        program: &Program,
        type_judgment: &ProgramJudgment,
    ) -> Result<SemanticsOutput, SemanticsError> {
        let value = self.eval_program(program)?;
        Ok(SemanticsOutput {
            normalized_source,
            judgment: Some(SemanticJudgment {
                module: program.module_decl.as_ref().map(|decl| decl.name.text.clone()),
                program_type: type_judgment.program_type.canonical_name(),
                denotation: value.to_semantic_value(),
                denotation_rendered: value.render(),
                bindings: self.bindings,
                steps: self.steps,
                soundness_statement: "If Γ ⊢ e : τ and semantic execution succeeds with e ⇓ v, then v inhabits the canonical domain denoted by τ for the current Stage 0 subset.".to_string(),
                span: program.span,
            }),
            errors: Vec::new(),
        })
    }

    fn eval_program(&mut self, program: &Program) -> Result<RuntimeValue, SemanticsError> {
        for statement in &program.statements {
            self.eval_statement(statement)?;
        }

        if let Some(tail) = &program.tail_expression {
            self.eval_expression(tail)
        } else {
            self.record_step("E-ProgramUnit", "program without tail expression denotes Unit", SourceSpan::default());
            Ok(RuntimeValue::Unit)
        }
    }

    fn eval_statement(&mut self, statement: &Statement) -> Result<(), SemanticsError> {
        match statement {
            Statement::Let(statement) => {
                let value = self.eval_expression(&statement.value)?;
                self.bind_pattern(&statement.pattern, &value)?;
                Ok(())
            }
            Statement::Expr(statement) => {
                let value = self.eval_expression(&statement.expression)?;
                self.record_step(
                    "E-ExprStmt",
                    format!("terminated expression statement produced {}", value.render()),
                    statement.span,
                );
                Ok(())
            }
        }
    }

    fn bind_pattern(&mut self, pattern: &Pattern, value: &RuntimeValue) -> Result<(), SemanticsError> {
        match &pattern.kind {
            PatternKind::Wildcard => {
                self.record_step("E-LetWildcard", format!("discarded {} into _", value.render()), pattern.span);
                Ok(())
            }
            PatternKind::Identifier(identifier) => {
                self.environment.values.insert(identifier.text.clone(), value.clone());
                self.bindings.push(BindingDenotation {
                    name: identifier.text.clone(),
                    value: value.to_semantic_value(),
                    rendered: value.render(),
                    span: identifier.span,
                });
                self.record_step(
                    "E-LetBind",
                    format!("bound {} = {}", identifier.text, value.render()),
                    identifier.span,
                );
                Ok(())
            }
            PatternKind::Integer(expected) => {
                let matches = matches!(value, RuntimeValue::Int(actual) if actual.to_string() == *expected);
                if matches {
                    self.record_step("E-PatInt", format!("matched integer pattern {expected}"), pattern.span);
                    Ok(())
                } else {
                    Err(SemanticsError::new(
                        SemanticsErrorKind::UnsupportedConstruct,
                        format!("integer pattern {expected} did not match {}", value.render()),
                        pattern.span,
                        false,
                    ))
                }
            }
            PatternKind::Boolean(expected) => {
                let matches = matches!(value, RuntimeValue::Bool(actual) if actual == expected);
                if matches {
                    self.record_step("E-PatBool", format!("matched boolean pattern {expected}"), pattern.span);
                    Ok(())
                } else {
                    Err(SemanticsError::new(
                        SemanticsErrorKind::UnsupportedConstruct,
                        format!("boolean pattern {expected} did not match {}", value.render()),
                        pattern.span,
                        false,
                    ))
                }
            }
            PatternKind::String(expected) => Err(SemanticsError::new(
                SemanticsErrorKind::UnsupportedConstruct,
                format!("string pattern `{expected}` is outside the Stage 0 executable semantic subset"),
                pattern.span,
                false,
            )),
        }
    }

    fn eval_expression(&mut self, expression: &Expression) -> Result<RuntimeValue, SemanticsError> {
        match &expression.kind {
            ExpressionKind::Identifier(identifier) => {
                let value = self.environment.values.get(&identifier.text).cloned().ok_or_else(|| {
                    SemanticsError::new(
                        SemanticsErrorKind::UnknownIdentifier,
                        format!("unknown identifier `{}` during semantic execution", identifier.text),
                        identifier.span,
                        false,
                    )
                })?;
                self.record_step("E-Var", format!("{} ↦ {}", identifier.text, value.render()), identifier.span);
                Ok(value)
            }
            ExpressionKind::Integer(value) => {
                let parsed = value.parse::<i64>().map_err(|_| {
                    SemanticsError::new(
                        SemanticsErrorKind::UnsupportedConstruct,
                        format!("integer literal `{value}` exceeds Stage 0 semantic range"),
                        expression.span,
                        false,
                    )
                })?;
                let result = RuntimeValue::Int(parsed);
                self.record_step("E-Int", result.render(), expression.span);
                Ok(result)
            }
            ExpressionKind::Boolean(value) => {
                let result = RuntimeValue::Bool(*value);
                self.record_step("E-Bool", result.render(), expression.span);
                Ok(result)
            }
            ExpressionKind::String(value) => Err(SemanticsError::new(
                SemanticsErrorKind::UnsupportedConstruct,
                format!("string literal `{value}` is outside the Stage 0 executable semantic subset"),
                expression.span,
                false,
            )),
            ExpressionKind::SelfReference(self_reference) => {
                let value = match self_reference.primitive.as_str() {
                    "current_program" => RuntimeValue::Meta("current_program"),
                    "current_receipt" => RuntimeValue::Meta("current_receipt"),
                    "ledger_state" => RuntimeValue::Meta("ledger_state"),
                    other => {
                        return Err(SemanticsError::new(
                            SemanticsErrorKind::UnsupportedConstruct,
                            format!("unsupported self reference primitive `@{other}`"),
                            self_reference.span,
                            false,
                        ))
                    }
                };
                self.record_step(
                    "E-SelfRef",
                    format!("@{}() ⇓ {}", self_reference.primitive.as_str(), value.render()),
                    self_reference.span,
                );
                Ok(value)
            }
            ExpressionKind::Group(group) => self.eval_expression(&group.expression),
            ExpressionKind::Try(try_expression) => Err(SemanticsError::new(
                SemanticsErrorKind::UnsupportedConstruct,
                "postfix `?` evaluation is deferred to a later runtime slice",
                try_expression.span,
                false,
            )),
            ExpressionKind::Block(block) => self.eval_block(block),
            ExpressionKind::If(if_expression) => self.eval_if(if_expression),
            ExpressionKind::Match(match_expression) => self.eval_match(match_expression),
            ExpressionKind::Prefix(prefix) => {
                let operand = self.eval_expression(&prefix.operand)?;
                match prefix.operator {
                    PrefixOperator::Negate => {
                        let value = match operand {
                            RuntimeValue::Int(int) => RuntimeValue::Int(-int),
                            other => {
                                return Err(SemanticsError::new(
                                    SemanticsErrorKind::UnsupportedConstruct,
                                    format!("prefix negation expected Int but found {}", other.render()),
                                    prefix.span,
                                    false,
                                ))
                            }
                        };
                        self.record_step("E-Neg", value.render(), prefix.span);
                        Ok(value)
                    }
                }
            }
            ExpressionKind::Binary { left, operator, right } => {
                self.eval_binary_expression(left, *operator, right, expression.span)
            }
            ExpressionKind::Call(call) => {
                self.eval_call_expression(call.span, &call.callee, &call.arguments)
            }
        }
    }

    fn eval_block(&mut self, block: &BlockExpression) -> Result<RuntimeValue, SemanticsError> {
        let saved_environment = self.environment.clone();
        for statement in &block.statements {
            self.eval_statement(statement)?;
        }

        let result = if let Some(tail) = &block.tail_expression {
            self.eval_expression(tail)?
        } else {
            RuntimeValue::Unit
        };

        self.record_step("E-Block", format!("block ⇓ {}", result.render()), block.span);
        self.environment = saved_environment;
        Ok(result)
    }

    fn eval_if(&mut self, if_expression: &IfExpression) -> Result<RuntimeValue, SemanticsError> {
        let condition = self.eval_expression(&if_expression.condition)?;
        match condition {
            RuntimeValue::Bool(true) => {
                let value = self.eval_expression(&if_expression.then_branch)?;
                self.record_step("E-IfTrue", format!("if-then ⇓ {}", value.render()), if_expression.span);
                Ok(value)
            }
            RuntimeValue::Bool(false) => {
                if let Some(else_branch) = &if_expression.else_branch {
                    let value = self.eval_expression(else_branch)?;
                    self.record_step("E-IfFalse", format!("if-else ⇓ {}", value.render()), if_expression.span);
                    Ok(value)
                } else {
                    self.record_step("E-IfUnit", "if without else ⇓ Unit", if_expression.span);
                    Ok(RuntimeValue::Unit)
                }
            }
            other => Err(SemanticsError::new(
                SemanticsErrorKind::UnsupportedConstruct,
                format!("if condition expected Bool but found {}", other.render()),
                if_expression.condition.span,
                false,
            )),
        }
    }

    fn eval_match(&mut self, match_expression: &MatchExpression) -> Result<RuntimeValue, SemanticsError> {
        let scrutinee = self.eval_expression(&match_expression.scrutinee)?;
        for arm in &match_expression.arms {
            let saved_environment = self.environment.clone();
            if self.pattern_matches(&arm.pattern, &scrutinee)? {
                self.bind_match_pattern(&arm.pattern, &scrutinee)?;
                let value = self.eval_expression(&arm.body)?;
                self.record_step("E-MatchArm", format!("match arm ⇓ {}", value.render()), arm.span);
                self.environment = saved_environment;
                return Ok(value);
            }
            self.environment = saved_environment;
        }

        Err(SemanticsError::new(
            SemanticsErrorKind::UnsupportedConstruct,
            "non-exhaustive match reached semantic execution",
            match_expression.span,
            false,
        ))
    }

    fn pattern_matches(&self, pattern: &Pattern, value: &RuntimeValue) -> Result<bool, SemanticsError> {
        Ok(match &pattern.kind {
            PatternKind::Wildcard => true,
            PatternKind::Identifier(_) => true,
            PatternKind::Integer(expected) => matches!(value, RuntimeValue::Int(actual) if actual.to_string() == *expected),
            PatternKind::Boolean(expected) => matches!(value, RuntimeValue::Bool(actual) if actual == expected),
            PatternKind::String(sample) => {
                return Err(SemanticsError::new(
                    SemanticsErrorKind::UnsupportedConstruct,
                    format!("string pattern `{sample}` is outside the Stage 0 executable semantic subset"),
                    pattern.span,
                    false,
                ))
            }
        })
    }

    fn bind_match_pattern(&mut self, pattern: &Pattern, value: &RuntimeValue) -> Result<(), SemanticsError> {
        match &pattern.kind {
            PatternKind::Identifier(identifier) => {
                self.environment.values.insert(identifier.text.clone(), value.clone());
                self.record_step("E-MatchBind", format!("{} ↦ {}", identifier.text, value.render()), identifier.span);
                Ok(())
            }
            PatternKind::Wildcard | PatternKind::Integer(_) | PatternKind::Boolean(_) => Ok(()),
            PatternKind::String(sample) => Err(SemanticsError::new(
                SemanticsErrorKind::UnsupportedConstruct,
                format!("string pattern `{sample}` is outside the Stage 0 executable semantic subset"),
                pattern.span,
                false,
            )),
        }
    }

    fn eval_binary_expression(
        &mut self,
        left: &Expression,
        operator: BinaryOperator,
        right: &Expression,
        span: SourceSpan,
    ) -> Result<RuntimeValue, SemanticsError> {
        let left = self.eval_expression(left)?;
        let right = self.eval_expression(right)?;

        let value = match operator {
            BinaryOperator::LogicalOr => RuntimeValue::Bool(self.expect_bool(&left, span)? || self.expect_bool(&right, span)?),
            BinaryOperator::LogicalAnd => RuntimeValue::Bool(self.expect_bool(&left, span)? && self.expect_bool(&right, span)?),
            BinaryOperator::Equal => RuntimeValue::Bool(left.render() == right.render()),
            BinaryOperator::NotEqual => RuntimeValue::Bool(left.render() != right.render()),
            BinaryOperator::Less => RuntimeValue::Bool(self.expect_int(&left, span)? < self.expect_int(&right, span)?),
            BinaryOperator::LessEqual => RuntimeValue::Bool(self.expect_int(&left, span)? <= self.expect_int(&right, span)?),
            BinaryOperator::Greater => RuntimeValue::Bool(self.expect_int(&left, span)? > self.expect_int(&right, span)?),
            BinaryOperator::GreaterEqual => RuntimeValue::Bool(self.expect_int(&left, span)? >= self.expect_int(&right, span)?),
            BinaryOperator::Add => RuntimeValue::Int(self.expect_int(&left, span)? + self.expect_int(&right, span)?),
            BinaryOperator::Subtract => RuntimeValue::Int(self.expect_int(&left, span)? - self.expect_int(&right, span)?),
            BinaryOperator::Multiply => RuntimeValue::Int(self.expect_int(&left, span)? * self.expect_int(&right, span)?),
            BinaryOperator::Divide => RuntimeValue::Int(self.expect_int(&left, span)? / self.expect_int(&right, span)?),
            BinaryOperator::Modulo => RuntimeValue::Int(self.expect_int(&left, span)? % self.expect_int(&right, span)?),
        };

        self.record_step("E-Binary", format!("binary ⇓ {}", value.render()), span);
        Ok(value)
    }

    fn eval_call_expression(
        &mut self,
        span: SourceSpan,
        callee: &Expression,
        arguments: &[Expression],
    ) -> Result<RuntimeValue, SemanticsError> {
        let name = match &callee.kind {
            ExpressionKind::Identifier(identifier) => identifier.text.clone(),
            _ => {
                return Err(SemanticsError::new(
                    SemanticsErrorKind::UnsupportedConstruct,
                    "Stage 0 semantic execution only supports builtin identifier call targets",
                    callee.span,
                    false,
                ))
            }
        };

        let values = arguments
            .iter()
            .map(|argument| self.eval_expression(argument))
            .collect::<Result<Vec<_>, _>>()?;

        let result = self.apply_builtin(&name, &values, span)?;
        self.record_step(
            "E-Call",
            format!("{name}({}) ⇓ {}", values.iter().map(RuntimeValue::render).collect::<Vec<_>>().join(", "), result.render()),
            span,
        );
        Ok(result)
    }

    fn apply_builtin(
        &mut self,
        name: &str,
        values: &[RuntimeValue],
        span: SourceSpan,
    ) -> Result<RuntimeValue, SemanticsError> {
        match name {
            "id" => Ok(values.get(0).cloned().ok_or_else(|| self.invalid_builtin(name, "expected 1 argument", span))?),
            "add" => Ok(RuntimeValue::Int(self.expect_int_index(values, 0, name, span)? + self.expect_int_index(values, 1, name, span)?)),
            "eq_int" => Ok(RuntimeValue::Bool(self.expect_int_index(values, 0, name, span)? == self.expect_int_index(values, 1, name, span)?)),
            "eq" => {
                let left = values.get(0).ok_or_else(|| self.invalid_builtin(name, "expected 2 arguments", span))?;
                let right = values.get(1).ok_or_else(|| self.invalid_builtin(name, "expected 2 arguments", span))?;
                Ok(RuntimeValue::Bool(left.render() == right.render()))
            },
            "not" => Ok(RuntimeValue::Bool(!self.expect_bool_index(values, 0, name, span)?)),
            "nat_succ" => Ok(RuntimeValue::Int(self.expect_int_index(values, 0, name, span)? + 1)),
            "ratio_from_ints" => Ok(RuntimeValue::Rational {
                numerator: self.expect_int_index(values, 0, name, span)?,
                denominator: self.expect_int_index(values, 1, name, span)?,
            }),
            "print_int" => {
                let value = self.expect_int_index(values, 0, name, span)?;
                self.record_step("E-EffectIo", format!("print_int observed Int({value})"), span);
                Ok(RuntimeValue::Unit)
            }
            "print" => {
                let value = values.get(0).ok_or_else(|| self.invalid_builtin(name, "expected 1 argument", span))?;
                self.record_step("E-EffectIo", format!("print observed {}", value.render()), span);
                Ok(RuntimeValue::Unit)
            },
            "read_clock" => {
                self.record_step("E-EffectTime", "read_clock observed deterministic tick 0", span);
                Ok(RuntimeValue::Int(0))
            }
            "entropy_u64" => {
                self.record_step("E-EffectEntropy", "entropy_u64 observed deterministic seed 0", span);
                Ok(RuntimeValue::Int(0))
            }
            "touch_state" => {
                let value = self.expect_int_index(values, 0, name, span)?;
                self.record_step("E-EffectState", format!("touch_state preserved Int({value})"), span);
                Ok(RuntimeValue::Int(value))
            }
            "prove_eq_int" => Ok(RuntimeValue::Bool(self.expect_int_index(values, 0, name, span)? == self.expect_int_index(values, 1, name, span)?)),
            "open_file" => Ok(self.fresh_resource("File")),
            "close_file" => { self.expect_resource_kind(values, 0, "File", name, span)?; Ok(RuntimeValue::Unit) }
            "open_socket" => Ok(self.fresh_resource("Socket")),
            "close_socket" => { self.expect_resource_kind(values, 0, "Socket", name, span)?; Ok(RuntimeValue::Unit) }
            "grant_capability" => { let _ = self.expect_int_index(values, 0, name, span)?; Ok(self.fresh_resource("Capability")) }
            "consume_capability" => { self.expect_resource_kind(values, 0, "Capability", name, span)?; Ok(RuntimeValue::Unit) }
            "observation_evidence" => Ok(RuntimeValue::Evidence("observation")),
            "proof_evidence" => Ok(RuntimeValue::Evidence("proof")),
            "necessity_evidence" => Ok(RuntimeValue::Evidence("necessity")),
            "possibility_evidence" => Ok(RuntimeValue::Evidence("possibility")),
            "assume_unknown" => Ok(RuntimeValue::Modal { modality: "Unknown", body: Box::new(values.get(0).cloned().ok_or_else(|| self.invalid_builtin(name, "expected 1 argument", span))?) }),
            "promote_unknown_to_hypothesis" => { self.expect_modal(values, 0, "Unknown", name, span)?; self.expect_evidence(values, 1, "observation", name, span)?; Ok(RuntimeValue::Modal { modality: "Hypothesis", body: Box::new(self.unwrap_modal(values[0].clone())) }) }
            "promote_hypothesis_to_fact" => { self.expect_modal(values, 0, "Hypothesis", name, span)?; self.expect_evidence(values, 1, "proof", name, span)?; Ok(RuntimeValue::Modal { modality: "Fact", body: Box::new(self.unwrap_modal(values[0].clone())) }) }
            "promote_fact_to_necessary" => { self.expect_modal(values, 0, "Fact", name, span)?; self.expect_evidence(values, 1, "necessity", name, span)?; Ok(RuntimeValue::Modal { modality: "Necessary", body: Box::new(self.unwrap_modal(values[0].clone())) }) }
            "promote_unknown_to_possible" => { self.expect_modal(values, 0, "Unknown", name, span)?; self.expect_evidence(values, 1, "possibility", name, span)?; Ok(RuntimeValue::Modal { modality: "Possible", body: Box::new(self.unwrap_modal(values[0].clone())) }) }
            "promote_possible_to_fact" => { self.expect_modal(values, 0, "Possible", name, span)?; self.expect_evidence(values, 1, "proof", name, span)?; Ok(RuntimeValue::Modal { modality: "Fact", body: Box::new(self.unwrap_modal(values[0].clone())) }) }
            "weaken_necessary_to_fact" => { self.expect_modal(values, 0, "Necessary", name, span)?; Ok(RuntimeValue::Modal { modality: "Fact", body: Box::new(self.unwrap_modal(values[0].clone())) }) }
            "reveal_fact" => { self.expect_modal(values, 0, "Fact", name, span)?; Ok(self.unwrap_modal(values[0].clone())) }
            other => Err(SemanticsError::new(
                SemanticsErrorKind::InvalidBuiltin,
                format!("unsupported builtin semantic definition for `{other}`"),
                span,
                false,
            )),
        }
    }

    fn unwrap_modal(&self, value: RuntimeValue) -> RuntimeValue {
        match value {
            RuntimeValue::Modal { body, .. } => *body,
            other => other,
        }
    }

    fn expect_int(&self, value: &RuntimeValue, span: SourceSpan) -> Result<i64, SemanticsError> {
        match value {
            RuntimeValue::Int(int) => Ok(*int),
            other => Err(SemanticsError::new(
                SemanticsErrorKind::UnsupportedConstruct,
                format!("expected Int during semantic execution but found {}", other.render()),
                span,
                false,
            )),
        }
    }

    fn expect_bool(&self, value: &RuntimeValue, span: SourceSpan) -> Result<bool, SemanticsError> {
        match value {
            RuntimeValue::Bool(boolean) => Ok(*boolean),
            other => Err(SemanticsError::new(
                SemanticsErrorKind::UnsupportedConstruct,
                format!("expected Bool during semantic execution but found {}", other.render()),
                span,
                false,
            )),
        }
    }

    fn expect_int_index(&self, values: &[RuntimeValue], index: usize, name: &str, span: SourceSpan) -> Result<i64, SemanticsError> {
        let value = values.get(index).ok_or_else(|| self.invalid_builtin(name, &format!("expected argument at position {index}"), span))?;
        self.expect_int(value, span)
    }

    fn expect_bool_index(&self, values: &[RuntimeValue], index: usize, name: &str, span: SourceSpan) -> Result<bool, SemanticsError> {
        let value = values.get(index).ok_or_else(|| self.invalid_builtin(name, &format!("expected argument at position {index}"), span))?;
        self.expect_bool(value, span)
    }

    fn expect_resource_kind(&self, values: &[RuntimeValue], index: usize, kind: &str, name: &str, span: SourceSpan) -> Result<(), SemanticsError> {
        let value = values.get(index).ok_or_else(|| self.invalid_builtin(name, &format!("expected argument at position {index}"), span))?;
        match value {
            RuntimeValue::Resource { kind: actual, .. } if *actual == kind => Ok(()),
            other => Err(self.invalid_builtin(name, &format!("expected {kind} resource but found {}", other.render()), span)),
        }
    }

    fn expect_evidence(&self, values: &[RuntimeValue], index: usize, kind: &str, name: &str, span: SourceSpan) -> Result<(), SemanticsError> {
        let value = values.get(index).ok_or_else(|| self.invalid_builtin(name, &format!("expected argument at position {index}"), span))?;
        match value {
            RuntimeValue::Evidence(actual) if *actual == kind => Ok(()),
            other => Err(self.invalid_builtin(name, &format!("expected Evidence({kind}) but found {}", other.render()), span)),
        }
    }

    fn expect_modal(&self, values: &[RuntimeValue], index: usize, modality: &str, name: &str, span: SourceSpan) -> Result<(), SemanticsError> {
        let value = values.get(index).ok_or_else(|| self.invalid_builtin(name, &format!("expected argument at position {index}"), span))?;
        match value {
            RuntimeValue::Modal { modality: actual, .. } if *actual == modality => Ok(()),
            other => Err(self.invalid_builtin(name, &format!("expected {modality} modal value but found {}", other.render()), span)),
        }
    }

    fn invalid_builtin(&self, name: &str, detail: &str, span: SourceSpan) -> SemanticsError {
        SemanticsError::new(
            SemanticsErrorKind::InvalidBuiltin,
            format!("builtin `{name}`: {detail}"),
            span,
            false,
        )
    }

    fn fresh_resource(&mut self, kind: &'static str) -> RuntimeValue {
        let id = self.next_resource_id;
        self.next_resource_id += 1;
        RuntimeValue::Resource { kind, id }
    }

    fn record_step(&mut self, rule: &str, detail: impl Into<String>, span: SourceSpan) {
        let step = OperationalStep {
            index: self.next_step_index,
            rule: rule.to_string(),
            detail: detail.into(),
            span,
        };
        self.next_step_index += 1;
        self.steps.push(step);
    }
}
