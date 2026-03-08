//! Seed minimal standard library for LyraLang Stage 0.
//!
//! The stdlib sources are written in `.lyra` files and compiled through the
//! current seed pipeline. Richer collections and user-defined functions are
//! deferred until later grammar/runtime slices; this package freezes the first
//! deterministic primitive/data/io/math baseline now.

pub mod error;
mod compiler;

pub use compiler::{
    compile_minimal_stdlib, manifest, CompiledStdlibModule, SeedStdlibModule,
    StdlibCompileOutput,
};
pub use error::{StdlibError, StdlibErrorKind};
