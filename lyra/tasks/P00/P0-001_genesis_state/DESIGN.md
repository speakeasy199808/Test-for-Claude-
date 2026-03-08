# Design — P0-001 Genesis State

## Architectural Model

The genesis state is the root of all Lyra state derivation. It is:
1. **Immutable once sealed** — no field may change after the constitutional hash is computed
2. **Deterministic** — identical inputs always produce identical outputs
3. **Self-describing** — carries its own schema version and constitutional version
4. **Verifiable** — the constitutional hash seals the entire genesis configuration

## Module Structure

```
k0/src/genesis/
├── mod.rs          — module root, re-exports public API
├── state.rs        — GenesisState struct, validation, canonical serialization
├── hash.rs         — ConstitutionalHash (SHA-3-256), hex encoding/decoding
└── trust_roots.rs  — TrustRoots, TrustRootEntry, canonical ordering
```

## GenesisState

Fields:
- `version: u32` — schema version (must be >= 1)
- `system_id: String` — ASCII identifier, no whitespace (e.g. "lyra")
- `sequence: u64` — always 0 for genesis
- `declared_at_unix_secs: u64` — deterministic constant (2024-01-01T00:00:00Z = 1704067200)
- `constitutional_version: u32` — references P0-003 spec version
- `trust_root_fingerprints: Vec<String>` — SHA-3-256 hex fingerprints of trust roots

Invariants enforced by `validate()`:
- version >= 1
- system_id non-empty, ASCII, no whitespace
- sequence == 0
- constitutional_version >= 1
- each fingerprint is exactly 64 lowercase hex chars

## ConstitutionalHash

Algorithm: SHA-3-256 (Keccak) over canonical JSON bytes of GenesisState.

Properties:
- Deterministic: same GenesisState → same hash
- Collision-resistant: any mutation → different hash
- Hex-encodable: 64-char lowercase hex string
- Roundtrip-safe: from_hex(to_hex(h)) == h

## TrustRoots

A canonically ordered set of trust root entries. Each entry has:
- `id: String` — unique ASCII identifier
- `kind: TrustRootKind` — SigningKey | SpecDocument | ConstitutionalSpec
- `fingerprint: String` — 64-char lowercase hex SHA-3-256
- `description: String` — human-readable description

Invariants:
- Entries sorted by id (ascending lexicographic)
- No duplicate ids
- All fingerprints are 64-char lowercase hex

## Constitutional Guarantee

The genesis state is sealed by the constitutional hash. The hash is computed
once at genesis and pinned as a golden fixture. Any subsequent change to the
genesis state definition is a breaking change that requires a new golden hash.

## Dependency on P0-003

The `constitutional_version` field references the P0-003 constitutional math
spec version. The genesis state is the first consumer of the constitutional
invariant set.
