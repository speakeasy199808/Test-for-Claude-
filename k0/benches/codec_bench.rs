//! Benchmarks for k0 canonical LyraCodec encoder/decoder (P0-017).
//!
//! Measures encode and decode throughput for various value types
//! and composite structures.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use k0::codec::{decoder, encoder, types::*};

fn make_uint_value() -> Value {
    Value::UInt(123_456_789)
}

fn make_string_value() -> Value {
    Value::Str("The quick brown fox jumps over the lazy dog".to_string())
}

fn make_bytes_value() -> Value {
    Value::Bytes(
        [0xDE, 0xAD, 0xBE, 0xEF]
            .iter()
            .copied()
            .cycle()
            .take(256)
            .collect(),
    )
}

fn make_struct_value() -> Value {
    Value::Struct {
        schema_version: 1,
        fields: vec![
            StructField {
                field_id: 1,
                value: Value::Str("lyra".to_string()),
            },
            StructField {
                field_id: 2,
                value: Value::UInt(42),
            },
            StructField {
                field_id: 3,
                value: Value::Bytes(vec![0xFF; 32]),
            },
            StructField {
                field_id: 10,
                value: Value::SInt(-100),
            },
        ],
    }
}

fn make_vector_value() -> Value {
    Value::Vector {
        elem_tag: TAG_VARINT_U,
        elements: (0..100).map(Value::UInt).collect(),
    }
}

fn make_map_value() -> Value {
    Value::Map {
        key_tag: TAG_STRING,
        value_tag: TAG_VARINT_U,
        entries: (0..50)
            .map(|i| (Value::Str(format!("key_{i:04}")), Value::UInt(i)))
            .collect(),
    }
}

fn bench_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("codec_encode");

    let uint_val = make_uint_value();
    group.bench_function("uint", |b| {
        b.iter(|| encoder::encode(black_box(&uint_val)));
    });

    let string_val = make_string_value();
    group.bench_function("string", |b| {
        b.iter(|| encoder::encode(black_box(&string_val)));
    });

    let bytes_val = make_bytes_value();
    group.bench_function("bytes_256", |b| {
        b.iter(|| encoder::encode(black_box(&bytes_val)));
    });

    let struct_val = make_struct_value();
    group.bench_function("struct_4fields", |b| {
        b.iter(|| encoder::encode(black_box(&struct_val)));
    });

    let vector_val = make_vector_value();
    group.bench_function("vector_100_uints", |b| {
        b.iter(|| encoder::encode(black_box(&vector_val)));
    });

    let map_val = make_map_value();
    group.bench_function("map_50_entries", |b| {
        b.iter(|| encoder::encode(black_box(&map_val)));
    });

    group.finish();
}

fn bench_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("codec_decode");

    let uint_bytes = encoder::encode(&make_uint_value()).unwrap();
    group.bench_function("uint", |b| {
        b.iter(|| decoder::decode(black_box(&uint_bytes)));
    });

    let string_bytes = encoder::encode(&make_string_value()).unwrap();
    group.bench_function("string", |b| {
        b.iter(|| decoder::decode(black_box(&string_bytes)));
    });

    let struct_bytes = encoder::encode(&make_struct_value()).unwrap();
    group.bench_function("struct_4fields", |b| {
        b.iter(|| decoder::decode(black_box(&struct_bytes)));
    });

    let vector_bytes = encoder::encode(&make_vector_value()).unwrap();
    group.bench_function("vector_100_uints", |b| {
        b.iter(|| decoder::decode(black_box(&vector_bytes)));
    });

    let map_bytes = encoder::encode(&make_map_value()).unwrap();
    group.bench_function("map_50_entries", |b| {
        b.iter(|| decoder::decode(black_box(&map_bytes)));
    });

    group.finish();
}

fn bench_roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("codec_roundtrip");

    let struct_val = make_struct_value();
    group.bench_function("struct_encode_decode", |b| {
        b.iter(|| {
            let encoded = encoder::encode(black_box(&struct_val)).unwrap();
            decoder::decode(black_box(&encoded)).unwrap()
        });
    });

    let map_val = make_map_value();
    group.bench_function("map_encode_decode", |b| {
        b.iter(|| {
            let encoded = encoder::encode(black_box(&map_val)).unwrap();
            decoder::decode(black_box(&encoded)).unwrap()
        });
    });

    group.finish();
}

criterion_group!(benches, bench_encode, bench_decode, bench_roundtrip);
criterion_main!(benches);
