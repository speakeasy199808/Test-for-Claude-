//! Error types for seed stdlib compilation.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Categories of stdlib compilation error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StdlibErrorKind {
    /// Type checking failed for a stdlib module.
    TypeError,
    /// Code generation failed for a stdlib module.
    CodegenError,
    /// Bytecode emission failed for a stdlib module.
    BytecodeError,
}

impl StdlibErrorKind {
    /// Returns a stable machine-readable label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::TypeError => "type_error",
            Self::CodegenError => "codegen_error",
            Self::BytecodeError => "bytecode_error",
        }
    }
}

/// A stdlib compilation diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{message}")]
pub struct StdlibError {
    /// Error category.
    pub kind: StdlibErrorKind,
    /// Module name responsible for the diagnostic.
    pub module: String,
    /// Human-readable message.
    pub message: String,
}

impl StdlibError {
    /// Creates a new stdlib diagnostic.
    #[must_use]
    pub fn new(kind: StdlibErrorKind, module: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            kind,
            module: module.into(),
            message: message.into(),
        }
    }
}
