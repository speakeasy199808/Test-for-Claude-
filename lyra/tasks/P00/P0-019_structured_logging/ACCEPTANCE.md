# Acceptance — P0-019 Structured Logging

## Acceptance Criteria
1. `LogLevel` enum with 5 levels: Trace < Debug < Info < Warn < Error.
2. `CorrelationId` newtype with display format `cid-{N}`.
3. `LogEntry` carries virtual timestamp, level, correlation ID, source, message, and context.
4. `LogEntry` serializes to JSON deterministically.
5. `LogSink` preserves insertion order (deterministic ordering).
6. `LogSink` supports filtering by level, correlation ID, and source prefix.
7. `LogSink` supports `at_or_above()` minimum level filtering.
8. `LogEntry` supports structured key-value context pairs.
9. All timestamps use `VirtualTime`, not wall clock.
10. All 10 unit tests pass.

## Verification Method
- `cargo test -p k0 --lib` — all logging module tests pass
- 280 total tests, 0 failures

## Evidence Required
- Test pass output
- `artifacts/structured-logging-spec.md`
