//! Internal lowering engine for the seed LyraLang code generator.

use std::collections::BTreeMap;

use crate::checker::ProgramJudgment;
use crate::codegen::error::{CodegenError, CodegenErrorKind};
use crate::codegen::CodegenProgram;
use crate::lexer::SourceSpan;
use crate::parser::{
    BinaryOperator, BlockExpression, Expression, ExpressionKind, IfExpression, MatchExpression,
    Pattern, PatternKind, PrefixOperator, Program, SelfReferencePrimitive, Statement,
};
use crate::types::Type;

const FORMAT_VERSION: &str = "lyravm-stage0-ir-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Register(u32);

impl Register {
    fn render(self) -> String {
        format!("r{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Label(u32);

impl Label {
    fn render(self) -> String {
        format!("L{}", self.0)
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    ConstUnit { dst: Register },
    ConstInt { dst: Register, value: String },
    ConstBool { dst: Register, value: bool },
    Move { dst: Register, src: Register },
    SelfReference { dst: Register, primitive: SelfReferencePrimitive },
    NegInt { dst: Register, src: Register },
    Binary { dst: Register, op: &'static str, left: Register, right: Register },
    BranchIf { condition: Register, then_label: Label, else_label: Label },
    Jump { label: Label },
    Label { label: Label },
    CallBuiltin { dst: Register, name: String, args: Vec<Register> },
    AssertInt { src: Register, expected: String },
    AssertBool { src: Register, expected: bool },
    Trap { message: &'static str },
}

impl Instruction {
    fn render(&self) -> String {
        match self {
            Self::ConstUnit { dst } => format!("{} = const.unit", dst.render()),
            Self::ConstInt { dst, value } => format!("{} = const.int {}", dst.render(), value),
            Self::ConstBool { dst, value } => format!("{} = const.bool {}", dst.render(), value),
            Self::Move { dst, src } => format!("{} = move {}", dst.render(), src.render()),
            Self::SelfReference { dst, primitive } => {
                format!("{} = selfref @{}", dst.render(), primitive.as_str())
            }
            Self::NegInt { dst, src } => format!("{} = neg.int {}", dst.render(), src.render()),
            Self::Binary { dst, op, left, right } => {
                format!("{} = {} {}, {}", dst.render(), op, left.render(), right.render())
            }
            Self::BranchIf { condition, then_label, else_label } => format!(
                "branch.if {} -> {}, {}",
                condition.render(),
                then_label.render(),
                else_label.render()
            ),
            Self::Jump { label } => format!("jump {}", label.render()),
            Self::Label { label } => format!("label {}", label.render()),
            Self::CallBuiltin { dst, name, args } => format!(
                "{} = call {}({})",
                dst.render(),
                name,
                args.iter().map(|arg| arg.render()).collect::<Vec<_>>().join(", ")
            ),
            Self::AssertInt { src, expected } => {
                format!("assert.int {} == {}", src.render(), expected)
            }
            Self::AssertBool { src, expected } => {
                format!("assert.bool {} == {}", src.render(), expected)
            }
            Self::Trap { message } => format!("trap \"{}\"", message),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct LoweringEnvironment {
    values: BTreeMap<String, Register>,
}

/// Internal lowering engine.
#[derive(Debug, Clone)]
pub struct LoweringEngine {
    next_register: u32,
    next_label: u32,
    environment: LoweringEnvironment,
    instructions: Vec<Instruction>,
}

impl LoweringEngine {
    /// Creates a lowering engine seeded from successful type-checking output.
    #[must_use]
    pub fn from_type_judgment(_judgment: &ProgramJudgment) -> Self {
        Self {
            next_register: 0,
            next_label: 0,
            environment: LoweringEnvironment::default(),
            instructions: Vec::new(),
        }
    }

    /// Lowers a parsed program into deterministic Stage 0 IR.
    pub fn lower_program(
        mut self,
        program: &Program,
        program_type: &Type,
    ) -> Result<CodegenProgram, CodegenError> {
        for statement in &program.statements {
            self.lower_statement(statement)?;
        }

        let entry_register = if let Some(tail_expression) = &program.tail_expression {
            self.lower_expression(tail_expression)?
        } else {
            let dst = self.fresh_register();
            self.instructions.push(Instruction::ConstUnit { dst });
            dst
        };

        Ok(CodegenProgram {
            module: program.module_decl.as_ref().map(|declaration| declaration.name.text.clone()),
            format_version: FORMAT_VERSION.to_string(),
            register_count: self.next_register,
            entry_register: entry_register.render(),
            result_type: program_type.canonical_name(),
            instructions: self.instructions.iter().map(Instruction::render).collect(),
            span: program.span,
        })
    }

    fn lower_statement(&mut self, statement: &Statement) -> Result<(), CodegenError> {
        match statement {
            Statement::Let(statement) => {
                let value = self.lower_expression(&statement.value)?;
                self.bind_pattern(&statement.pattern, value, statement.span)
            }
            Statement::Expr(statement) => {
                let _ = self.lower_expression(&statement.expression)?;
                Ok(())
            }
        }
    }

    fn bind_pattern(
        &mut self,
        pattern: &Pattern,
        value: Register,
        span: SourceSpan,
    ) -> Result<(), CodegenError> {
        match &pattern.kind {
            PatternKind::Wildcard => Ok(()),
            PatternKind::Identifier(identifier) => {
                self.environment.values.insert(identifier.text.clone(), value);
                Ok(())
            }
            PatternKind::Integer(expected) => {
                self.instructions.push(Instruction::AssertInt {
                    src: value,
                    expected: expected.clone(),
                });
                Ok(())
            }
            PatternKind::Boolean(expected) => {
                self.instructions.push(Instruction::AssertBool {
                    src: value,
                    expected: *expected,
                });
                Ok(())
            }
            PatternKind::String(_) => Err(CodegenError::new(
                CodegenErrorKind::UnsupportedConstruct,
                "string pattern lowering is deferred in Stage 0",
                span,
                false,
            )),
        }
    }

    fn lower_expression(&mut self, expression: &Expression) -> Result<Register, CodegenError> {
        match &expression.kind {
            ExpressionKind::Identifier(identifier) => {
                let src = self.environment.values.get(&identifier.text).copied().ok_or_else(|| {
                    CodegenError::new(
                        CodegenErrorKind::UnsupportedConstruct,
                        format!("unknown identifier during codegen `{}`", identifier.text),
                        identifier.span,
                        false,
                    )
                })?;
                let dst = self.fresh_register();
                self.instructions.push(Instruction::Move { dst, src });
                Ok(dst)
            }
            ExpressionKind::Integer(value) => {
                let dst = self.fresh_register();
                self.instructions.push(Instruction::ConstInt {
                    dst,
                    value: value.clone(),
                });
                Ok(dst)
            }
            ExpressionKind::Boolean(value) => {
                let dst = self.fresh_register();
                self.instructions.push(Instruction::ConstBool { dst, value: *value });
                Ok(dst)
            }
            ExpressionKind::String(_) => Err(CodegenError::new(
                CodegenErrorKind::UnsupportedConstruct,
                "string literal lowering is deferred in Stage 0",
                expression.span,
                false,
            )),
            ExpressionKind::SelfReference(self_reference) => {
                let primitive = self_reference.primitive;
                let dst = self.fresh_register();
                self.instructions.push(Instruction::SelfReference { dst, primitive });
                Ok(dst)
            }
            ExpressionKind::Group(group) => self.lower_expression(&group.expression),
            ExpressionKind::Try(try_expression) => Err(CodegenError::new(
                CodegenErrorKind::UnsupportedConstruct,
                "postfix `?` lowering is deferred to a later runtime slice",
                try_expression.span,
                false,
            )),
            ExpressionKind::Block(block) => self.lower_block(block),
            ExpressionKind::If(if_expression) => self.lower_if(if_expression),
            ExpressionKind::Match(match_expression) => self.lower_match(match_expression),
            ExpressionKind::Prefix(prefix) => match prefix.operator {
                PrefixOperator::Negate => {
                    let src = self.lower_expression(&prefix.operand)?;
                    let dst = self.fresh_register();
                    self.instructions.push(Instruction::NegInt { dst, src });
                    Ok(dst)
                }
            },
            ExpressionKind::Binary { left, operator, right } => {
                let left = self.lower_expression(left)?;
                let right = self.lower_expression(right)?;
                let dst = self.fresh_register();
                self.instructions.push(Instruction::Binary {
                    dst,
                    op: binary_opcode(*operator),
                    left,
                    right,
                });
                Ok(dst)
            }
            ExpressionKind::Call(call) => self.lower_call(&call.callee, &call.arguments, call.span),
        }
    }

    fn lower_block(&mut self, block: &BlockExpression) -> Result<Register, CodegenError> {
        let saved_environment = self.environment.clone();
        for statement in &block.statements {
            self.lower_statement(statement)?;
        }
        let result = if let Some(tail_expression) = &block.tail_expression {
            self.lower_expression(tail_expression)?
        } else {
            let dst = self.fresh_register();
            self.instructions.push(Instruction::ConstUnit { dst });
            dst
        };
        self.environment = saved_environment;
        Ok(result)
    }

    fn lower_if(&mut self, if_expression: &IfExpression) -> Result<Register, CodegenError> {
        let condition = self.lower_expression(&if_expression.condition)?;
        let then_label = self.fresh_label();
        let else_label = self.fresh_label();
        let end_label = self.fresh_label();
        let dst = self.fresh_register();

        self.instructions.push(Instruction::BranchIf {
            condition,
            then_label,
            else_label,
        });

        self.instructions.push(Instruction::Label { label: then_label });
        let then_value = self.lower_expression(&if_expression.then_branch)?;
        self.instructions.push(Instruction::Move { dst, src: then_value });
        self.instructions.push(Instruction::Jump { label: end_label });

        self.instructions.push(Instruction::Label { label: else_label });
        let else_value = if let Some(else_branch) = &if_expression.else_branch {
            self.lower_expression(else_branch)?
        } else {
            let unit = self.fresh_register();
            self.instructions.push(Instruction::ConstUnit { dst: unit });
            unit
        };
        self.instructions.push(Instruction::Move { dst, src: else_value });
        self.instructions.push(Instruction::Label { label: end_label });
        Ok(dst)
    }

    fn lower_match(&mut self, match_expression: &MatchExpression) -> Result<Register, CodegenError> {
        let scrutinee = self.lower_expression(&match_expression.scrutinee)?;
        let end_label = self.fresh_label();
        let dst = self.fresh_register();
        let mut pending_next = None;

        for arm in &match_expression.arms {
            if let Some(next_label) = pending_next.take() {
                self.instructions.push(Instruction::Label { label: next_label });
            }

            let arm_label = self.fresh_label();
            match &arm.pattern.kind {
                PatternKind::Wildcard | PatternKind::Identifier(_) => {
                    self.instructions.push(Instruction::Jump { label: arm_label });
                }
                PatternKind::Integer(expected) => {
                    let predicate = self.fresh_register();
                    let expected_register = self.fresh_register();
                    self.instructions.push(Instruction::ConstInt {
                        dst: expected_register,
                        value: expected.clone(),
                    });
                    self.instructions.push(Instruction::Binary {
                        dst: predicate,
                        op: "eq",
                        left: scrutinee,
                        right: expected_register,
                    });
                    let next_label = self.fresh_label();
                    self.instructions.push(Instruction::BranchIf {
                        condition: predicate,
                        then_label: arm_label,
                        else_label: next_label,
                    });
                    pending_next = Some(next_label);
                }
                PatternKind::Boolean(expected) => {
                    let predicate = self.fresh_register();
                    let expected_register = self.fresh_register();
                    self.instructions.push(Instruction::ConstBool {
                        dst: expected_register,
                        value: *expected,
                    });
                    self.instructions.push(Instruction::Binary {
                        dst: predicate,
                        op: "eq",
                        left: scrutinee,
                        right: expected_register,
                    });
                    let next_label = self.fresh_label();
                    self.instructions.push(Instruction::BranchIf {
                        condition: predicate,
                        then_label: arm_label,
                        else_label: next_label,
                    });
                    pending_next = Some(next_label);
                }
                PatternKind::String(_) => {
                    return Err(CodegenError::new(
                        CodegenErrorKind::UnsupportedConstruct,
                        "string pattern lowering is deferred in Stage 0",
                        arm.pattern.span,
                        false,
                    ));
                }
            }

            self.instructions.push(Instruction::Label { label: arm_label });
            let saved_environment = self.environment.clone();
            if let PatternKind::Identifier(identifier) = &arm.pattern.kind {
                self.environment.values.insert(identifier.text.clone(), scrutinee);
            }
            let body = self.lower_expression(&arm.body)?;
            self.instructions.push(Instruction::Move { dst, src: body });
            self.instructions.push(Instruction::Jump { label: end_label });
            self.environment = saved_environment;
        }

        if let Some(next_label) = pending_next {
            self.instructions.push(Instruction::Label { label: next_label });
            self.instructions.push(Instruction::Trap {
                message: "non-exhaustive match in Stage 0 codegen",
            });
        }

        self.instructions.push(Instruction::Label { label: end_label });
        Ok(dst)
    }

    fn lower_call(
        &mut self,
        callee: &Expression,
        arguments: &[Expression],
        span: SourceSpan,
    ) -> Result<Register, CodegenError> {
        let name = match &callee.kind {
            ExpressionKind::Identifier(identifier) => identifier.text.clone(),
            _ => {
                return Err(CodegenError::new(
                    CodegenErrorKind::UnsupportedCallTarget,
                    "Stage 0 codegen requires call targets to be named identifiers",
                    span,
                    false,
                ));
            }
        };

        let mut lowered_arguments = Vec::with_capacity(arguments.len());
        for argument in arguments {
            lowered_arguments.push(self.lower_expression(argument)?);
        }

        let dst = self.fresh_register();
        self.instructions.push(Instruction::CallBuiltin {
            dst,
            name,
            args: lowered_arguments,
        });
        Ok(dst)
    }

    fn fresh_register(&mut self) -> Register {
        let register = Register(self.next_register);
        self.next_register += 1;
        register
    }

    fn fresh_label(&mut self) -> Label {
        let label = Label(self.next_label);
        self.next_label += 1;
        label
    }
}

fn binary_opcode(operator: BinaryOperator) -> &'static str {
    match operator {
        BinaryOperator::LogicalOr => "or.bool",
        BinaryOperator::LogicalAnd => "and.bool",
        BinaryOperator::Equal => "eq",
        BinaryOperator::NotEqual => "neq",
        BinaryOperator::Less => "lt.int",
        BinaryOperator::LessEqual => "lte.int",
        BinaryOperator::Greater => "gt.int",
        BinaryOperator::GreaterEqual => "gte.int",
        BinaryOperator::Add => "add.int",
        BinaryOperator::Subtract => "sub.int",
        BinaryOperator::Multiply => "mul.int",
        BinaryOperator::Divide => "div.int",
        BinaryOperator::Modulo => "mod.int",
    }
}
