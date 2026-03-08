//! [`LogSink`] — deterministic log collector.
//!
//! Collects log entries in insertion order, preserving the deterministic
//! virtual-time ordering of the Lyra system.

use crate::logging::entry::{CorrelationId, LogEntry, LogLevel};

/// A deterministic log sink that collects entries in insertion order.
///
/// The sink preserves the order in which entries are logged, which
/// corresponds to virtual-time ordering when entries are produced
/// by a single-threaded deterministic execution.
#[derive(Debug, Clone)]
pub struct LogSink {
    entries: Vec<LogEntry>,
}

impl LogSink {
    /// Create a new empty log sink.
    pub fn new() -> Self {
        LogSink {
            entries: Vec::new(),
        }
    }

    /// Log an entry to this sink.
    pub fn log(&mut self, entry: LogEntry) {
        self.entries.push(entry);
    }

    /// Returns all logged entries in insertion order.
    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }

    /// Returns the number of logged entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if no entries have been logged.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Clear all logged entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Filter entries by log level.
    pub fn by_level(&self, level: LogLevel) -> Vec<&LogEntry> {
        self.entries.iter().filter(|e| e.level() == level).collect()
    }

    /// Filter entries by correlation ID.
    pub fn by_correlation_id(&self, cid: CorrelationId) -> Vec<&LogEntry> {
        self.entries
            .iter()
            .filter(|e| e.correlation_id() == cid)
            .collect()
    }

    /// Filter entries by source module prefix.
    pub fn by_source_prefix(&self, prefix: &str) -> Vec<&LogEntry> {
        self.entries
            .iter()
            .filter(|e| e.source().starts_with(prefix))
            .collect()
    }

    /// Filter entries at or above the given minimum level.
    pub fn at_or_above(&self, min_level: LogLevel) -> Vec<&LogEntry> {
        self.entries
            .iter()
            .filter(|e| e.level() >= min_level)
            .collect()
    }
}

impl Default for LogSink {
    fn default() -> Self {
        Self::new()
    }
}
