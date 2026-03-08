//! Internal compiler surface for the seed minimal stdlib.

use serde::{Deserialize, Serialize};

use crate::bytecode;
use crate::checker;
use crate::codegen;
use crate::stdlib::error::{StdlibError, StdlibErrorKind};

const STD_DATA_BOOL_GUARD: &str = include_str!("../../../fixtures/lyralang/stdlib/modules/std_data_bool_guard.lyra");
const STD_IO_PRINT_STATUS: &str = include_str!("../../../fixtures/lyralang/stdlib/modules/std_io_print_status.lyra");
const STD_MATH_INT_EQ: &str = include_str!("../../../fixtures/lyralang/stdlib/modules/std_math_int_eq.lyra");
const STD_MATH_RATIONAL_SEED: &str = include_str!("../../../fixtures/lyralang/stdlib/modules/std_math_rational_seed.lyra");

/// A source module included in the seed stdlib manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeedStdlibModule {
    /// Canonical module name.
    pub name: String,
    /// Category within the minimal stdlib.
    pub category: String,
    /// Repository-relative source path.
    pub source_path: String,
    /// Lyra source content.
    pub source: String,
}

/// A compiled stdlib module artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompiledStdlibModule {
    /// Canonical module name.
    pub name: String,
    /// Category within the minimal stdlib.
    pub category: String,
    /// Repository-relative source path.
    pub source_path: String,
    /// Inferred program type.
    pub program_type: String,
    /// Emitted codegen format version.
    pub codegen_format_version: String,
    /// Emitted bytecode format version.
    pub bytecode_format_version: String,
}

/// Compilation output for the seed minimal stdlib.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StdlibCompileOutput {
    /// Manifest version.
    pub manifest_version: String,
    /// Static source manifest.
    pub manifest: Vec<SeedStdlibModule>,
    /// Successfully compiled modules.
    pub compiled_modules: Vec<CompiledStdlibModule>,
    /// Diagnostics emitted while compiling the stdlib.
    pub errors: Vec<StdlibError>,
}

/// Returns the static seed stdlib source manifest.
#[must_use]
pub fn manifest() -> Vec<SeedStdlibModule> {
    vec![
        SeedStdlibModule {
            name: "std.data.bool_guard".to_string(),
            category: "data".to_string(),
            source_path: "fixtures/lyralang/stdlib/modules/std_data_bool_guard.lyra".to_string(),
            source: STD_DATA_BOOL_GUARD.to_string(),
        },
        SeedStdlibModule {
            name: "std.io.print_status".to_string(),
            category: "io".to_string(),
            source_path: "fixtures/lyralang/stdlib/modules/std_io_print_status.lyra".to_string(),
            source: STD_IO_PRINT_STATUS.to_string(),
        },
        SeedStdlibModule {
            name: "std.math.int_eq".to_string(),
            category: "math".to_string(),
            source_path: "fixtures/lyralang/stdlib/modules/std_math_int_eq.lyra".to_string(),
            source: STD_MATH_INT_EQ.to_string(),
        },
        SeedStdlibModule {
            name: "std.math.rational_seed".to_string(),
            category: "math".to_string(),
            source_path: "fixtures/lyralang/stdlib/modules/std_math_rational_seed.lyra".to_string(),
            source: STD_MATH_RATIONAL_SEED.to_string(),
        },
    ]
}

/// Compiles the seed minimal stdlib through the current Stage 0 pipeline.
#[must_use]
pub fn compile_minimal_stdlib() -> StdlibCompileOutput {
    let manifest = manifest();
    let mut compiled_modules = Vec::new();
    let mut errors = Vec::new();

    for module in &manifest {
        let type_output = checker::check(&module.source);
        if !type_output.errors.is_empty() {
            for error in type_output.errors {
                errors.push(StdlibError::new(StdlibErrorKind::TypeError, &module.name, error.message));
            }
            continue;
        }

        let Some(judgment) = type_output.judgment else {
            errors.push(StdlibError::new(
                StdlibErrorKind::TypeError,
                &module.name,
                "type checker completed without a program judgment",
            ));
            continue;
        };

        let codegen_output = codegen::generate(&module.source);
        if !codegen_output.errors.is_empty() {
            for error in codegen_output.errors {
                errors.push(StdlibError::new(StdlibErrorKind::CodegenError, &module.name, error.message));
            }
            continue;
        }

        let Some(codegen_program) = codegen_output.program else {
            errors.push(StdlibError::new(
                StdlibErrorKind::CodegenError,
                &module.name,
                "code generator completed without a program",
            ));
            continue;
        };

        let bytecode_output = bytecode::emit(&module.source);
        if !bytecode_output.errors.is_empty() {
            for error in bytecode_output.errors {
                errors.push(StdlibError::new(StdlibErrorKind::BytecodeError, &module.name, error.message));
            }
            continue;
        }

        let Some(bytecode_program) = bytecode_output.program else {
            errors.push(StdlibError::new(
                StdlibErrorKind::BytecodeError,
                &module.name,
                "bytecode emitter completed without a program",
            ));
            continue;
        };

        compiled_modules.push(CompiledStdlibModule {
            name: module.name.clone(),
            category: module.category.clone(),
            source_path: module.source_path.clone(),
            program_type: judgment.program_type.canonical_name(),
            codegen_format_version: codegen_program.format_version,
            bytecode_format_version: bytecode_program.format_version,
        });
    }

    StdlibCompileOutput {
        manifest_version: "lyralang-stdlib-v1".to_string(),
        manifest,
        compiled_modules,
        errors,
    }
}
