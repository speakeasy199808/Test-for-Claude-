//! Error types for the self-verification module.

/// Errors arising from self-verification operations.
#[derive(Debug, thiserror::Error)]
pub enum SelfVerifyError {
    /// The digest computation failed internally.
    #[error("digest computation failed: {reason}")]
    DigestFailure {
        /// Description of the failure.
        reason: String,
    },
}
