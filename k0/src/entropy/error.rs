//! Error types for the entropy module (P0-010).

use thiserror::Error;

/// Errors produced by entropy pool operations.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum EntropyError {
    /// Requested zero bytes from the pool.
    ///
    /// Callers must request at least one byte.
    #[error("entropy request must be at least 1 byte, got 0")]
    ZeroLengthRequest,

    /// Requested more bytes than the pool can produce in a single call.
    ///
    /// Split large requests into multiple calls.
    #[error("entropy request too large: requested {requested}, max {max}")]
    RequestTooLarge {
        /// Number of bytes requested.
        requested: usize,
        /// Maximum allowed per call.
        max: usize,
    },
}
