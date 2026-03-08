# Genesis Traceability — P0-001

## Invariant → Module → Fixture → Test Mapping

| Invariant (P0-003) | Module | Fixture | Test |
|---|---|---|---|
| Determinism: identical input → identical output | `k0/src/genesis/state.rs` | `fixtures/genesis/genesis_state.json` | `canonical_genesis_serializes_deterministically` |
| Determinism: hash is stable | `k0/src/genesis/hash.rs` | `fixtures/genesis/constitutional_hash.golden` | `constitutional_hash_is_deterministic` |
| Non-bypassability: validate() must pass | `k0/src/genesis/state.rs` | — | `nonzero_sequence_is_rejected`, `empty_system_id_is_rejected` |
| Canonical representation: ordered JSON | `k0/src/genesis/state.rs` | `fixtures/genesis/genesis_state.json` | `canonical_genesis_serializes_deterministically` |
| Canonical representation: ordered trust roots | `k0/src/genesis/trust_roots.rs` | `fixtures/genesis/trust_roots_empty.json` | `non_canonical_ordering_is_rejected` |
| Explicit ownership: genesis lives in k0/ | `k0/src/genesis/` | — | (structural) |
| Replayability: golden hash pins expected output | `k0/src/genesis/hash.rs` | `fixtures/genesis/constitutional_hash.golden` | `canonical_genesis_hash_is_known` |

## Module Inventory

| Module | Path | Exports |
|---|---|---|
| Genesis module root | `k0/src/genesis/mod.rs` | `GenesisState`, `GENESIS_VERSION`, `ConstitutionalHash`, `TrustRoots` |
| Genesis state | `k0/src/genesis/state.rs` | `GenesisState`, `GENESIS_VERSION`, `GenesisError` |
| Constitutional hash | `k0/src/genesis/hash.rs` | `ConstitutionalHash`, `ConstitutionalHashError` |
| Trust roots | `k0/src/genesis/trust_roots.rs` | `TrustRoots`, `TrustRootEntry`, `TrustRootKind`, `TrustRootError` |

## Fixture Inventory

| Fixture | Path | Description |
|---|---|---|
| Genesis state | `fixtures/genesis/genesis_state.json` | Canonical `GenesisState::canonical()` as JSON |
| Constitutional hash golden | `fixtures/genesis/constitutional_hash.golden` | Pinned SHA-3-256 hex of canonical genesis state |
| Empty trust roots | `fixtures/genesis/trust_roots_empty.json` | Valid empty `TrustRoots` for genesis baseline |

## Test Inventory

| Test | Module | Invariant Covered |
|---|---|---|
| `canonical_genesis_is_valid` | state | Non-bypassability: canonical state passes validate() |
| `canonical_genesis_version_is_one` | state | Schema version pinned |
| `canonical_genesis_sequence_is_zero` | state | Sequence invariant |
| `canonical_genesis_system_id_is_lyra` | state | System identity |
| `canonical_genesis_serializes_deterministically` | state | Determinism |
| `nonzero_sequence_is_rejected` | state | Non-bypassability |
| `empty_system_id_is_rejected` | state | Non-bypassability |
| `whitespace_system_id_is_rejected` | state | Non-bypassability |
| `invalid_trust_root_fingerprint_is_rejected` | state | Non-bypassability |
| `constitutional_hash_is_deterministic` | hash | Determinism |
| `constitutional_hash_hex_is_64_chars` | hash | Canonical representation |
| `constitutional_hash_hex_roundtrip` | hash | Replayability |
| `mutated_state_produces_different_hash` | hash | Collision resistance |
| `from_hex_rejects_short_string` | hash | Non-bypassability |
| `from_hex_rejects_invalid_chars` | hash | Non-bypassability |
| `canonical_genesis_hash_is_known` | hash | Golden hash pinning |
| `empty_trust_roots_is_valid` | trust_roots | Canonical baseline |
| `single_valid_entry_is_accepted` | trust_roots | Valid entry acceptance |
| `non_canonical_ordering_is_rejected` | trust_roots | Canonical ordering |
| `duplicate_id_is_rejected` | trust_roots | Non-bypassability |
| `uppercase_fingerprint_is_rejected` | trust_roots | Canonical representation |
| `short_fingerprint_is_rejected` | trust_roots | Non-bypassability |

## Notes
- The constitutional hash golden value in `fixtures/genesis/constitutional_hash.golden` is computed
  by running `cargo test -p k0 -- genesis::hash::tests::canonical_genesis_hash_is_known --nocapture`
  and recording the printed hash.
- Any change to `GenesisState::canonical()` or its serialization is a breaking change requiring
  a new golden hash and a version bump.
