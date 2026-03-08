//! Seed bytecode emitter for the LyraLang Stage 0 register-VM IR.

pub mod error;
mod emitter;

use serde::{Deserialize, Serialize};

use crate::codegen;
use crate::lexer::SourceSpan;

pub use error::{BytecodeError, BytecodeErrorKind};
use emitter::BytecodeEmitter;

/// A canonical bytecode instruction record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BytecodeInstruction {
    /// Canonical opcode.
    pub opcode: String,
    /// Canonical operand sequence.
    pub operands: Vec<String>,
    /// Original canonical IR text for auditability.
    pub text: String,
}

/// Canonical bytecode object emitted for a Stage 0 program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BytecodeProgram {
    /// Optional module name.
    pub module: Option<String>,
    /// Bytecode object format version.
    pub format_version: String,
    /// Source IR format version.
    pub ir_format_version: String,
    /// Canonical result type.
    pub result_type: String,
    /// Number of virtual registers referenced by the program.
    pub register_count: u32,
    /// Entry register index.
    pub entry_register: u32,
    /// Number of bytecode instructions.
    pub instruction_count: u32,
    /// Structured instruction records.
    pub instructions: Vec<BytecodeInstruction>,
    /// Canonically encoded LyraCodec bytes.
    pub encoded: Vec<u8>,
    /// Source span for the full program.
    pub span: SourceSpan,
}

/// Result bundle returned by the seed bytecode emitter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BytecodeOutput {
    /// Normalized source passed through earlier compiler stages.
    pub normalized_source: String,
    /// Bytecode object when emission succeeded.
    pub program: Option<BytecodeProgram>,
    /// Diagnostics emitted during bytecode emission.
    pub errors: Vec<BytecodeError>,
}

/// Deterministic seed bytecode emitter.
#[derive(Debug, Clone, Default)]
pub struct SeedBytecodeEmitter;

impl SeedBytecodeEmitter {
    /// Creates a new bytecode emitter.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Parses, type-checks, lowers, and canonically emits bytecode from source text.
    #[must_use]
    pub fn emit_source(&self, source: &str) -> BytecodeOutput {
        let codegen_output = codegen::generate(source);
        let normalized_source = codegen_output.normalized_source.clone();

        if !codegen_output.errors.is_empty() {
            let errors = codegen_output
                .errors
                .into_iter()
                .map(|error| {
                    BytecodeError::new(
                        BytecodeErrorKind::CodegenError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return BytecodeOutput {
                normalized_source,
                program: None,
                errors,
            };
        }

        let Some(program) = codegen_output.program else {
            return BytecodeOutput {
                normalized_source,
                program: None,
                errors: vec![BytecodeError::new(
                    BytecodeErrorKind::CodegenError,
                    "code generator completed without a program",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        match BytecodeEmitter::default().emit_program(&program) {
            Ok(program) => BytecodeOutput {
                normalized_source,
                program: Some(program),
                errors: Vec::new(),
            },
            Err(error) => BytecodeOutput {
                normalized_source,
                program: None,
                errors: vec![error],
            },
        }
    }
}

/// Parses, type-checks, lowers, and canonically emits bytecode with the default emitter.
#[must_use]
pub fn emit(source: &str) -> BytecodeOutput {
    SeedBytecodeEmitter::new().emit_source(source)
}
