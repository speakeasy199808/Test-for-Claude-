# P0-019 — Structured Logging

## Mission
Structured log format (JSON). Log levels. Correlation IDs for tracing across crate boundaries. Deterministic log ordering.

## Scope
- `LogLevel` enum: Trace, Debug, Info, Warn, Error (ordered)
- `CorrelationId` for cross-crate operation tracing
- `LogEntry` with virtual timestamp, level, correlation ID, source, message, context
- `LogSink` deterministic collector with filtering by level, correlation ID, source
- JSON serialization via serde for machine-readable output
- Virtual-time timestamps (no wall clock)

## Primary Archetype
Core Module Implementation

## Primary Ownership Root
`k0/src/logging/`

## Deliverables
- `k0/src/logging/mod.rs` — module root with 10 tests
- `k0/src/logging/entry.rs` — LogLevel, CorrelationId, LogEntry
- `k0/src/logging/sink.rs` — LogSink collector
