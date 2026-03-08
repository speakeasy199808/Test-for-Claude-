# Design — P0-019 Structured Logging

## Architecture

### Type Hierarchy
```
LogLevel (enum, 5 variants, Ord)
CorrelationId (u64 newtype)
LogEntry
  ├── timestamp: u64 (from VirtualTime)
  ├── level: LogLevel
  ├── correlation_id: CorrelationId
  ├── source: String
  ├── message: String
  └── context: Vec<(String, String)>
LogSink
  └── entries: Vec<LogEntry>
```

### Determinism Guarantee
- All timestamps are `VirtualTime` values, not wall clock
- `LogSink` preserves insertion order — no sorting, no reordering
- JSON serialization via serde is deterministic (struct field order is fixed)
- No HashMap or HashSet used in log entry storage

### Filtering API
| Method | Filter By |
|---|---|
| `by_level(level)` | Exact level match |
| `by_correlation_id(cid)` | Exact correlation ID match |
| `by_source_prefix(prefix)` | Source module path prefix |
| `at_or_above(min_level)` | Minimum severity threshold |

## Design Decisions
1. **Vec-based sink** — deterministic ordering, no HashMap nondeterminism
2. **VirtualTime timestamps** — constitutional determinism requirement
3. **Serde derive** — machine-readable JSON output
4. **String-based context** — simple key-value pairs, no complex types
5. **CorrelationId as u64** — lightweight, deterministic, generated from entropy pool
