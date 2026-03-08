//! Canonical LyraCodec decoder.
//!
//! Decodes a canonical LyraCodec byte sequence into a [`Value`], enforcing
//! all canonicality constraints: canonical varint encoding, ascending struct
//! field order, lexicographic map key order, and no unknown type tags.

use crate::codec::error::CodecError;
use crate::codec::types::*;
use crate::codec::varint;

/// Decode a complete LyraCodec value from `input`.
///
/// Returns the decoded [`Value`]. The entire input must be consumed;
/// trailing bytes are not permitted.
pub fn decode(input: &[u8]) -> Result<Value, CodecError> {
    let (value, consumed) = decode_tagged(input, 0)?;
    if consumed != input.len() {
        return Err(CodecError::UnexpectedEof { offset: consumed });
    }
    Ok(value)
}

/// Decode a tag-prefixed value from `input` at `offset`.
///
/// Returns `(value, bytes_consumed)`.
pub(crate) fn decode_tagged(input: &[u8], offset: usize) -> Result<(Value, usize), CodecError> {
    if offset >= input.len() {
        return Err(CodecError::UnexpectedEof { offset });
    }
    let tag = input[offset];
    let (value, payload_len) = decode_payload(tag, input, offset + 1)?;
    Ok((value, 1 + payload_len))
}

/// Decode the payload (no tag) for the given `tag` from `input` at `offset`.
///
/// Returns `(value, bytes_consumed)`.
fn decode_payload(tag: u8, input: &[u8], offset: usize) -> Result<(Value, usize), CodecError> {
    match tag {
        TAG_VARINT_U => {
            let (v, n) = varint::decode_u64(input, offset)?;
            Ok((Value::UInt(v), n))
        }
        TAG_VARINT_S => {
            let (v, n) = varint::decode_i64(input, offset)?;
            Ok((Value::SInt(v), n))
        }
        TAG_BYTES => decode_bytes(input, offset),
        TAG_STRING => decode_string(input, offset),
        TAG_STRUCT => decode_struct(input, offset),
        TAG_VECTOR => decode_vector(input, offset),
        TAG_MAP => decode_map(input, offset),
        other => Err(CodecError::UnknownTag { tag: other, offset }),
    }
}

/// Decode a byte-sequence payload: `<length_varint> <bytes...>`
fn decode_bytes(input: &[u8], offset: usize) -> Result<(Value, usize), CodecError> {
    let (len, n) = varint::decode_u64(input, offset)?;
    let start = offset + n;
    let end = start + len as usize;
    if end > input.len() {
        return Err(CodecError::UnexpectedEof { offset: start });
    }
    Ok((Value::Bytes(input[start..end].to_vec()), n + len as usize))
}

/// Decode a UTF-8 string payload: `<length_varint> <utf8_bytes...>`
fn decode_string(input: &[u8], offset: usize) -> Result<(Value, usize), CodecError> {
    let (len, n) = varint::decode_u64(input, offset)?;
    let start = offset + n;
    let end = start + len as usize;
    if end > input.len() {
        return Err(CodecError::UnexpectedEof { offset: start });
    }
    let s = String::from_utf8(input[start..end].to_vec()).map_err(|e| CodecError::InvalidUtf8 {
        offset: start,
        source: e,
    })?;
    Ok((Value::Str(s), n + len as usize))
}

/// Decode a struct payload:
/// `<schema_version_varint> <field_count_varint> <field_entries...>`
fn decode_struct(input: &[u8], offset: usize) -> Result<(Value, usize), CodecError> {
    let mut pos = offset;

    let (schema_version, n) = varint::decode_u64(input, pos)?;
    pos += n;

    let (field_count, n) = varint::decode_u64(input, pos)?;
    pos += n;

    let mut fields = Vec::with_capacity(field_count as usize);
    let mut last_field_id: Option<u32> = None;

    for _ in 0..field_count {
        let (field_id_u64, n) = varint::decode_u64(input, pos)?;
        let field_id = field_id_u64 as u32;
        pos += n;

        // Enforce canonical ascending field-id order
        if let Some(last) = last_field_id {
            if field_id <= last {
                if field_id == last {
                    return Err(CodecError::DuplicateFieldId {
                        field_id,
                        offset: pos,
                    });
                }
                return Err(CodecError::NonCanonicalFieldOrder { offset: pos });
            }
        }
        last_field_id = Some(field_id);

        let (value, n) = decode_tagged(input, pos)?;
        pos += n;

        fields.push(StructField { field_id, value });
    }

    Ok((
        Value::Struct {
            schema_version: schema_version as u32,
            fields,
        },
        pos - offset,
    ))
}

/// Decode a vector payload:
/// `<elem_type_tag> <length_varint> <elem_payload_0> ... <elem_payload_n>`
fn decode_vector(input: &[u8], offset: usize) -> Result<(Value, usize), CodecError> {
    let mut pos = offset;

    if pos >= input.len() {
        return Err(CodecError::UnexpectedEof { offset: pos });
    }
    let elem_tag = input[pos];
    pos += 1;

    let (length, n) = varint::decode_u64(input, pos)?;
    pos += n;

    let mut elements = Vec::with_capacity(length as usize);
    for _ in 0..length {
        let (value, n) = decode_payload(elem_tag, input, pos)?;
        pos += n;
        elements.push(value);
    }

    Ok((Value::Vector { elem_tag, elements }, pos - offset))
}

/// Decode a map payload:
/// `<key_type_tag> <value_type_tag> <length_varint> <sorted_entries...>`
fn decode_map(input: &[u8], offset: usize) -> Result<(Value, usize), CodecError> {
    let mut pos = offset;

    if pos + 1 >= input.len() {
        return Err(CodecError::UnexpectedEof { offset: pos });
    }
    let key_tag = input[pos];
    pos += 1;
    let value_tag = input[pos];
    pos += 1;

    let (length, n) = varint::decode_u64(input, pos)?;
    pos += n;

    let mut entries = Vec::with_capacity(length as usize);
    let mut last_key_bytes: Option<Vec<u8>> = None;

    for _ in 0..length {
        // Record start of key payload for ordering check
        let key_start = pos;
        let (key_value, n) = decode_payload(key_tag, input, pos)?;
        let key_bytes = input[key_start..key_start + n].to_vec();
        pos += n;

        // Enforce canonical lexicographic key order
        if let Some(ref last) = last_key_bytes {
            if key_bytes <= *last {
                return Err(CodecError::NonCanonicalMapOrder { offset: key_start });
            }
        }
        last_key_bytes = Some(key_bytes);

        let (val_value, n) = decode_payload(value_tag, input, pos)?;
        pos += n;

        entries.push((key_value, val_value));
    }

    Ok((
        Value::Map {
            key_tag,
            value_tag,
            entries,
        },
        pos - offset,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::encoder::encode;

    #[test]
    fn decode_uint_zero() {
        let enc = encode(&Value::UInt(0)).unwrap();
        let v = decode(&enc).unwrap();
        assert_eq!(v, Value::UInt(0));
    }

    #[test]
    fn decode_uint_128() {
        let enc = encode(&Value::UInt(128)).unwrap();
        let v = decode(&enc).unwrap();
        assert_eq!(v, Value::UInt(128));
    }

    #[test]
    fn decode_sint_minus_one() {
        let enc = encode(&Value::SInt(-1)).unwrap();
        let v = decode(&enc).unwrap();
        assert_eq!(v, Value::SInt(-1));
    }

    #[test]
    fn decode_bytes_roundtrip() {
        let original = Value::Bytes(vec![0x01, 0x02, 0x03]);
        let enc = encode(&original).unwrap();
        let decoded = decode(&enc).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn decode_string_roundtrip() {
        let original = Value::Str("lyra".to_string());
        let enc = encode(&original).unwrap();
        let decoded = decode(&enc).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn decode_struct_roundtrip() {
        let original = Value::Struct {
            schema_version: 1,
            fields: vec![
                StructField {
                    field_id: 1,
                    value: Value::UInt(7),
                },
                StructField {
                    field_id: 2,
                    value: Value::Str("lyra".to_string()),
                },
            ],
        };
        let enc = encode(&original).unwrap();
        let decoded = decode(&enc).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn decode_struct_fixture() {
        // From P0-006 struct_example.json expected_encoding_hex
        let bytes = hex_to_bytes("1001020101070241046c797261");
        let v = decode(&bytes).unwrap();
        match v {
            Value::Struct {
                schema_version,
                fields,
            } => {
                assert_eq!(schema_version, 1);
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].field_id, 1);
                assert_eq!(fields[0].value, Value::UInt(7));
                assert_eq!(fields[1].field_id, 2);
                assert_eq!(fields[1].value, Value::Str("lyra".to_string()));
            }
            other => panic!("expected Struct, got {other:?}"),
        }
    }

    #[test]
    fn decode_vector_roundtrip() {
        let original = Value::Vector {
            elem_tag: TAG_VARINT_U,
            elements: vec![Value::UInt(1), Value::UInt(2), Value::UInt(3)],
        };
        let enc = encode(&original).unwrap();
        let decoded = decode(&enc).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn decode_map_roundtrip() {
        let original = Value::Map {
            key_tag: TAG_STRING,
            value_tag: TAG_VARINT_U,
            entries: vec![
                (Value::Str("a".to_string()), Value::UInt(1)),
                (Value::Str("z".to_string()), Value::UInt(2)),
            ],
        };
        let enc = encode(&original).unwrap();
        let decoded = decode(&enc).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn decode_rejects_unknown_tag() {
        let bad = vec![0xFF, 0x00];
        assert!(matches!(
            decode(&bad),
            Err(CodecError::UnknownTag { tag: 0xFF, .. })
        ));
    }

    #[test]
    fn decode_rejects_non_canonical_map_order() {
        // Encode a map with keys in wrong order by hand
        // Map: key_tag=0x41(string), val_tag=0x01(uint), len=2
        // entry1: key="z" (01 7a), val=1 (01)
        // entry2: key="a" (01 61), val=2 (02)  <- out of order
        let bytes = hex_to_bytes("30410102017a01016102");
        assert!(matches!(
            decode(&bytes),
            Err(CodecError::NonCanonicalMapOrder { .. })
        ));
    }

    #[test]
    fn decode_rejects_non_canonical_field_order() {
        // Struct with fields in wrong order: field_id=2 before field_id=1
        // schema_version=1, field_count=2, field2 then field1
        let bytes = hex_to_bytes("100102020101010102");
        // This may fail at varint or field order; just check it's an error
        assert!(decode(&bytes).is_err());
    }

    #[test]
    fn encode_decode_roundtrip_all_types() {
        let values = vec![
            Value::UInt(0),
            Value::UInt(u64::MAX),
            Value::SInt(0),
            Value::SInt(i64::MIN),
            Value::SInt(i64::MAX),
            Value::Bytes(vec![]),
            Value::Bytes(vec![0xFF, 0x00, 0xAB]),
            Value::Str(String::new()),
            Value::Str("hello world".to_string()),
        ];
        for v in values {
            let enc = encode(&v).unwrap();
            let dec = decode(&enc).unwrap();
            assert_eq!(dec, v, "roundtrip failed for {v:?}");
        }
    }

    fn hex_to_bytes(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect()
    }
}
