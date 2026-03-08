//! LEB128 varint encoding and decoding for LyraCodec.
//!
//! # Unsigned LEB128
//! Each byte contributes 7 bits of the value. The high bit is a continuation
//! flag: 1 means more bytes follow, 0 means this is the last byte.
//! The encoding is canonical: no unnecessary continuation bytes are permitted.
//!
//! # Signed Zigzag + LEB128
//! Signed integers are first mapped through zigzag encoding
//! (`(n << 1) ^ (n >> 63)`) to produce a non-negative integer, then encoded
//! as unsigned LEB128. This keeps small negative numbers compact.

use crate::codec::error::CodecError;

/// Encode an unsigned 64-bit integer as LEB128 into `out`.
pub fn encode_u64(mut value: u64, out: &mut Vec<u8>) {
    loop {
        let byte = (value & 0x7F) as u8;
        value >>= 7;
        if value == 0 {
            out.push(byte); // final byte: continuation bit clear
            break;
        } else {
            out.push(byte | 0x80); // more bytes follow
        }
    }
}

/// Decode an unsigned LEB128 integer from `input` starting at `offset`.
///
/// Returns `(value, bytes_consumed)` on success.
pub fn decode_u64(input: &[u8], offset: usize) -> Result<(u64, usize), CodecError> {
    let mut result: u64 = 0;
    let mut shift = 0u32;
    let mut pos = offset;

    loop {
        if pos >= input.len() {
            return Err(CodecError::UnexpectedEof { offset: pos });
        }
        let byte = input[pos];
        pos += 1;

        // Each group is 7 bits; max 10 groups for u64 (70 bits > 64)
        if shift >= 63 {
            // Last allowed byte: for u64 only bits 0 of the 10th byte are valid
            if shift == 63 && (byte & 0xFE) != 0 {
                return Err(CodecError::VarintOverflow { offset: pos - 1 });
            }
            if shift > 63 {
                return Err(CodecError::VarintOverflow { offset: pos - 1 });
            }
        }

        result |= ((byte & 0x7F) as u64) << shift;
        shift += 7;

        if byte & 0x80 == 0 {
            // Canonical check: the last byte must not be zero unless it's the
            // first (and only) byte — i.e. no trailing zero continuation bytes.
            if byte == 0 && shift > 7 {
                return Err(CodecError::NonCanonicalVarint { offset: pos - 1 });
            }
            return Ok((result, pos - offset));
        }
    }
}

/// Encode a signed 64-bit integer using zigzag + LEB128 into `out`.
pub fn encode_i64(value: i64, out: &mut Vec<u8>) {
    let zigzag = ((value << 1) ^ (value >> 63)) as u64;
    encode_u64(zigzag, out);
}

/// Decode a zigzag + LEB128 signed integer from `input` starting at `offset`.
///
/// Returns `(value, bytes_consumed)` on success.
pub fn decode_i64(input: &[u8], offset: usize) -> Result<(i64, usize), CodecError> {
    let (zigzag, consumed) = decode_u64(input, offset)?;
    let value = ((zigzag >> 1) as i64) ^ -((zigzag & 1) as i64);
    Ok((value, consumed))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode_u64_vec(v: u64) -> Vec<u8> {
        let mut out = Vec::new();
        encode_u64(v, &mut out);
        out
    }

    fn encode_i64_vec(v: i64) -> Vec<u8> {
        let mut out = Vec::new();
        encode_i64(v, &mut out);
        out
    }

    #[test]
    fn u64_zero_encodes_to_single_zero_byte() {
        assert_eq!(encode_u64_vec(0), vec![0x00]);
    }

    #[test]
    fn u64_one_encodes_to_single_one_byte() {
        assert_eq!(encode_u64_vec(1), vec![0x01]);
    }

    #[test]
    fn u64_127_encodes_to_single_byte() {
        assert_eq!(encode_u64_vec(127), vec![0x7F]);
    }

    #[test]
    fn u64_128_encodes_to_two_bytes() {
        // 128 = 0b10000000 → LEB128: 0x80 0x01
        assert_eq!(encode_u64_vec(128), vec![0x80, 0x01]);
    }

    #[test]
    fn u64_300_encodes_correctly() {
        // 300 = 0b100101100 → LEB128: 0xAC 0x02
        assert_eq!(encode_u64_vec(300), vec![0xAC, 0x02]);
    }

    #[test]
    fn u64_roundtrip_zero() {
        let enc = encode_u64_vec(0);
        let (v, n) = decode_u64(&enc, 0).unwrap();
        assert_eq!(v, 0);
        assert_eq!(n, 1);
    }

    #[test]
    fn u64_roundtrip_max() {
        let enc = encode_u64_vec(u64::MAX);
        let (v, _) = decode_u64(&enc, 0).unwrap();
        assert_eq!(v, u64::MAX);
    }

    #[test]
    fn u64_roundtrip_various() {
        for val in [
            0u64,
            1,
            63,
            64,
            127,
            128,
            255,
            256,
            16383,
            16384,
            2097151,
            u32::MAX as u64,
            u64::MAX,
        ] {
            let enc = encode_u64_vec(val);
            let (decoded, consumed) = decode_u64(&enc, 0).unwrap();
            assert_eq!(decoded, val, "roundtrip failed for {val}");
            assert_eq!(consumed, enc.len(), "consumed wrong for {val}");
        }
    }

    #[test]
    fn i64_zero_encodes_to_zero() {
        assert_eq!(encode_i64_vec(0), vec![0x00]);
    }

    #[test]
    fn i64_minus_one_encodes_to_one() {
        // zigzag(-1) = 1
        assert_eq!(encode_i64_vec(-1), vec![0x01]);
    }

    #[test]
    fn i64_one_encodes_to_two() {
        // zigzag(1) = 2
        assert_eq!(encode_i64_vec(1), vec![0x02]);
    }

    #[test]
    fn i64_roundtrip_various() {
        for val in [
            0i64,
            1,
            -1,
            63,
            -64,
            127,
            -128,
            i32::MIN as i64,
            i32::MAX as i64,
            i64::MIN,
            i64::MAX,
        ] {
            let enc = encode_i64_vec(val);
            let (decoded, consumed) = decode_i64(&enc, 0).unwrap();
            assert_eq!(decoded, val, "roundtrip failed for {val}");
            assert_eq!(consumed, enc.len(), "consumed wrong for {val}");
        }
    }

    #[test]
    fn decode_u64_rejects_empty_input() {
        assert!(matches!(
            decode_u64(&[], 0),
            Err(CodecError::UnexpectedEof { offset: 0 })
        ));
    }

    #[test]
    fn decode_u64_rejects_non_canonical_trailing_zero() {
        // 0x80 0x00 is non-canonical for 0 (should just be 0x00)
        assert!(matches!(
            decode_u64(&[0x80, 0x00], 0),
            Err(CodecError::NonCanonicalVarint { .. })
        ));
    }

    #[test]
    fn decode_u64_with_offset() {
        // prefix byte 0xFF, then varint 128 = [0x80, 0x01]
        let buf = vec![0xFF, 0x80, 0x01];
        let (v, n) = decode_u64(&buf, 1).unwrap();
        assert_eq!(v, 128);
        assert_eq!(n, 2);
    }
}
