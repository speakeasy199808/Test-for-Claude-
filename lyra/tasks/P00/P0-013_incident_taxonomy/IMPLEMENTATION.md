# Implementation Notes — P0-013 Incident Taxonomy

## Work Package Shape
Type system + specification module. No external dependencies beyond `k0::time`. Pure value types, no I/O, no ambient state.

## Produced Components

### `k0/src/incident/taxonomy.rs`
- `IncidentSeverity` — `Low / Medium / High / Critical` (ordered, `Critical > High > Medium > Low`)
  - `name()`, `is_constitutional()`, `Display`
- `IncidentKind` — 14 variants with stable INC-NNN codes
  - `severity()` — canonical severity for each kind
  - `description()` — canonical description for each kind
  - `code()` — stable INC-NNN code
  - `is_constitutional()` — true iff severity is Critical
  - `constitutional_kinds()` — static slice of the 4 Critical kinds
  - `Display` — `"INC-001 (critical)"`
- 21 unit tests

### `k0/src/incident/record.rs`
- `Incident` — `{ kind, severity, label, timestamp, context }`
  - `new(kind, label, timestamp, context)` — severity derived from kind
  - `new_bare(kind, label, timestamp)` — empty context
  - `is_constitutional()`, `code()`
  - `Display` — `"[INC-001] critical at t:5 label=\"my-fn\""`
- 10 unit tests

### `k0/src/incident/mod.rs`
- Public re-exports: `Incident`, `IncidentKind`, `IncidentSeverity`
- 8 integration tests

## Ownership Placement
- Primary: `k0/src/incident/` (constitutional classification substrate)
- Task control plane: `lyra/tasks/P00/P0-013_incident_taxonomy/`

## Test Evidence
```
cargo test -p k0 --lib
test result: ok. 230 passed; 0 failed; 0 ignored
```
Incident-specific tests: 39 (21 in `taxonomy.rs`, 10 in `record.rs`, 8 in `mod.rs`)

## Dependency Posture
- Consumes: `k0::time` (VirtualTime)
- Enables: P0-014 (recovery protocols), P0-023 (foundation integration)
