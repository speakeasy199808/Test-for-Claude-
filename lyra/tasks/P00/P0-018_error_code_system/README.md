# P0-018 — Error Code System

## Mission
Globally unique error codes (E0001–E9999) with categories. Machine-readable error metadata. Error catalog with explanations and suggested fixes.

## Scope
- `ErrorCode` newtype (u16, range 1–9999, displayed as `E0001`–`E9999`)
- `ErrorCategory` enum with 10 categories mapped to code ranges
- `ErrorEntry` struct with code, category, message, explanation, suggestion
- `ErrorCatalog` registry with lookup, registration, and category filtering
- Default catalog with seed entries across all categories
- Serde serialization for machine-readable error metadata
- Comprehensive unit tests

## Primary Archetype
Core Module Implementation

## Primary Ownership Root
`k0/src/errors/`

## Secondary Touched Roots
`lyra/tasks/`

## Deliverables
- `k0/src/errors/mod.rs` — module root with tests
- `k0/src/errors/code.rs` — ErrorCode, ErrorCategory, ErrorEntry types
- `k0/src/errors/catalog.rs` — ErrorCatalog registry with default entries
