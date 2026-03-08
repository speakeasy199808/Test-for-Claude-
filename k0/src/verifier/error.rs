//! Error types for the determinism verifier (P0-011).

use thiserror::Error;

/// Errors produced by the determinism verifier.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum VerifierError {
    /// The verified function produced different outputs on two identical runs.
    ///
    /// This is a constitutional violation: determinism is required.
    #[error("determinism violation: outputs differ on identical inputs (label={label:?})")]
    DeterminismViolation {
        /// Human-readable label for the verification point.
        label: String,
        /// Hex encoding of the first run's output.
        first_hex: String,
        /// Hex encoding of the second run's output.
        second_hex: String,
    },

    /// The verified function produced an empty output on both runs.
    ///
    /// Empty outputs are suspicious and must be explicitly allowed by the caller.
    #[error(
        "verifier received empty output for label={label:?}; use verify_allow_empty if intentional"
    )]
    EmptyOutput {
        /// Human-readable label for the verification point.
        label: String,
    },
}
