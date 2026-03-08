# LyraCodec Canonical Encoding Specification (P0-006)

Status: Normative  
Owner: `interfaces/`  
Version: 1.0.0

## 1. Scope
Canonical encoding for Lyra data types:
- varints
- structs
- vectors
- maps

Constraints:
- versioned schemas required
- floating-point types are forbidden in canonical form

## 2. General Encoding Rules
1. All values are encoded as `<type_tag><payload>`.
2. Integer fields use unsigned LEB128 varint unless schema specifies signed zigzag+varint.
3. Field ordering in structs is canonical by ascending field id.
4. Map entries are sorted by canonical key byte representation.
5. Vectors preserve element order exactly.
6. No NaN/float/double types in canonical data.
7. Each top-level payload carries schema version header.

## 3. Type Tags
- `0x01` = varint unsigned
- `0x02` = varint signed (zigzag)
- `0x10` = struct
- `0x20` = vector
- `0x30` = map
- `0x40` = bytes
- `0x41` = utf8 string

## 4. Struct Encoding
`0x10 <schema_version_varint> <field_count_varint> <field_entries...>`

Each field entry:
`<field_id_varint> <value_type_tag> <value_payload>`

## 5. Vector Encoding
`0x20 <elem_type_tag> <length_varint> <elem_payload_0> ... <elem_payload_n>`

## 6. Map Encoding
`0x30 <key_type_tag> <value_type_tag> <length_varint> <sorted_entries...>`

Each entry:
`<key_payload> <value_payload>`

Entries MUST be sorted by lexicographic ordering of canonical encoded key payload bytes.

## 7. Versioned Schema Contract
Every schema publishes:
- `schema_id`
- `schema_version`
- field id map
- compatibility policy

Decoders MUST reject unknown major versions.

## 8. Forbidden Canonical Forms
The following are invalid canonical payloads:
- floating point encodings of any width
- unordered map entries
- duplicate struct field ids
- non-canonical varint encodings

## 9. Fixtures and Traceability
Fixtures are stored under:
`lyra/tasks/P00/P0-006_lyracodec_spec/fixtures/codec/`

Traceability:
`lyra/tasks/P00/P0-006_lyracodec_spec/artifacts/codec-traceability.md`
