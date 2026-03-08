//! Benchmarks for k0 virtual time system (P0-017).
//!
//! Measures tick, advance, merge, and reset_to operations.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use k0::time::{VirtualClock, VirtualTime};

fn bench_tick(c: &mut Criterion) {
    c.bench_function("virtual_clock_tick", |b| {
        let mut clock = VirtualClock::new();
        b.iter(|| {
            black_box(clock.tick());
        });
    });
}

fn bench_advance(c: &mut Criterion) {
    let mut group = c.benchmark_group("virtual_clock_advance");

    group.bench_function("advance_1", |b| {
        let mut clock = VirtualClock::new();
        b.iter(|| {
            clock.advance(black_box(1));
        });
    });

    group.bench_function("advance_1000", |b| {
        let mut clock = VirtualClock::new();
        b.iter(|| {
            clock.advance(black_box(1000));
        });
    });

    group.finish();
}

fn bench_merge(c: &mut Criterion) {
    c.bench_function("virtual_clock_merge", |b| {
        let mut clock_a = VirtualClock::new();
        clock_a.advance(500);
        let mut clock_b = VirtualClock::new();
        clock_b.advance(1000);
        b.iter(|| {
            let mut c = clock_a.clone();
            c.merge(black_box(&clock_b));
            black_box(c);
        });
    });
}

fn bench_virtual_time_next(c: &mut Criterion) {
    c.bench_function("virtual_time_next", |b| {
        let t = VirtualTime::new(1_000_000);
        b.iter(|| {
            black_box(black_box(t).next());
        });
    });
}

criterion_group!(
    benches,
    bench_tick,
    bench_advance,
    bench_merge,
    bench_virtual_time_next
);
criterion_main!(benches);
