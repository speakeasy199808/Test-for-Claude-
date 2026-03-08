# Implementation Notes — P0-007 Canonical Encoder

## Work Package Shape
Multi-module Rust implementation in `k0/src/codec/` with task-local control plane, fixtures, and artifacts.

## Produced Components

| File | Description |
|---|---|
| `k0/src/codec/mod.rs` | Public API: re-exports `encode`, `decode`, `CodecError`, `Value`, `StructField`, all tag constants |
| `k0/src/codec/types.rs` | `TAG_*` constants, `Value` enum (no float variants), `StructField` |
| `k0/src/codec/varint.rs` | `encode_u64`, `decode_u64`, `encode_i64`, `decode_i64` — LEB128 + zigzag |
| `k0/src/codec/encoder.rs` | `encode(value) -> Result<Vec<u8>>` — sorts struct fields and map entries canonically |
| `k0/src/codec/decoder.rs` | `decode(bytes) -> Result<Value>` — enforces all canonicality constraints |
| `k0/src/codec/error.rs` | `CodecError` with structured context fields for all error cases |

## Test Coverage

- **varint**: zero, one, 127, 128, 300, u64::MAX, i64::MIN/MAX, roundtrips, non-canonical rejection, overflow
- **encoder**: all type families, golden fixture match (struct_example), field sort, map key sort, determinism
- **decoder**: all type families, roundtrips, fixture decode, non-canonical map order rejection, non-canonical field order rejection, unknown tag rejection

Total: 42 codec tests + 22 genesis tests = 64 tests passing.

## Ownership Placement
- Production code: `k0/src/codec/` (primary ownership root `k0/`)
- Task control plane: `lyra/tasks/P00/P0-007_canonical_encoder/`
- Spec consumed: `interfaces/specs/lyracodec.md` (P0-006)

## Dependency Posture
- Hard prerequisite: P0-006 LyraCodec spec
- Enables: P0-008 (digest), P0-011 (determinism verifier), P0-023 (foundation integration)
- No new crate dependencies required (uses `thiserror` already in workspace)

## Float Prohibition Implementation
The `Value` enum has no float variant. The type system enforces the prohibition at compile time — no runtime check is needed or possible.
