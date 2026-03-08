# Acceptance — P0-001 Genesis State

## Acceptance Criteria

1. `k0/src/genesis/state.rs` defines `GenesisState` with:
   - All fields deterministic (no floats, no ambient randomness)
   - `validate()` enforces all structural invariants
   - `to_canonical_bytes()` produces deterministic JSON output
   - Unit tests cover valid canonical state and all invalid cases

2. `k0/src/genesis/hash.rs` defines `ConstitutionalHash` with:
   - SHA-3-256 digest of canonical genesis state bytes
   - Deterministic: same state always produces same hash
   - Hex roundtrip is lossless
   - Mutated state produces different hash (collision resistance test)
   - Golden hash test pins the canonical genesis hash

3. `k0/src/genesis/trust_roots.rs` defines `TrustRoots` with:
   - Canonical ordering enforced (ascending id)
   - Duplicate id detection
   - Fingerprint format validation (64-char lowercase hex)
   - Empty trust roots valid for genesis

4. All genesis modules compile under `cargo check -p k0`.

5. `cargo test -p k0` passes all genesis tests.

6. Genesis state fixture matches the canonical `GenesisState::canonical()` definition.

7. Constitutional hash golden fixture records the pinned hash of the canonical genesis state.

## Verification Method
- `cargo check -p k0` — zero errors
- `cargo test -p k0 -- genesis` — all genesis tests pass
- Fixture review against `GenesisState::canonical()` definition
- Golden hash fixture reviewed for consistency

## Evidence Required
- `artifacts/genesis-traceability.md`
- `fixtures/genesis/genesis_state.json`
- `fixtures/genesis/constitutional_hash.golden`
- `fixtures/genesis/trust_roots_empty.json`
- `cargo test -p k0` pass (recorded in implementation notes)
