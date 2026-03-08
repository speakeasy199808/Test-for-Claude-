# P0-007 — Canonical Encoder

## Mission
Rust implementation of LyraCodec. Fixed-point rationals only. Deterministic byte output for identical inputs.

## Scope
- Rust implementation of LyraCodec encoder and decoder
- All types: unsigned varint, signed varint (zigzag), bytes, UTF-8 string, struct, vector, map
- Fixed-point rationals only — floating-point types are forbidden
- Deterministic byte output for identical inputs
- Canonical struct field ordering (ascending field_id)
- Canonical map key ordering (lexicographic encoded key bytes)
- Canonical varint encoding (no unnecessary continuation bytes)

## Primary Archetype
Core Module Implementation

## Primary Ownership Root
`k0/`

## Secondary Touched Roots
`interfaces/`, `fixtures/`, `lyra/tasks/`

## Deliverables
- `k0/src/codec/mod.rs` — public API and re-exports
- `k0/src/codec/types.rs` — type tags and `Value` enum
- `k0/src/codec/varint.rs` — LEB128 unsigned and zigzag-signed varint
- `k0/src/codec/encoder.rs` — canonical encoder
- `k0/src/codec/decoder.rs` — canonical decoder with canonicality enforcement
- `k0/src/codec/error.rs` — `CodecError` type
- Task control plane: README, ACCEPTANCE, DESIGN, IMPLEMENTATION, task.toml
- Fixtures: golden byte vectors for all type families
- Artifacts: encoder traceability manifest
