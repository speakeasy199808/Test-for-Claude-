//! Benchmarks for k0 entropy pool (P0-017).
//!
//! Measures throughput of deterministic random number generation
//! and pool operations.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use k0::entropy::EntropyPool;

fn make_pool() -> EntropyPool {
    EntropyPool::new([0x42u8; 32])
}

fn bench_next_u64(c: &mut Criterion) {
    c.bench_function("entropy_next_u64", |b| {
        let mut pool = make_pool();
        b.iter(|| {
            black_box(pool.next_u64());
        });
    });
}

fn bench_next_u32(c: &mut Criterion) {
    c.bench_function("entropy_next_u32", |b| {
        let mut pool = make_pool();
        b.iter(|| {
            black_box(pool.next_u32());
        });
    });
}

fn bench_next_bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("entropy_next_bytes");

    for size in [8, 32, 64, 128, 256].iter() {
        let mut pool = make_pool();
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &sz| {
            b.iter(|| {
                black_box(pool.next_bytes(sz).unwrap());
            });
        });
    }

    group.finish();
}

fn bench_fork(c: &mut Criterion) {
    c.bench_function("entropy_fork", |b| {
        let mut pool = make_pool();
        b.iter(|| {
            black_box(pool.fork());
        });
    });
}

fn bench_from_seed_bytes(c: &mut Criterion) {
    let seed = b"lyra deterministic entropy seed material for benchmarking";
    c.bench_function("entropy_from_seed_bytes", |b| {
        b.iter(|| {
            black_box(EntropyPool::from_seed_bytes(black_box(seed)));
        });
    });
}

criterion_group!(
    benches,
    bench_next_u64,
    bench_next_u32,
    bench_next_bytes,
    bench_fork,
    bench_from_seed_bytes
);
criterion_main!(benches);
