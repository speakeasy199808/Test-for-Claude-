# Acceptance — P0-017 Benchmarking Harness

## Acceptance Criteria
1. Criterion 0.5 is declared as a workspace dependency with `html_reports` feature.
2. k0 `Cargo.toml` includes criterion as a dev-dependency with `[[bench]]` entries.
3. `k0/benches/digest_bench.rs` benchmarks SHA-3-256 and BLAKE3 across multiple input sizes (32B–64KB) with throughput metrics.
4. `k0/benches/codec_bench.rs` benchmarks encode and decode for all value types (uint, string, bytes, struct, vector, map) plus roundtrip.
5. `k0/benches/time_bench.rs` benchmarks tick, advance, merge, and VirtualTime::next operations.
6. `k0/benches/entropy_bench.rs` benchmarks next_u64, next_u32, next_bytes (multiple sizes), fork, and from_seed_bytes.
7. All benchmarks compile successfully with `cargo bench --no-run -p k0`.
8. Benchmarks use `black_box` to prevent dead-code elimination.
9. Throughput-based benchmarks report bytes/second metrics.

## Verification Method
- `cargo bench --no-run -p k0` compiles without errors
- Benchmark structure review against declared scope

## Evidence Required
- Successful compilation output
- `artifacts/benchmark-harness-spec.md`
