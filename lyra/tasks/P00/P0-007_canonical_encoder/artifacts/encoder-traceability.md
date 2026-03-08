# Encoder Traceability — P0-007

## Spec Rule → Implementation → Test Mapping

| Spec Rule (P0-006) | Module | Test |
|---|---|---|
| Unsigned LEB128 varint | `k0/src/codec/varint.rs` | `u64_zero_encodes_to_single_zero_byte`, `u64_128_encodes_to_two_bytes`, `u64_roundtrip_various` |
| Signed zigzag + LEB128 | `k0/src/codec/varint.rs` | `i64_minus_one_encodes_to_one`, `i64_roundtrip_various` |
| Canonical varint (no trailing zeros) | `k0/src/codec/varint.rs` | `decode_u64_rejects_non_canonical_trailing_zero` |
| Struct field ordering (ascending field_id) | `k0/src/codec/encoder.rs` | `encode_struct_sorts_fields`, `encode_struct_matches_fixture` |
| Struct field ordering enforced on decode | `k0/src/codec/decoder.rs` | `decode_rejects_non_canonical_field_order` |
| Map key ordering (lexicographic encoded bytes) | `k0/src/codec/encoder.rs` | `encode_map_sorts_by_key_bytes` |
| Map key ordering enforced on decode | `k0/src/codec/decoder.rs` | `decode_rejects_non_canonical_map_order` |
| Float prohibition | `k0/src/codec/types.rs` | (type-level: no float variant in `Value` enum) |
| Bytes encoding (length-prefixed) | `k0/src/codec/encoder.rs` | `encode_bytes`, `decode_bytes_roundtrip` |
| UTF-8 string encoding (length-prefixed) | `k0/src/codec/encoder.rs` | `encode_string`, `decode_string_roundtrip` |
| Vector encoding (elem_tag + length + payloads) | `k0/src/codec/encoder.rs` | `encode_vector_uint`, `decode_vector_roundtrip` |
| Unknown tag rejection | `k0/src/codec/decoder.rs` | `decode_rejects_unknown_tag` |
| Determinism | `k0/src/codec/encoder.rs` | `encode_is_deterministic` |
| P0-006 struct fixture | `k0/src/codec/encoder.rs` + `decoder.rs` | `encode_struct_matches_fixture`, `decode_struct_fixture` |

## Module Inventory

| Module | Path | Exports |
|---|---|---|
| Codec root | `k0/src/codec/mod.rs` | `encode`, `decode`, `CodecError`, `Value`, `StructField`, `TAG_*` |
| Types | `k0/src/codec/types.rs` | `TAG_VARINT_U`, `TAG_VARINT_S`, `TAG_STRUCT`, `TAG_VECTOR`, `TAG_MAP`, `TAG_BYTES`, `TAG_STRING`, `Value`, `StructField` |
| Varint | `k0/src/codec/varint.rs` | `encode_u64`, `decode_u64`, `encode_i64`, `decode_i64` |
| Encoder | `k0/src/codec/encoder.rs` | `encode` |
| Decoder | `k0/src/codec/decoder.rs` | `decode` |
| Error | `k0/src/codec/error.rs` | `CodecError` |

## Fixture Inventory

| Fixture | Path | Description |
|---|---|---|
| Golden vectors | `fixtures/codec/golden_vectors.json` | Canonical byte vectors for all type families |
| P0-006 varint examples | `lyra/tasks/P00/P0-006_lyracodec_spec/fixtures/codec/varint_examples.json` | Varint golden vectors from spec |
| P0-006 struct example | `lyra/tasks/P00/P0-006_lyracodec_spec/fixtures/codec/struct_example.json` | Struct golden vector from spec |
| P0-006 map ordering invalid | `lyra/tasks/P00/P0-006_lyracodec_spec/fixtures/codec/map_ordering_invalid.json` | Invalid map fixture (non-canonical order) |

## Test Summary

- Total codec tests: 42
- Varint tests: 14
- Encoder tests: 14
- Decoder tests: 14
- All pass: `cargo test -p k0` → 64/64 ok
