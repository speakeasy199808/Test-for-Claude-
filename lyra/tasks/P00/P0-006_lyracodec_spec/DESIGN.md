# Design — P0-006 LyraCodec Spec

## Codec Model

LyraCodec is the canonical binary encoding format for all Lyra data types. It is the single authoritative wire format at every system boundary. All encoders and decoders system-wide MUST conform to this specification.

### Core Design Principles

1. **Determinism** — identical logical values produce byte-identical encodings on every platform and runtime.
2. **No floating-point** — canonical form is restricted to integer, rational (fixed-point), string, bytes, struct, vector, and map types.
3. **Canonical ordering** — struct fields ordered by ascending field ID; map entries ordered by lexicographic canonical key bytes.
4. **Versioned schemas** — every top-level payload carries a schema version header; decoders reject unknown major versions.
5. **Self-describing type tags** — every value is prefixed with a type tag byte enabling forward-compatible decoding.

## Encoding Layers

### Layer 1 — Primitive Encoding
- Unsigned integers: LEB128 varint (unsigned)
- Signed integers: zigzag encoding then LEB128 varint
- Bytes: length-prefixed raw bytes
- UTF-8 strings: length-prefixed UTF-8 bytes

### Layer 2 — Composite Encoding
- Structs: schema-version-prefixed, field-id-ordered field entries
- Vectors: element-type-tagged, length-prefixed element sequence
- Maps: key-type and value-type tagged, length-prefixed, lexicographically sorted entries

### Layer 3 — Schema Contract
- Every schema publishes: `schema_id`, `schema_version`, field ID map, compatibility policy
- Decoders MUST reject unknown major versions
- Encoders MUST emit the schema version in every top-level struct payload

## Type Tag Table

| Tag Byte | Type |
|---|---|
| `0x01` | varint unsigned (LEB128) |
| `0x02` | varint signed (zigzag + LEB128) |
| `0x10` | struct |
| `0x20` | vector |
| `0x30` | map |
| `0x40` | bytes (raw, length-prefixed) |
| `0x41` | utf8 string (length-prefixed) |

## Forbidden Canonical Forms

The following are invalid and MUST be rejected by any conforming decoder or validator:
- Floating-point encodings of any width (IEEE 754 f32, f64, or any variant)
- Non-canonical varint encodings (e.g., over-long encodings with unnecessary leading zero bytes)
- Unordered map entries (entries not sorted by canonical key byte representation)
- Duplicate struct field IDs within a single struct payload
- Unknown type tags in a strict-mode decoder

## Consumption Contract

Downstream tasks consume this specification as the authoritative source for:
- P0-007 `canonical_encoder` — Rust implementation of LyraCodec encoder/decoder
- P0-011 `determinism_verifier` — verifies encoder output is deterministic
- P0-023 `foundation_integration` — end-to-end codec round-trip validation

## Fixture Strategy

Fixtures are organized as:
- `varint_examples.json` + `varint_examples.hex` — varint encoding vectors
- `struct_example.json` + `struct_example.hex` — struct encoding example
- `map_ordering_invalid.json` — negative fixture: rejected due to non-canonical ordering

Hex fixtures contain the raw canonical byte sequences for byte-level verification by downstream encoder implementations.
