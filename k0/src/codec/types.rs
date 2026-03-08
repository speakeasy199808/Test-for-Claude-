//! LyraCodec type tags and the canonical [`Value`] enum.
//!
//! Type tags are single-byte discriminants that prefix every encoded value.
//! The [`Value`] enum is the in-memory representation of any LyraCodec value.

/// Type tag: unsigned varint (LEB128).
pub const TAG_VARINT_U: u8 = 0x01;
/// Type tag: signed varint (zigzag + LEB128).
pub const TAG_VARINT_S: u8 = 0x02;
/// Type tag: struct (schema-versioned, field-id-ordered).
pub const TAG_STRUCT: u8 = 0x10;
/// Type tag: vector (homogeneous, order-preserving).
pub const TAG_VECTOR: u8 = 0x20;
/// Type tag: map (key-sorted, homogeneous key and value types).
pub const TAG_MAP: u8 = 0x30;
/// Type tag: raw byte sequence.
pub const TAG_BYTES: u8 = 0x40;
/// Type tag: UTF-8 string.
pub const TAG_STRING: u8 = 0x41;

/// A single field within a [`Value::Struct`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructField {
    /// Canonical field identifier. Fields are encoded in ascending order.
    pub field_id: u32,
    /// The value of this field.
    pub value: Value,
}

/// A canonical LyraCodec value.
///
/// Floating-point variants are intentionally absent — they are forbidden
/// in canonical form per the LyraCodec specification (P0-006).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    /// Unsigned 64-bit integer, encoded as LEB128.
    UInt(u64),
    /// Signed 64-bit integer, encoded as zigzag + LEB128.
    SInt(i64),
    /// Raw byte sequence, length-prefixed.
    Bytes(Vec<u8>),
    /// UTF-8 string, length-prefixed.
    Str(String),
    /// Schema-versioned struct with canonically ordered fields.
    Struct {
        /// Schema version for this struct payload.
        schema_version: u32,
        /// Fields, which will be sorted by ascending `field_id` on encode.
        fields: Vec<StructField>,
    },
    /// Homogeneous vector; element order is preserved exactly.
    Vector {
        /// Type tag of every element in this vector.
        elem_tag: u8,
        /// The elements of this vector.
        elements: Vec<Value>,
    },
    /// Key-sorted homogeneous map.
    Map {
        /// Type tag of every key in this map.
        key_tag: u8,
        /// Type tag of every value in this map.
        value_tag: u8,
        /// Entries; will be sorted by canonical encoded key bytes on encode.
        entries: Vec<(Value, Value)>,
    },
}

impl Value {
    /// Return the type tag byte for this value.
    pub fn tag(&self) -> u8 {
        match self {
            Value::UInt(_) => TAG_VARINT_U,
            Value::SInt(_) => TAG_VARINT_S,
            Value::Bytes(_) => TAG_BYTES,
            Value::Str(_) => TAG_STRING,
            Value::Struct { .. } => TAG_STRUCT,
            Value::Vector { .. } => TAG_VECTOR,
            Value::Map { .. } => TAG_MAP,
        }
    }
}
