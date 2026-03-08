# Codec Traceability — P0-006

| Rule | Fixture | Expected Outcome |
|---|---|---|
| Canonical varint encoding | fixtures/codec/varint_examples.json | Encodings match declared hex vectors |
| Canonical struct field-id ordering | fixtures/codec/struct_example.json | Encoded payload matches expected hex |
| Canonical map key ordering | fixtures/codec/map_ordering_invalid.json | Rejected due to non-canonical entry ordering |
| Float prohibition in canonical form | spec-only rule (to be validated by encoder in P0-007) | Decoder/validator rejects float-bearing payloads |

## Notes
This traceability file links specification rules in `interfaces/specs/lyracodec.md` to concrete fixtures for downstream implementation and verification tasks.
