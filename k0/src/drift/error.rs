//! Error types for the drift detection module (P0-012).

use thiserror::Error;

/// Errors produced by the drift detector.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DriftError {
    /// A constitutional drift event was detected: a computation produced
    /// different outputs on two identical runs.
    ///
    /// This is a constitutional violation per P0-003.
    #[error("constitutional drift detected at label={label:?}: outputs differ")]
    ConstitutionalDrift {
        /// Human-readable label for the drift point.
        label: String,
        /// Hex encoding of the first run's output.
        first_hex: String,
        /// Hex encoding of the second run's output.
        second_hex: String,
    },

    /// An operational drift event was detected: a computation produced
    /// an empty output where a non-empty output was expected.
    #[error("operational drift detected at label={label:?}: empty output")]
    EmptyOutputDrift {
        /// Human-readable label for the drift point.
        label: String,
    },
}

impl DriftError {
    /// Returns `true` if this is a constitutional drift violation.
    pub fn is_constitutional(&self) -> bool {
        matches!(self, DriftError::ConstitutionalDrift { .. })
    }

    /// Returns `true` if this is an operational drift event.
    pub fn is_operational(&self) -> bool {
        matches!(self, DriftError::EmptyOutputDrift { .. })
    }
}
