//! Error code types: [`ErrorCode`], [`ErrorCategory`], and [`ErrorEntry`].

use serde::{Deserialize, Serialize};
use std::fmt;

/// A globally unique error code in the range E0001–E9999.
///
/// Error codes are displayed with zero-padded four-digit format: `E0001`, `E1234`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ErrorCode(u16);

impl ErrorCode {
    /// Create a new error code. Returns `None` if `number` is 0 or > 9999.
    pub fn new(number: u16) -> Option<Self> {
        if (1..=9999).contains(&number) {
            Some(ErrorCode(number))
        } else {
            None
        }
    }

    /// Returns the numeric value of this error code.
    pub fn number(self) -> u16 {
        self.0
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "E{:04}", self.0)
    }
}

/// Error category, determined by the code range.
///
/// | Range | Category |
/// |---|---|
/// | E0001–E0999 | Constitutional |
/// | E1000–E1999 | Codec |
/// | E2000–E2999 | Digest |
/// | E3000–E3999 | Time |
/// | E4000–E4999 | Entropy |
/// | E5000–E5999 | Incident |
/// | E6000–E6999 | Verification |
/// | E7000–E7999 | Resource |
/// | E8000–E8999 | Policy |
/// | E9000–E9999 | Reserved |
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorCategory {
    /// E0001–E0999: Constitutional / determinism violations.
    Constitutional,
    /// E1000–E1999: Codec / serialization errors.
    Codec,
    /// E2000–E2999: Digest / cryptographic errors.
    Digest,
    /// E3000–E3999: Time / causal ordering errors.
    Time,
    /// E4000–E4999: Entropy / randomness errors.
    Entropy,
    /// E5000–E5999: Incident / recovery errors.
    Incident,
    /// E6000–E6999: Verification / proof errors.
    Verification,
    /// E7000–E7999: Resource / capacity errors.
    Resource,
    /// E8000–E8999: Policy / governance errors.
    Policy,
    /// E9000–E9999: Reserved for future extension.
    Reserved,
}

impl ErrorCategory {
    /// Determine the category from an error code based on its numeric range.
    pub fn from_code(code: ErrorCode) -> Self {
        match code.number() {
            1..=999 => ErrorCategory::Constitutional,
            1000..=1999 => ErrorCategory::Codec,
            2000..=2999 => ErrorCategory::Digest,
            3000..=3999 => ErrorCategory::Time,
            4000..=4999 => ErrorCategory::Entropy,
            5000..=5999 => ErrorCategory::Incident,
            6000..=6999 => ErrorCategory::Verification,
            7000..=7999 => ErrorCategory::Resource,
            8000..=8999 => ErrorCategory::Policy,
            9000..=9999 => ErrorCategory::Reserved,
            _ => ErrorCategory::Reserved,
        }
    }

    /// Returns the human-readable name of this category.
    pub fn name(&self) -> &'static str {
        match self {
            ErrorCategory::Constitutional => "Constitutional",
            ErrorCategory::Codec => "Codec",
            ErrorCategory::Digest => "Digest",
            ErrorCategory::Time => "Time",
            ErrorCategory::Entropy => "Entropy",
            ErrorCategory::Incident => "Incident",
            ErrorCategory::Verification => "Verification",
            ErrorCategory::Resource => "Resource",
            ErrorCategory::Policy => "Policy",
            ErrorCategory::Reserved => "Reserved",
        }
    }
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// A complete error catalog entry with code, category, message,
/// explanation, and suggested fix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorEntry {
    /// The globally unique error code.
    pub code: ErrorCode,
    /// The error category (derived from code range).
    pub category: ErrorCategory,
    /// Short, one-line error message.
    pub message: &'static str,
    /// Detailed explanation of what caused this error.
    pub explanation: &'static str,
    /// Suggested fix or remediation steps.
    pub suggestion: &'static str,
}

impl fmt::Display for ErrorEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.code, self.category, self.message)
    }
}
