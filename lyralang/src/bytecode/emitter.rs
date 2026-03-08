//! Internal canonical bytecode emitter for Stage 0 IR.

use k0::codec::{encode, StructField, Value, TAG_STRING, TAG_STRUCT};

use crate::bytecode::error::{BytecodeError, BytecodeErrorKind};
use crate::bytecode::{BytecodeInstruction, BytecodeProgram};
use crate::codegen::CodegenProgram;
use crate::lexer::SourceSpan;

const BYTECODE_FORMAT_VERSION: &str = "lyravm-bytecode-v1";

/// Internal bytecode emitter.
#[derive(Debug, Clone, Default)]
pub struct BytecodeEmitter;

impl BytecodeEmitter {
    /// Emits a canonical bytecode object from deterministic Stage 0 IR.
    pub fn emit_program(&self, program: &CodegenProgram) -> Result<BytecodeProgram, BytecodeError> {
        let instructions = program
            .instructions
            .iter()
            .map(|text| Self::normalize_instruction(text, program.span))
            .collect::<Result<Vec<_>, _>>()?;

        let entry_register = Self::parse_register_index(&program.entry_register, program.span)?;
        let value = self.bytecode_value(program, &instructions, entry_register);
        let encoded = encode(&value).map_err(|error| {
            BytecodeError::new(
                BytecodeErrorKind::EncodingError,
                format!("failed to canonically encode bytecode object: {error}"),
                program.span,
                false,
            )
        })?;

        Ok(BytecodeProgram {
            module: program.module.clone(),
            format_version: BYTECODE_FORMAT_VERSION.to_string(),
            ir_format_version: program.format_version.clone(),
            result_type: program.result_type.clone(),
            register_count: program.register_count,
            entry_register,
            instruction_count: instructions.len() as u32,
            instructions,
            encoded,
            span: program.span,
        })
    }

    fn bytecode_value(
        &self,
        program: &CodegenProgram,
        instructions: &[BytecodeInstruction],
        entry_register: u32,
    ) -> Value {
        let instruction_values = instructions
            .iter()
            .map(|instruction| Value::Struct {
                schema_version: 1,
                fields: vec![
                    StructField { field_id: 1, value: Value::Str(instruction.opcode.clone()) },
                    StructField {
                        field_id: 2,
                        value: Value::Vector {
                            elem_tag: TAG_STRING,
                            elements: instruction.operands.iter().cloned().map(Value::Str).collect(),
                        },
                    },
                    StructField { field_id: 3, value: Value::Str(instruction.text.clone()) },
                ],
            })
            .collect();

        Value::Struct {
            schema_version: 1,
            fields: vec![
                StructField { field_id: 1, value: Value::Str(BYTECODE_FORMAT_VERSION.to_string()) },
                StructField { field_id: 2, value: Value::Str(program.module.clone().unwrap_or_default()) },
                StructField { field_id: 3, value: Value::Str(program.format_version.clone()) },
                StructField { field_id: 4, value: Value::Str(program.result_type.clone()) },
                StructField { field_id: 5, value: Value::UInt(u64::from(program.register_count)) },
                StructField { field_id: 6, value: Value::UInt(u64::from(entry_register)) },
                StructField { field_id: 7, value: Value::UInt(instructions.len() as u64) },
                StructField {
                    field_id: 8,
                    value: Value::Vector {
                        elem_tag: TAG_STRUCT,
                        elements: instruction_values,
                    },
                },
            ],
        }
    }

    fn normalize_instruction(text: &str, span: SourceSpan) -> Result<BytecodeInstruction, BytecodeError> {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return Err(BytecodeError::new(
                BytecodeErrorKind::InvalidIr,
                "empty IR instruction cannot be emitted",
                span,
                false,
            ));
        }

        let mut opcode = String::new();
        let operands = if let Some((destination, rhs)) = trimmed.split_once(" = ") {
            let tokens = Self::tokenize(rhs);
            if let Some(first) = tokens.first() {
                opcode = first.clone();
            }
            let mut operands = vec![destination.trim().to_string()];
            operands.extend(tokens.into_iter().skip(1));
            operands
        } else {
            let tokens = Self::tokenize(trimmed);
            if let Some(first) = tokens.first() {
                opcode = first.clone();
            }
            tokens.into_iter().skip(1).collect()
        };

        if opcode.is_empty() {
            return Err(BytecodeError::new(
                BytecodeErrorKind::InvalidIr,
                format!("could not derive opcode from canonical IR instruction `{trimmed}`"),
                span,
                false,
            ));
        }

        Ok(BytecodeInstruction {
            opcode,
            operands,
            text: trimmed.to_string(),
        })
    }

    fn tokenize(input: &str) -> Vec<String> {
        input
            .replace("->", " ")
            .replace('(', " ")
            .replace(')', " ")
            .replace(',', " ")
            .replace("==", " ")
            .replace('"', " ")
            .split_whitespace()
            .map(str::to_string)
            .collect()
    }

    fn parse_register_index(register: &str, span: SourceSpan) -> Result<u32, BytecodeError> {
        let digits = register.strip_prefix('r').ok_or_else(|| {
            BytecodeError::new(
                BytecodeErrorKind::InvalidIr,
                format!("entry register `{register}` is not in canonical rN form"),
                span,
                false,
            )
        })?;
        digits.parse::<u32>().map_err(|_| {
            BytecodeError::new(
                BytecodeErrorKind::InvalidIr,
                format!("entry register `{register}` does not contain a valid register index"),
                span,
                false,
            )
        })
    }
}
