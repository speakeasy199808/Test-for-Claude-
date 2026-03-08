//! Seed code generator for LyraLang Stage 0.
//!
//! This module lowers the currently executable Stage 0 AST into a
//! deterministic register-VM intermediate representation. The byte-level
//! encoding is intentionally deferred to P1-020.

pub mod error;
mod generator;

use serde::{Deserialize, Serialize};

use crate::checker;
use crate::lexer::SourceSpan;
use crate::parser::parse;

pub use error::{CodegenError, CodegenErrorKind};
use generator::LoweringEngine;

/// Deterministic code-generation output for a Stage 0 program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodegenProgram {
    /// Optional module name.
    pub module: Option<String>,
    /// Canonical IR format version.
    pub format_version: String,
    /// Number of allocated virtual registers.
    pub register_count: u32,
    /// Canonical name of the entry/result register.
    pub entry_register: String,
    /// Canonical textual spelling of the result type.
    pub result_type: String,
    /// Canonical rendered instruction stream.
    pub instructions: Vec<String>,
    /// Source span for the full program.
    pub span: SourceSpan,
}

/// Result bundle returned by the seed code generator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodegenOutput {
    /// Normalized source passed through earlier compiler stages.
    pub normalized_source: String,
    /// Generated program when lowering succeeded.
    pub program: Option<CodegenProgram>,
    /// Diagnostics emitted during lowering.
    pub errors: Vec<CodegenError>,
}

/// Deterministic seed code generator.
#[derive(Debug, Clone, Default)]
pub struct CodeGenerator;

impl CodeGenerator {
    /// Creates a new seed code generator.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Parses, type-checks, and lowers source text.
    #[must_use]
    pub fn generate_source(&self, source: &str) -> CodegenOutput {
        let type_output = checker::check(source);
        let normalized_source = type_output.normalized_source.clone();

        if !type_output.errors.is_empty() {
            let errors = type_output
                .errors
                .into_iter()
                .map(|error| {
                    CodegenError::new(
                        CodegenErrorKind::TypeError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return CodegenOutput {
                normalized_source,
                program: None,
                errors,
            };
        }

        let parse_output = parse(source);
        if !parse_output.errors.is_empty() {
            let errors = parse_output
                .errors
                .into_iter()
                .map(|error| {
                    CodegenError::new(
                        CodegenErrorKind::ParseError,
                        error.message,
                        error.span,
                        error.recovered,
                    )
                })
                .collect();
            return CodegenOutput {
                normalized_source,
                program: None,
                errors,
            };
        }

        let Some(judgment) = type_output.judgment else {
            return CodegenOutput {
                normalized_source,
                program: None,
                errors: vec![CodegenError::new(
                    CodegenErrorKind::TypeError,
                    "type checker completed without a program judgment",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        let Some(program) = parse_output.program else {
            return CodegenOutput {
                normalized_source,
                program: None,
                errors: vec![CodegenError::new(
                    CodegenErrorKind::ParseError,
                    "parser completed without a program AST",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        match LoweringEngine::from_type_judgment(&judgment).lower_program(&program, &judgment.program_type) {
            Ok(program) => CodegenOutput {
                normalized_source,
                program: Some(program),
                errors: Vec::new(),
            },
            Err(error) => CodegenOutput {
                normalized_source,
                program: None,
                errors: vec![error],
            },
        }
    }
}

/// Parses, type-checks, and lowers source text with the default seed generator.
#[must_use]
pub fn generate(source: &str) -> CodegenOutput {
    CodeGenerator::new().generate_source(source)
}
