//! Codec error type for LyraCodec encode/decode operations.

/// Errors that can arise during LyraCodec encoding or decoding.
#[derive(Debug, thiserror::Error)]
pub enum CodecError {
    /// Input buffer is too short to contain a complete value.
    #[error("unexpected end of input at offset {offset}")]
    UnexpectedEof {
        /// Byte offset at which the buffer was exhausted.
        offset: usize,
    },

    /// A type tag byte was not recognised by this codec version.
    #[error("unknown type tag 0x{tag:02x} at offset {offset}")]
    UnknownTag {
        /// The unrecognised tag byte.
        tag: u8,
        /// Byte offset of the tag.
        offset: usize,
    },

    /// A varint encoding is non-canonical (e.g. has unnecessary continuation bytes).
    #[error("non-canonical varint encoding at offset {offset}")]
    NonCanonicalVarint {
        /// Byte offset of the varint.
        offset: usize,
    },

    /// A varint value overflows the target integer type.
    #[error("varint overflow at offset {offset}")]
    VarintOverflow {
        /// Byte offset of the varint.
        offset: usize,
    },

    /// A UTF-8 string payload is not valid UTF-8.
    #[error("invalid UTF-8 in string payload at offset {offset}: {source}")]
    InvalidUtf8 {
        /// Byte offset of the string payload.
        offset: usize,
        /// The underlying UTF-8 decode error.
        source: std::string::FromUtf8Error,
    },

    /// Map entries are not in canonical (lexicographic key-byte) order.
    #[error("map entries are not in canonical key order at offset {offset}")]
    NonCanonicalMapOrder {
        /// Byte offset of the out-of-order entry.
        offset: usize,
    },

    /// Struct fields are not in canonical ascending field-id order.
    #[error("struct fields are not in canonical ascending field-id order at offset {offset}")]
    NonCanonicalFieldOrder {
        /// Byte offset of the out-of-order field.
        offset: usize,
    },

    /// A struct contains a duplicate field id.
    #[error("duplicate field id {field_id} in struct at offset {offset}")]
    DuplicateFieldId {
        /// The duplicated field id.
        field_id: u32,
        /// Byte offset of the duplicate.
        offset: usize,
    },

    /// The element type tag in a vector or map does not match the actual value tag.
    #[error(
        "type tag mismatch: declared 0x{declared:02x}, found 0x{found:02x} at offset {offset}"
    )]
    TagMismatch {
        /// The type tag declared in the container header.
        declared: u8,
        /// The type tag found on the actual element.
        found: u8,
        /// Byte offset of the mismatch.
        offset: usize,
    },
}
