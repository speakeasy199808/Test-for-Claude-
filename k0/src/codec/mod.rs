//! Canonical LyraCodec encoder/decoder (P0-007).
//!
//! Implements the LyraCodec binary encoding specification from P0-006
//! (`interfaces/specs/lyracodec.md`). All encoding is deterministic:
//! identical inputs always produce identical byte sequences.
//!
//! # Encoding Rules
//! - All integers use unsigned LEB128 (or zigzag + LEB128 for signed).
//! - Struct fields are emitted in ascending `field_id` order.
//! - Map entries are sorted by lexicographic ordering of the canonical
//!   encoded key payload bytes.
//! - Floating-point types are forbidden in canonical form.
//! - Every top-level payload carries a schema version varint header.
//!
//! # Module Layout
//! - [`types`]   — type tags and the [`Value`] enum
//! - [`varint`]  — LEB128 unsigned and zigzag-signed varint codec
//! - [`encoder`] — encode [`Value`] to canonical bytes
//! - [`decoder`] — decode canonical bytes to [`Value`]
//! - [`error`]   — [`CodecError`] type

pub mod decoder;
pub mod encoder;
pub mod error;
pub mod types;
pub mod varint;

pub use decoder::decode;
pub use encoder::encode;
pub use error::CodecError;
pub use types::{
    StructField, Value, TAG_BYTES, TAG_MAP, TAG_STRING, TAG_STRUCT, TAG_VARINT_S, TAG_VARINT_U,
    TAG_VECTOR,
};
