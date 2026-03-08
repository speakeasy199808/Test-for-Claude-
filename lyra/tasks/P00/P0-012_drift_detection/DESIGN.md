# Design — P0-012 Drift Detection

## Constitutional Basis
Drift detection is the runtime enforcement layer for the determinism invariant (P0-003). While P0-011 provides the primitive double-run verifier, P0-012 adds the monitoring, classification, and reporting layer needed for operational use.

## Architecture

### Module Layout
```
k0/src/drift/
├── mod.rs        — public API, re-exports, integration tests
├── detector.rs   — DriftDetector, DriftSeverity, DriftEvent, DriftReport
└── error.rs      — DriftError (ConstitutionalDrift, EmptyOutputDrift)
```

### Layering
```
DriftDetector (P0-012)
    └── DeterminismVerifier (P0-011)
            └── VirtualClock (P0-009)
```

### Core Types

**`DriftSeverity`** (ordered: Operational < Constitutional)
- `Operational` — empty output where non-empty expected; suspicious but not a P0-003 violation
- `Constitutional` — outputs differ on identical runs; direct P0-003 violation

**`DriftEvent`**
- `label: String` — human-readable monitoring point name
- `severity: DriftSeverity` — classification
- `first_hex: String` — hex of first run (empty for operational)
- `second_hex: String` — hex of second run (empty for operational)
- `timestamp: VirtualTime` — when detected

**`DriftReport`**
- `events: Vec<DriftEvent>` — all detected events in order
- `total_checks: usize` — total calls to `check()`
- `passed: usize` — checks with no drift
- `drift_count()`, `constitutional_count()`, `operational_count()` — query helpers
- `is_clean()`, `has_constitutional_drift()` — boolean gates

**`DriftDetector`**
- `check(label, f)` — runs `f` twice via verifier; classifies and records any drift
- `check_allow_empty(label, f)` — same but empty output is not operational drift
- `report()` — returns a `DriftReport` snapshot
- `is_clean()`, `has_constitutional_drift()` — fast-path queries

**`DriftError`**
- `ConstitutionalDrift { label, first_hex, second_hex }` — P0-003 violation
- `EmptyOutputDrift { label }` — operational anomaly

## Design Decisions
1. **Severity ordering** — `Constitutional > Operational` enables `max()` aggregation
2. **Structured report** — `DriftReport` is a value type (cloneable snapshot), not a live view
3. **Wraps verifier** — reuses P0-011 double-run logic; no duplication
4. **Timestamp from verifier** — uses the verifier's internal clock post-tick for event timestamp
5. **Empty output as operational** — distinguishes "suspicious" from "constitutional violation"
