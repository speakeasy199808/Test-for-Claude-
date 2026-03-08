# P0-001 — Genesis State

## Mission
Implement the initial canonical state of the Lyra system: genesis state definition,
constitutional hash, and trust root declarations in the `k0/` crate.

## Scope
- Genesis state struct with canonical serialization
- Constitutional hash (SHA-3-256 of canonical genesis state bytes)
- Trust root declarations and validation
- Fixtures and golden outputs for genesis state and hash
- Integration with constitutional math invariants (P0-003)

## Primary Archetype
Core Module Implementation

## Work Package Class
multi-module

## Primary Ownership Root
`k0/`

## Secondary Touched Roots
`interfaces/`, `fixtures/`, `docs/`, `lyra/tasks/`

## Deliverables
- `k0/src/genesis/mod.rs` — genesis module root
- `k0/src/genesis/state.rs` — GenesisState struct, validation, canonical serialization
- `k0/src/genesis/hash.rs` — ConstitutionalHash (SHA-3-256), hex roundtrip
- `k0/src/genesis/trust_roots.rs` — TrustRoots, TrustRootEntry, canonical ordering
- Genesis state fixture (JSON)
- Constitutional hash golden fixture
- Trust roots fixture (JSON)
- Acceptance record and traceability artifact
