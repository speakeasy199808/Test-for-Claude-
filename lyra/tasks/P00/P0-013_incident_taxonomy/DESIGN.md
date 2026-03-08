# Design — P0-013 Incident Taxonomy

## Constitutional Basis
The incident taxonomy is the classification layer for all P0-003 invariant violations and operational failures. Every subsystem that detects a failure MUST classify it using this taxonomy before propagating it upward.

## Architecture

### Module Layout
```
k0/src/incident/
├── mod.rs        — public API, re-exports, integration tests
├── taxonomy.rs   — IncidentKind, IncidentSeverity
└── record.rs     — Incident (structured event)
```

### Core Types

**`IncidentSeverity`** (ordered: Low < Medium < High < Critical)
- `Low`      — informational / recoverable
- `Medium`   — operational anomaly
- `High`     — significant operational failure
- `Critical` — constitutional violation (P0-003 breach)

**`IncidentKind`** — 14 canonical kinds with stable codes
```
INC-000  Unknown                  Low
INC-001  DeterminismViolation     Critical  ← P0-003
INC-002  ConstitutionalBreach     Critical  ← P0-003
INC-003  TrustRootViolation       Critical  ← P0-003
INC-004  DigestMismatch           Critical  ← P0-003
INC-005  EncodingError            High
INC-006  DecodingError            High
INC-007  SchemaVersionMismatch    High
INC-008  StateTransitionRejected  High
INC-009  EntropyAnomaly           Medium
INC-010  TimeAnomaly              Medium
INC-011  OperationalDrift         Medium
INC-012  BoundaryViolation        Medium
INC-013  RecoverableError         Low
```

**`Incident`** — structured event record
- `kind: IncidentKind` — canonical classification
- `severity: IncidentSeverity` — derived from kind (never set independently)
- `label: String` — monitoring point name
- `timestamp: VirtualTime` — when detected
- `context: String` — optional evidence (hex, error message, etc.)

## Design Decisions
1. **Severity derived from kind** — severity is never set independently; it is always the canonical value for the kind. This prevents misclassification.
2. **Stable codes** — INC-NNN codes are stable identifiers for log correlation, alerting, and downstream tooling.
3. **Constitutional kinds are a closed set** — only 4 kinds are constitutional; this set is fixed by P0-003.
4. **`constitutional_kinds()` is a static slice** — enables exhaustive iteration without allocation.
5. **`Incident` is a value type** — cloneable, no interior mutability, no ambient state.
