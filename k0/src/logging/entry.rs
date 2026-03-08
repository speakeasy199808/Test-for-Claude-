//! Log entry types: [`LogLevel`], [`CorrelationId`], and [`LogEntry`].

use crate::time::VirtualTime;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Log severity level, ordered from least to most severe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum LogLevel {
    /// Finest-grained informational events.
    Trace,
    /// Detailed debugging information.
    Debug,
    /// Informational messages highlighting progress.
    Info,
    /// Potentially harmful situations.
    Warn,
    /// Error events that might still allow continued operation.
    Error,
}

impl LogLevel {
    /// Returns the human-readable name of this log level.
    pub fn name(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// A correlation ID for tracing operations across crate boundaries.
///
/// Correlation IDs are deterministic identifiers assigned at the start
/// of a logical operation and propagated through all related log entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CorrelationId(u64);

impl CorrelationId {
    /// Create a new correlation ID with the given value.
    pub fn new(id: u64) -> Self {
        CorrelationId(id)
    }

    /// Returns the numeric value of this correlation ID.
    pub fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for CorrelationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cid-{}", self.0)
    }
}

/// A structured log entry with deterministic virtual-time timestamp.
///
/// Log entries are the fundamental unit of the Lyra logging system.
/// They carry a virtual timestamp (not wall clock), severity level,
/// correlation ID for cross-crate tracing, source module path,
/// message, and optional structured key-value context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogEntry {
    timestamp: u64,
    level: LogLevel,
    correlation_id: CorrelationId,
    source: String,
    message: String,
    context: Vec<(String, String)>,
}

impl LogEntry {
    /// Create a new log entry.
    pub fn new(
        timestamp: VirtualTime,
        level: LogLevel,
        correlation_id: CorrelationId,
        source: &str,
        message: &str,
    ) -> Self {
        LogEntry {
            timestamp: timestamp.as_u64(),
            level,
            correlation_id,
            source: source.to_string(),
            message: message.to_string(),
            context: Vec::new(),
        }
    }

    /// Add a key-value context pair to this log entry.
    pub fn add_context(&mut self, key: &str, value: &str) {
        self.context.push((key.to_string(), value.to_string()));
    }

    /// Returns the virtual timestamp of this entry.
    pub fn timestamp(&self) -> VirtualTime {
        VirtualTime::new(self.timestamp)
    }

    /// Returns the log level.
    pub fn level(&self) -> LogLevel {
        self.level
    }

    /// Returns the correlation ID.
    pub fn correlation_id(&self) -> CorrelationId {
        self.correlation_id
    }

    /// Returns the source module path.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Returns the log message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the structured context key-value pairs.
    pub fn context(&self) -> Vec<(&str, &str)> {
        self.context
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    }
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[t={}] {} [{}] {}: {}",
            self.timestamp, self.level, self.correlation_id, self.source, self.message
        )
    }
}
