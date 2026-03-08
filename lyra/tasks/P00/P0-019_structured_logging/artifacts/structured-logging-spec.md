# P0-019 Structured Logging — Evidence Artifact

## Module: `k0::logging`

**Status:** Implemented and tested
**Location:** `k0/src/logging/` (entry.rs, sink.rs, mod.rs)

---

## Architecture

The structured logging module provides a deterministic, JSON-serializable
logging system for the Lyra system. All log entries are timestamped with
`VirtualTime` (not wall clock), ensuring reproducible log output across
identical runs.

### Core Types

| Type | File | Purpose |
|---|---|---|
| `LogLevel` | `entry.rs` | Severity enum: Trace, Debug, Info, Warn, Error (ordered) |
| `CorrelationId` | `entry.rs` | Cross-crate tracing identifier (`cid-{u64}`) |
| `LogEntry` | `entry.rs` | Structured log record with virtual timestamp, level, correlation ID, source, message, and key-value context |
| `LogSink` | `sink.rs` | Deterministic log collector preserving insertion order |

### LogEntry Fields

```rust
pub struct LogEntry {
    timestamp: u64,           // VirtualTime counter (not wall clock)
    level: LogLevel,          // Severity level
    correlation_id: CorrelationId, // Cross-crate trace ID
    source: String,           // Module path (e.g., "k0::genesis")
    message: String,          // Human-readable message
    context: Vec<(String, String)>, // Structured key-value pairs
}
```

### LogLevel Ordering

```
Trace < Debug < Info < Warn < Error
```

All levels implement `Ord`, `Serialize`, `Deserialize`.

### CorrelationId

- Newtype over `u64`
- Display format: `cid-{value}`
- Used to trace operations across crate boundaries
- Deterministic: assigned at operation start, propagated through all related entries

### LogSink API

| Method | Description |
|---|---|
| `new()` | Create empty sink |
| `log(entry)` | Append entry (preserves insertion order) |
| `entries()` | All entries in insertion order |
| `len()` / `is_empty()` | Size queries |
| `clear()` | Remove all entries |
| `by_level(level)` | Filter by exact level |
| `by_correlation_id(cid)` | Filter by correlation ID |
| `by_source_prefix(prefix)` | Filter by source module prefix |
| `at_or_above(min_level)` | Filter entries at or above minimum severity |

---

## Determinism Guarantees

1. **Virtual timestamps only** — `LogEntry` stores `VirtualTime` (u64 counter), never `std::time`
2. **Deterministic JSON serialization** — `serde::Serialize` on `LogEntry` produces identical JSON for identical inputs
3. **Insertion-order preservation** — `LogSink` maintains entries in the order they were logged
4. **No ambient nondeterminism** — no wall clock, no thread IDs, no random values

## Serialization Format

LogEntry serializes to deterministic JSON:

```json
{
  "timestamp": 1,
  "level": "Info",
  "correlation_id": 42,
  "source": "k0::genesis",
  "message": "Genesis state initialized",
  "context": []
}
```

Fields are emitted in declaration order (serde default for structs).

---

## Test Coverage

10 unit tests in `k0/src/logging/mod.rs`:

1. `log_entry_serializes_to_json` — JSON output contains expected fields
2. `log_entry_json_is_deterministic` — identical entries produce identical JSON
3. `log_sink_preserves_insertion_order` — entries retrieved in log order
4. `log_sink_filter_by_level` — level filtering works correctly
5. `log_sink_filter_by_correlation_id` — correlation ID filtering works
6. `log_level_ordering` — Trace < Debug < Info < Warn < Error
7. `correlation_id_display` — displays as `cid-{value}`
8. `log_entry_with_context` — key-value context pairs stored and retrieved
9. `log_sink_clear` — clear removes all entries
10. `log_entry_timestamp_is_virtual` — timestamp roundtrips through VirtualTime

---

## Acceptance Criteria Met

- [x] `LogLevel` enum with 5 severity levels, ordered
- [x] `CorrelationId` for cross-crate tracing
- [x] `LogEntry` with virtual timestamp, level, correlation ID, source, message, context
- [x] `LogSink` with deterministic insertion-order collection
- [x] JSON serialization is deterministic (identical inputs → identical output)
- [x] No wall clock dependency — all timestamps are `VirtualTime`
- [x] Filtering by level, correlation ID, source prefix, minimum severity
- [x] All public items documented (`#![deny(missing_docs)]`)
- [x] No unsafe code (`#![forbid(unsafe_code)]`)
- [x] All clippy lints pass (`#![deny(clippy::all)]`)
- [x] 10 unit tests passing
