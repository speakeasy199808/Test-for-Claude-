//! Structured Deterministic Logging (P0-019).
//!
//! Provides a structured, JSON-serializable logging system for the Lyra
//! system with deterministic ordering guarantees.
//!
//! # Design
//! - [`LogLevel`] defines severity levels: Trace, Debug, Info, Warn, Error.
//! - [`CorrelationId`] provides cross-crate tracing via unique identifiers.
//! - [`LogEntry`] is a structured log record with timestamp, level, correlation
//!   ID, source module, message, and optional structured context.
//! - [`LogSink`] collects log entries in deterministic order.
//!
//! # Determinism Guarantee
//! All log entries are timestamped with [`VirtualTime`], not wall clock.
//! Log ordering is determined by virtual time sequence, ensuring
//! reproducible log output across identical runs.

pub mod entry;
pub mod sink;

pub use entry::{CorrelationId, LogEntry, LogLevel};
pub use sink::LogSink;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::VirtualTime;

    #[test]
    fn log_entry_serializes_to_json() {
        let entry = LogEntry::new(
            VirtualTime::new(1),
            LogLevel::Info,
            CorrelationId::new(42),
            "k0::genesis",
            "Genesis state initialized",
        );
        let json = serde_json::to_string(&entry).unwrap();
        assert!(json.contains("\"level\":\"Info\""));
        assert!(json.contains("\"message\":\"Genesis state initialized\""));
    }

    #[test]
    fn log_entry_json_is_deterministic() {
        let entry = LogEntry::new(
            VirtualTime::new(1),
            LogLevel::Warn,
            CorrelationId::new(7),
            "k0::codec",
            "Non-canonical encoding detected",
        );
        let json1 = serde_json::to_string(&entry).unwrap();
        let json2 = serde_json::to_string(&entry).unwrap();
        assert_eq!(json1, json2);
    }

    #[test]
    fn log_sink_preserves_insertion_order() {
        let mut sink = LogSink::new();
        sink.log(LogEntry::new(
            VirtualTime::new(1),
            LogLevel::Info,
            CorrelationId::new(1),
            "mod_a",
            "first",
        ));
        sink.log(LogEntry::new(
            VirtualTime::new(2),
            LogLevel::Debug,
            CorrelationId::new(1),
            "mod_b",
            "second",
        ));
        let entries = sink.entries();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].message(), "first");
        assert_eq!(entries[1].message(), "second");
    }

    #[test]
    fn log_sink_filter_by_level() {
        let mut sink = LogSink::new();
        sink.log(LogEntry::new(
            VirtualTime::new(1),
            LogLevel::Debug,
            CorrelationId::new(1),
            "mod_a",
            "debug msg",
        ));
        sink.log(LogEntry::new(
            VirtualTime::new(2),
            LogLevel::Error,
            CorrelationId::new(1),
            "mod_a",
            "error msg",
        ));
        let errors = sink.by_level(LogLevel::Error);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].message(), "error msg");
    }

    #[test]
    fn log_sink_filter_by_correlation_id() {
        let mut sink = LogSink::new();
        let cid_a = CorrelationId::new(10);
        let cid_b = CorrelationId::new(20);
        sink.log(LogEntry::new(
            VirtualTime::new(1),
            LogLevel::Info,
            cid_a,
            "mod_a",
            "trace A",
        ));
        sink.log(LogEntry::new(
            VirtualTime::new(2),
            LogLevel::Info,
            cid_b,
            "mod_b",
            "trace B",
        ));
        sink.log(LogEntry::new(
            VirtualTime::new(3),
            LogLevel::Info,
            cid_a,
            "mod_a",
            "trace A continued",
        ));
        let trace_a = sink.by_correlation_id(cid_a);
        assert_eq!(trace_a.len(), 2);
    }

    #[test]
    fn log_level_ordering() {
        assert!(LogLevel::Trace < LogLevel::Debug);
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Error);
    }

    #[test]
    fn correlation_id_display() {
        let cid = CorrelationId::new(42);
        assert_eq!(format!("{cid}"), "cid-42");
    }

    #[test]
    fn log_entry_with_context() {
        let mut entry = LogEntry::new(
            VirtualTime::new(5),
            LogLevel::Info,
            CorrelationId::new(1),
            "k0::digest",
            "Hash computed",
        );
        entry.add_context("algorithm", "sha3-256");
        entry.add_context("input_len", "1024");
        assert_eq!(entry.context().len(), 2);
        assert_eq!(entry.context()[0], ("algorithm", "sha3-256"));
    }

    #[test]
    fn log_sink_clear() {
        let mut sink = LogSink::new();
        sink.log(LogEntry::new(
            VirtualTime::new(1),
            LogLevel::Info,
            CorrelationId::new(1),
            "mod_a",
            "msg",
        ));
        assert_eq!(sink.len(), 1);
        sink.clear();
        assert_eq!(sink.len(), 0);
    }

    #[test]
    fn log_entry_timestamp_is_virtual() {
        let entry = LogEntry::new(
            VirtualTime::new(999),
            LogLevel::Info,
            CorrelationId::new(1),
            "k0",
            "test",
        );
        assert_eq!(entry.timestamp().as_u64(), 999);
    }
}
