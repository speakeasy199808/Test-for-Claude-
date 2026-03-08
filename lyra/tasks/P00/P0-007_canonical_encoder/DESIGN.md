# Design — P0-007 Canonical Encoder

## Architecture

The canonical encoder is a pure Rust module family at `k0/src/codec/`. It has no external I/O, no ambient nondeterminism, and no floating-point types anywhere in its public or private API.

### Module Decomposition

| Module | Responsibility |
|---|---|
| `types.rs` | Type tag constants, `Value` enum, `StructField` struct |
| `varint.rs` | LEB128 unsigned encode/decode; zigzag + LEB128 signed encode/decode |
| `encoder.rs` | `encode(value) -> Vec<u8>` — deterministic canonical serialization |
| `decoder.rs` | `decode(bytes) -> Result<Value>` — canonical deserialization with constraint enforcement |
| `error.rs` | `CodecError` — all error variants with structured context |
| `mod.rs` | Public API surface, re-exports |

### Type Tag Map

| Tag | Hex | Type |
|---|---|---|
| `TAG_VARINT_U` | `0x01` | Unsigned 64-bit integer (LEB128) |
| `TAG_VARINT_S` | `0x02` | Signed 64-bit integer (zigzag + LEB128) |
| `TAG_STRUCT` | `0x10` | Schema-versioned struct |
| `TAG_VECTOR` | `0x20` | Homogeneous vector |
| `TAG_MAP` | `0x30` | Key-sorted homogeneous map |
| `TAG_BYTES` | `0x40` | Raw byte sequence |
| `TAG_STRING` | `0x41` | UTF-8 string |

### Encoding Formats

**Unsigned varint:** LEB128. Each byte contributes 7 bits; high bit = continuation.

**Signed varint:** Zigzag mapping `(n << 1) ^ (n >> 63)` then LEB128.

**Bytes:** `<TAG_BYTES> <length_varint> <raw_bytes>`

**String:** `<TAG_STRING> <length_varint> <utf8_bytes>`

**Struct:** `<TAG_STRUCT> <schema_version_varint> <field_count_varint> [<field_id_varint> <tagged_value>]...`
Fields sorted by ascending `field_id` at encode time.

**Vector:** `<TAG_VECTOR> <elem_type_tag> <length_varint> [<elem_payload>]...`
Element order is preserved exactly.

**Map:** `<TAG_MAP> <key_type_tag> <value_type_tag> <length_varint> [<key_payload> <value_payload>]...`
Entries sorted by lexicographic ordering of canonical encoded key payload bytes.

### Canonicality Constraints (enforced by decoder)

1. Varint: no unnecessary continuation bytes (non-canonical trailing zeros rejected).
2. Struct: fields must appear in strictly ascending `field_id` order; duplicate field ids rejected.
3. Map: entries must appear in strictly ascending lexicographic order of encoded key bytes.
4. No unknown type tags.
5. No floating-point types (absent from `Value` enum entirely).

### Float Prohibition

The `Value` enum has no `Float`, `Double`, or `FixedPoint` variant. Any attempt to encode a floating-point value requires the caller to convert to a fixed-point integer representation first. This is enforced at the type level — there is no runtime check needed.

## Dependency Posture

- Consumes: P0-006 LyraCodec spec (`interfaces/specs/lyracodec.md`)
- Consumed by: P0-008 (digest algorithms use codec for canonical input), P0-011 (determinism verifier), P0-023 (foundation integration)
- No external crate dependencies beyond `thiserror` (already in workspace)
