//! Canonical LyraCodec encoder.
//!
//! Encodes a [`Value`] into a deterministic byte sequence following the
//! LyraCodec specification (P0-006). Struct fields are sorted by ascending
//! field id; map entries are sorted by lexicographic ordering of the
//! canonical encoded key payload bytes.

use crate::codec::error::CodecError;
use crate::codec::types::*;
use crate::codec::varint;

/// Encode a [`Value`] into canonical LyraCodec bytes.
///
/// The output is fully deterministic: identical values always produce
/// identical byte sequences.
pub fn encode(value: &Value) -> Result<Vec<u8>, CodecError> {
    let mut out = Vec::new();
    encode_tagged(value, &mut out)?;
    Ok(out)
}

/// Encode a value with its type tag prefix.
fn encode_tagged(value: &Value, out: &mut Vec<u8>) -> Result<(), CodecError> {
    out.push(value.tag());
    encode_payload(value, out)
}

/// Encode only the payload (no tag prefix) of a value.
fn encode_payload(value: &Value, out: &mut Vec<u8>) -> Result<(), CodecError> {
    match value {
        Value::UInt(v) => {
            varint::encode_u64(*v, out);
        }
        Value::SInt(v) => {
            varint::encode_i64(*v, out);
        }
        Value::Bytes(data) => {
            varint::encode_u64(data.len() as u64, out);
            out.extend_from_slice(data);
        }
        Value::Str(s) => {
            let bytes = s.as_bytes();
            varint::encode_u64(bytes.len() as u64, out);
            out.extend_from_slice(bytes);
        }
        Value::Struct {
            schema_version,
            fields,
        } => {
            encode_struct(*schema_version, fields, out)?;
        }
        Value::Vector { elem_tag, elements } => {
            encode_vector(*elem_tag, elements, out)?;
        }
        Value::Map {
            key_tag,
            value_tag,
            entries,
        } => {
            encode_map(*key_tag, *value_tag, entries, out)?;
        }
    }
    Ok(())
}

/// Encode a struct payload.
///
/// Format: `<schema_version_varint> <field_count_varint> <field_entries...>`
/// Fields are sorted by ascending `field_id`.
fn encode_struct(
    schema_version: u32,
    fields: &[StructField],
    out: &mut Vec<u8>,
) -> Result<(), CodecError> {
    // Sort fields by ascending field_id (canonical ordering)
    let mut sorted: Vec<&StructField> = fields.iter().collect();
    sorted.sort_by_key(|f| f.field_id);

    varint::encode_u64(schema_version as u64, out);
    varint::encode_u64(sorted.len() as u64, out);

    for field in sorted {
        varint::encode_u64(field.field_id as u64, out);
        encode_tagged(&field.value, out)?;
    }
    Ok(())
}

/// Encode a vector payload.
///
/// Format: `<elem_type_tag> <length_varint> <elem_payload_0> ... <elem_payload_n>`
fn encode_vector(elem_tag: u8, elements: &[Value], out: &mut Vec<u8>) -> Result<(), CodecError> {
    out.push(elem_tag);
    varint::encode_u64(elements.len() as u64, out);

    for elem in elements {
        encode_payload(elem, out)?;
    }
    Ok(())
}

/// Encode a map payload.
///
/// Format: `<key_type_tag> <value_type_tag> <length_varint> <sorted_entries...>`
/// Entries are sorted by lexicographic ordering of canonical encoded key payload bytes.
fn encode_map(
    key_tag: u8,
    value_tag: u8,
    entries: &[(Value, Value)],
    out: &mut Vec<u8>,
) -> Result<(), CodecError> {
    // Compute canonical key bytes for sorting
    let mut keyed: Vec<(Vec<u8>, &Value, &Value)> = Vec::with_capacity(entries.len());
    for (k, v) in entries {
        let mut key_bytes = Vec::new();
        encode_payload(k, &mut key_bytes)?;
        keyed.push((key_bytes, k, v));
    }
    keyed.sort_by(|a, b| a.0.cmp(&b.0));

    out.push(key_tag);
    out.push(value_tag);
    varint::encode_u64(keyed.len() as u64, out);

    for (key_bytes, _k, v) in &keyed {
        out.extend_from_slice(key_bytes);
        encode_payload(v, out)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    #[test]
    fn encode_uint_zero() {
        let v = Value::UInt(0);
        let enc = encode(&v).unwrap();
        assert_eq!(hex(&enc), "0100"); // tag 0x01, varint 0x00
    }

    #[test]
    fn encode_uint_one() {
        let v = Value::UInt(1);
        let enc = encode(&v).unwrap();
        assert_eq!(hex(&enc), "0101");
    }

    #[test]
    fn encode_uint_127() {
        let v = Value::UInt(127);
        let enc = encode(&v).unwrap();
        assert_eq!(hex(&enc), "017f");
    }

    #[test]
    fn encode_uint_128() {
        let v = Value::UInt(128);
        let enc = encode(&v).unwrap();
        assert_eq!(hex(&enc), "018001");
    }

    #[test]
    fn encode_sint_zero() {
        let v = Value::SInt(0);
        let enc = encode(&v).unwrap();
        assert_eq!(hex(&enc), "0200"); // tag 0x02, zigzag(0)=0
    }

    #[test]
    fn encode_sint_minus_one() {
        let v = Value::SInt(-1);
        let enc = encode(&v).unwrap();
        assert_eq!(hex(&enc), "0201"); // zigzag(-1)=1
    }

    #[test]
    fn encode_sint_one() {
        let v = Value::SInt(1);
        let enc = encode(&v).unwrap();
        assert_eq!(hex(&enc), "0202"); // zigzag(1)=2
    }

    #[test]
    fn encode_bytes() {
        let v = Value::Bytes(vec![0xDE, 0xAD]);
        let enc = encode(&v).unwrap();
        assert_eq!(hex(&enc), "4002dead"); // tag 0x40, len=2, payload
    }

    #[test]
    fn encode_string() {
        let v = Value::Str("lyra".to_string());
        let enc = encode(&v).unwrap();
        assert_eq!(hex(&enc), "41046c797261"); // tag 0x41, len=4, "lyra"
    }

    #[test]
    fn encode_struct_matches_fixture() {
        // From P0-006 struct_example.json:
        // schema_id: "example.person", schema_version: 1
        // field_ids: id=1, name=2
        // value: { id: 7, name: "lyra" }
        // expected: "1001020101070241046c797261"
        let v = Value::Struct {
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
        let enc = encode(&v).unwrap();
        assert_eq!(hex(&enc), "1001020101070241046c797261");
    }

    #[test]
    fn encode_struct_sorts_fields() {
        // Fields given out of order; encoder must sort by field_id
        let v = Value::Struct {
            schema_version: 1,
            fields: vec![
                StructField {
                    field_id: 2,
                    value: Value::Str("lyra".to_string()),
                },
                StructField {
                    field_id: 1,
                    value: Value::UInt(7),
                },
            ],
        };
        let enc = encode(&v).unwrap();
        // Same output as above — canonical ordering
        assert_eq!(hex(&enc), "1001020101070241046c797261");
    }

    #[test]
    fn encode_vector_uint() {
        let v = Value::Vector {
            elem_tag: TAG_VARINT_U,
            elements: vec![Value::UInt(1), Value::UInt(2), Value::UInt(3)],
        };
        let enc = encode(&v).unwrap();
        // tag 0x20, elem_tag 0x01, len=3, payloads: 01 02 03
        assert_eq!(hex(&enc), "200103010203");
    }

    #[test]
    fn encode_map_sorts_by_key_bytes() {
        // Map with string keys, given out of order
        let v = Value::Map {
            key_tag: TAG_STRING,
            value_tag: TAG_VARINT_U,
            entries: vec![
                (Value::Str("z".to_string()), Value::UInt(2)),
                (Value::Str("a".to_string()), Value::UInt(1)),
            ],
        };
        let enc = encode(&v).unwrap();
        // tag 0x30, key_tag 0x41, val_tag 0x01, len=2
        // sorted: "a" (01 61) -> 01, "z" (01 7a) -> 02
        assert_eq!(hex(&enc), "30410102016101017a02");
    }

    #[test]
    fn encode_is_deterministic() {
        let v = Value::Struct {
            schema_version: 1,
            fields: vec![
                StructField {
                    field_id: 1,
                    value: Value::UInt(42),
                },
                StructField {
                    field_id: 2,
                    value: Value::Str("test".to_string()),
                },
            ],
        };
        let enc1 = encode(&v).unwrap();
        let enc2 = encode(&v).unwrap();
        assert_eq!(enc1, enc2, "encoding must be deterministic");
    }
}
