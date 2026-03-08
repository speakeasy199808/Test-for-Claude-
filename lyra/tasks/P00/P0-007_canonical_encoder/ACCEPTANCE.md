# Acceptance — P0-007 Canonical Encoder

## Acceptance Criteria

1. Rust implementation exists for all LyraCodec types:
   - Unsigned varint (LEB128)
   - Signed varint (zigzag + LEB128)
   - Bytes (length-prefixed)
   - UTF-8 string (length-prefixed)
   - Struct (schema-versioned, field-id-ordered)
   - Vector (homogeneous, order-preserving)
   - Map (key-sorted, homogeneous)
2. Floating-point types are absent from the `Value` enum and all encoding paths.
3. Encoding is deterministic: identical `Value` inputs always produce identical byte outputs.
4. Struct fields are encoded in ascending `field_id` order regardless of input order.
5. Map entries are encoded in lexicographic order of canonical encoded key payload bytes.
6. Varint encoding is canonical: no unnecessary continuation bytes.
7. Decoder enforces all canonicality constraints and rejects non-canonical inputs.
8. All golden byte vectors from P0-006 fixtures are reproduced exactly.
9. Encode→decode roundtrip is lossless for all value types.

## Verification Method
- Unit tests in `k0/src/codec/` covering all types, golden vectors, and rejection cases
- `cargo test -p k0` passes with 0 failures

## Evidence Required
- `artifacts/encoder-traceability.md`
- `fixtures/codec/*.json` (golden byte vectors)
- `cargo test -p k0` output: all codec tests pass
