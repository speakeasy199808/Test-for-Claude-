//! Benchmarks for k0 digest algorithms (P0-017).
//!
//! Measures throughput of SHA-3-256 (primary) and BLAKE3 (secondary)
//! across various input sizes.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use k0::digest;

fn bench_sha3_256(c: &mut Criterion) {
    let mut group = c.benchmark_group("sha3_256");

    for size in [32, 256, 1024, 4096, 16384, 65536].iter() {
        let input = vec![0xABu8; *size];
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, data| {
            b.iter(|| digest::sha3_256(black_box(data)));
        });
    }

    group.finish();
}

fn bench_blake3(c: &mut Criterion) {
    let mut group = c.benchmark_group("blake3");

    for size in [32, 256, 1024, 4096, 16384, 65536].iter() {
        let input = vec![0xABu8; *size];
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, data| {
            b.iter(|| digest::blake3(black_box(data)));
        });
    }

    group.finish();
}

fn bench_digest_routing(c: &mut Criterion) {
    let mut group = c.benchmark_group("digest_routing");
    let input = vec![0xABu8; 1024];

    group.throughput(Throughput::Bytes(1024));

    group.bench_function("sha3_256_via_routing", |b| {
        b.iter(|| digest::digest(digest::DigestAlgorithm::Sha3_256, black_box(&input)));
    });

    group.bench_function("blake3_via_routing", |b| {
        b.iter(|| digest::digest(digest::DigestAlgorithm::Blake3, black_box(&input)));
    });

    group.finish();
}

criterion_group!(benches, bench_sha3_256, bench_blake3, bench_digest_routing);
criterion_main!(benches);
